import {useState, useRef, useEffect, useMemo} from 'react';
import {Box} from 'lucide-react';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';
import {LogStreamControls} from '#/components/projects/detail/logs/log-stream-controls';
import {
	useLogStream,
	downloadLogs,
} from '#/components/projects/detail/logs/use-log-stream';

const deploymentClient = createServiceClient(DeploymentService);

const LINE_OPTIONS = [
	{value: '50', label: '50 lines'},
	{value: '100', label: '100 lines'},
	{value: '200', label: '200 lines'},
	{value: '500', label: '500 lines'},
	{value: '1000', label: '1000 lines'},
];

interface LogsTabProps {
	app: any;
}

export function LogsTab({app}: LogsTabProps) {
	const [lines, setLines] = useState('500');
	const [timestamps, setTimestamps] = useState(false);
	const [isLive, setIsLive] = useState(true);
	const [selectedTarget, setSelectedTarget] = useState('');
	const scrollRef = useRef<HTMLDivElement>(null);

	const availableTargets = useMemo(() => {
		const list: string[] = [];
		const baseName = app?.app_name || app?.name;
		if (baseName) list.push(baseName);
		if (app?.containers && Array.isArray(app.containers)) {
			app.containers.forEach((c: any) => {
				const name = c?.name || c?.container_name;
				if (name && !list.includes(name)) list.push(name);
			});
		}
		return list.length > 0 ? list : ['app'];
	}, [app]);

	const activeTarget =
		selectedTarget ||
		availableTargets[0] ||
		app?.app_name ||
		app?.name ||
		'app';

	// ConnectRPC log stream: panel tails the container/service logs on the
	// Agent and pushes each chunk as a DeploymentEvent.
	const {
		logs: streamedLogs,
		isLoading,
		refetch,
	} = useLogStream(
		signal =>
			deploymentClient.streamServiceLogs(
				{
					target: activeTarget,
					serverId: app?.server_id ? BigInt(app.server_id) : undefined,
					tail: BigInt(lines),
					timestamps,
					follow: isLive,
				},
				{signal},
			),
		[activeTarget, lines, timestamps, isLive],
		'Container log stream ended.',
	);

	useEffect(() => {
		if (isLive && scrollRef.current) {
			scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
		}
	}, [streamedLogs, isLive]);

	const handleDownload = () =>
		downloadLogs(streamedLogs, `${activeTarget || 'logs'}.txt`);

	return (
		<div className="flex flex-col gap-4 rounded-xl bg-card p-5 shadow-sm">
			<div className="flex flex-wrap items-center justify-between gap-4">
				<div className="flex flex-wrap items-center gap-3">
					<div>
						<h3 className="text-sm font-bold text-foreground">
							Container Logs
						</h3>
						<p className="mt-1 text-xs text-muted-foreground">
							Real-time terminal output stream from the running container
						</p>
					</div>

					{availableTargets.length > 0 && (
						<Select
							value={activeTarget}
							onValueChange={val => {
								if (val) setSelectedTarget(val);
							}}>
							<SelectTrigger className="ml-2 h-9 min-w-[170px]  bg-muted/30 font-mono text-xs font-bold shadow-2xs hover:bg-muted/60">
								<Box className="mr-1 size-3.5 shrink-0 text-primary" />
								<SelectValue placeholder="Select Container" />
							</SelectTrigger>
							<SelectContent className="border- bg-card">
								<div className="mb-1 border-b  px-3 py-1.5 text-[9px] font-bold tracking-wider text-muted-foreground uppercase">
									App Containers
								</div>
								{availableTargets.map(target => (
									<SelectItem
										key={target}
										value={target}
										className="font-mono text-xs font-semibold">
										Target: {target}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					)}
				</div>

				<LogStreamControls
					isLive={isLive}
					onToggleLive={() => setIsLive(!isLive)}
					timestamps={timestamps}
					onToggleTimestamps={setTimestamps}
					lines={lines}
					onLinesChange={setLines}
					lineOptions={LINE_OPTIONS}
					onRefresh={refetch}
					onDownload={handleDownload}
				/>
			</div>

			<DeploymentViewer
				logs={streamedLogs}
				isLoading={isLoading}
				isLive={isLive}
				loadingText={`Connecting to real-time container log stream for '${activeTarget}'...`}
				emptyText={`No container log entries found for '${activeTarget}'. The application container may be stopped or initializing.`}
				onDownload={handleDownload}
				onReload={refetch}
			/>
		</div>
	);
}
