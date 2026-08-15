import { useEffect, useRef, useState } from 'react';
import type { Socket } from 'socket.io-client';
import { socketFor } from '#/live/socket';
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
				socketRef.current.off('connect');
				socketRef.current.off('started');
				socketRef.current.off('output');
				socketRef.current.off('error');
				socketRef.current.off('exit');
				socketRef.current.off('disconnect');
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

		// Use global singleton live socket manager
		const currentSocket = socketFor('/terminal').socket;
		socketRef.current = currentSocket;

		if (!currentSocket.connected) {
			currentSocket.connect();
		}

		const handleConnect = () => {
			setStatus('connected');
			if (isRemoteServer && serverId) {
				currentSocket.emit('server:start', { server_id: serverId, shell, command: shell });
			} else {
				currentSocket.emit('docker:start', { container: targetContainer, shell });
			}
		};

		currentSocket.on('connect', handleConnect);

		if (currentSocket.connected) {
			handleConnect();
		}

		currentSocket.on('started', (data: { kind?: string; host?: string }) => {
			startedRef.current = true;
			const hostInfo = data?.host ? ` on ${data.host}` : '';
			term?.writeln(`\x1b[32mTerminal session started (${data?.kind || 'docker'}${hostInfo}). Type commands below:\x1b[0m\r\n`);
			term?.focus();
		});

		currentSocket.on('output', (evt: { data: string }) => {
			if (evt?.data) {
				term?.write(evt.data);
			}
		});

		currentSocket.on('error', (err: unknown) => {
			const message = typeof err === 'string' ? err : (err as { message?: string })?.message || 'Terminal socket error';
			term?.writeln(`\r\n\x1b[31mError: ${message}\x1b[0m\r\n`);
			setStatus('error');
		});

		currentSocket.on('exit', (evt: { code: number }) => {
			term?.writeln(`\r\n\x1b[33mProcess exited with code ${evt?.code ?? 0}\x1b[0m\r\n`);
			setStatus('disconnected');
		});

		currentSocket.on('disconnect', (reason) => {
			if (socketRef.current !== currentSocket) return;
			setStatus('disconnected');
			if (reason !== 'io client disconnect') {
				term?.writeln(`\r\n\x1b[31mSocket disconnected (${reason}). Reconnecting...\x1b[0m\r\n`);
			}
		});

		return () => {
			if (socketRef.current) {
				socketRef.current.off('connect', handleConnect);
				socketRef.current.off('started');
				socketRef.current.off('output');
				socketRef.current.off('error');
				socketRef.current.off('exit');
				socketRef.current.off('disconnect');
				socketRef.current = null;
			}
		};
	}, [isOpen, targetContainer, shell, isRemoteServer, serverId, termRef]);

	return {
		socket: socketRef.current,
		status,
	};
}
