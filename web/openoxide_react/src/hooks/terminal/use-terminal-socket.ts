import {useEffect, useRef, useState} from 'react';
import type {Terminal} from '@xterm/xterm';
import {create} from '@bufbuild/protobuf';
import {createServiceClient} from '#/api/connect';
import {
	TerminalService,
	TerminalKind,
	type OpenTerminalRequest,
	OpenTerminalRequestSchema,
} from '#/gen/connect/openoxide/api/v1/terminal_pb';
import {handleTerminalKey} from './terminal-keyhandler';
import {
	consumeTerminalStream,
	type TerminalStatus,
} from './terminal-stream';

interface UseTerminalSocketOptions {
	isOpen: boolean;
	targetContainer: string;
	shell: string;
	isRemoteServer: boolean;
	/** Escape through the Agent to the host itself instead of a container. */
	hostMode?: boolean;
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
	hostMode = false,
	serverId,
	termInstance,
}: UseTerminalSocketOptions) {
	const [status, setStatus] = useState<TerminalStatus>('disconnected');
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
		} else if (hostMode) {
			termInstance.writeln(
				`\x1b[33mOpening host root shell on [${targetContainer}]...\x1b[0m\r\n`,
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
			client.executionCommand({sessionId, command}).catch(() => {});
		};

		const sendInput = (text: string): void =>
			sendCommand({
				case: 'input',
				value: {data: new TextEncoder().encode(text)},
			});

		const request: OpenTerminalRequest = create(
			OpenTerminalRequestSchema,
			{
				kind: isRemoteServer
					? TerminalKind.REMOTE_SERVER
					: TerminalKind.DOCKER,
				serverId: serverId ? BigInt(serverId) : undefined,
				container:
					isRemoteServer || hostMode ? undefined : targetContainer,
				host: hostMode,
				shell,
				cols: termInstance.cols || 80,
				rows: termInstance.rows || 24,
			},
		);

		void consumeTerminalStream({
			client,
			request,
			signal,
			term: termInstance,
			targetContainer,
			isRemoteServer,
			hostMode,
			setStatus,
			setActiveHostIp,
			sessionIdRef,
		});

		const dataDisposable = termInstance.onData(data => {
			if (sessionIdRef.current) sendInput(data);
		});
		const resizeDisposable = termInstance.onResize(({cols, rows}) => {
			if (sessionIdRef.current) {
				sendCommand({case: 'resize', value: {cols, rows}});
			}
		});

		termInstance.attachCustomKeyEventHandler((event: KeyboardEvent) =>
			handleTerminalKey(event, termInstance, sendInput),
		);

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
		hostMode,
		serverId,
		termInstance,
	]);

	return {
		sessionId: sessionIdRef.current,
		status,
		activeHostIp,
	};
}
