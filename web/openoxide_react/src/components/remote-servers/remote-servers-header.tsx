import {Button} from '#/components/ui/button';
import {Server, Plus, RefreshCw} from 'lucide-react';
import type {RemoteServerResponse} from '#/types/api-helpers';

interface RemoteServersHeaderProps {
	onOpenCreate: () => void;
	onRefresh: () => void;
	isRefetching: boolean;
	servers?: RemoteServerResponse[];
}

export function RemoteServersHeader({
	onOpenCreate,
	onRefresh,
	isRefetching,
}: RemoteServersHeaderProps) {
	return (
		<div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
			<div className="flex items-center gap-3">
				<div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg border border-primary/20 bg-primary/10">
					<Server className="h-4 w-4 text-primary" />
				</div>
				<div>
					<h1 className="text-base leading-none font-semibold text-foreground">
						Remote Servers
					</h1>
					<p className="mt-1 text-xs text-muted-foreground">
						Connect and manage Linux nodes for deployment &amp; Docker
						Swarm
					</p>
				</div>
			</div>

			<div className="flex items-center gap-2 sm:ml-auto">
				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isRefetching}
					className="h-8 cursor-pointer gap-1.5 text-xs">
					<RefreshCw
						className={`h-3.5 w-3.5 ${isRefetching ? 'animate-spin' : ''}`}
					/>
					Refresh
				</Button>
				<Button
					size="sm"
					onClick={onOpenCreate}
					className="h-8 cursor-pointer gap-1.5 text-xs">
					<Plus className="h-3.5 w-3.5" />
					Add Server
				</Button>
			</div>
		</div>
	);
}
