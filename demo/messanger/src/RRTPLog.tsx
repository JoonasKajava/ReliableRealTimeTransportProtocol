import {Button, Descriptions, Input, List, Modal, Space} from "antd";
import VirtualList from 'rc-virtual-list';
import {atom, useRecoilValue, useSetRecoilState} from "recoil";
import dayjs from "dayjs";
import {useCallback, useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";

import localizedFormat from "dayjs/plugin/localizedFormat";
import {LogErrorMessage, LogSuccessMessage, NetworkFileInfo} from "./rust_type_definitions.ts";
import prettyBytes from "pretty-bytes";
import {UploadOutlined} from "@ant-design/icons";
import {open} from '@tauri-apps/api/dialog';
import {useAsyncEffect} from "ahooks";
import {downloadDir, join} from "@tauri-apps/api/path";
import {invoke} from "@tauri-apps/api";

dayjs.extend(localizedFormat)
export const logState = atom<{ title: string, description: string, timestamp: dayjs.Dayjs }[]>({
    key: 'log',
    default: []
})


export const LogMessageTitleMap: Record<LogSuccessMessage['type'] | LogErrorMessage['type'], string> = {
    FileAccepted: "Remote Has Accepted Your File",
    FileRejected: "Remote Rejected Your File",
    FileResponseSent: "Sent File Response",
    FileInfoReceived: "Received File Info",
    ReceivedAcknowledgement: "Received Acknowledgment",
    UnknownMessage: "Unknown Event occurred",
    ConnectedToRemote: "Connected To Remote",
    ConnectionError: "Connection Error",
    FileInfoSent: "Sent File Info",
    FileSendError: "Error Sending File",
    LocalSocketBindFailed: "Binding Local Socket Failed",
    LocalSocketBindSuccess: "Binding Local Socket Successful",
    LocalSocketNotBound: "Local Socket Not Bound",
    MessageReceived: "Received Message",
    MessageSendError: "Sending Message Failed",
    MessageSent: "Sent Message"


}
export const RRTPLog = () => {

    const log = useRecoilValue(logState);

    const setLog = useLog();

    const [incomingFileInfo, setIncomingFileInfo] = useState<undefined | NetworkFileInfo>(undefined);

    const [selectedFileName, setSelectedFileName] = useState<string | undefined>(undefined)

    useEffect(() => {
        if (incomingFileInfo) {
            setSelectedFileName(incomingFileInfo.file_name)
        }
    }, [incomingFileInfo]);

    const [selectedUploadFolder, setSelectedUploadFolder] = useState<string | undefined>(undefined)

    useAsyncEffect(async () => {
        if (!selectedUploadFolder) {
            setSelectedUploadFolder(await downloadDir());
        }
    }, [selectedUploadFolder, setSelectedUploadFolder])

    useEffect(() => {
        const unlisten = listen<LogSuccessMessage>("log", (event) => {
            if (event.payload.type === "FileInfoReceived") {
                setIncomingFileInfo(event.payload.content)
            }
            setLog(event.payload);
        });
        return () => {
            unlisten.then((unlisten) => unlisten());
        }
    }, [useLog]);


    const handleUploadDirectory = useCallback(async () => {
        const selected = await open({
            directory: true
        });
        if (typeof selected === 'string') {
            setSelectedUploadFolder(selected);
        }
    }, []);

    const handleFileResponse = useCallback(async (response: boolean) => {
        if (!selectedUploadFolder || !selectedFileName) {
            return;
        }
        invoke<LogSuccessMessage>("respond_to_file_info", {
            ready: response,
            file: await join(selectedUploadFolder, selectedFileName)
        }).then((result) => {
            setIncomingFileInfo(undefined);
            setLog(result);
        }).catch((err: LogErrorMessage) => {
            setLog(err);
        });
    }, [setLog, selectedUploadFolder, selectedFileName])


    return <>
        {incomingFileInfo &&
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
                        children: incomingFileInfo.file_name
                    },
                        {
                            label: "MIME",
                            children: incomingFileInfo.file_mime ?? "Unknown"
                        },
                        {
                            label: "Size",
                            children: prettyBytes(incomingFileInfo.file_size_in_bytes)
                        }]}/>

                    <Button icon={<UploadOutlined/>} onClick={handleUploadDirectory}>Upload Directory</Button>

                    <Input addonBefore={selectedUploadFolder} defaultValue={incomingFileInfo.file_name}/>
                </Space>
            </Modal>}
        <List>
            <VirtualList data={log} height={400} itemHeight={50} itemKey={"timestamp"}>
                {(item) => <List.Item>
                    <List.Item.Meta title={item.title} description={item.description}/>
                    <div>{item.timestamp.format("LTS")}</div>
                </List.Item>}
            </VirtualList>
        </List>
    </>;
};

export function useLog() {
    const setLog = useSetRecoilState(logState);
    return (message: LogSuccessMessage | LogErrorMessage) => {
        setLog((prev) => [{
            timestamp: dayjs(),
            title: LogMessageTitleMap[message.type],
            description: JSON.stringify(message.content)
        }, ...prev]);
    }
}