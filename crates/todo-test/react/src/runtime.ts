import {useEffect, useMemo, useState} from 'react';
import {io, type Socket} from 'socket.io-client';

type Endpoint<A extends unknown[], T> = {namespace: string; endpoint: string; event: string; args: A};
type Update<T> = {endpoint: string; args: unknown[]; data: T};
const sockets = new Map<string, Socket>();

export function createLiveHook<A extends unknown[], T>(metadata: Omit<Endpoint<A, T>, 'args'>) {
  return (...args: A) => {
    const key = JSON.stringify(args);
    const endpoint = useMemo(() => ({...metadata, args}), [key]);
    const [data, setData] = useState<T>();
    const [connected, setConnected] = useState(false);
    useEffect(() => {
      let socket = sockets.get(endpoint.namespace);
      if (!socket) {
        socket = io(endpoint.namespace, {path: '/socket.io'});
        sockets.set(endpoint.namespace, socket);
      }
      const subscription = {endpoint: endpoint.endpoint, args: endpoint.args};
      const subscribe = () => { setConnected(true); socket!.emit('live:subscribe', subscription); };
      const update = (message: Update<T>) => {
        if (message.endpoint === endpoint.endpoint && JSON.stringify(message.args) === key) setData(message.data);
      };
      socket.on('connect', subscribe);
      socket.on('disconnect', () => setConnected(false));
      socket.on('live:update', update);
      if (socket.connected) subscribe();
      return () => { socket!.emit('live:unsubscribe', subscription); socket!.off('live:update', update); };
    }, [endpoint, key]);
    return {data, connected};
  };
}
