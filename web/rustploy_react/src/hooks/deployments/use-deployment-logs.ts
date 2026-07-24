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
					`http://das.tail25b5a0.ts.net:4000/deployments/${selectedDeployment.id}/logs`,
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
					const lines = buffer.split('\n');
					buffer = lines.pop() || '';

					for (const line of lines) {
						if (line.startsWith('data:')) {
							try {
								const jsonStr = line.slice(5).trim();
								if (jsonStr) {
									const data = JSON.parse(jsonStr);
									if (data.line) {
										setLogs(prev => prev + data.line + '\n');
									} else if (data.message) {
										setLogs(prev => prev + data.message + '\n');
									}
								}
							} catch {
								setLogs(prev => prev + line.slice(5) + '\n');
							}
						} else if (line.trim()) {
							setLogs(prev => prev + line + '\n');
						}
					}
				}
			} catch (err: any) {
				if (err.name !== 'AbortError') {
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
