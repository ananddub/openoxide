import {Button} from '#/components/ui/button';
import {KeyRound, Plus, RefreshCw} from 'lucide-react';

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
			<div className="flex items-center gap-3">
				<div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg border border-primary/20 bg-primary/10">
					<KeyRound className="h-4 w-4 text-primary" />
				</div>
				<div>
					<h1 className="text-base leading-none font-semibold text-foreground">
						SSH Keys
					</h1>
					<p className="mt-1 text-xs text-muted-foreground">
						Manage key pairs for remote servers &amp; Git repos
					</p>
				</div>
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
