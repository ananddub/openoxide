import {useState, useEffect} from 'react';
import {createPortal} from 'react-dom';
import {Terminal as TerminalIcon, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';

const deploymentClient = createServiceClient(DeploymentService);

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
				const call = deploymentClient.streamServiceLogs(
					{
						target: activeService,
						serverId: compose?.server_id
							? BigInt(compose.server_id)
							: undefined,
						tail: 500n,
						follow: true,
					},
					{signal: controller.signal},
				);
				setIsLoading(false);

				let buffer = '';
				for await (const event of call) {
					if (!isMounted) return;
					if (event.error) {
						setStreamedLogs(prev => [...prev, event.error!]);
						continue;
					}
					buffer += event.payload;
					const lines = buffer.split('\n');
					buffer = lines.pop() || '';
					for (const rawLine of lines) {
						const cleaned = rawLine.replace(/\r$/, '').trim();
						if (!cleaned || cleaned.includes('keep-alive')) continue;
						if (isMounted) setStreamedLogs(prev => [...prev, cleaned]);
					}
				}
				if (buffer.trim() && isMounted) {
					setStreamedLogs(prev => [...prev, buffer.trim()]);
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
	}, [isOpen, activeService, compose?.server_id]);

	if (!isOpen || typeof document === 'undefined') return null;

	const modalJSX = (
		<div className="fixed inset-0 z-[999999] flex items-center justify-center bg-black/75 p-4 backdrop-blur-md">
			<div className="flex h-[84vh] w-full max-w-6xl animate-in flex-col overflow-hidden rounded-xl bg-[#09090b] shadow-lg duration-150 zoom-in-95 fade-in">
				{/* Modal Header */}
				<div className="flex shrink-0 items-center justify-between border-b  bg-card/30 px-5 py-3.5">
					<div className="flex items-center gap-2.5">
						<TerminalIcon className="h-4 w-4 text-primary" />
						<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
							Container Logs
							<span className="rounded bg-primary/10 px-2 py-0.5 font-mono text-xs font-normal text-primary">
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
