import {Button} from '#/components/ui/button';
import {Database, Plus, RefreshCw} from 'lucide-react';

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
		<div className="flex flex-col justify-between gap-4 border-b border-border/50 pb-4 sm:flex-row sm:items-center">
			<div className="flex items-center gap-3">
				<div className="shrink-0 rounded-xl bg-primary/10 p-2.5 text-primary">
					<Database className="h-6 w-6" />
				</div>
				<div>
					<h1 className="text-xl font-bold tracking-tight text-foreground">
						Container Registries
					</h1>
					<p className="text-xs text-muted-foreground">
						Manage Docker Hub, GitHub Container Registry (GHCR), and
						private image registries
					</p>
				</div>
			</div>

			<div className="flex shrink-0 items-center gap-2">
				<Button
					variant="outline"
					size="icon"
					onClick={onRefresh}
					disabled={isRefreshing}
					title="Refresh Registries"
					className="h-9 w-9">
					<RefreshCw
						className={`h-4 w-4 ${isRefreshing ? 'animate-spin text-primary' : ''}`}
					/>
				</Button>

				<Button
					onClick={onAddRegistry}
					className="h-9 gap-1.5 px-4 text-xs font-semibold shadow-sm">
					<Plus className="h-4 w-4" />
					Add Registry
				</Button>
			</div>
		</div>
	);
}
