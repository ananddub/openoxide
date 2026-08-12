import {Button} from '#/components/ui/button';
import {Server, Plus, RefreshCw} from 'lucide-react';
import type {RemoteServerResponse} from '#/types/api-helpers';

interface RemoteServersHeaderProps {
	onOpenCreate: () => void;
	onRefresh: () => void;
	isRefetching: boolean;
	servers?: RemoteServerResponse[];
}

export function RemoteServersHeader({onOpenCreate, onRefresh, isRefetching}: RemoteServersHeaderProps) {
	return (
		<div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
			<div className="flex items-center gap-3">
				<div className="w-9 h-9 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center shrink-0">
					<Server className="w-4 h-4 text-primary" />
				</div>
				<div>
					<h1 className="text-base font-semibold text-foreground leading-none">Remote Servers</h1>
					<p className="text-xs text-muted-foreground mt-1">
						Connect and manage Linux nodes for deployment &amp; Docker Swarm
					</p>
				</div>
			</div>

			<div className="flex items-center gap-2 sm:ml-auto">
				<Button variant="outline" size="sm" onClick={onRefresh} disabled={isRefetching} className="h-8 text-xs gap-1.5 cursor-pointer">
					<RefreshCw className={`w-3.5 h-3.5 ${isRefetching ? 'animate-spin' : ''}`} />
					Refresh
				</Button>
				<Button size="sm" onClick={onOpenCreate} className="h-8 text-xs gap-1.5 cursor-pointer">
					<Plus className="w-3.5 h-3.5" />
					Add Server
				</Button>
			</div>
		</div>
	);
}
