import type { Socket } from 'socket.io-client';

export type LiveEndpoint<TArgs extends readonly unknown[], TData> = {
	namespace: string;
	endpoint: string;
	event: string;
	args: TArgs;
	parse?: (value: unknown) => TData;
	refetch?: (args: readonly unknown[]) => Promise<TData>;
};

export type Listener<T> = (value: T) => void;

export type Entry = {
	endpoint: LiveEndpoint<readonly unknown[], unknown>;
	listeners: Set<Listener<unknown>>;
	errorListeners: Set<Listener<Error>>;
	value?: unknown;
	hasValue: boolean;
	version: number;
};

export type SocketEntry = { socket: Socket; ready: boolean };

export type RefetchState = { running: boolean; pending: boolean };

export type LiveUpdate = { endpoint: string; args: unknown; data: unknown };

export type LiveInvalidation = { endpoint: string; args: unknown };
