import { useEffect, useRef, useState } from 'react';
import { io, type Socket } from 'socket.io-client';
import type { Terminal } from '@xterm/xterm';

interface UseTerminalSocketOptions {
	isOpen: boolean;
	targetContainer: string;
	shell: string;
	isRemoteServer: boolean;
	serverId?: number;
	termRef: React.RefObject<Terminal | null>;
}

export function useTerminalSocket({
	isOpen,
	targetContainer,
	shell,
	isRemoteServer,
	serverId,
	termRef,
}: UseTerminalSocketOptions) {
	const [status, setStatus] = useState<'connecting' | 'connected' | 'error' | 'disconnected'>('disconnected');
	const socketRef = useRef<Socket | null>(null);
	const startedRef = useRef(false);

	useEffect(() => {
		if (!isOpen) {
			if (socketRef.current) {
				socketRef.current.disconnect();
				socketRef.current = null;
			}
			startedRef.current = false;
			setStatus('disconnected');
			return;
		}

		const term = termRef.current;
		startedRef.current = false;
		setStatus('connecting');
		term?.writeln(`\x1b[33mConnecting to container/host '${targetContainer}'...\x1b[0m\r\n`);

		let token: string | undefined;
		try {
			token = JSON.parse(localStorage.getItem('openoxide-auth-session') ?? 'null')?.tokens?.access_token;
		} catch {}

		// Connect directly via WebSocket to eliminate Polling -> WebSocket upgrade disconnect loops ("io server disconnect")
		const currentSocket = io('/terminal', {
			path: '/socket.io',
			transports: ['websocket'],
			reconnection: true,
			reconnectionAttempts: 10,
			reconnectionDelay: 1000,
			auth: (callback) => {
				let currentToken = token;
				try {
					currentToken = JSON.parse(localStorage.getItem('openoxide-auth-session') ?? 'null')?.tokens?.access_token;
				} catch {}
				callback({ token: currentToken });
			},
		});

		socketRef.current = currentSocket;

		const startTerminal = () => {
			if (startedRef.current) return;
			startedRef.current = true;
			setStatus('connected');
			term?.writeln(`\x1b[32mSocket connected. Starting shell [${shell}] on '${targetContainer}'...\x1b[0m\r\n`);

			if (isRemoteServer && serverId) {
				currentSocket.emit('server:start', { server_id: serverId, command: shell });
			} else {
				currentSocket.emit('docker:start', { container: targetContainer, shell });
			}
		};

		currentSocket.on('connect', () => {
			startTerminal();
		});

		currentSocket.on('connect_error', (error: Error) => {
			if (socketRef.current !== currentSocket) return;
			setStatus('error');
			term?.writeln(`\r\n\x1b[31mTerminal connection failed: ${error.message}\x1b[0m\r\n`);
		});

		currentSocket.on('started', (data: { kind?: string }) => {
			term?.writeln(`\x1b[32mTerminal session started (${data?.kind || 'docker'}). Type commands below:\x1b[0m\r\n`);
			term?.focus();
		});

		currentSocket.on('output', (evt: { data: string }) => {
			if (evt?.data) term?.write(evt.data);
		});

		currentSocket.on('error', (err: unknown) => {
			const msg = typeof err === 'string' ? err : (err as { message?: string })?.message || 'Error';
			term?.writeln(`\r\n\x1b[31mError: ${msg}\x1b[0m\r\n`);
			setStatus('error');
		});

		currentSocket.on('exit', (evt: { code: number }) => {
			term?.writeln(`\r\n\x1b[33mProcess exited with code ${evt?.code ?? 0}\x1b[0m\r\n`);
			setStatus('disconnected');
			startedRef.current = false;
		});

		currentSocket.on('disconnect', (reason) => {
			if (socketRef.current !== currentSocket) return;
			if (reason === 'io client disconnect') {
				setStatus('disconnected');
			} else {
				setStatus('disconnected');
				term?.writeln(`\r\n\x1b[31mSocket disconnected (${reason}). Reconnecting...\x1b[0m\r\n`);
				startedRef.current = false;
			}
		});

		return () => {
			currentSocket.disconnect();
			if (socketRef.current === currentSocket) {
				socketRef.current = null;
			}
			startedRef.current = false;
		};
	}, [isOpen, targetContainer, shell, isRemoteServer, serverId, termRef]);

	return { status, socketRef };
}
