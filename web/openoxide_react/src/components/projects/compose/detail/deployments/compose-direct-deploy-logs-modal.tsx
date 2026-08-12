import {useState, useEffect} from 'react';
import {createPortal} from 'react-dom';
import {Terminal as TerminalIcon, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {extractLogLines} from '#/hooks/deployments/use-deployment-logs';
import {$api} from '#/api/query';

interface ComposeDirectDeployLogsModalProps {
	isOpen: boolean;
	onClose: () => void;
	composeId: number;
	serviceName?: string;
}

export function ComposeDirectDeployLogsModal({
	isOpen,
	onClose,
	composeId,
	serviceName,
}: ComposeDirectDeployLogsModalProps) {
	const [liveLogs, setLiveLogs] = useState<string[]>([]);
	const [isConnecting, setIsConnecting] = useState(true);

	// Fetch latest deployment for this compose stack
	const {data: rawDeployments = [], isLoading: isLoadingDeployments} = $api.useQuery(
		'get',
		'/deployments',
		{
			params: {
				query: {
					compose_id: composeId,
					limit: 5,
				} as any,
			},
		},
		{
			enabled: isOpen && !!composeId,
		}
	);

	const deployments = Array.isArray(rawDeployments) ? rawDeployments : [];
	const latestDeployment = deployments[0];
	const latestId = latestDeployment?.id;

	useEffect(() => {
		if (!isOpen || !latestId) return;

		let isMounted = true;
		let controller = new AbortController();
		setIsConnecting(true);

		const initialFallback = (latestDeployment as any)?.log_content || (latestDeployment as any)?.message || latestDeployment?.description;
		if (initialFallback) {
			setLiveLogs(initialFallback.split('\n'));
		} else {
			setLiveLogs([]);
		}

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

				const response = await fetch(`/api/deployments/${latestId}/logs`, {
					headers: {
						Authorization: accessToken ? `Bearer ${accessToken}` : '',
					},
					signal: controller.signal,
				});

				if (isMounted) setIsConnecting(false);

				if (!response.ok) {
					if (isMounted && initialFallback) {
						setLiveLogs(initialFallback.split('\n'));
					}
					return;
				}

				const reader = response.body?.getReader();
				const decoder = new TextDecoder();
				if (!reader) return;

				let buffer = '';

				while (isMounted) {
					const {done, value} = await reader.read();
					if (done) {
						if (buffer.trim()) {
							const finalLines = extractLogLines(buffer);
							if (finalLines.length > 0 && isMounted) {
								setLiveLogs(prev => [...prev, ...finalLines]);
							}
						}
						break;
					}

					buffer += decoder.decode(value, {stream: true});
					const lines = buffer.split('\n');
					buffer = lines.pop() || '';

					for (const line of lines) {
						const extracted = extractLogLines(line);
						if (extracted.length > 0 && isMounted) {
							setLiveLogs(prev => [...prev, ...extracted]);
						}
					}
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted) {
					setIsConnecting(false);
					if (initialFallback) {
						setLiveLogs(initialFallback.split('\n'));
					}
				}
			}
		};

		startStream();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [isOpen, latestId, latestDeployment]);

	if (!isOpen || typeof document === 'undefined') return null;

	const modalJSX = (
		<div className="fixed inset-0 z-[999999] bg-black/75 backdrop-blur-md flex items-center justify-center p-4">
			<div className="bg-[#09090b] border border-border/80 rounded-2xl shadow-2xl w-full max-w-6xl h-[84vh] flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-150">
				{/* Modal Header */}
				<div className="px-5 py-3.5 border-b border-border/60 flex items-center justify-between bg-card/30 shrink-0">
					<div className="flex items-center gap-2.5">
						<TerminalIcon className="w-4 h-4 text-primary" />
						<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
							Deployment Logs
							{serviceName && (
								<span className="text-xs font-mono font-normal px-2 py-0.5 rounded bg-primary/10 text-primary border border-primary/20">
									{serviceName}
								</span>
							)}
						</h3>
					</div>
					<Button
						variant="ghost"
						size="icon"
						onClick={onClose}
						className="h-7 w-7 text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded-lg"
					>
						<X className="w-4 h-4" />
					</Button>
				</div>

				{/* Single Seamless Log Viewer */}
				<div className="flex-1 p-4 overflow-y-auto bg-[#09090b] flex flex-col">
					<DeploymentViewer
						logs={liveLogs}
						isLoading={isLoadingDeployments || isConnecting}
						isLive={true}
						borderless={true}
						heightClass="h-full"
						loadingText="Connecting to deployment logs..."
						emptyText="No deployment logs recorded."
					/>
				</div>
			</div>
		</div>
	);

	return createPortal(modalJSX, document.body);
}
