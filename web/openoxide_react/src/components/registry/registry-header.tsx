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
		<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-border/50">
			<div className="flex items-center gap-3">
				<div className="p-2.5 rounded-xl bg-primary/10 text-primary shrink-0">
					<Database className="w-6 h-6" />
				</div>
				<div>
					<h1 className="text-xl font-bold tracking-tight text-foreground">Container Registries</h1>
					<p className="text-xs text-muted-foreground">
						Manage Docker Hub, GitHub Container Registry (GHCR), and private image registries
					</p>
				</div>
			</div>

			<div className="flex items-center gap-2 shrink-0">
				<Button
					variant="outline"
					size="icon"
					onClick={onRefresh}
					disabled={isRefreshing}
					title="Refresh Registries"
					className="h-9 w-9"
				>
					<RefreshCw className={`w-4 h-4 ${isRefreshing ? 'animate-spin text-primary' : ''}`} />
				</Button>

				<Button
					onClick={onAddRegistry}
					className="h-9 text-xs font-semibold gap-1.5 px-4 shadow-sm"
				>
					<Plus className="w-4 h-4" />
					Add Registry
				</Button>
			</div>
		</div>
	);
}
