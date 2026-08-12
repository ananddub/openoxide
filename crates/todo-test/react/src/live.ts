import {useEffect, useState} from 'react';
import {io} from 'socket.io-client';

export type Todo = {id: number; title: string; done: boolean};
type Update<T> = {endpoint: string; args: unknown[]; data: T};

const socket = io('/todo', {path: '/socket.io'});
const subscription = {endpoint: 'TodoSocket::todos', args: []};

export function useTodos(initial: Todo[]) {
  const [todos, setTodos] = useState(initial);
  const [connected, setConnected] = useState(socket.connected);

  useEffect(() => {
    const subscribe = () => {
      setConnected(true);
      socket.emit('live:subscribe', subscription);
    };
    const disconnect = () => setConnected(false);
    const update = (message: Update<Todo[]>) => {
      if (message.endpoint === subscription.endpoint && JSON.stringify(message.args) === '[]') setTodos(message.data);
    };
    socket.on('connect', subscribe);
    socket.on('disconnect', disconnect);
    socket.on('live:update', update);
    if (socket.connected) subscribe();
    return () => {
      socket.emit('live:unsubscribe', subscription);
      socket.off('connect', subscribe);
      socket.off('disconnect', disconnect);
      socket.off('live:update', update);
    };
  }, []);

  return {todos, connected};
}
