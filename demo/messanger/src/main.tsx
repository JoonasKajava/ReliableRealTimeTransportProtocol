import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import {RecoilRoot, atom} from "recoil";
import {DevSupport} from "@react-buddy/ide-toolbox";
import {ComponentPreviews, useInitial} from "./dev";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <RecoilRoot>
            <DevSupport ComponentPreviews={ComponentPreviews}
                        useInitialHook={useInitial}
            >
                <App/>
            </DevSupport>
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