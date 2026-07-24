import {Package, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';

interface DockerHeaderProps {
	totalContainers: number;
	runningContainers: number;
	onRefresh: () => void;
	isRefreshing: boolean;
}

export function DockerHeader({totalContainers, runningContainers, onRefresh, isRefreshing}: DockerHeaderProps) {
	return (
		<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between flex-wrap gap-4 shadow-sm">
			<div>
				<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
					<Package className="w-4 h-4 text-primary" /> Docker Engine Containers
				</h3>
				<p className="text-xs text-muted-foreground mt-1">Manage and inspect all system-wide Docker containers running on the server host</p>
			</div>

			<div className="flex items-center gap-3">
				<Badge variant="outline" className="text-xs font-mono px-3 py-1">
					Running: <span className="text-emerald-400 font-bold ml-1">{runningContainers}</span> / {totalContainers}
				</Badge>
				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isRefreshing}
					className="h-8 text-xs font-semibold flex items-center gap-1.5"
				>
					<RefreshCw className={`w-3.5 h-3.5 ${isRefreshing ? 'animate-spin' : ''}`} /> Refresh
				</Button>
			</div>
		</section>
	);
}
