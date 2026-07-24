import * as React from 'react';
import {toast} from 'sonner';
import type {Deployment} from './use-deployments';

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
					throw new Error('Failed to fetch logs');
				}

				setIsLogsLoading(false);
				const reader = response.body?.getReader();
				const decoder = new TextDecoder();

				if (!reader) return;

				let buffer = '';
				while (isMounted) {
					const {done, value} = await reader.read();
					if (done) break;

					buffer += decoder.decode(value, {stream: true});
					const rawLines = buffer.split('\n');
					buffer = rawLines.pop() || '';

					for (const rawLine of rawLines) {
						const trimmed = rawLine.trim();
						if (
							!trimmed ||
							trimmed.startsWith('event:') ||
							trimmed.includes('event: log') ||
							trimmed.includes('event: deployment') ||
							trimmed.startsWith('id:') ||
							trimmed.startsWith(':') ||
							trimmed.includes('keep-alive')
						) {
							continue;
						}

						if (trimmed.startsWith('data:')) {
							try {
								const jsonStr = trimmed.slice(5).trim();
								if (!jsonStr || jsonStr.includes('keep-alive')) continue;
								const data = JSON.parse(jsonStr);
								if (data.type === 'keep-alive') continue;
								const lineContent = data.line !== undefined ? data.line : (data.data || data.message || jsonStr);
								if (lineContent && isMounted) {
									setLogs(prev => prev ? `${prev}\n${lineContent}` : lineContent);
								}
							} catch {
								const rawContent = trimmed.slice(5).trim();
								if (rawContent && !rawContent.includes('keep-alive') && isMounted) {
									setLogs(prev => prev ? `${prev}\n${rawContent}` : rawContent);
								}
							}
						} else if (trimmed && isMounted) {
							if (
								!trimmed.startsWith('event:') &&
								!trimmed.includes('event: log') &&
								!trimmed.includes('event: deployment') &&
								!trimmed.includes('keep-alive')
							) {
								setLogs(prev => prev ? `${prev}\n${trimmed}` : trimmed);
							}
						}
					}
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted) {
					toast.error('Failed to load logs');
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
