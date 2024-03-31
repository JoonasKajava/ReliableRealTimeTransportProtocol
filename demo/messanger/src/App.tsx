import "./App.css";
import {Col, ConfigProvider, Row} from "antd";
import {RRTPStatus} from "./RRTPStatus.tsx";
import {RRTPConnectionSettings} from "./RRTPConnectionSettings.tsx";
import {RRTPLog} from "./RRTPLog.tsx";
import {RRTPMessageSendingInputs} from "./RRTPMessageSendingInputs.tsx";
import {FileManager} from "./FileManager.tsx";


function App() {
    return <ConfigProvider
        // theme={{algorithm: theme.darkAlgorithm}}
    >
        <Row>
            <Col span={12}>
                <RRTPConnectionSettings/>
            </Col>
            <Col span={12}>
                <RRTPStatus/>
            </Col>
        </Row>
        <Row gutter={16}>
            <Col span={12}>
                <FileManager/>
                <RRTPLog/>
            </Col>
            <Col span={12}>
                <RRTPMessageSendingInputs/>
            </Col>
        </Row>
    </ConfigProvider>
}

export default App;
