type Sid = number; // u32
type Uid = number; // u32

/** Position and size of a window, see the Rust version. */
export type WsWinsize = {
  x: number;
  y: number;
  rows: number;
  cols: number;
};

/** Information about a user, see the Rust version */
export type WsUser = {
  name: string;
  cursor: [number, number] | null;
  focus: number | null;
  canWrite: boolean;
};

/** Server message type, see the Rust version. */
export type WsServer = {
  hello?: [Uid, string];
  invalidAuth?: [];
  users?: [Uid, WsUser][];
  userDiff?: [Uid, WsUser | null];
  shells?: [Sid, WsWinsize][];
  chunks?: [Sid, number, Uint8Array[]];
  hear?: [Uid, string, string];
  shellLatency?: number | bigint;
  pong?: number | bigint;
  fileOffer?: FileMeta;
  fileChunk?: [number, number, Uint8Array];  // [fileId, chunkIdx, data]
  fileDone?: [number, boolean];              // [fileId, success]
  error?: string;
};

/** Metadata about a file transfer. */
export type FileMeta = {
  id: number;
  name: string;
  size: number;
  mime: string | null;
  uploader: number;
  uploaderName: string;
};

/** Client message type, see the Rust version. */
export type WsClient = {
  authenticate?: [Uint8Array, Uint8Array | null];
  setName?: string;
  setCursor?: [number, number] | null;
  setFocus?: number | null;
  create?: [number, number];
  close?: Sid;
  move?: [Sid, WsWinsize | null];
  data?: [Sid, Uint8Array, bigint];
  subscribe?: [Sid, number];
  chat?: string;
  ping?: bigint;
  fileUpload?: [string, number, string | null];     // [name, size, mime?]
  fileChunk?: [number, number, Uint8Array];          // [fileId, chunkIdx, data]
  fileFinished?: [number, boolean];                  // [fileId, success]
  fileDownload?: [number];                           // [fileId]
};
