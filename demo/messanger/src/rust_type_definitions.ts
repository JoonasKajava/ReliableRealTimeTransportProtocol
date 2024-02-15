/*
 Generated by typeshare 1.7.0
*/

export interface FileMetadata {
	file_name: string;
	file_mime?: string;
	file_size_in_bytes: number;
}

export interface FileInfo {
	metadata: FileMetadata;
	src?: string;
}

export type LogSuccessMessage = 
	| { type: "UnknownMessage", content: string }
	| { type: "MessageReceived", content: string }
	| { type: "MessageSent", content: string }
	| { type: "LocalSocketBindSuccess", content: string }
	| { type: "ConnectedToRemote", content: string }
	| { type: "FileInfoSent", content: FileMetadata }
	| { type: "FileInfoReceived", content: FileMetadata }
	| { type: "FileDataReceived", content?: undefined }
	| { type: "ReceivedAcknowledgement", content?: undefined }
	| { type: "FileRejected", content: FileMetadata }
	| { type: "FileAccepted", content: FileMetadata }
	| { type: "FileResponseSent", content?: undefined }
	| { type: "Error", content: string }
	| {
	type: "ReceivedFrame", content: {
		len: number;
	}
}
	| {
	type: "SendFrame", content: {
		len: number;
	}
};

export type LogErrorMessage = 
	| { type: "MessageSendError", content: string }
	| { type: "LocalSocketBindFailed", content: string }
	| { type: "LocalSocketNotBound", content?: undefined }
	| { type: "ConnectionError", content: string }
	| { type: "FileSendError", content: string }
	| { type: "InvalidFileResponse", content: string };

