import {List} from "antd";
import VirtualList from 'rc-virtual-list';
import {atom, useRecoilState, useRecoilValue, useSetRecoilState} from "recoil";
import dayjs from "dayjs";
import {useEffect} from "react";
import {listen} from "@tauri-apps/api/event";

import localizedFormat from "dayjs/plugin/localizedFormat";
import {LogErrorMessage, LogSuccessMessage} from "./rust_type_definitions.ts";
import {fileManagerState} from "./FileManager.tsx";

dayjs.extend(localizedFormat)
export const logState = atom<{ title: string, description: string, timestamp: dayjs.Dayjs }[]>({
    key: 'log',
    default: []
})


export const LogMessageTitleMap: Record<LogSuccessMessage['type'] | LogErrorMessage['type'], string> = {
    SendFrame: "Sent Frame",
    ReceivedFrame: "Received Frame",
    Error: "Unknown Error",
    FileDataReceived: "Received file data from remote",
    InvalidFileResponse: "Invalid File Response",
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

    const [fileManagerStateValue, setFileManagerState] = useRecoilState(fileManagerState);

    const setLog = useLog();

    useEffect(() => {
        const unlisten = listen<LogSuccessMessage>("log", (event) => {
            switch (event.payload.type) {
                case "FileInfoReceived":
                    setFileManagerState({
                        type: "incoming_file",
                        file: event.payload.content
                    })
                    break;
                case "FileRejected":
                    setFileManagerState({type: undefined})
                    break;
                case "FileAccepted":
                    setFileManagerState({type: "sending_file", file: event.payload.content, sentBytes: 0})
                    break;
                case "ReceivedFrame":
                    if (fileManagerStateValue.type === "receiving_file") {
                        setFileManagerState({
                            type: "receiving_file",
                            file: fileManagerStateValue.file,
                            receivedBytes: fileManagerStateValue.receivedBytes + event.payload.content.len
                        })
                    }
                    break;
                case "SendFrame":
                    if (fileManagerStateValue.type === "sending_file") {
                        setFileManagerState({
                            type: "sending_file",
                            file: fileManagerStateValue.file,
                            sentBytes: fileManagerStateValue.sentBytes + event.payload.content.len
                        })
                    }
                    break;
            }
            setLog(event.payload);
        });
        return () => {
            unlisten.then((unlisten) => unlisten());
        }
    }, [useLog]);


    return <>
        <List>
            <VirtualList data={log} height={400} itemHeight={50} itemKey={"timestamp"}>
                {(item) => <List.Item>
                    <List.Item.Meta style={{wordBreak: "break-all", paddingRight: "2rem"}} title={item.title}
                                    description={item.description}/>
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