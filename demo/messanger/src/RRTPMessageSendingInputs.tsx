import {Button, Form, Space, Tag} from "antd";
import TextArea from "antd/es/input/TextArea";
import {ClockCircleOutlined, UploadOutlined} from '@ant-design/icons';
import {useRecoilValue} from "recoil";
import {connectionStatusState} from "./main.tsx";
import {useCallback, useState} from "react";
import {invoke} from "@tauri-apps/api";
import {LogMessageTitleMap, useLog} from "./RRTPLog.tsx";
import {open} from "@tauri-apps/api/dialog";
import {LogErrorMessage, LogSuccessMessage} from "./rust_type_definitions.ts";

export const RRTPMessageSendingInputs = () => {

    const connectionStatus = useRecoilValue(connectionStatusState);
    const setLog = useLog();
    const [message, setMessage] = useState("")

    const [selectedFile, setSelectedFile] = useState<null | string>(null)

    const onMessageSendClick = useCallback(() => {
        invoke<LogSuccessMessage>("send_message", {message: message}).then((result) => {
            setLog(LogMessageTitleMap[result.type], result.content as string);
        }).catch((err: LogErrorMessage) => {
            setLog(LogMessageTitleMap[err.type], err.content as string)
        });
    }, [setLog, message]);

    const onFileSelectClick = useCallback(async () => {
        const selectedFile = await open({multiple: false});

        if (typeof selectedFile === "string") {
            setSelectedFile(selectedFile);
        }

    }, []);


    const onFileSendClick = useCallback(() => {
        invoke<LogSuccessMessage>("send_file", {filePath: selectedFile}).then((result) => {
            setLog(LogMessageTitleMap[result.type], result.content as string);
        }).catch((err: LogErrorMessage) => {
            setLog(LogMessageTitleMap[err.type], err.content as string)
        });

    }, [setLog, selectedFile]);


    return (
        <Form>
            <Form.Item name="message" label="Message">
                <Space>
                    <TextArea disabled={!connectionStatus.remote} placeholder="Hello World" rows={2} value={message}
                              onChange={(e) => setMessage(e.target.value)}/>
                    <Button disabled={!connectionStatus.remote} type="primary"
                            onClick={onMessageSendClick}>Send</Button>
                </Space>
            </Form.Item>
            <Form.Item name="file" label="File">
                <Space direction={"vertical"}>
                    <Space>
                        <Button disabled={!connectionStatus.remote} icon={<UploadOutlined/>}
                                onClick={onFileSelectClick}>Click
                            to Upload</Button>
                        <Button disabled={!connectionStatus.remote} type="primary"
                                onClick={onFileSendClick}>Send</Button>
                    </Space>
                    <Space>
                        {selectedFile && <Tag icon={<ClockCircleOutlined/>} color={"default"}>{selectedFile}</Tag>}
                    </Space>
                </Space>
            </Form.Item>
        </Form>
    );
};