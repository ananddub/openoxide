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
		<div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-medium tracking-tight text-foreground/80">
					SSH Keys
				</h1>
				<p className="text-sm text-muted-foreground/70">
					Manage key pairs for remote servers & Git repos
				</p>
			</div>

			<div className="flex items-center gap-2 sm:ml-auto">
				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isRefetching}
					className="h-8 cursor-pointer gap-1.5 text-xs">
					<RefreshCw
						className={`h-3.5 w-3.5 ${isRefetching ? 'animate-spin' : ''}`}
					/>
					Refresh
				</Button>
				<Button
					size="sm"
					onClick={onOpenAdd}
					className="h-8 cursor-pointer gap-1.5 text-xs">
					<Plus className="h-3.5 w-3.5" />
					Add SSH Key
				</Button>
			</div>
		</div>
	);
}
