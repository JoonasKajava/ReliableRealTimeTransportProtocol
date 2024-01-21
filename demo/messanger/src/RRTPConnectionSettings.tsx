import {Button, Form, Input, Space} from "antd";
import {useRecoilState} from "recoil";
import {connectionStatusState} from "./main.tsx";
import {useCallback, useState} from "react";
import {useLog} from "./RRTPLog.tsx";
import {invoke} from "@tauri-apps/api";
import {LogErrorMessage, LogSuccessMessage} from "./rust_type_definitions.ts";

export const RRTPConnectionSettings = () => {
    const [connectionStatus, setConnectionStatus] = useRecoilState(connectionStatusState);

    const setLog = useLog();

    const [localAddress, setLocalAddress] = useState("localhost:12345")

    const [remoteAddress, setRemoteAddress] = useState("localhost:12345")


    const onBindClick = useCallback(() => {

        invoke<LogSuccessMessage>("bind", {address: localAddress}).then((result) => {
            if (result.type === "LocalSocketBindSuccess") {
                setLog("Local Socket Bound", result.content);
                setConnectionStatus((prev) => ({...prev, local: true}));
            }

        }).catch((err: LogErrorMessage) => {
            if (err.type != "LocalSocketBindFailed") {
                console.error(err);
                return;
            }
            setLog("Local Socket Bind Failed", err.content);
            setConnectionStatus((prev) => ({...prev, local: false}));
        });

    }, [setConnectionStatus, setLog, localAddress]);


    const onConnectClick = useCallback(() => {

        invoke<LogSuccessMessage>("connect", {address: remoteAddress}).then((result) => {
            if (result.type != "ConnectedToRemote") {
                console.error(result);
                return;
            }
            setLog("Connection Successful", result.content);
            setConnectionStatus((prev) => ({...prev, remote: true}));
        }).catch((err: LogErrorMessage) => {
            if (err.type != "ConnectionError") {
                console.error(err);
                return;
            }
            setLog("Connection To Remote Failed", err.content);
            setConnectionStatus((prev) => ({...prev, remote: false}));
        });

    }, [setConnectionStatus, setLog, remoteAddress]);


    return (
        <Form>
            <Form.Item name="local_addr" label="Local Address">
                <Space>
                    <Input value={localAddress} onChange={(e) => setLocalAddress(e.target.value)}
                           disabled={connectionStatus.local} placeholder="127.0.0.1:12345"/>
                    <Button disabled={connectionStatus.local} onClick={onBindClick} type="primary">Bind</Button>
                </Space>
            </Form.Item>
            <Form.Item name="remote_addr" label="Remote Address">
                <Space>
                    <Input disabled={connectionStatus.remote || !connectionStatus.local} value={remoteAddress}
                           onChange={(e) => setRemoteAddress(e.target.value)}
                           placeholder="127.0.0.1:12345"/>
                    <Button disabled={connectionStatus.remote || !connectionStatus.local} type="primary"
                            onClick={onConnectClick}>Connect</Button>
                </Space>
            </Form.Item>
        </Form>

    );
};