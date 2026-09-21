import {Titlebar} from '#/components/shared/titlebar';
import {HardDrive} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Plus, RefreshCw} from 'lucide-react';

interface DestinationsHeaderProps {
	totalCount: number;
	onAdd: () => void;
	onRefresh: () => void;
	isRefreshing: boolean;
}

export function DestinationsHeader({
	onAdd,
	onRefresh,
	isRefreshing,
}: DestinationsHeaderProps) {
	return (
		<Titlebar
			breadcrumbs={[{label: 'S3 Destinations'}]}
			icon={<HardDrive className="text-foreground/70" />}
			title="S3 Destinations"
			description="AWS S3, Cloudflare R2, MinIO & S3-compatible storage for backups"
			actions={
				<>
					<Button
						variant="outline"
						size="sm"
						onClick={onRefresh}
						disabled={isRefreshing}>
						<RefreshCw
							className={`size-3.5 text-muted-foreground ${isRefreshing ? 'animate-spin' : ''}`}
						/>
						Refresh
					</Button>
					<Button size="sm" onClick={onAdd}>
						<Plus className="size-3.5" />
						Add Destination
					</Button>
				</>
			}
		/>
	);
}
