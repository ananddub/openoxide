import {HardDrive, Plus, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';

interface DestinationsHeaderProps {
	totalCount: number;
	onAdd: () => void;
	onRefresh: () => void;
	isRefreshing: boolean;
}

export function DestinationsHeader({onAdd, onRefresh, isRefreshing}: DestinationsHeaderProps) {
	return (
		<div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
			<div className="flex items-center gap-3">
				<div className="w-9 h-9 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center shrink-0">
					<HardDrive className="w-4 h-4 text-primary" />
				</div>
				<div>
					<h1 className="text-base font-semibold text-foreground leading-none">S3 Destinations</h1>
					<p className="text-xs text-muted-foreground mt-1">
						AWS S3, Cloudflare R2, MinIO &amp; S3-compatible storage for backups
					</p>
				</div>
			</div>

			<div className="flex items-center gap-2 sm:ml-auto">
				<Button variant="outline" size="sm" onClick={onRefresh} disabled={isRefreshing} className="h-8 text-xs gap-1.5 cursor-pointer">
					<RefreshCw className={`w-3.5 h-3.5 ${isRefreshing ? 'animate-spin' : ''}`} />
					Refresh
				</Button>
				<Button size="sm" onClick={onAdd} className="h-8 text-xs gap-1.5 cursor-pointer">
					<Plus className="w-3.5 h-3.5" />
					Add Destination
				</Button>
			</div>
		</div>
	);
}
