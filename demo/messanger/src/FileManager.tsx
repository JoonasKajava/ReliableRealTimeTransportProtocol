import {Alert, Button, Descriptions, Input, Modal, Progress, Space, Spin} from "antd";
import prettyBytes from "pretty-bytes";
import {UploadOutlined} from "@ant-design/icons";
import {useCallback, useEffect, useState} from "react";
import {open} from "@tauri-apps/api/dialog";
import {invoke} from "@tauri-apps/api";
import {LogErrorMessage, LogSuccessMessage, NetworkFileInfo} from "./rust_type_definitions.ts";
import {downloadDir, join} from "@tauri-apps/api/path";
import {useAsyncEffect} from "ahooks";
import {useLog} from "./RRTPLog.tsx";
import {atom, useRecoilState} from "recoil";


declare type ManagerState = {
    type: undefined
} | {
    type: "incoming_file"
    file: NetworkFileInfo
} | {
    type: "waiting_for_file_data"
} | {
    type: "sending_file"
} | {
    type: "receiving_file"
}

export const fileManagerState = atom<ManagerState>({
    key: 'fileManager',
    default: {
        type: undefined
    }
});

export const FileManager = () => {

    const setLog = useLog();

    const [managerState, setManagerState] = useRecoilState(fileManagerState);

    const [selectedFileName, setSelectedFileName] = useState<string | undefined>(undefined)

    const [selectedUploadFolder, setSelectedUploadFolder] = useState<string | undefined>(undefined)

    useAsyncEffect(async () => {
        if (!selectedUploadFolder) {
            setSelectedUploadFolder(await downloadDir());
        }

    }, [selectedUploadFolder, setSelectedUploadFolder])

    useEffect(() => {
        if (managerState.type === "incoming_file") {
            setSelectedFileName(managerState.file.file_name);
        }
    }, [managerState, setSelectedFileName]);


    const handleUploadDirectory = useCallback(async () => {
        const selected = await open({
            directory: true
        });
        if (typeof selected === 'string') {
            setSelectedUploadFolder(selected);
        }
    }, []);

    const getFileName = () => {
        return selectedFileName ?? ((managerState.type === "incoming_file") ? managerState.file.file_name : undefined);
    };

    const handleFileResponse = useCallback(async (response: boolean) => {
        if (managerState.type !== "incoming_file") return;

        let fileName = getFileName();
        if (!selectedUploadFolder || !fileName) {
            return;
        }
        invoke<LogSuccessMessage>("respond_to_file_info", {
            ready: response,
            file: await join(selectedUploadFolder, fileName)
        }).then((result) => {
            setManagerState({type: "waiting_for_file_data"});
            setLog(result);
        }).catch((err: LogErrorMessage) => {
            setLog(err);
        });
    }, [setLog, selectedUploadFolder, setManagerState, managerState.type])


    return <>
        {managerState.type === "waiting_for_file_data" &&
            <Spin tip="Waiting For Remote To Start Uploading">
                <Alert
                    message="File Download Progress"
                    description={<Progress percent={0}/>}
                    type="info"
                />
            </Spin>}
        {managerState.type === "sending_file" &&
            <Alert
                message="File Download Progress"
                description={<Progress percent={0}/>}
                type="info"
            />
        }

        {managerState.type === "receiving_file" &&
            <Alert
                message="File Upload Progress"
                description={<Progress percent={0}/>}
                type="info"
            />
        }
        {(managerState.type === "incoming_file") &&
            <Modal title={"Incoming File"} open={true} maskClosable={false} closeIcon={false} cancelText={"Reject"}
                   okText={"Accept"}
                   okButtonProps={{disabled: !selectedUploadFolder}}
                   onCancel={() => handleFileResponse(false)}
                   onOk={() => handleFileResponse(true)}
                   destroyOnClose={true}
            >
                <Space direction={"vertical"}>
                    <Descriptions column={1} title={"Remote wants to send following file"} items={[{
                        label: "File Name",
                        children: managerState.file.file_name
                    },
                        {
                            label: "MIME",
                            children: managerState.file.file_mime ?? "Unknown"
                        },
                        {
                            label: "Size",
                            children: prettyBytes(managerState.file.file_size_in_bytes)
                        }]}/>

                    <Button icon={<UploadOutlined/>} onClick={handleUploadDirectory}>Upload Directory</Button>

                    <Input addonBefore={selectedUploadFolder} onChange={(e) => setSelectedFileName(e.target.value)}
                           defaultValue={managerState.file.file_name}/>
                </Space>
            </Modal>}
    </>
};