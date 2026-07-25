import {Button} from '#/components/ui/button';
import {Server, Plus, RefreshCw} from 'lucide-react';

interface RemoteServersHeaderProps {
	onOpenCreate: () => void;
	onRefresh: () => void;
	isRefetching: boolean;
}

export function RemoteServersHeader({
	onOpenCreate,
	onRefresh,
	isRefetching,
}: RemoteServersHeaderProps) {
	return (
		<div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 pb-4 border-b border-border/40">
			<div>
				<h1 className="text-xl font-bold tracking-tight text-foreground flex items-center gap-2.5">
					<Server className="w-6 h-6 text-primary" />
					<span>Remote Servers</span>
				</h1>
				<p className="text-xs text-muted-foreground mt-1">
					Connect and manage remote Linux host servers, Docker build nodes, and Swarm clusters.
				</p>
			</div>

			<div className="flex items-center gap-2">
				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isRefetching}
					className="h-9 text-xs font-semibold gap-1.5"
				>
					<RefreshCw className={`w-3.5 h-3.5 ${isRefetching ? 'animate-spin' : ''}`} />
					Reload
				</Button>

				<Button
					size="sm"
					onClick={onOpenCreate}
					className="h-9 text-xs font-semibold gap-1.5 px-4 shadow-sm"
				>
					<Plus className="w-4 h-4" />
					Add Remote Server
				</Button>
			</div>
		</div>
	);
}
