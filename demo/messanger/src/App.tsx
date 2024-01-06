import "./App.css";
import {Col, ConfigProvider, Row, theme} from "antd";
import {RRTPStatus} from "./RRTPStatus.tsx";
import {RRTPConnectionSettings} from "./RRTPConnectionSettings.tsx";
import {RRTPLog} from "./RRTPLog.tsx";


function App() {
    return <ConfigProvider theme={{algorithm: theme.darkAlgorithm}}>
        <Row>
            <Col span={12}>
                <RRTPConnectionSettings/>
            </Col>
            <Col span={12}>
                <RRTPStatus/>
            </Col>
        </Row>
        <Row>
            <Col span={18}>
                <RRTPLog/>
            </Col>
            <Col span={6}>

            </Col>
        </Row>
    </ConfigProvider>
}

export default App;
