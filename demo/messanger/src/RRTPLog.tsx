import {List} from "antd";
import VirtualList from 'rc-virtual-list';
import {atom, useRecoilValue, useSetRecoilState} from "recoil";
import * as dayjs from "dayjs";
import {useEffect} from "react";
import {listen} from "@tauri-apps/api/event";

import localizedFormat from "dayjs/plugin/localizedFormat";

dayjs.extend(localizedFormat)
export const logState = atom<{ title: string, description: string, timestamp: dayjs.Dayjs }[]>({
    key: 'log',
    default: []
})

export const RRTPLog = () => {

    const log = useRecoilValue(logState);

    const setLog = useLog();

    useEffect(() => {
        const unlisten = listen<string>("message", (event) => {
            setLog("Message Received", event.payload);
        });
        return () => {
            unlisten.then((unlisten) => unlisten());
        }
    }, [useLog]);

    return (
        <List>
            <VirtualList data={log} height={400} itemHeight={50} itemKey={"timestamp"}>
                {(item) => <List.Item>
                    <List.Item.Meta title={item.title} description={item.description}/>
                    <div>{item.timestamp.format("LTS")}</div>
                </List.Item>}
            </VirtualList>
        </List>
    );
};

export function useLog() {
    const setLog = useSetRecoilState(logState);
    return (title: string, description: string) => {
        setLog((prev) => [{
            timestamp: dayjs(),
            title,
            description
        }, ...prev]);
    }
}