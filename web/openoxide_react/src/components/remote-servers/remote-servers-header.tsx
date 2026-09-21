import {Button} from '#/components/ui/button';
import {Plus, RefreshCw} from 'lucide-react';
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
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-medium tracking-tight text-foreground/80">
					Remote Servers
				</h1>
				<p className="text-sm text-muted-foreground/70">
					Connect and manage Linux nodes for deployment & Docker Swarm
				</p>
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
