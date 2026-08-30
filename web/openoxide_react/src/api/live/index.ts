// Live-data hooks over ConnectRPC queries, replacing the former
// `virtual:openoxide-live` Socket.IO hooks. Hook names and payload shapes
// are preserved; only the transport changed.
export * from './query';
export * from './projects';
