import {Badge, Descriptions, DescriptionsProps} from "antd";

export const RRTPStatus = () => {

    const items: DescriptionsProps['items'] = [
        {
            label: "Local Socket",
            children: <Badge status={"error"} text={"Not Bound"} />
        },
        {
            label: "Connection To Remote",
            children: <Badge status={"error"} text={"Not Connected"} />
        }
    ];

    return (
        <Descriptions column={1} title={"RRTP Status"} items={items} />
    );
};