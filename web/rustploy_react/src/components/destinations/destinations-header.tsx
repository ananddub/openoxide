import {HardDrive, Plus, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';

interface DestinationsHeaderProps {
	totalCount: number;
	onAdd: () => void;
	onRefresh: () => void;
	isRefreshing: boolean;
}

export function DestinationsHeader({
	totalCount,
	onAdd,
	onRefresh,
	isRefreshing,
}: DestinationsHeaderProps) {
	return (
		<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-border/40 pb-5">
			<div>
				<div className="flex items-center gap-2">
					<HardDrive className="w-6 h-6 text-primary" />
					<h1 className="text-2xl font-extrabold tracking-tight text-foreground bg-gradient-to-r from-foreground to-foreground/80 bg-clip-text">
						S3 Storage Destinations
					</h1>
				</div>
				<p className="text-xs text-muted-foreground mt-1">
					Manage AWS S3, Cloudflare R2, MinIO & S3-compatible cloud storage buckets for volume snapshots and automated database backups
				</p>
			</div>

			<div className="flex items-center gap-3">
				<Badge variant="outline" className="text-xs font-mono px-3 py-1.5 bg-muted/20">
					Active S3 Buckets: {totalCount}
				</Badge>
				<Button
					variant="outline"
					size="icon"
					onClick={onRefresh}
					disabled={isRefreshing}
					className="w-9 h-9 border-border rounded-lg"
				>
					<RefreshCw className={`w-4 h-4 ${isRefreshing ? 'animate-spin text-primary' : ''}`} />
				</Button>
				<Button
					onClick={onAdd}
					size="sm"
					className="h-9 text-xs font-semibold flex items-center gap-1.5 px-4 shadow-sm"
				>
					<Plus className="w-4 h-4" /> Add S3 Destination
				</Button>
			</div>
		</div>
	);
}
