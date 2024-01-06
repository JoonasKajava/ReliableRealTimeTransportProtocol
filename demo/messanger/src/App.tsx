import "./App.css";
import {Button, Col, ConfigProvider, Form, Input, Row, Space, theme} from "antd";
import {RRTPStatus} from "./RRTPStatus.tsx";

function App() {

    return <ConfigProvider theme={{algorithm: theme.darkAlgorithm}}>
        <Row>
            <Col span={12}>
                <Form>
                    <Form.Item name="local_addr" label="Local Address">
                        <Space>
                            <Input placeholder="127.0.0.1:12345"/>
                            <Button type="primary">Bind</Button>
                        </Space>
                    </Form.Item>
                    <Form.Item name="remote_addr" label="Remote Address">
                        <Space>
                            <Input placeholder="127.0.0.1:12345"/>
                            <Button type="primary">Connect</Button>
                        </Space>
                    </Form.Item>
                </Form>
            </Col>
            <Col span={12}>
                <RRTPStatus/>
            </Col>
        </Row>
    </ConfigProvider>
}

export default App;
