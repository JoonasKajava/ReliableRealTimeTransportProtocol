import {Badge, Descriptions, DescriptionsProps} from "antd";
import {useRecoilValue} from "recoil";

import {connectionStatusState} from "./main.tsx";

export const RRTPStatus = () => {

    const connectionStatus = useRecoilValue(connectionStatusState);

    const items: DescriptionsProps['items'] = [
        {
            label: "Local Socket",
            children: <Badge status={connectionStatus.local ? "success": "error"} text={connectionStatus.local ? "Bound" : "Not Bound"} />
        },
        {
            label: "Connection To Remote",
            children: <Badge status={connectionStatus.remote ? "success": "error"} text={connectionStatus.remote ? "Connected" : "Not Connected"} />
        }
    ];

    return (
        <Descriptions column={1} title={"RRTP Status"} items={items} />
    );
};