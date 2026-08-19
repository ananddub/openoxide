import {useState, useEffect} from 'react';
import {createPortal} from 'react-dom';
import {Terminal as TerminalIcon, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';

interface ComposeDirectContainerLogsModalProps {
	isOpen: boolean;
	onClose: () => void;
	compose: any;
	serviceName?: string;
}

export function ComposeDirectContainerLogsModal({
	isOpen,
	onClose,
	compose,
	serviceName,
}: ComposeDirectContainerLogsModalProps) {
	const [streamedLogs, setStreamedLogs] = useState<string[]>([]);
	const [isLoading, setIsLoading] = useState(true);

	const activeService = serviceName || compose?.app_name || 'app';

	useEffect(() => {
		if (!isOpen) return;

		let isMounted = true;
		let controller = new AbortController();

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

				const streamUrl = `/api/deployments/docker/service/${activeService}/logs?tail=500&follow=true`;

				const response = await fetch(streamUrl, {
					headers: {
						Authorization: accessToken ? `Bearer ${accessToken}` : '',
					},
					signal: controller.signal,
				});

				if (isMounted) setIsLoading(false);

				if (!response.ok) return;

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
						const trimmed = line.trim();
						if (
							!trimmed ||
							trimmed.startsWith('event:') ||
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
	}, [isOpen, activeService]);

	if (!isOpen || typeof document === 'undefined') return null;

	const modalJSX = (
		<div className="fixed inset-0 z-[999999] flex items-center justify-center bg-black/75 p-4 backdrop-blur-md">
			<div className="flex h-[84vh] w-full max-w-6xl animate-in flex-col overflow-hidden rounded-2xl border border-border/80 bg-[#09090b] shadow-2xl duration-150 zoom-in-95 fade-in">
				{/* Modal Header */}
				<div className="flex shrink-0 items-center justify-between border-b border-border/60 bg-card/30 px-5 py-3.5">
					<div className="flex items-center gap-2.5">
						<TerminalIcon className="h-4 w-4 text-primary" />
						<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
							Container Logs
							<span className="rounded border border-primary/20 bg-primary/10 px-2 py-0.5 font-mono text-xs font-normal text-primary">
								{activeService}
							</span>
						</h3>
					</div>
					<Button
						variant="ghost"
						size="icon"
						onClick={onClose}
						className="h-7 w-7 rounded-lg text-muted-foreground hover:bg-muted/80 hover:text-foreground">
						<X className="h-4 w-4" />
					</Button>
				</div>

				{/* Single Seamless Log Viewer */}
				<div className="flex flex-1 flex-col overflow-y-auto bg-[#09090b] p-4">
					<DeploymentViewer
						logs={streamedLogs}
						isLoading={isLoading}
						isLive={true}
						borderless={true}
						heightClass="h-full"
						loadingText={`Connecting to '${activeService}' container logs...`}
						emptyText={`No log output received for '${activeService}'.`}
					/>
				</div>
			</div>
		</div>
	);

	return createPortal(modalJSX, document.body);
}
