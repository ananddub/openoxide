import { useEffect, useRef, useState } from 'react';
import type { Socket } from 'socket.io-client';
import { socketFor } from '#/live/socket';
import type { Terminal } from '@xterm/xterm';

const CONTROL_KEY_MAP: Record<string, string> = {
	l: '\x0c', c: '\x03', d: '\x04', z: '\x1a', u: '\x15', a: '\x01', e: '\x05', k: '\x0b', w: '\x17',
};

interface UseTerminalSocketOptions {
	isOpen: boolean;
	targetContainer: string;
	shell: string;
	isRemoteServer: boolean;
	serverId?: number;
	termInstance: Terminal | null;
}

export function useTerminalSocket({
	isOpen,
	targetContainer,
	shell,
	isRemoteServer,
	serverId,
	termInstance,
}: UseTerminalSocketOptions) {
	const [status, setStatus] = useState<'connecting' | 'connected' | 'error' | 'disconnected'>('disconnected');
	const [activeHostIp, setActiveHostIp] = useState<string | null>(null);
	const socketRef = useRef<Socket | null>(null);
	const isFirstMountRef = useRef<boolean>(true);

	useEffect(() => {
		if (!isOpen) {
			isFirstMountRef.current = true;
			if (socketRef.current) {
				socketRef.current.off('connect');
				socketRef.current.off('started');
				socketRef.current.off('output');
				socketRef.current.off('error');
				socketRef.current.off('exit');
				socketRef.current.off('disconnect');
				socketRef.current = null;
			}
			setStatus('disconnected');
			setActiveHostIp(null);
			return;
		}

		isFirstMountRef.current = true;
		setStatus('connecting');

		if (termInstance) {
			termInstance.reset();
			if (isRemoteServer) {
				termInstance.writeln(`\x1b[33mConnecting to Remote Server [${targetContainer}] via SSH...\x1b[0m\r\n`);
			} else {
				termInstance.writeln(`\x1b[33mConnecting to Docker Container [${targetContainer}]...\x1b[0m\r\n`);
			}
		}

		// Use global singleton live socket manager
		const socket = socketFor('/terminal').socket;
		socketRef.current = socket;

		if (!socket.connected) {
			socket.connect();
		}

		const emitStartSession = (sock: Socket, shellMode: string) => {
			if (!sock.connected) return;
			const cols = termInstance?.cols || 80;
			const rows = termInstance?.rows || 24;
			if (isRemoteServer && serverId) {
				sock.emit('server:start', { server_id: serverId, shell: shellMode, command: shellMode, cols, rows });
			} else {
				sock.emit('docker:start', { container: targetContainer, server_id: serverId || undefined, shell: shellMode, cols, rows });
			}
		};

		const handleConnect = () => {
			setStatus('connected');
			emitStartSession(socket, shell);
		};

		socket.on('connect', handleConnect);

		if (socket.connected) {
			handleConnect();
		}

		socket.on('started', (data: { kind?: string; host?: string }) => {
			if (data?.host) setActiveHostIp(data.host);
			if (termInstance) {
				termInstance.reset();
				const connectedTarget = data?.host || targetContainer;
				const label = data?.kind === 'remote-server' ? 'SSH Remote Server' : 'Docker Container';
				termInstance.writeln(`\x1b[32mTerminal session started on ${connectedTarget} (${label}). Type commands below:\x1b[0m\r\n`);
				termInstance.focus();
				if (socket.connected && termInstance.cols && termInstance.rows) {
					socket.emit('resize', { cols: termInstance.cols, rows: termInstance.rows });
				}
			}
		});

		socket.on('output', (evt: { data: string }) => {
			if (evt?.data) {
				termInstance?.write(evt.data);
			}
		});

		socket.on('error', (err: unknown) => {
			const message = typeof err === 'string' ? err : (err as { message?: string })?.message || 'Terminal socket error';
			termInstance?.writeln(`\r\n\x1b[31mError: ${message}\x1b[0m\r\n`);
			setStatus('error');
		});

		socket.on('exit', (evt: { code: number }) => {
			termInstance?.writeln(`\r\n\x1b[33mProcess exited with code ${evt?.code ?? 0}\x1b[0m\r\n`);
			setStatus('disconnected');
		});

		socket.on('disconnect', (reason) => {
			if (socketRef.current !== socket) return;
			setStatus('disconnected');
			if (reason !== 'io client disconnect') {
				termInstance?.writeln(`\r\n\x1b[31mSocket disconnected (${reason}). Reconnecting...\x1b[0m\r\n`);
			}
		});

		let dataDisposable: { dispose: () => void } | undefined;
		let resizeDisposable: { dispose: () => void } | undefined;

		if (termInstance) {
			dataDisposable = termInstance.onData((data) => {
				if (socket.connected) socket.emit('input', { data });
			});
			resizeDisposable = termInstance.onResize(({ cols, rows }) => {
				if (socket.connected) socket.emit('resize', { cols, rows });
			});

			termInstance.attachCustomKeyEventHandler((event: KeyboardEvent) => {
				if (event.ctrlKey || event.metaKey) {
					const k = event.key.toLowerCase();
					if (k === 'c' && termInstance.hasSelection()) {
						if (event.type === 'keydown') navigator.clipboard.writeText(termInstance.getSelection());
						event.preventDefault(); return false;
					}
					if (k === 'v') {
						if (event.type === 'keydown') {
							navigator.clipboard.readText().then((text) => {
								if (text && socket.connected) socket.emit('input', { data: text });
							}).catch(() => {});
						}
						event.preventDefault(); return false;
					}
					if (CONTROL_KEY_MAP[k]) {
						if (event.type === 'keydown' && socket.connected) {
							if (k === 'l') { termInstance.clear(); socket.emit('input', { data: 'clear\r' }); }
							else { socket.emit('input', { data: CONTROL_KEY_MAP[k] }); }
						}
						event.preventDefault(); return false;
					}
				}
				return true;
			});
		}

		return () => {
			dataDisposable?.dispose();
			resizeDisposable?.dispose();
			isFirstMountRef.current = true;
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
	}, [isOpen, targetContainer, shell, isRemoteServer, serverId, termInstance]);

	// Dynamic Shell / Target Container Switch
	useEffect(() => {
		if (isFirstMountRef.current) {
			isFirstMountRef.current = false;
			return;
		}
		if (!isOpen || !socketRef.current?.connected || !termInstance) return;

		termInstance.reset();
		const connectedTarget = activeHostIp || targetContainer;
		if (isRemoteServer) {
			termInstance.writeln(`\x1b[33mSwitching shell to [${shell}] on Remote Server [${connectedTarget}]...\x1b[0m\r\n`);
		} else {
			termInstance.writeln(`\x1b[33mSwitching shell to [${shell}] on Container [${connectedTarget}]...\x1b[0m\r\n`);
		}
		setStatus('connecting');

		const cols = termInstance.cols || 80;
		const rows = termInstance.rows || 24;

		if (isRemoteServer && serverId) {
			socketRef.current.emit('server:start', { server_id: serverId, shell, command: shell, cols, rows });
		} else {
			socketRef.current.emit('docker:start', { container: targetContainer, server_id: serverId || undefined, shell, cols, rows });
		}
	}, [targetContainer, shell, isOpen, isRemoteServer, serverId, termInstance, activeHostIp]);

	return {
		socket: socketRef.current,
		status,
		activeHostIp,
	};
}
