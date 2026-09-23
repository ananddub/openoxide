import {Cpu, HardDrive, ExternalLink, Terminal} from 'lucide-react';
import type {ReactNode} from 'react';

interface NodeInfo {
	id: number;
	name: string;
	url?: string;
}

const Row = ({label, children}: {label: string; children: ReactNode}) => (
	<div className="flex items-center justify-between">
		<span className="font-sans text-muted-foreground">{label}</span>
		{children}
	</div>
);

export function InspectorOverviewTab({node}: {node: NodeInfo}) {
	return (
		<div className="m-0 flex flex-1 flex-col gap-4 overflow-y-auto p-5">
			<div className="grid grid-cols-2 gap-3">
				<div className="rounded-xl bg-muted/20 p-3">
					<span className="flex items-center gap-1 text-xs text-muted-foreground">
						<Cpu className="h-3.5 w-3.5 text-sky-400" /> CPU Usage
					</span>
					<p className="mt-1 font-mono text-base font-bold text-foreground">
						0.15%
					</p>
				</div>
				<div className="rounded-xl bg-muted/20 p-3">
					<span className="flex items-center gap-1 text-xs text-muted-foreground">
						<HardDrive className="h-3.5 w-3.5 text-violet-400" /> Memory
						Usage
					</span>
					<p className="mt-1 font-mono text-base font-bold text-foreground">
						142 MB / 512 MB
					</p>
				</div>
			</div>

			<div className="flex flex-col gap-2.5 rounded-xl bg-muted/20 p-4 font-mono text-xs">
				<Row label="Service ID:">
					<span className="font-bold text-foreground">#{node.id}</span>
				</Row>
				<Row label="Internal Hostname:">
					<span className="font-bold text-foreground">{node.name}</span>
				</Row>
				{node.url && (
					<Row label="Public Endpoint:">
						<a
							href={node.url}
							target="_blank"
							rel="noreferrer"
							className="flex items-center gap-1 text-primary hover:underline">
							{node.url} <ExternalLink className="h-3 w-3" />
						</a>
					</Row>
				)}
			</div>
		</div>
	);
}

export function InspectorConfigTab() {
	return (
		<div className="m-0 flex flex-1 flex-col gap-4 overflow-y-auto p-5">
			<div className="flex flex-col gap-3 rounded-xl bg-muted/20 p-4">
				<h4 className="text-xs font-bold tracking-wider text-foreground uppercase">
					Service Configurations
				</h4>
				<p className="text-xs text-muted-foreground">
					Configure environment variables, resource limits, and replica
					scaling directly from this panel.
				</p>

				<div className="mt-2 flex flex-col gap-2">
					<div className="flex items-center justify-between rounded-lg bg-background p-2.5 font-mono text-xs">
						<span className="text-muted-foreground">Replicas:</span>
						<span className="font-bold text-foreground">1 Instance</span>
					</div>
					<div className="flex items-center justify-between rounded-lg bg-background p-2.5 font-mono text-xs">
						<span className="text-muted-foreground">Network Mode:</span>
						<span className="font-bold text-emerald-400">
							OpenOxide Overlay
						</span>
					</div>
				</div>
			</div>
		</div>
	);
}

export function InspectorLogsTab({node}: {node: NodeInfo}) {
	return (
		<div className="m-0 flex flex-1 flex-col gap-2 overflow-y-auto bg-neutral-50 p-4 font-mono text-xs dark:bg-black/90">
			<p className="flex items-center gap-1.5 text-[11px] text-muted-foreground">
				<Terminal className="h-3.5 w-3.5" /> Streaming live stdout/stderr
				logs from Docker engine...
			</p>
			<div className="space-y-1">
				<p className="text-emerald-600 dark:text-emerald-400">
					[INFO] Container {node.name} online and active.
				</p>
				<p className="text-emerald-600 dark:text-emerald-400">
					[INFO] Health check HTTP 200 OK (0ms latency).
				</p>
			</div>
		</div>
	);
}
