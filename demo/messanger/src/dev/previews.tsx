import {ComponentPreview, Previews} from "@react-buddy/ide-toolbox";
import {PaletteTree} from "./palette";
import {RRTPLog} from "../RRTPLog.tsx";
import {RRTPConnectionSettings} from "../RRTPConnectionSettings.tsx";

const ComponentPreviews = () => {
    return (
        <Previews palette={<PaletteTree/>}>
            <ComponentPreview path="/RRTPLog">
                <RRTPLog/>
            </ComponentPreview>
            <ComponentPreview
                path="/RRTPConnectionSettings">
                <RRTPConnectionSettings/>
            </ComponentPreview>
        </Previews>
    );
};

export default ComponentPreviews;