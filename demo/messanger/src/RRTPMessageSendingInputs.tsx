import {Button, Form, Space, Tag} from "antd";
import TextArea from "antd/es/input/TextArea";
import {UploadOutlined, ClockCircleOutlined} from '@ant-design/icons';
import {useRecoilValue} from "recoil";
import {connectionStatusState} from "./main.tsx";
import {useCallback, useState} from "react";
import {invoke} from "@tauri-apps/api";
import {useLog} from "./RRTPLog.tsx";
import {open} from "@tauri-apps/api/dialog";

export const RRTPMessageSendingInputs = () => {

    const connectionStatus = useRecoilValue(connectionStatusState);
    const setLog = useLog();
    const [message, setMessage] = useState("")

    const [selectedFile, setSelectedFile] = useState<null | string>(null)

    const onMessageSendClick = useCallback(() => {
        invoke<string>("send_message", {message: message}).then((result) => {
            setLog("Message Sent", result);
        }).catch((err) => {
            setLog("Sending Message Failed", err);
        });
    }, [setLog, message]);

    const onFileSelectClick = useCallback(async () => {
        const selectedFile = await open({multiple: false});

        if (typeof selectedFile === "string") {
            setSelectedFile(selectedFile);
        }

    }, []);


    const onFileSendClick = useCallback(() => {
        invoke<string>("send_file", {filePath: selectedFile}).then((result) => {
            setLog("File Sent", result);
        }).catch((err) => {
            setLog("Sending File Failed", err);
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