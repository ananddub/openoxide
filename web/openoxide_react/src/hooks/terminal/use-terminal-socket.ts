import {useEffect, useRef, useState} from 'react';
import type {Terminal} from '@xterm/xterm';
import {createServiceClient} from '#/api/connect';
import {
	TerminalService,
	TerminalKind,
	type OpenTerminalRequest,
} from '#/gen/connect/openoxide/api/v1/terminal_pb';

const CONTROL_KEY_MAP: Record<string, string> = {
	l: '\x0c',
	c: '\x03',
	d: '\x04',
	z: '\x1a',
	u: '\x15',
	a: '\x01',
	e: '\x05',
	k: '\x0b',
	w: '\x17',
};

interface UseTerminalSocketOptions {
	isOpen: boolean;
	targetContainer: string;
	shell: string;
	isRemoteServer: boolean;
	serverId?: number;
	termInstance: Terminal | null;
}

/**
 * Interactive terminal over the TerminalService ConnectRPC API.
 *
 * Browsers cannot stream request bodies over the Connect protocol, so the
 * session is split in two legs (mirroring the backend proto): a
 * server-streaming OpenTerminal RPC that carries all output for the
 * session lifetime, and a unary ExecutionCommand RPC that feeds input,
 * resize and stop commands addressed by session id.
 */
export function useTerminalSocket({
	isOpen,
	targetContainer,
	shell,
	isRemoteServer,
	serverId,
	termInstance,
}: UseTerminalSocketOptions) {
	const [status, setStatus] = useState<
		'connecting' | 'connected' | 'error' | 'disconnected'
	>('disconnected');
	const [activeHostIp, setActiveHostIp] = useState<string | null>(null);
	const sessionIdRef = useRef<string | null>(null);
	const abortRef = useRef<AbortController | null>(null);
	const client = createServiceClient(TerminalService);

	useEffect(() => {
		if (!isOpen || !termInstance) {
			abortRef.current?.abort();
			abortRef.current = null;
			sessionIdRef.current = null;
			setStatus('disconnected');
			setActiveHostIp(null);
			return;
		}

		setStatus('connecting');
		termInstance.reset();
		if (isRemoteServer) {
			termInstance.writeln(
				`\x1b[33mConnecting to Remote Server [${targetContainer}] via SSH...\x1b[0m\r\n`,
			);
		} else {
			termInstance.writeln(
				`\x1b[33mConnecting to Docker Container [${targetContainer}]...\x1b[0m\r\n`,
			);
		}

		const abort = new AbortController();
		abortRef.current = abort;
		const signal = abort.signal;

		const sendCommand = (
			command:
				| {case: 'input'; value: {data: Uint8Array}}
				| {case: 'resize'; value: {cols: number; rows: number}}
				| {case: 'stop'; value: Record<string, never>},
		): void => {
			const sessionId = sessionIdRef.current;
			if (!sessionId) return;
			client
				.executionCommand({sessionId, command})
				.catch(() => {});
		};

		const request: OpenTerminalRequest = {
			kind: isRemoteServer
				? TerminalKind.REMOTE_SERVER
				: TerminalKind.DOCKER,
			serverId: serverId ? BigInt(serverId) : undefined,
			container: isRemoteServer ? undefined : targetContainer,
			shell,
			cols: termInstance.cols || 80,
			rows: termInstance.rows || 24,
		};

		(async () => {
			try {
				for await (const event of client.openTerminal(request, {signal})) {
					if (signal.aborted) return;
					if (event.sessionId) sessionIdRef.current = event.sessionId;
					switch (event.payload.case) {
						case 'started': {
							setStatus('connected');
							if (event.payload.value.host) {
								setActiveHostIp(event.payload.value.host);
							}
							termInstance.reset();
							const connectedTarget =
								event.payload.value.host || targetContainer;
							const label = isRemoteServer
								? 'SSH Remote Server'
								: 'Docker Container';
							termInstance.writeln(
								`\x1b[32mTerminal session started on ${connectedTarget} (${label}). Type commands below:\x1b[0m\r\n`,
							);
							termInstance.focus();
							break;
						}
						case 'output': {
							const text = new TextDecoder().decode(
								event.payload.value.data,
							);
							if (text) termInstance.write(text);
							break;
						}
						case 'error': {
							termInstance.writeln(
								`\r\n\x1b[31mError: ${event.payload.value.message}\x1b[0m\r\n`,
							);
							setStatus('error');
							return;
						}
						case 'exit': {
							termInstance.writeln(
								`\r\n\x1b[33mProcess exited with code ${event.payload.value.code ?? 0}\x1b[0m\r\n`,
							);
							setStatus('disconnected');
							return;
						}
					}
				}
				if (!signal.aborted) {
					setStatus('disconnected');
				}
			} catch (error) {
				if (signal.aborted) return;
				const message =
					error instanceof Error ? error.message : 'terminal stream failed';
				termInstance.writeln(`\r\n\x1b[31mError: ${message}\x1b[0m\r\n`);
				setStatus('error');
			}
		})();

		const dataDisposable = termInstance.onData(data => {
			if (sessionIdRef.current) {
				sendCommand({
					case: 'input',
					value: {data: new TextEncoder().encode(data)},
				});
			}
		});
		const resizeDisposable = termInstance.onResize(({cols, rows}) => {
			if (sessionIdRef.current) {
				sendCommand({case: 'resize', value: {cols, rows}});
			}
		});

		termInstance.attachCustomKeyEventHandler((event: KeyboardEvent) => {
			if (event.ctrlKey || event.metaKey) {
				const k = event.key.toLowerCase();
				if (k === 'c' && termInstance.hasSelection()) {
					if (event.type === 'keydown')
						navigator.clipboard.writeText(termInstance.getSelection());
					event.preventDefault();
					return false;
				}
				if (k === 'v') {
					if (event.type === 'keydown') {
						navigator.clipboard
							.readText()
							.then(text => {
								if (text && sessionIdRef.current) {
									sendCommand({
										case: 'input',
										value: {data: new TextEncoder().encode(text)},
									});
								}
							})
							.catch(() => {});
					}
					event.preventDefault();
					return false;
				}
				if (CONTROL_KEY_MAP[k]) {
					if (event.type === 'keydown' && sessionIdRef.current) {
						if (k === 'l') {
							termInstance.clear();
							sendCommand({
								case: 'input',
								value: {data: new TextEncoder().encode('clear\r')},
							});
						} else {
							sendCommand({
								case: 'input',
								value: {
									data: new TextEncoder().encode(CONTROL_KEY_MAP[k]),
								},
							});
						}
					}
					event.preventDefault();
					return false;
				}
			}
			return true;
		});

		return () => {
			dataDisposable?.dispose();
			resizeDisposable?.dispose();
			sendCommand({case: 'stop', value: {}});
			abort.abort();
			abortRef.current = null;
			sessionIdRef.current = null;
		};
	}, [
		isOpen,
		targetContainer,
		shell,
		isRemoteServer,
		serverId,
		termInstance,
	]);

	return {
		sessionId: sessionIdRef.current,
		status,
		activeHostIp,
	};
}
