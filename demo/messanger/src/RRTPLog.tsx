import {List} from "antd";
import VirtualList from 'rc-virtual-list';
import {atom, useRecoilValue, useSetRecoilState} from "recoil";
import * as dayjs from "dayjs";

export const logState= atom<{ title: string, description: string, timestamp:  dayjs.Dayjs }[]>({
    key: 'log',
    default: []
})

export const RRTPLog = () => {

    const log = useRecoilValue(logState);

    return (
        <List>
            <VirtualList data={log} height={400} itemHeight={50} itemKey={"title"}>
                {(item) => <List.Item>
                    <List.Item.Meta title={item.title} description={item.description}/>
                    <div>{item.timestamp.format()}</div>
                </List.Item>}
            </VirtualList>
        </List>
    );
};

export function useLog() {
    const setLog = useSetRecoilState(logState);
    return (title: string, description: string) => {
        setLog((prev) => [...prev, {
            timestamp: dayjs(),
            title,
            description
        }]);
    }
}