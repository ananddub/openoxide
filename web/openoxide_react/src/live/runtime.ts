import {io, type Socket} from 'socket.io-client';
import {
	liveArgsKey,
	matchesLiveInvalidation,
} from '@openoxide/vite/live-key';

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

type SocketEntry = {socket: Socket; ready: boolean};
type RefetchState = {running: boolean; pending: boolean};

type LiveUpdate = {endpoint: string; args: unknown; data: unknown};
type LiveInvalidation = {endpoint: string; args: unknown};

const sockets = new Map<string, SocketEntry>();
const entries = new Map<string, Entry>();
const refetches = new Map<string, RefetchState>();

function queueRefetch(key: string, entry: Entry) {
	if (!entry.endpoint.refetch) return;
	let state = refetches.get(key);
	if (!state) {
		state = {running: false, pending: false};
		refetches.set(key, state);
	}
	state.pending = true;
	if (state.running) return;
	state.running = true;
	void (async () => {
		try {
			while (state.pending) {
				state.pending = false;
				try {
					const value = await entry.endpoint.refetch!(entry.endpoint.args);
					entry.value = value;
					entry.hasValue = true;
					entry.version++;
					for (const notify of entry.listeners) notify(value);
				} catch (cause) {
					const error =
						cause instanceof Error ? cause : new Error(String(cause));
					for (const notify of entry.errorListeners) notify(error);
				}
			}
		} finally {
			state.running = false;
			if (!state.pending) refetches.delete(key);
		}
	})();
}

function accessToken() {
	try {
		return JSON.parse(
			localStorage.getItem('openoxide-auth-session') ?? 'null',
		)?.tokens?.access_token as string | undefined;
	} catch {
		return undefined;
	}
}

function safeStringify(value: unknown) {
	return liveArgsKey(value);
}

function roomKey(endpoint: LiveEndpoint<readonly unknown[], unknown>) {
	return `${endpoint.namespace}:${endpoint.endpoint}:${safeStringify(endpoint.args)}`;
}

function socketBaseUrl() {
	if (import.meta.env.VITE_SOCKET_URL)
		return import.meta.env.VITE_SOCKET_URL;
	// In development connect directly to Axum: Vite's proxy can return 502
	// during Socket.IO polling/upgrade reconnects.
	if (import.meta.env.DEV) return 'http://127.0.0.1:4000';
	return '';
}

function socketFor(namespace: string) {
	let existing = sockets.get(namespace);
	if (existing) return existing;

	const socket = io(`${socketBaseUrl()}${namespace}`, {
		path: '/socket.io',
		transports: ['websocket', 'polling'],
		tryAllTransports: true,
		upgrade: false,
		auth: callback => callback({token: accessToken()}),
		reconnection: true,
		reconnectionAttempts: Infinity,
		reconnectionDelay: 1000,
		reconnectionDelayMax: 30000,
		randomizationFactor: 0.5,
	});
	const socketEntry: SocketEntry = {socket, ready: false};
	socket.on('connect', () => {
		socketEntry.ready = false;
	});
	socket.on('socket:ready', () => {
		socketEntry.ready = true;
		for (const entry of entries.values()) {
			if (entry.endpoint.namespace === namespace)
				socket.emit('live:subscribe', subscribeMessage(entry.endpoint));
		}
	});
	socket.on(
		'live:subscribed',
		(message: {endpoint: string; args: unknown}) => {
			const key = `${namespace}:${message.endpoint}:${safeStringify(message.args)}`;
			const entry = entries.get(key);
			if (!entry) return;
			console.debug('[openoxide-live] resubscribed', key);
			queueRefetch(key, entry);
		},
	);
	socket.on('disconnect', reason => {
		socketEntry.ready = false;
		if (reason === 'io server disconnect') socket.connect();
	});
	socket.on('connect_error', error => {
		console.error(
			'[openoxide-live] socket connection failed',
			namespace,
			error,
		);
	});
	const recover = () => {
		if (!socket.connected) socket.connect();
	};
	window.addEventListener('online', recover);
	document.addEventListener('visibilitychange', () => {
		if (document.visibilityState === 'visible') recover();
	});
	socket.on('live:update', (update: LiveUpdate) => {
		const key = `${namespace}:${update.endpoint}:${safeStringify(update.args)}`;
		const matching = entries.get(key)
			? [entries.get(key)!]
			: update.args == null
				? [...entries.values()].filter(
						entry =>
							entry.endpoint.namespace === namespace &&
							entry.endpoint.endpoint === update.endpoint,
					)
				: [];
		console.debug('[openoxide-live] received update', key, {
			matching: matching.length,
			items: Array.isArray(update.data) ? update.data.length : undefined,
		});
		for (const entry of matching) {
			try {
				const value = entry.endpoint.parse
					? entry.endpoint.parse(update.data)
					: update.data;
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
		const matching = [...entries.entries()].filter(
			([, entry]) =>
				entry.endpoint.namespace === namespace &&
				matchesLiveInvalidation(
					entry.endpoint.endpoint,
					entry.endpoint.args,
					invalidation,
				),
		);
		for (const [key, entry] of matching) {
			if (!entry.endpoint.refetch) continue;
			console.debug('[openoxide-live] invalidated', key);
			queueRefetch(key, entry);
		}
	});
	sockets.set(namespace, socketEntry);
	return socketEntry;
}

function subscribeMessage(
	endpoint: LiveEndpoint<readonly unknown[], unknown>,
) {
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
		const socketEntry = socketFor(endpoint.namespace);
		entry = {
			endpoint,
			listeners: new Set(),
			errorListeners: new Set(),
			hasValue: false,
			version: 0,
		};
		entries.set(key, entry);

		if (socketEntry.ready)
			socketEntry.socket.emit(
				'live:subscribe',
				subscribeMessage(endpoint),
			);
		if (endpoint.refetch) {
			void endpoint
				.refetch(endpoint.args)
				.then(value => {
					const current = entries.get(key);
					if (!current) return;
					current.value = value;
					current.hasValue = true;
					for (const notify of current.listeners) notify(value);
				})
				.catch(cause => {
					const current = entries.get(key);
					if (!current) return;
					const error =
						cause instanceof Error ? cause : new Error(String(cause));
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

		const socketEntry = socketFor(endpoint.namespace);
		if (socketEntry.ready)
			socketEntry.socket.emit(
				'live:unsubscribe',
				subscribeMessage(endpoint),
			);
		entries.delete(key);
	};
}
