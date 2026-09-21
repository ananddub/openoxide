import {Titlebar} from '#/components/shared/titlebar';
import {Package} from 'lucide-react';
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
		<Titlebar
			breadcrumbs={[{label: 'Container Registries'}]}
			icon={<Package className="text-foreground/70" />}
			title="Container Registries"
			description="Manage Docker Hub, GitHub Container Registry (GHCR), and private image registries"
			actions={
				<>
					<Button
						variant="outline"
						size="sm"
						onClick={onRefresh}
						disabled={isRefreshing}
						title="Refresh Registries">
						<RefreshCw
							className={`size-3.5 text-muted-foreground ${isRefreshing ? 'animate-spin' : ''}`}
						/>
						Refresh
					</Button>
					<Button size="sm" onClick={onAddRegistry}>
						<Plus className="size-3.5" />
						Add Registry
					</Button>
				</>
			}
		/>
	);
}
