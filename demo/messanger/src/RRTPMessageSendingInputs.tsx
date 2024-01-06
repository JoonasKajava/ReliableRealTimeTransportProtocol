import {Button, Form, Space, Upload} from "antd";
import TextArea from "antd/es/input/TextArea";
import { UploadOutlined } from '@ant-design/icons';
import {useRecoilValue} from "recoil";
import {connectionStatusState} from "./main.tsx";
import {useCallback, useState} from "react";
import {invoke} from "@tauri-apps/api";
import {useLog} from "./RRTPLog.tsx";

export const RRTPMessageSendingInputs = () => {

    const connectionStatus = useRecoilValue(connectionStatusState);
    const setLog = useLog();
    const [message, setMessage] = useState("")

    const onSendClick = useCallback(() => {
        invoke<string>("send_message", {message: message}).then((result) => {
            setLog("Message Sent", result);
        }).catch((err) => {
            setLog("Sending Message Failed", err);
        });
    }, [setLog, message]);

    return (
        <Form>
            <Form.Item name="message" label="Message">
                <Space>
                    <TextArea disabled={!connectionStatus.remote} placeholder="Hello World" rows={2} value={message}
                              onChange={(e) => setMessage(e.target.value)}/>
                    <Button disabled={!connectionStatus.remote} type="primary" onClick={onSendClick}>Send</Button>
                </Space>
            </Form.Item>
            <Form.Item name="file" label="File">
                <Space>
                    <Upload>
                        <Button disabled={!connectionStatus.remote} icon={<UploadOutlined/>}>Click to Upload</Button>
                    </Upload>
                    <Button disabled={!connectionStatus.remote} type="primary" onClick={onSendClick}>Send</Button>
                </Space>
            </Form.Item>
        </Form>
    );
};