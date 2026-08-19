import {Layers, Package, Activity, Zap} from 'lucide-react';
import {
	Card,
	CardContent,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';

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
		<div className="grid grid-cols-1 gap-3.5 sm:grid-cols-2 lg:grid-cols-4">
			{/* Metric 1: Projects & Apps */}
			<Card className="border border-border/60 bg-card/60 shadow-xs backdrop-blur-sm">
				<CardHeader className="flex flex-row items-center justify-between space-y-0 p-4 pb-2">
					<CardTitle className="text-xs font-semibold tracking-wider text-muted-foreground uppercase">
						Projects & Apps
					</CardTitle>
					<Layers className="size-4 shrink-0 text-primary" />
				</CardHeader>
				<CardContent className="p-4 pt-0">
					<div className="font-mono text-2xl font-bold text-foreground">
						{totalProjects}
					</div>
					<p className="mt-1 flex items-center gap-1.5 text-[11px] text-muted-foreground">
						<span className="size-1.5 rounded-full bg-emerald-500" />
						{totalDeployments} active deployments
					</p>
				</CardContent>
			</Card>

			{/* Metric 2: Docker Containers */}
			<Card className="border border-border/60 bg-card/60 shadow-xs backdrop-blur-sm">
				<CardHeader className="flex flex-row items-center justify-between space-y-0 p-4 pb-2">
					<CardTitle className="text-xs font-semibold tracking-wider text-muted-foreground uppercase">
						Docker Engine
					</CardTitle>
					<Package className="size-4 shrink-0 text-primary" />
				</CardHeader>
				<CardContent className="p-4 pt-0">
					<div className="font-mono text-2xl font-bold text-foreground">
						{runningContainers}{' '}
						<span className="text-xs font-normal text-muted-foreground">
							/ {totalContainers}
						</span>
					</div>
					<p className="mt-1 flex items-center gap-1.5 text-[11px] text-muted-foreground">
						<span
							className={`size-1.5 rounded-full ${runningContainers > 0 ? 'animate-pulse bg-emerald-500' : 'bg-amber-500'}`}
						/>
						Containers running active
					</p>
				</CardContent>
			</Card>

			{/* Metric 3: Traefik Proxy Status */}
			<Card className="border border-border/60 bg-card/60 shadow-xs backdrop-blur-sm">
				<CardHeader className="flex flex-row items-center justify-between space-y-0 p-4 pb-2">
					<CardTitle className="text-xs font-semibold tracking-wider text-muted-foreground uppercase">
						Traefik Proxy
					</CardTitle>
					<Zap className="size-4 shrink-0 text-primary" />
				</CardHeader>
				<CardContent className="p-4 pt-0">
					<div className="flex items-center gap-2 font-mono text-2xl font-bold text-foreground">
						{isTraefikHealthy ? (
							<span className="text-lg font-bold text-emerald-500">
								Online
							</span>
						) : (
							<span className="text-lg font-bold text-amber-500">
								Standby
							</span>
						)}
					</div>
					<p className="mt-1 flex items-center gap-1.5 text-[11px] text-muted-foreground">
						<span
							className={`size-1.5 rounded-full ${isTraefikHealthy ? 'bg-emerald-500' : 'bg-amber-500'}`}
						/>
						Reverse proxy & routing active
					</p>
				</CardContent>
			</Card>

			{/* Metric 4: System Vitals Status */}
			<Card className="border border-border/60 bg-card/60 shadow-xs backdrop-blur-sm">
				<CardHeader className="flex flex-row items-center justify-between space-y-0 p-4 pb-2">
					<CardTitle className="text-xs font-semibold tracking-wider text-muted-foreground uppercase">
						Host Vitals
					</CardTitle>
					<Activity className="size-4 shrink-0 text-primary" />
				</CardHeader>
				<CardContent className="p-4 pt-0">
					<div className="font-mono text-2xl text-lg font-bold text-emerald-500 text-foreground">
						Healthy
					</div>
					<p className="mt-1 flex items-center gap-1.5 text-[11px] text-muted-foreground">
						<span className="size-1.5 animate-pulse rounded-full bg-emerald-500" />
						Realtime monitoring active
					</p>
				</CardContent>
			</Card>
		</div>
	);
}
