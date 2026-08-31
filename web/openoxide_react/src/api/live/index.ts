// Live-data hooks over ConnectRPC queries, replacing the former
// `virtual:openoxide-live` Socket.IO hooks. Hook names and payload shapes
// are preserved; only the transport changed.
export * from './query';
export * from './projects';
export * from './ai';
export * from './applications';
export * from './backups';
export * from './certificates';
export * from './compose';
export * from './databases';
export * from './deployments';
export * from './destinations';
export * from './dns';
export * from './domains';
export * from './environments';
export * from './monitoring';
export * from './notifications';
export * from './organization';
export * from './overview';
export * from './permissions';
export * from './registry';
export * from './resource';
export * from './schedules';
export * from './servers';
export * from './settings';
export * from './ssh_keys';
export * from './swarm';
export * from './tags';
export * from './traefik';
export * from './vault';
