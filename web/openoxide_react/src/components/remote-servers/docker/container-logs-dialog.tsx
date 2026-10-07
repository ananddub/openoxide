import {useEffect, useState} from 'react';
import {Download, Loader2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {formatApiError} from '#/api/utils';
import {createServiceClient} from '#/api/live/query';
import {downloadTextFile} from '#/utils/download-text-file';
import type {ServerContainer} from '#/api/live/docker';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';

const deploymentClient = createServiceClient(DeploymentService);

/** How many stored lines the stream replays before going live. */
const REPLAY_LINES = 500n;

/**
 * `docker logs` for one container on this server. The stream opens with the
 * dialog and aborts when it closes, so an idle panel holds no connection to
 * the agent — the only way this costs RAM is while somebody is looking.
 */
export function ContainerLogsDialog({
	serverId,
	container,
	onClose,
}: {
	serverId: bigint;
	container: ServerContainer;
	onClose: () => void;
}) {
	const [logs, setLogs] = useState<string[]>([]);
	const [isLoading, setIsLoading] = useState(true);
	const [follow, setFollow] = useState(true);
	const [streamError, setStreamError] = useState<string | null>(null);
	const [isExporting, setIsExporting] = useState(false);

	useEffect(() => {
		const controller = new AbortController();
		let mounted = true;

		setLogs([]);
		setIsLoading(true);
		setStreamError(null);

		const run = async () => {
			let buffer = '';
			try {
				const stream = await deploymentClient.streamContainerLogs(
					{
						target: container.name,
						serverId,
						tail: REPLAY_LINES,
						follow,
					},
					{signal: controller.signal},
				);
				setIsLoading(false);

				for await (const event of stream) {
					if (!mounted) return;
					const failure = event.error;
					if (failure) {
						setLogs(previous => [...previous, failure]);
						continue;
					}
					buffer += event.payload;
					const lines = buffer.split('\n');
					buffer = lines.pop() ?? '';
					const finished = lines
						.map(line => line.replace(/\r$/, '').trim())
						.filter(line => line !== '' && !line.includes('keep-alive'));
					if (finished.length > 0) {
						setLogs(previous => [...previous, ...finished]);
					}
				}
				if (buffer.trim() !== '') {
					setLogs(previous => [...previous, buffer.trim()]);
				}
			} catch (error) {
				if (!mounted || controller.signal.aborted) return;
				setIsLoading(false);
				setStreamError(formatApiError(error));
			}
		};

		void run();

		return () => {
			mounted = false;
			controller.abort();
		};
	}, [container.name, serverId, follow]);

	const exportLogs = async () => {
		setIsExporting(true);
		try {
			const result = await deploymentClient.exportContainerLogs({
				target: container.name,
				serverId,
			});
			if (
				downloadTextFile(
					result.fileName || `${container.name}.log`,
					result.content,
				)
			) {
				toast.success(`Exported logs for ${container.name}`);
			}
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setIsExporting(false);
		}
	};

	return (
		<Dialog open onOpenChange={open => !open && onClose()}>
			<DialogContent className="max-w-5xl">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2 text-sm">
						Logs — {container.name}
						<span className="font-mono text-[11px] font-normal text-muted-foreground">
							{container.image}
						</span>
						<Button
							variant="outline"
							size="sm"
							disabled={isExporting}
							onClick={() => void exportLogs()}
							className="ml-auto h-7 cursor-pointer gap-1.5 text-xs">
							{isExporting ? (
								<Loader2 className="size-3.5 animate-spin" />
							) : (
								<Download className="size-3.5" />
							)}
							Export
						</Button>
					</DialogTitle>
				</DialogHeader>
				{streamError && (
					<p className="rounded-lg bg-rose-500/10 px-3 py-2 text-xs text-rose-500">
						{streamError}
					</p>
				)}
				<div className="h-[70vh]">
					<DeploymentViewer
						logs={logs}
						isLoading={isLoading}
						isLive={follow}
						borderless
						heightClass="h-full"
						onToggleLive={() => setFollow(value => !value)}
						loadingText={`Opening the log stream for '${container.name}'…`}
						emptyText={`'${container.name}' has produced no log output.`}
					/>
				</div>
			</DialogContent>
		</Dialog>
	);
}
