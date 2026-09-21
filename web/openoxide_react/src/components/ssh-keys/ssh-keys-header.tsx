import {Titlebar} from '#/components/shared/titlebar';
import {KeyRound} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Plus, RefreshCw} from 'lucide-react';

interface SshKeysHeaderProps {
	onOpenAdd: () => void;
	onRefresh: () => void;
	isRefetching: boolean;
	keys?: unknown[];
}

export function SshKeysHeader({
	onOpenAdd,
	onRefresh,
	isRefetching,
}: SshKeysHeaderProps) {
	return (
		<Titlebar
			breadcrumbs={[{label: 'SSH Keys'}]}
			icon={<KeyRound className="text-foreground/70" />}
			title="SSH Keys"
			description="Manage key pairs for remote servers & Git repos"
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
					<Button size="sm" onClick={onOpenAdd}>
						<Plus className="size-3.5" />
						Add SSH Key
					</Button>
				</>
			}
		/>
	);
}
