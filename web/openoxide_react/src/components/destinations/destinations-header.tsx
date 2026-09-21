import {Plus, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';

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
		<div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-medium tracking-tight text-foreground/80">
					S3 Destinations
				</h1>
				<p className="text-sm text-muted-foreground/70">
					AWS S3, Cloudflare R2, MinIO & S3-compatible storage for backups
				</p>
			</div>

			<div className="flex items-center gap-2 sm:ml-auto">
				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isRefreshing}
					className="h-8 cursor-pointer gap-1.5 text-xs">
					<RefreshCw
						className={`h-3.5 w-3.5 ${isRefreshing ? 'animate-spin' : ''}`}
					/>
					Refresh
				</Button>
				<Button
					size="sm"
					onClick={onAdd}
					className="h-8 cursor-pointer gap-1.5 text-xs">
					<Plus className="h-3.5 w-3.5" />
					Add Destination
				</Button>
			</div>
		</div>
	);
}
