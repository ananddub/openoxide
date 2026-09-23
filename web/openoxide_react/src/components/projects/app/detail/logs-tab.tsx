import {useState, useRef, useEffect, useMemo} from 'react';
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
import {
	useLogStream,
	downloadLogs,
} from '#/components/projects/detail/logs/use-log-stream';
import {SectionHeader, Toolbar} from '#/components/settings/list-card';

const deploymentClient = createServiceClient(DeploymentService);

interface LogsTabProps {
	app: any;
}

export function LogsTab({app}: LogsTabProps) {
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
					tail: 500n,
					timestamps: false,
					follow: isLive,
				},
				{signal},
			),
		[activeTarget, isLive],
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
		<div className="flex w-full flex-col gap-5">
			<SectionHeader
				title="Logs"
				description="Live container output streamed from the server."
			/>

			{availableTargets.length > 1 && (
				<Toolbar>
					<Select
						value={activeTarget}
						onValueChange={val => val && setSelectedTarget(val)}>
						<SelectTrigger className="h-9 w-auto min-w-[180px] rounded-lg bg-card font-mono text-xs">
							<SelectValue placeholder="Select container" />
						</SelectTrigger>
						<SelectContent className="bg-card">
							{availableTargets.map(target => (
								<SelectItem
									key={target}
									value={target}
									className="font-mono text-xs">
									{target}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</Toolbar>
			)}

			<DeploymentViewer
				logs={streamedLogs}
				isLoading={isLoading}
				isLive={isLive}
				onToggleLive={() => setIsLive(!isLive)}
				loadingText={`Connecting to '${activeTarget}' log stream...`}
				emptyText={`No log entries for '${activeTarget}'. The container may be stopped or still starting.`}
				onDownload={handleDownload}
				onReload={refetch}
			/>
		</div>
	);
}
