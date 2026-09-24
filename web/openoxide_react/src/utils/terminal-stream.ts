import type {Terminal} from '@xterm/xterm';
import type {
	OpenTerminalRequest,
	TerminalEvent,
} from '#/gen/connect/openoxide/api/v1/terminal_pb';

export type TerminalStatus =
	| 'connecting'
	| 'connected'
	| 'error'
	| 'disconnected';

export interface TerminalStreamContext {
	client: {
		openTerminal: (
			request: OpenTerminalRequest,
			opts: {signal: AbortSignal},
		) => AsyncIterable<TerminalEvent>;
	};
	request: OpenTerminalRequest;
	signal: AbortSignal;
	term: Terminal;
	targetContainer: string;
	isRemoteServer: boolean;
	hostMode: boolean;
	setStatus: (status: TerminalStatus) => void;
	setActiveHostIp: (ip: string | null) => void;
	sessionIdRef: {current: string | null};
}

/** Drives the server-streaming OpenTerminal leg: renders output into the xterm instance and updates connection status as events arrive. Returns when the session ends (error/exit) or the stream closes; abort-safe. */
export async function consumeTerminalStream(
	ctx: TerminalStreamContext,
): Promise<void> {
	const {
		client,
		request,
		signal,
		term,
		targetContainer,
		isRemoteServer,
		hostMode,
		setStatus,
		setActiveHostIp,
		sessionIdRef,
	} = ctx;

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
					term.reset();
					const connectedTarget =
						event.payload.value.host || targetContainer;
					const label = isRemoteServer
						? 'SSH Remote Server'
						: hostMode
							? 'Host Shell'
							: 'Docker Container';
					term.writeln(
						`\x1b[32mTerminal session started on ${connectedTarget} (${label}). Type commands below:\x1b[0m\r\n`,
					);
					term.focus();
					break;
				}
				case 'output': {
					const text = new TextDecoder().decode(event.payload.value.data);
					if (text) term.write(text);
					break;
				}
				case 'error': {
					term.writeln(
						`\r\n\x1b[31mError: ${event.payload.value.message}\x1b[0m\r\n`,
					);
					setStatus('error');
					return;
				}
				case 'exit': {
					term.writeln(
						`\r\n\x1b[33mProcess exited with code ${event.payload.value.code ?? 0}\x1b[0m\r\n`,
					);
					setStatus('disconnected');
					return;
				}
			}
		}
		if (!signal.aborted) setStatus('disconnected');
	} catch (error) {
		if (signal.aborted) return;
		const message =
			error instanceof Error ? error.message : 'terminal stream failed';
		term.writeln(`\r\n\x1b[31mError: ${message}\x1b[0m\r\n`);
		setStatus('error');
	}
}
