import {Titlebar} from '#/components/shared/titlebar';
import {Server} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Plus, RefreshCw} from 'lucide-react';

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
		<Titlebar
			breadcrumbs={[{label: 'Remote Servers'}]}
			icon={<Server className="text-foreground/70" />}
			title="Remote Servers"
			description="Connect and manage Linux nodes for deployment & Docker Swarm"
			actions={
				<>
					<Button
						variant="outline"
						size="sm"
						onClick={onRefresh}
						disabled={isRefetching}>
						<RefreshCw
							className={`size-3.5 text-muted-foreground ${isRefetching ? 'animate-spin' : ''}`}
						/>
						Refresh
					</Button>
					<Button size="sm" onClick={onOpenCreate}>
						<Plus className="size-3.5" />
						Add Server
					</Button>
				</>
			}
		/>
	);
}
