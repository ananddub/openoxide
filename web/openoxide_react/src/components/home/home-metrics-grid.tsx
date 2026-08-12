import {Layers, Package, Activity, Zap} from 'lucide-react';
import {Card, CardContent, CardHeader, CardTitle} from '#/components/ui/card';

interface HomeMetricsGridProps {
	totalProjects: number;
	totalContainers: number;
	runningContainers: number;
	totalDeployments: number;
	isTraefikHealthy: boolean;
}

export function HomeMetricsGrid({
	totalProjects,
	totalContainers,
	runningContainers,
	totalDeployments,
	isTraefikHealthy,
}: HomeMetricsGridProps) {
	return (
		<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3.5">
			{/* Metric 1: Projects & Apps */}
			<Card className="border border-border/60 shadow-xs bg-card/60 backdrop-blur-sm">
				<CardHeader className="flex flex-row items-center justify-between pb-2 space-y-0 p-4">
					<CardTitle className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
						Projects & Apps
					</CardTitle>
					<Layers className="size-4 text-primary shrink-0" />
				</CardHeader>
				<CardContent className="p-4 pt-0">
					<div className="text-2xl font-bold text-foreground font-mono">{totalProjects}</div>
					<p className="text-[11px] text-muted-foreground mt-1 flex items-center gap-1.5">
						<span className="size-1.5 rounded-full bg-emerald-500" />
						{totalDeployments} active deployments
					</p>
				</CardContent>
			</Card>

			{/* Metric 2: Docker Containers */}
			<Card className="border border-border/60 shadow-xs bg-card/60 backdrop-blur-sm">
				<CardHeader className="flex flex-row items-center justify-between pb-2 space-y-0 p-4">
					<CardTitle className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
						Docker Engine
					</CardTitle>
					<Package className="size-4 text-primary shrink-0" />
				</CardHeader>
				<CardContent className="p-4 pt-0">
					<div className="text-2xl font-bold text-foreground font-mono">
						{runningContainers} <span className="text-xs font-normal text-muted-foreground">/ {totalContainers}</span>
					</div>
					<p className="text-[11px] text-muted-foreground mt-1 flex items-center gap-1.5">
						<span className={`size-1.5 rounded-full ${runningContainers > 0 ? 'bg-emerald-500 animate-pulse' : 'bg-amber-500'}`} />
						Containers running active
					</p>
				</CardContent>
			</Card>

			{/* Metric 3: Traefik Proxy Status */}
			<Card className="border border-border/60 shadow-xs bg-card/60 backdrop-blur-sm">
				<CardHeader className="flex flex-row items-center justify-between pb-2 space-y-0 p-4">
					<CardTitle className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
						Traefik Proxy
					</CardTitle>
					<Zap className="size-4 text-primary shrink-0" />
				</CardHeader>
				<CardContent className="p-4 pt-0">
					<div className="text-2xl font-bold text-foreground font-mono flex items-center gap-2">
						{isTraefikHealthy ? (
							<span className="text-emerald-500 text-lg font-bold">Online</span>
						) : (
							<span className="text-amber-500 text-lg font-bold">Standby</span>
						)}
					</div>
					<p className="text-[11px] text-muted-foreground mt-1 flex items-center gap-1.5">
						<span className={`size-1.5 rounded-full ${isTraefikHealthy ? 'bg-emerald-500' : 'bg-amber-500'}`} />
						Reverse proxy & routing active
					</p>
				</CardContent>
			</Card>

			{/* Metric 4: System Vitals Status */}
			<Card className="border border-border/60 shadow-xs bg-card/60 backdrop-blur-sm">
				<CardHeader className="flex flex-row items-center justify-between pb-2 space-y-0 p-4">
					<CardTitle className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
						Host Vitals
					</CardTitle>
					<Activity className="size-4 text-primary shrink-0" />
				</CardHeader>
				<CardContent className="p-4 pt-0">
					<div className="text-2xl font-bold text-foreground font-mono text-emerald-500 text-lg">
						Healthy
					</div>
					<p className="text-[11px] text-muted-foreground mt-1 flex items-center gap-1.5">
						<span className="size-1.5 rounded-full bg-emerald-500 animate-pulse" />
						Realtime monitoring active
					</p>
				</CardContent>
			</Card>
		</div>
	);
}
