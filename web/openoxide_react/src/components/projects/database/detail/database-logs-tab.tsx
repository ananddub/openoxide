import {useState, useEffect} from 'react';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';

interface DatabaseLogsTabProps {
	database: any;
}

export function DatabaseLogsTab({database}: DatabaseLogsTabProps) {
	const [isLoading, setIsLoading] = useState(true);
	const [streamedLogs, setStreamedLogs] = useState<string[]>([]);
	const [refetchTrigger, setRefetchTrigger] = useState(0);

	const appName = database?.app_name || database?.name || 'database';

	useEffect(() => {
		let isMounted = true;
		const controller = new AbortController();

		setStreamedLogs([]);
		setIsLoading(true);

		const startStream = async () => {
			try {
				const sessionRaw = localStorage.getItem('openoxide-auth-session');
				let accessToken = '';
				if (sessionRaw) {
					try {
						const session = JSON.parse(sessionRaw);
						accessToken = session?.tokens?.access_token || '';
					} catch {}
				}

				const serverIdParam = database?.server_id
					? `&server_id=${database.server_id}`
					: '';
				const streamUrl = `/api/deployments/docker/service/${encodeURIComponent(appName)}/logs?tail=200&timestamps=false&follow=true${serverIdParam}`;

				const response = await fetch(streamUrl, {
					headers: {
						Authorization: accessToken ? `Bearer ${accessToken}` : '',
					},
					signal: controller.signal,
				});

				if (!response.ok) {
					setIsLoading(false);
					return;
				}

				setIsLoading(false);
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

					for (const line of rawLines) {
						const trimmed = line.trim();
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

						if (line.startsWith('data:')) {
							try {
								const jsonStr = line.slice(5).trim();
								if (!jsonStr || jsonStr.includes('keep-alive')) continue;
								const data = JSON.parse(jsonStr);
								const text =
									data.line !== undefined
										? data.line
										: data.message || jsonStr;
								if (text && isMounted)
									setStreamedLogs(prev => [...prev, text]);
							} catch {
								const raw = line.slice(5).trim();
								if (raw && !raw.includes('keep-alive') && isMounted)
									setStreamedLogs(prev => [...prev, raw]);
							}
						} else if (line.trim() && isMounted) {
							setStreamedLogs(prev => [...prev, line]);
						}
					}
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted) {
					setIsLoading(false);
				}
			}
		};

		startStream();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [database?.id, appName, refetchTrigger]);

	const handleDownload = () => {
		const blob = new Blob([streamedLogs.join('\n')], {type: 'text/plain'});
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `${appName}-logs.txt`;
		a.click();
		URL.revokeObjectURL(url);
	};

	return (
		<DeploymentViewer
			logs={streamedLogs}
			isLoading={isLoading}
			isLive={true}
			isDeployment={false}
			loadingText={`Connecting to '${appName}' container logs...`}
			emptyText={`No runtime log output received for database '${appName}'.`}
			onDownload={handleDownload}
			onReload={() => setRefetchTrigger(prev => prev + 1)}
		/>
	);
}
