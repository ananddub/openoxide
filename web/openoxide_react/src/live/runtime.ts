import {
	liveArgsKey,
	matchesLiveInvalidation,
} from '@openoxide/vite/live-key';
import {socketFor, subscribeMessage} from './socket';
import type {
	Entry,
	Listener,
	LiveEndpoint,
	LiveInvalidation,
	LiveUpdate,
	RefetchState,
} from './types';

const entries = new Map<string, Entry>();
const refetches = new Map<string, RefetchState>();

function safeStringify(value: unknown): string {
	return liveArgsKey(value);
}

function roomKey(
	endpoint: LiveEndpoint<readonly unknown[], unknown>,
): string {
	return `${endpoint.namespace}:${endpoint.endpoint}:${safeStringify(endpoint.args)}`;
}

function notifyListeners<T>(listeners: Set<Listener<T>>, value: T) {
	// Microtask batching prevents UI lockup when multiple state updates fire in rapid succession during reconnect
	queueMicrotask(() => {
		for (const notify of listeners) {
			try {
				notify(value);
			} catch (err) {
				console.error('[openoxide-live] listener error:', err);
			}
		}
	});
}

function queueRefetch(key: string, entry: Entry, delayMs = 0) {
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
			if (delayMs > 0) {
				await new Promise(r => setTimeout(r, delayMs));
			}
			while (state.pending) {
				state.pending = false;
				try {
					const value = await entry.endpoint.refetch!(entry.endpoint.args);
					entry.value = value;
					entry.hasValue = true;
					entry.version++;
					notifyListeners(entry.listeners, value);
				} catch (cause) {
					const error =
						cause instanceof Error ? cause : new Error(String(cause));
					notifyListeners(entry.errorListeners, error);
					// Backoff on failure to prevent tight CPU looping during backend reboot
					await new Promise(r => setTimeout(r, 1500));
				}
			}
		} finally {
			state.running = false;
			if (!state.pending) refetches.delete(key);
		}
	})();
}

function attachSocketListeners(namespace: string) {
	const socketEntry = socketFor(namespace);
	const {socket} = socketEntry;

	if (socket.listeners('socket:ready').length === 0) {
		socket.on('socket:ready', () => {
			socketEntry.ready = true;
			for (const entry of entries.values()) {
				if (entry.endpoint.namespace === namespace) {
					socket.emit('live:subscribe', subscribeMessage(entry.endpoint));
				}
			}
		});

		socket.on(
			'live:subscribed',
			(message: {endpoint: string; args: unknown}) => {
				const key = `${namespace}:${message.endpoint}:${safeStringify(message.args)}`;
				const entry = entries.get(key);
				if (entry) {
					queueRefetch(key, entry, Math.floor(Math.random() * 80));
				}
			},
		);

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

			for (const entry of matching) {
				try {
					const value = entry.endpoint.parse
						? entry.endpoint.parse(update.data)
						: update.data;
					entry.value = value;
					entry.hasValue = true;
					entry.version++;
					notifyListeners(entry.listeners, value);
				} catch {
					// Ignore parse errors
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
				if (entry.endpoint.refetch) {
					queueRefetch(key, entry);
				}
			}
		});
	}

	return socketEntry;
}

export function subscribeLive<T>(
	endpoint: LiveEndpoint<readonly unknown[], T>,
	listener: Listener<T>,
	onError?: Listener<Error>,
) {
	const key = roomKey(endpoint);
	let entry = entries.get(key);
	if (!entry) {
		const socketEntry = attachSocketListeners(endpoint.namespace);
		entry = {
			endpoint,
			listeners: new Set(),
			errorListeners: new Set(),
			hasValue: false,
			version: 0,
		};
		entries.set(key, entry);

		if (socketEntry.ready) {
			socketEntry.socket.emit(
				'live:subscribe',
				subscribeMessage(endpoint),
			);
		}
		if (endpoint.refetch) {
			void endpoint
				.refetch(endpoint.args)
				.then(value => {
					const current = entries.get(key);
					if (!current) return;
					current.value = value;
					current.hasValue = true;
					notifyListeners(current.listeners, value);
				})
				.catch(cause => {
					const current = entries.get(key);
					if (!current) return;
					const error =
						cause instanceof Error ? cause : new Error(String(cause));
					notifyListeners(current.errorListeners, error);
				});
		}
	}

	entry.listeners.add(listener as Listener<unknown>);
	if (onError) entry.errorListeners.add(onError);
	if (entry.hasValue) {
		const cached = entry.value as T;
		queueMicrotask(() => listener(cached));
	}

	return () => {
		const current = entries.get(key);
		if (!current) return;
		current.listeners.delete(listener as Listener<unknown>);
		if (onError) current.errorListeners.delete(onError);
		if (current.listeners.size !== 0) return;

		const socketEntry = socketFor(endpoint.namespace);
		if (socketEntry.ready) {
			socketEntry.socket.emit(
				'live:unsubscribe',
				subscribeMessage(endpoint),
			);
		}
		entries.delete(key);
	};
}
export type {LiveEndpoint} from './types';
