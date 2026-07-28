import * as React from 'react';
import {toast} from 'sonner';
import type {Deployment} from './use-deployments';

export function extractLogLines(rawData: unknown): string[] {
	if (!rawData) return [];
	if (typeof rawData === 'string') {
		const trimmed = rawData.trim();
		if (
			!trimmed ||
			trimmed.startsWith('event:') ||
			trimmed.includes('event: log') ||
			trimmed.includes('event: deployment') ||
			trimmed.startsWith('id:') ||
			trimmed.startsWith(':') ||
			trimmed.includes('keep-alive')
		) {
			return [];
		}
		if (trimmed.startsWith('data:')) {
			const content = trimmed.slice(5).trim();
			if (!content || content.includes('keep-alive')) return [];
			try {
				const parsed = JSON.parse(content);
				return extractLogLines(parsed);
			} catch {
				return content.split('\n').filter(l => l.trim() && !l.includes('keep-alive'));
			}
		}
		return rawData.split('\n');
	}

	if (typeof rawData === 'object' && rawData !== null) {
		const obj = rawData as Record<string, unknown>;
		if (obj.type === 'keep-alive' || obj.event === 'keep-alive') return [];
		const candidate = obj.line ?? obj.message ?? obj.data ?? obj.log ?? obj.text ?? obj.content ?? obj.output;
		if (candidate !== undefined && candidate !== null) {
			if (typeof candidate === 'object') {
				return [JSON.stringify(candidate)];
			}
			return String(candidate).split('\n').filter(l => l.trim() && !l.includes('keep-alive'));
		}
		return [JSON.stringify(rawData)];
	}

	return [String(rawData)];
}

export function useDeploymentLogs(selectedDeployment: Deployment | null) {
	const [logs, setLogs] = React.useState<string>('');
	const [isLogsLoading, setIsLogsLoading] = React.useState(false);
	const [copied, setCopied] = React.useState(false);

	// Copy logs helper
	const handleCopyLogs = () => {
		navigator.clipboard.writeText(logs);
		setCopied(true);
		toast.success('Logs copied to clipboard');
		setTimeout(() => setCopied(false), 2000);
	};

	// Parse logs reader
	React.useEffect(() => {
		if (!selectedDeployment) return;

		let isMounted = true;
		let controller = new AbortController();
		setLogs('');
		setIsLogsLoading(true);

		const depWithLogs = selectedDeployment as Deployment & {log_content?: string};

		// If deployment has stored log_content, initialize with it immediately
		if (depWithLogs.log_content) {
			setLogs(depWithLogs.log_content);
		}

		const readLogs = async () => {
			try {
				const sessionRaw = localStorage.getItem('rustploy-auth-session');
				let accessToken = '';
				if (sessionRaw) {
					try {
						const session = JSON.parse(sessionRaw);
						accessToken = session?.tokens?.access_token || '';
					} catch {}
				}

				const response = await fetch(
					`/api/deployments/${selectedDeployment.id}/logs`,
					{
						headers: {
							Authorization: accessToken ? `Bearer ${accessToken}` : '',
						},
						signal: controller.signal,
					},
				);

				if (!response.ok) {
					if (isMounted && depWithLogs.log_content) {
						setLogs(depWithLogs.log_content);
					}
					setIsLogsLoading(false);
					return;
				}

				const reader = response.body?.getReader();
				if (!reader) {
					if (isMounted && depWithLogs.log_content) {
						setLogs(depWithLogs.log_content);
					}
					setIsLogsLoading(false);
					return;
				}

				const decoder = new TextDecoder();
				let buffer = '';
				while (isMounted) {
					const {done, value} = await reader.read();
					if (done) {
						if (buffer.trim()) {
							const finalLines = extractLogLines(buffer);
							if (finalLines.length > 0 && isMounted) {
								setLogs(prev => prev ? `${prev}\n${finalLines.join('\n')}` : finalLines.join('\n'));
							}
						}
						break;
					}

					buffer += decoder.decode(value, {stream: true});
					const rawLines = buffer.split('\n');
					buffer = rawLines.pop() || '';

					for (const rawLine of rawLines) {
						const extracted = extractLogLines(rawLine);
						if (extracted.length > 0 && isMounted) {
							setLogs(prev => prev ? `${prev}\n${extracted.join('\n')}` : extracted.join('\n'));
						}
					}
				}
			} catch (err: unknown) {
				if ((err as {name?: string})?.name !== 'AbortError' && isMounted) {
					if (depWithLogs.log_content) {
						setLogs(depWithLogs.log_content);
					}
					setIsLogsLoading(false);
				}
			}
		};

		readLogs();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [selectedDeployment]);

	return {
		logs,
		isLogsLoading,
		copied,
		handleCopyLogs,
	};
}
