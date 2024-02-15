import {Alert, Button, Descriptions, Input, Modal, Progress, Space, Spin} from "antd";
import prettyBytes from "pretty-bytes";
import {UploadOutlined} from "@ant-design/icons";
import {useCallback, useEffect, useState} from "react";
import {open} from "@tauri-apps/api/dialog";
import {invoke} from "@tauri-apps/api";
import {FileMetadata, LogErrorMessage, LogSuccessMessage} from "./rust_type_definitions.ts";
import {downloadDir, join} from "@tauri-apps/api/path";
import {useAsyncEffect} from "ahooks";
import {useLog} from "./RRTPLog.tsx";
import {atom, useRecoilState} from "recoil";


declare type FileSendingState = {
    type: undefined
    file?: undefined
} | {
    type: "waiting_for_remote",
    file: FileMetadata
} | {
    type: "sending_file",
    file: FileMetadata
    sentBytes: number
};

declare type FileReceivingState = {
    type: undefined,
    file?: undefined
} | {
    type: "incoming_file",
    file: FileMetadata
} | {
    type: "receiving_file",
    file: FileMetadata,
    receivedBytes: number
}
export const fileSendingState = atom<FileSendingState>({
    key: 'file_sending_state',
    default: {
        type: undefined
    }
});

export const fileReceivingState = atom<FileReceivingState>({
    key: 'file_receiving_state',
    default: {
        type: undefined
    }
});
export const FileManager = () => {

    const setLog = useLog();

    const [sendingState, setSendingState] = useRecoilState(fileSendingState);
    const [receiving_state, setReceivingState] = useRecoilState(fileReceivingState);

    const [selectedFileName, setSelectedFileName] = useState<string | undefined>(undefined)

    const [selectedUploadFolder, setSelectedUploadFolder] = useState<string | undefined>(undefined)

    useAsyncEffect(async () => {
        if (!selectedUploadFolder) {
            setSelectedUploadFolder(await downloadDir());
        }

    }, [selectedUploadFolder, setSelectedUploadFolder])

    useEffect(() => {
        if (receiving_state.type === "incoming_file") {
            setSelectedFileName(receiving_state.file.file_name);
        }
    }, [sendingState, setSelectedFileName]);


    const handleUploadDirectory = useCallback(async () => {
        const selected = await open({
            directory: true
        });
        if (typeof selected === 'string') {
            setSelectedUploadFolder(selected);
        }
    }, []);

    const getFileName = () => {
        return selectedFileName ?? ((receiving_state.type === "incoming_file") ? receiving_state.file.file_name : undefined);
    };

    const handleFileResponse = useCallback(async (response: boolean) => {
        if (receiving_state.type !== "incoming_file") return;

        let fileName = getFileName();
        if (!selectedUploadFolder || !fileName) {
            return;
        }
        invoke<LogSuccessMessage>("respond_to_file_info", {
            ready: response,
            file: await join(selectedUploadFolder, fileName)
        }).then((result) => {
            setReceivingState({type: "receiving_file", file: receiving_state.file, receivedBytes: 0});
            setLog(result);
        }).catch((err: LogErrorMessage) => {
            setLog(err);
        });
    }, [setLog, selectedUploadFolder, receiving_state.type, setReceivingState, receiving_state.file])


    return <>
        {sendingState.type === "sending_file" &&
            <Alert
                message="File Upload Progress"
                description={<Progress
                    percent={(sendingState.file.file_size_in_bytes / sendingState.sentBytes) * 100}/>}
                type="info"
            />
        }
        {sendingState.type === "waiting_for_remote" &&
            <Spin tip="Waiting For Remote To Start Uploading">
                <Alert
                    message="File Download Progress"
                    description={<Progress percent={0}/>}
                    type="info"
                />
            </Spin>}
        {receiving_state.type === "receiving_file" &&
            <Alert
                message="File Download Progress"
                description={<Progress
                    percent={(receiving_state.file.file_size_in_bytes / receiving_state.receivedBytes) * 100}/>}
                type="info"
            />
        }
        {(receiving_state.type === "incoming_file") &&
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
                        children: receiving_state.file.file_name
                    },
                        {
                            label: "MIME",
                            children: receiving_state.file.file_mime ?? "Unknown"
                        },
                        {
                            label: "Size",
                            children: prettyBytes(receiving_state.file.file_size_in_bytes)
                        }]}/>

                    <Button icon={<UploadOutlined/>} onClick={handleUploadDirectory}>Upload Directory</Button>

                    <Input addonBefore={selectedUploadFolder} onChange={(e) => setSelectedFileName(e.target.value)}
                           defaultValue={receiving_state.file.file_name}/>
                </Space>
            </Modal>}
    </>
};