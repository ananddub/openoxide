import {Globe} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import type {GlobalContainerItem} from './docker-inspect-modal';

interface InspectBodyProps {
	type: 'logs' | 'config' | 'mount' | 'network';
	container: GlobalContainerItem;
	logsStream: string[];
}

/** Renders the tab-specific detail body for a container inspect modal. */
export function InspectBody({
	type,
	container,
	logsStream,
}: InspectBodyProps) {
	if (type === 'logs') {
		return (
			<DeploymentViewer
				logs={logsStream || []}
				isLoading={(logsStream || []).length === 0}
				isLive={true}
				isDeployment={false}
				heightClass="h-[400px]"
				loadingText={`Connecting to '${container.name}' container logs...`}
				emptyText="No container log output received."
			/>
		);
	}

	if (type === 'config') {
		return (
			<div className="flex flex-col gap-4">
				<div className="flex flex-col gap-2 rounded-lg bg-muted/30 p-3 font-mono text-xs">
					<span className="text-[10px] font-bold text-muted-foreground uppercase">
						Environment Variables
					</span>
					<pre className="break-all whitespace-pre-wrap text-zinc-200">
						{JSON.stringify(container.env || {}, null, 2)}
					</pre>
				</div>

				<div className="flex flex-col gap-2 rounded-lg bg-muted/30 p-3 font-mono text-xs">
					<span className="text-[10px] font-bold text-muted-foreground uppercase">
						Container Specifications
					</span>
					<div className="grid grid-cols-2 gap-2 text-foreground">
						<div>
							<b>ID:</b> {container.id}
						</div>
						<div>
							<b>Name:</b> {container.name}
						</div>
						<div>
							<b>Image:</b> {container.image}
						</div>
						<div>
							<b>Status:</b> {container.statusText}
						</div>
					</div>
				</div>
			</div>
		);
	}

	if (type === 'mount') {
		return (
			<div className="flex flex-col gap-3">
				<p className="text-xs text-muted-foreground">
					Volume mounts and directory bind paths for this container
				</p>
				{(container.mounts || []).map((m, idx) => (
					<div
						key={idx}
						className="flex flex-col gap-1.5 rounded-lg bg-muted/30 p-3 font-mono text-xs">
						<div className="flex items-center justify-between">
							<span className="font-bold text-primary">Host Source:</span>
							<Badge
								variant="outline"
								className="text-[10px] font-bold uppercase">
								{m.mode}
							</Badge>
						</div>
						<span className="truncate rounded bg-background p-1.5 text-foreground">
							{m.source}
						</span>
						<span className="mt-1 font-bold text-primary">
							Container Destination:
						</span>
						<span className="truncate rounded bg-background p-1.5 text-foreground">
							{m.destination}
						</span>
					</div>
				))}
			</div>
		);
	}

	return (
		<div className="flex flex-col gap-4">
			<div className="flex flex-col gap-2 rounded-lg bg-muted/30 p-3">
				<span className="text-xs font-bold text-foreground">
					Connected Docker Networks
				</span>
				<div className="flex flex-wrap gap-2">
					{(container.networks || []).map((net, i) => (
						<Badge
							key={i}
							variant="secondary"
							className="font-mono text-xs">
							<Globe className="mr-1 h-3 w-3 text-emerald-400" /> {net}
						</Badge>
					))}
				</div>
			</div>

			<div className="flex flex-col gap-2 rounded-lg bg-muted/30 p-3 font-mono text-xs">
				<span className="text-xs font-bold text-foreground">
					Exposed Container Ports
				</span>
				<span className="rounded bg-background p-2 text-foreground">
					{container.ports}
				</span>
			</div>
		</div>
	);
}
