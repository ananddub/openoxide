import {Button} from '#/components/ui/button';
import {Plus, RefreshCw} from 'lucide-react';

interface RegistryHeaderProps {
	onAddRegistry: () => void;
	onRefresh: () => void;
	isRefreshing: boolean;
}

export function RegistryHeader({
	onAddRegistry,
	onRefresh,
	isRefreshing,
}: RegistryHeaderProps) {
	return (
		<div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-medium tracking-tight text-foreground/80">
					Container Registries
				</h1>
				<p className="text-sm text-muted-foreground/70">
					Manage Docker Hub, GitHub Container Registry (GHCR), and private
					image registries
				</p>
			</div>

			<div className="flex items-center gap-2 sm:ml-auto">
				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isRefreshing}
					title="Refresh Registries"
					className="h-8 cursor-pointer gap-1.5 text-xs">
					<RefreshCw
						className={`h-3.5 w-3.5 ${isRefreshing ? 'animate-spin' : ''}`}
					/>
					Refresh
				</Button>
				<Button
					size="sm"
					onClick={onAddRegistry}
					className="h-8 cursor-pointer gap-1.5 text-xs">
					<Plus className="h-3.5 w-3.5" />
					Add Registry
				</Button>
			</div>
		</div>
	);
}
