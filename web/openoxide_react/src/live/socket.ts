import {io} from 'socket.io-client';
import type {SocketEntry} from './types';

const sockets = new Map<string, SocketEntry>();
let listenersInitialized = false;

function accessToken(): string | undefined {
	try {
		return JSON.parse(
			localStorage.getItem('openoxide-auth-session') ?? 'null',
		)?.tokens?.access_token as string | undefined;
	} catch {
		return undefined;
	}
}

function socketBaseUrl(): string {
	if (import.meta.env.VITE_SOCKET_URL)
		return import.meta.env.VITE_SOCKET_URL;
	if (import.meta.env.DEV) return 'http://127.0.0.1:4000';
	return '';
}

let recoveryTimeout: ReturnType<typeof setTimeout> | null = null;

function setupGlobalRecovery() {
	if (listenersInitialized || typeof window === 'undefined') return;
	listenersInitialized = true;

	const recoverAll = () => {
		if (recoveryTimeout) clearTimeout(recoveryTimeout);
		recoveryTimeout = setTimeout(() => {
			for (const entry of sockets.values()) {
				if (!entry.socket.connected && !entry.socket.active) {
					entry.socket.connect();
				}
			}
		}, 300);
	};

	window.addEventListener('online', recoverAll);
	document.addEventListener('visibilitychange', () => {
		if (document.visibilityState === 'visible') recoverAll();
	});
}

export function socketFor(namespace: string): SocketEntry {
	let existing = sockets.get(namespace);
	if (existing) return existing;

	setupGlobalRecovery();

	const socket = io(`${socketBaseUrl()}${namespace}`, {
		path: '/socket.io',
		transports: ['websocket'], // Use pure WebSocket to prevent HTTP 502 polling spam during backend restart
		auth: callback => callback({token: accessToken()}),
		reconnection: true,
		reconnectionAttempts: Infinity,
		reconnectionDelay: 1000,
		reconnectionDelayMax: 10000,
		randomizationFactor: 0.5,
		timeout: 10000,
	});

	const socketEntry: SocketEntry = {socket, ready: false};

	socket.on('connect', () => {
		socketEntry.ready = false;
	});

	socket.on('disconnect', reason => {
		socketEntry.ready = false;
		if (reason === 'io server disconnect') {
			socket.connect();
		}
	});

	socket.on('connect_error', error => {
		console.debug(
			'[openoxide-live] reconnecting...',
			namespace,
			error.message,
		);
	});

	sockets.set(namespace, socketEntry);
	return socketEntry;
}

export function subscribeMessage(endpoint: {
	endpoint: string;
	args: unknown;
}) {
	return {endpoint: endpoint.endpoint, args: endpoint.args};
}
