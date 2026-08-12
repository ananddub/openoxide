import {io, type Socket} from 'socket.io-client';

export type LiveEndpoint<TArgs extends readonly unknown[], TData> = {
	namespace: string;
	endpoint: string;
	event: string;
	args: TArgs;
	parse?: (value: unknown) => TData;
};

type Listener<T> = (value: T) => void;
type Entry = {
	endpoint: LiveEndpoint<readonly unknown[], unknown>;
	listeners: Set<Listener<unknown>>;
	value?: unknown;
	hasValue: boolean;
	version: number;
};

type LiveUpdate = {endpoint: string; args: unknown; data: unknown};

const sockets = new Map<string, Socket>();
const entries = new Map<string, Entry>();

function roomKey(endpoint: LiveEndpoint<readonly unknown[], unknown>) {
	return `${endpoint.namespace}:${endpoint.endpoint}:${JSON.stringify(endpoint.args)}`;
}

function socketFor(namespace: string) {
	let socket = sockets.get(namespace);
	if (socket) return socket;

	socket = io(namespace, {path: '/socket.io', transports: ['websocket', 'polling']});
	socket.on('connect', () => {
		for (const entry of entries.values()) {
			if (entry.endpoint.namespace === namespace) socket.emit('live:subscribe', subscribeMessage(entry.endpoint));
		}
	});
	socket.on('live:update', (update: LiveUpdate) => {
		const key = `${namespace}:${update.endpoint}:${JSON.stringify(update.args)}`;
		const matching = entries.get(key)
			? [entries.get(key)!]
			: update.args == null
				? [...entries.values()].filter((entry) => entry.endpoint.namespace === namespace && entry.endpoint.endpoint === update.endpoint)
				: [];
		for (const entry of matching) {
			try {
				const value = entry.endpoint.parse ? entry.endpoint.parse(update.data) : update.data;
				entry.value = value;
				entry.hasValue = true;
				entry.version++;
				for (const notify of entry.listeners) notify(value);
			} catch {
				// A generated runtime validator can surface this through the hook's error state.
			}
		}
	});
	sockets.set(namespace, socket);
	return socket;
}

function subscribeMessage(endpoint: LiveEndpoint<readonly unknown[], unknown>) {
	return {endpoint: endpoint.endpoint, args: endpoint.args};
}

export function subscribeLive<T>(endpoint: LiveEndpoint<readonly unknown[], T>, listener: Listener<T>) {
	const key = roomKey(endpoint);
	let entry = entries.get(key);
	if (!entry) {
		const socket = socketFor(endpoint.namespace);
		entry = {endpoint, listeners: new Set(), hasValue: false, version: 0};
		entries.set(key, entry);

		socket.emit('live:subscribe', subscribeMessage(endpoint));
	}

	entry.listeners.add(listener as Listener<unknown>);
	if (entry.hasValue) listener(entry.value as T);

	return () => {
		const current = entries.get(key);
		if (!current) return;
		current.listeners.delete(listener as Listener<unknown>);
		if (current.listeners.size !== 0) return;

		const socket = socketFor(endpoint.namespace);
		socket.emit('live:unsubscribe', subscribeMessage(endpoint));
		entries.delete(key);
	};
}
