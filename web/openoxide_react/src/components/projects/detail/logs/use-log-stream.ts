import {useState, useEffect, useCallback} from 'react';

type LogEvent = {payload?: string; error?: string};

/**
 * Drives a ConnectRPC log stream: buffers chunks on newline boundaries,
 * strips carriage returns and keep-alive frames, and accumulates clean
 * lines. Resubscribes whenever `deps` change (or `refetch` fires) and
 * aborts the previous stream on cleanup.
 */
export function useLogStream(
	subscribe: (signal: AbortSignal) => AsyncIterable<LogEvent>,
	deps: unknown[],
	endMessage?: string,
) {
	const [logs, setLogs] = useState<string[]>([]);
	const [isLoading, setIsLoading] = useState(true);
	const [nonce, setNonce] = useState(0);

	useEffect(() => {
		let isMounted = true;
		const controller = new AbortController();
		setLogs([]);
		setIsLoading(true);

		(async () => {
			let buffer = '';
			try {
				const call = subscribe(controller.signal);
				setIsLoading(false);
				for await (const event of call) {
					if (!isMounted) return;
					if (event.error) {
						setLogs(prev => [...prev, event.error!]);
						continue;
					}
					buffer += event.payload ?? '';
					const rawLines = buffer.split('\n');
					buffer = rawLines.pop() || '';
					for (const rawLine of rawLines) {
						const cleaned = rawLine.replace(/\r$/, '').trim();
						if (!cleaned || cleaned.includes('keep-alive')) continue;
						if (isMounted) setLogs(prev => [...prev, cleaned]);
					}
				}
				if (buffer.trim() && isMounted) {
					setLogs(prev => [...prev, buffer.trim()]);
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted && endMessage) {
					setLogs(prev => [...prev, endMessage]);
				}
			} finally {
				if (isMounted) setIsLoading(false);
			}
		})();

		return () => {
			isMounted = false;
			controller.abort();
		};
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [...deps, nonce]);

	const refetch = useCallback(() => setNonce(n => n + 1), []);
	return {logs, setLogs, isLoading, refetch};
}

/** Serialize accumulated log lines and trigger a browser download. */
export const downloadLogs = (lines: string[], filename: string) => {
	const blob = new Blob([lines.join('\n')], {type: 'text/plain'});
	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = filename;
	a.click();
	URL.revokeObjectURL(url);
};
