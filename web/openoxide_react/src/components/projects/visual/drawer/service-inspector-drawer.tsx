import {useState} from 'react';
import {
	Sheet,
	SheetContent,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
} from '#/components/ui/tabs';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {
	Box,
	Cpu,
	Database,
	HardDrive,
	Terminal,
	ExternalLink,
	RotateCw,
	Activity,
	Sliders,
	Play,
	Square,
} from 'lucide-react';

interface ServiceInspectorDrawerProps {
	isOpen: boolean;
	onClose: () => void;
	node?: {
		id: number;
		type: 'app' | 'database' | 'compose' | 'destination';
		name: string;
		status?: string;
		dbType?: string;
		url?: string;
	} | null;
}

export function ServiceInspectorDrawer({
	isOpen,
	onClose,
	node,
}: ServiceInspectorDrawerProps) {
	const [activeTab, setActiveTab] = useState('overview');

	if (!node) return null;

	const isDb = node.type === 'database';
	const isCompose = node.type === 'compose';
	const isRunning =
		node.status?.toLowerCase() === 'running' ||
		node.status?.toLowerCase() === 'active';

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent
				side="right"
				className="flex w-[450px] flex-col border-l border-border bg-card p-0 font-sans sm:w-[540px]">
				{/* Drawer Header */}
				<SheetHeader className="flex flex-col gap-2 border-b border-border/60 bg-muted/20 p-5">
					<div className="flex items-center justify-between gap-3">
						<div className="flex min-w-0 items-center gap-3">
							<div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-xl border border-primary/20 bg-primary/10 text-primary">
								{isDb ? (
									<Database className="h-4.5 w-4.5 text-emerald-400" />
								) : isCompose ? (
									<Box className="h-4.5 w-4.5 text-violet-400" />
								) : (
									<Box className="h-4.5 w-4.5 text-blue-400" />
								)}
							</div>
							<div className="min-w-0 flex-1">
								<SheetTitle className="truncate text-base font-bold text-foreground">
									{node.name}
								</SheetTitle>
								<div className="mt-0.5 flex items-center gap-2">
									<Badge
										variant="outline"
										className="py-0 font-mono text-[10px] uppercase">
										{node.dbType || node.type}
									</Badge>
									<span className="flex items-center gap-1 font-mono text-[11px] text-muted-foreground">
										<span
											className={`h-2 w-2 rounded-full ${
												isRunning ? 'bg-emerald-500' : 'bg-amber-500'
											}`}
										/>
										{isRunning ? 'Running' : 'Stopped'}
									</span>
								</div>
							</div>
						</div>

						{/* Quick Actions */}
						<div className="flex shrink-0 items-center gap-1.5">
							{isRunning ? (
								<Button
									variant="outline"
									size="sm"
									className="h-8 cursor-pointer gap-1 border-amber-500/30 text-xs text-amber-400 hover:bg-amber-500/10">
									<Square className="h-3.5 w-3.5 fill-current" /> Stop
								</Button>
							) : (
								<Button
									variant="outline"
									size="sm"
									className="h-8 cursor-pointer gap-1 border-emerald-500/30 text-xs text-emerald-400 hover:bg-emerald-500/10">
									<Play className="h-3.5 w-3.5 fill-current" /> Start
								</Button>
							)}

							<Button
								variant="outline"
								size="sm"
								className="h-8 cursor-pointer gap-1.5 text-xs">
								<RotateCw className="h-3.5 w-3.5" /> Restart
							</Button>
						</div>
					</div>
				</SheetHeader>

				{/* Tabs Navigation: Overview, Config, Live Logs */}
				<Tabs
					value={activeTab}
					onValueChange={setActiveTab}
					className="flex min-h-0 flex-1 flex-col">
					<div className="border-b border-border/60 bg-muted/10 px-5">
						<TabsList className="h-10 gap-4 bg-transparent p-0">
							<TabsTrigger
								value="overview"
								className="rounded-none border-b-2 border-transparent px-0 py-2 text-xs font-semibold data-[state=active]:border-primary data-[state=active]:bg-transparent">
								<Activity className="mr-1.5 h-3.5 w-3.5" /> Overview
							</TabsTrigger>
							<TabsTrigger
								value="config"
								className="rounded-none border-b-2 border-transparent px-0 py-2 text-xs font-semibold data-[state=active]:border-primary data-[state=active]:bg-transparent">
								<Sliders className="mr-1.5 h-3.5 w-3.5" /> Config
							</TabsTrigger>
							<TabsTrigger
								value="logs"
								className="rounded-none border-b-2 border-transparent px-0 py-2 text-xs font-semibold data-[state=active]:border-primary data-[state=active]:bg-transparent">
								<Terminal className="mr-1.5 h-3.5 w-3.5" /> Live Logs
							</TabsTrigger>
						</TabsList>
					</div>

					{/* Overview Tab */}
					<TabsContent
						value="overview"
						className="m-0 flex flex-1 flex-col gap-4 overflow-y-auto p-5">
						<div className="grid grid-cols-2 gap-3">
							<div className="rounded-xl border border-border/40 bg-muted/20 p-3">
								<span className="flex items-center gap-1 text-xs text-muted-foreground">
									<Cpu className="h-3.5 w-3.5 text-sky-400" /> CPU Usage
								</span>
								<p className="mt-1 font-mono text-base font-bold text-foreground">
									0.15%
								</p>
							</div>
							<div className="rounded-xl border border-border/40 bg-muted/20 p-3">
								<span className="flex items-center gap-1 text-xs text-muted-foreground">
									<HardDrive className="h-3.5 w-3.5 text-violet-400" />{' '}
									Memory Usage
								</span>
								<p className="mt-1 font-mono text-base font-bold text-foreground">
									142 MB / 512 MB
								</p>
							</div>
						</div>

						<div className="flex flex-col gap-2.5 rounded-xl border border-border/40 bg-muted/20 p-4 font-mono text-xs">
							<div className="flex items-center justify-between">
								<span className="font-sans text-muted-foreground">
									Service ID:
								</span>
								<span className="font-bold text-foreground">
									#{node.id}
								</span>
							</div>
							<div className="flex items-center justify-between">
								<span className="font-sans text-muted-foreground">
									Internal Hostname:
								</span>
								<span className="font-bold text-foreground">
									{node.name}
								</span>
							</div>
							{node.url && (
								<div className="flex items-center justify-between">
									<span className="font-sans text-muted-foreground">
										Public Endpoint:
									</span>
									<a
										href={node.url}
										target="_blank"
										rel="noreferrer"
										className="flex items-center gap-1 text-primary hover:underline">
										{node.url} <ExternalLink className="h-3 w-3" />
									</a>
								</div>
							)}
						</div>
					</TabsContent>

					{/* Config Tab */}
					<TabsContent
						value="config"
						className="m-0 flex flex-1 flex-col gap-4 overflow-y-auto p-5">
						<div className="flex flex-col gap-3 rounded-xl border border-border/40 bg-muted/20 p-4">
							<h4 className="text-xs font-bold tracking-wider text-foreground uppercase">
								Service Configurations
							</h4>
							<p className="text-xs text-muted-foreground">
								Configure environment variables, resource limits, and
								replica scaling directly from this panel.
							</p>

							<div className="mt-2 flex flex-col gap-2">
								<div className="flex items-center justify-between rounded-lg border border-border/60 bg-background p-2.5 font-mono text-xs">
									<span className="text-muted-foreground">Replicas:</span>
									<span className="font-bold text-foreground">
										1 Instance
									</span>
								</div>
								<div className="flex items-center justify-between rounded-lg border border-border/60 bg-background p-2.5 font-mono text-xs">
									<span className="text-muted-foreground">
										Network Mode:
									</span>
									<span className="font-bold text-emerald-400">
										OpenOxide Overlay
									</span>
								</div>
							</div>
						</div>
					</TabsContent>

					{/* Logs Tab */}
					<TabsContent
						value="logs"
						className="m-0 flex flex-1 flex-col gap-2 overflow-y-auto bg-black/90 p-4 font-mono text-xs text-emerald-400">
						<p className="text-[11px] text-muted-foreground">
							[LOGS] Streaming live stdout/stderr logs from Docker
							engine...
						</p>
						<div className="space-y-1">
							<p className="text-emerald-400">
								[INFO] Container {node.name} online and active.
							</p>
							<p className="text-emerald-400">
								[INFO] Health check HTTP 200 OK (0ms latency).
							</p>
						</div>
					</TabsContent>
				</Tabs>
			</SheetContent>
		</Sheet>
	);
}
