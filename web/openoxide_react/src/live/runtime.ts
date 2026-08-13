import {io, type Socket} from 'socket.io-client';

export type LiveEndpoint<TArgs extends readonly unknown[], TData> = {
	namespace: string;
	endpoint: string;
	event: string;
	args: TArgs;
	parse?: (value: unknown) => TData;
	refetch?: (args: readonly unknown[]) => Promise<TData>;
};

type Listener<T> = (value: T) => void;
type Entry = {
	endpoint: LiveEndpoint<readonly unknown[], unknown>;
	listeners: Set<Listener<unknown>>;
	errorListeners: Set<Listener<Error>>;
	value?: unknown;
	hasValue: boolean;
	version: number;
};

type LiveUpdate = {endpoint: string; args: unknown; data: unknown};
type LiveInvalidation = {endpoint: string; args: unknown};

const sockets = new Map<string, Socket>();
const entries = new Map<string, Entry>();

function accessToken() {
	try {
		return JSON.parse(localStorage.getItem('openoxide-auth-session') ?? 'null')?.tokens?.access_token as string | undefined;
	} catch {
		return undefined;
	}
}

function safeStringify(val: unknown) {
	return JSON.stringify(val, (_key, value) => (typeof value === 'bigint' ? value.toString() : value));
}

function roomKey(endpoint: LiveEndpoint<readonly unknown[], unknown>) {
	return `${endpoint.namespace}:${endpoint.endpoint}:${safeStringify(endpoint.args)}`;
}

function socketFor(namespace: string) {
	let socket = sockets.get(namespace);
	if (socket) return socket;

	socket = io(namespace, {
		path: '/socket.io',
		transports: ['websocket', 'polling'],
		auth: callback => callback({token: accessToken()}),
	});
	socket.on('connect', () => {
		for (const entry of entries.values()) {
			if (entry.endpoint.namespace === namespace) socket.emit('live:subscribe', subscribeMessage(entry.endpoint));
		}
	});
	socket.on('live:update', (update: LiveUpdate) => {
		const key = `${namespace}:${update.endpoint}:${safeStringify(update.args)}`;
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
	socket.on('live:invalidate', (invalidation: LiveInvalidation) => {
		const key = `${namespace}:${invalidation.endpoint}:${safeStringify(invalidation.args)}`;
		const entry = entries.get(key);
		if (!entry?.endpoint.refetch) return;
		void entry.endpoint.refetch(entry.endpoint.args).then((value) => {
			entry.value = value;
			entry.hasValue = true;
			entry.version++;
			for (const notify of entry.listeners) notify(value);
		}).catch((cause) => {
			const error = cause instanceof Error ? cause : new Error(String(cause));
			for (const notify of entry.errorListeners) notify(error);
		});
	});
	sockets.set(namespace, socket);
	return socket;
}

function subscribeMessage(endpoint: LiveEndpoint<readonly unknown[], unknown>) {
	return {endpoint: endpoint.endpoint, args: endpoint.args};
}

export function subscribeLive<T>(
	endpoint: LiveEndpoint<readonly unknown[], T>,
	listener: Listener<T>,
	onError?: Listener<Error>,
) {
	const key = roomKey(endpoint);
	let entry = entries.get(key);
	if (!entry) {
		const socket = socketFor(endpoint.namespace);
		entry = {endpoint, listeners: new Set(), errorListeners: new Set(), hasValue: false, version: 0};
		entries.set(key, entry);

		socket.emit('live:subscribe', subscribeMessage(endpoint));
		if (endpoint.refetch) {
			void endpoint.refetch(endpoint.args).then((value) => {
				const current = entries.get(key);
				if (!current) return;
				current.value = value;
				current.hasValue = true;
				for (const notify of current.listeners) notify(value);
			}).catch((cause) => {
				const current = entries.get(key);
				if (!current) return;
				const error = cause instanceof Error ? cause : new Error(String(cause));
				for (const notify of current.errorListeners) notify(error);
			});
		}
	}

	entry.listeners.add(listener as Listener<unknown>);
	if (onError) entry.errorListeners.add(onError);
	if (entry.hasValue) listener(entry.value as T);

	return () => {
		const current = entries.get(key);
		if (!current) return;
		current.listeners.delete(listener as Listener<unknown>);
		if (onError) current.errorListeners.delete(onError);
		if (current.listeners.size !== 0) return;

		const socket = socketFor(endpoint.namespace);
		socket.emit('live:unsubscribe', subscribeMessage(endpoint));
		entries.delete(key);
	};
}
