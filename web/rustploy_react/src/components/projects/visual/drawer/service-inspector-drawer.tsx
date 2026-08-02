import {useState} from 'react';
import {
	Sheet,
	SheetContent,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Tabs, TabsContent, TabsList, TabsTrigger} from '#/components/ui/tabs';
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
		node.status?.toLowerCase() === 'running' || node.status?.toLowerCase() === 'active';

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent
				side="right"
				className="w-[450px] sm:w-[540px] bg-card border-l border-border p-0 flex flex-col font-sans"
			>
				{/* Drawer Header */}
				<SheetHeader className="p-5 border-b border-border/60 bg-muted/20 flex flex-col gap-2">
					<div className="flex items-center justify-between gap-3">
						<div className="flex items-center gap-3 min-w-0">
							<div className="w-9 h-9 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0">
								{isDb ? (
									<Database className="w-4.5 h-4.5 text-emerald-400" />
								) : isCompose ? (
									<Box className="w-4.5 h-4.5 text-violet-400" />
								) : (
									<Box className="w-4.5 h-4.5 text-blue-400" />
								)}
							</div>
							<div className="min-w-0 flex-1">
								<SheetTitle className="text-base font-bold text-foreground truncate">
									{node.name}
								</SheetTitle>
								<div className="flex items-center gap-2 mt-0.5">
									<Badge variant="outline" className="text-[10px] uppercase font-mono py-0">
										{node.dbType || node.type}
									</Badge>
									<span className="flex items-center gap-1 text-[11px] text-muted-foreground font-mono">
										<span
											className={`w-2 h-2 rounded-full ${
												isRunning ? 'bg-emerald-500' : 'bg-amber-500'
											}`}
										/>
										{isRunning ? 'Running' : 'Stopped'}
									</span>
								</div>
							</div>
						</div>

						{/* Quick Actions */}
						<div className="flex items-center gap-1.5 shrink-0">
							{isRunning ? (
								<Button
									variant="outline"
									size="sm"
									className="h-8 text-xs gap-1 border-amber-500/30 text-amber-400 hover:bg-amber-500/10 cursor-pointer"
								>
									<Square className="w-3.5 h-3.5 fill-current" /> Stop
								</Button>
							) : (
								<Button
									variant="outline"
									size="sm"
									className="h-8 text-xs gap-1 border-emerald-500/30 text-emerald-400 hover:bg-emerald-500/10 cursor-pointer"
								>
									<Play className="w-3.5 h-3.5 fill-current" /> Start
								</Button>
							)}

							<Button
								variant="outline"
								size="sm"
								className="h-8 text-xs gap-1.5 cursor-pointer"
							>
								<RotateCw className="w-3.5 h-3.5" /> Restart
							</Button>
						</div>
					</div>
				</SheetHeader>

				{/* Tabs Navigation: Overview, Config, Live Logs */}
				<Tabs
					value={activeTab}
					onValueChange={setActiveTab}
					className="flex-1 flex flex-col min-h-0"
				>
					<div className="px-5 border-b border-border/60 bg-muted/10">
						<TabsList className="bg-transparent gap-4 p-0 h-10">
							<TabsTrigger
								value="overview"
								className="text-xs font-semibold px-0 py-2 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none"
							>
								<Activity className="w-3.5 h-3.5 mr-1.5" /> Overview
							</TabsTrigger>
							<TabsTrigger
								value="config"
								className="text-xs font-semibold px-0 py-2 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none"
							>
								<Sliders className="w-3.5 h-3.5 mr-1.5" /> Config
							</TabsTrigger>
							<TabsTrigger
								value="logs"
								className="text-xs font-semibold px-0 py-2 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none"
							>
								<Terminal className="w-3.5 h-3.5 mr-1.5" /> Live Logs
							</TabsTrigger>
						</TabsList>
					</div>

					{/* Overview Tab */}
					<TabsContent
						value="overview"
						className="p-5 flex flex-col gap-4 overflow-y-auto m-0 flex-1"
					>
						<div className="grid grid-cols-2 gap-3">
							<div className="p-3 bg-muted/20 border border-border/40 rounded-xl">
								<span className="text-xs text-muted-foreground flex items-center gap-1">
									<Cpu className="w-3.5 h-3.5 text-sky-400" /> CPU Usage
								</span>
								<p className="text-base font-bold font-mono text-foreground mt-1">
									0.15%
								</p>
							</div>
							<div className="p-3 bg-muted/20 border border-border/40 rounded-xl">
								<span className="text-xs text-muted-foreground flex items-center gap-1">
									<HardDrive className="w-3.5 h-3.5 text-violet-400" /> Memory Usage
								</span>
								<p className="text-base font-bold font-mono text-foreground mt-1">
									142 MB / 512 MB
								</p>
							</div>
						</div>

						<div className="p-4 bg-muted/20 border border-border/40 rounded-xl flex flex-col gap-2.5 text-xs font-mono">
							<div className="flex justify-between items-center">
								<span className="text-muted-foreground font-sans">Service ID:</span>
								<span className="text-foreground font-bold">#{node.id}</span>
							</div>
							<div className="flex justify-between items-center">
								<span className="text-muted-foreground font-sans">Internal Hostname:</span>
								<span className="text-foreground font-bold">{node.name}</span>
							</div>
							{node.url && (
								<div className="flex justify-between items-center">
									<span className="text-muted-foreground font-sans">Public Endpoint:</span>
									<a
										href={node.url}
										target="_blank"
										rel="noreferrer"
										className="text-primary hover:underline flex items-center gap-1"
									>
										{node.url} <ExternalLink className="w-3 h-3" />
									</a>
								</div>
							)}
						</div>
					</TabsContent>

					{/* Config Tab */}
					<TabsContent
						value="config"
						className="p-5 flex flex-col gap-4 overflow-y-auto m-0 flex-1"
					>
						<div className="p-4 bg-muted/20 border border-border/40 rounded-xl flex flex-col gap-3">
							<h4 className="text-xs font-bold text-foreground uppercase tracking-wider">
								Service Configurations
							</h4>
							<p className="text-xs text-muted-foreground">
								Configure environment variables, resource limits, and replica scaling directly from this panel.
							</p>

							<div className="flex flex-col gap-2 mt-2">
								<div className="flex justify-between items-center p-2.5 bg-background border border-border/60 rounded-lg text-xs font-mono">
									<span className="text-muted-foreground">Replicas:</span>
									<span className="text-foreground font-bold">1 Instance</span>
								</div>
								<div className="flex justify-between items-center p-2.5 bg-background border border-border/60 rounded-lg text-xs font-mono">
									<span className="text-muted-foreground">Network Mode:</span>
									<span className="text-emerald-400 font-bold">Rustploy Overlay</span>
								</div>
							</div>
						</div>
					</TabsContent>

					{/* Logs Tab */}
					<TabsContent
						value="logs"
						className="p-4 flex flex-col gap-2 overflow-y-auto m-0 flex-1 bg-black/90 font-mono text-xs text-emerald-400"
					>
						<p className="text-muted-foreground text-[11px]">
							[LOGS] Streaming live stdout/stderr logs from Docker engine...
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
