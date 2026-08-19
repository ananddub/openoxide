import {HardDrive, Plus, RefreshCw} from 'lucide-react';
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
			<div className="flex items-center gap-3">
				<div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg border border-primary/20 bg-primary/10">
					<HardDrive className="h-4 w-4 text-primary" />
				</div>
				<div>
					<h1 className="text-base leading-none font-semibold text-foreground">
						S3 Destinations
					</h1>
					<p className="mt-1 text-xs text-muted-foreground">
						AWS S3, Cloudflare R2, MinIO &amp; S3-compatible storage for
						backups
					</p>
				</div>
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
