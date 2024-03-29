/*
 Generated by typeshare 1.7.0
*/

export interface NetworkFileInfo {
    file_name: string;
    file_mime?: string;
    file_size_in_bytes: number;
}

export type LogSuccessMessage =
    | { type: "UnknownMessage", content: string }
    | { type: "MessageReceived", content: string }
    | { type: "MessageSent", content: string }
    | { type: "LocalSocketBindSuccess", content: string }
    | { type: "ConnectedToRemote", content: string }
    | { type: "FileInfoSent", content: NetworkFileInfo }
    | { type: "FileInfoReceived", content: NetworkFileInfo }
    | { type: "FileDataReceived", content?: undefined }
    | { type: "ReceivedAcknowledgement", content?: undefined }
    | { type: "FileRejected", content?: undefined }
    | { type: "FileAccepted", content?: undefined }
    | { type: "FileResponseSent", content?: undefined };

export type LogErrorMessage =
    | { type: "MessageSendError", content: string }
    | { type: "LocalSocketBindFailed", content: string }
    | { type: "LocalSocketNotBound", content?: undefined }
    | { type: "ConnectionError", content: string }
    | { type: "FileSendError", content: string }
    | { type: "InvalidFileResponse", content: string };

