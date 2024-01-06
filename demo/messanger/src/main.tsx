import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import {RecoilRoot, atom} from "recoil";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <RecoilRoot>
            <App/>
        </RecoilRoot>
    </React.StrictMode>,
);
export const connectionStatusState = atom({
    key: 'connectionStatus',
    default: {
        local: false,
        remote: false
    }
})