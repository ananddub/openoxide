import {Button} from '#/components/ui/button';
import {KeyRound, Plus, RefreshCw} from 'lucide-react';

interface SshKeysHeaderProps {
	onOpenAdd: () => void;
	onRefresh: () => void;
	isRefetching: boolean;
	keys?: unknown[];
}

export function SshKeysHeader({onOpenAdd, onRefresh, isRefetching}: SshKeysHeaderProps) {
	return (
		<div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
			<div className="flex items-center gap-3">
				<div className="w-9 h-9 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center shrink-0">
					<KeyRound className="w-4 h-4 text-primary" />
				</div>
				<div>
					<h1 className="text-base font-semibold text-foreground leading-none">SSH Keys</h1>
					<p className="text-xs text-muted-foreground mt-1">
						Manage key pairs for remote servers &amp; Git repos
					</p>
				</div>
			</div>

			<div className="flex items-center gap-2 sm:ml-auto">
				<Button variant="outline" size="sm" onClick={onRefresh} disabled={isRefetching} className="h-8 text-xs gap-1.5 cursor-pointer">
					<RefreshCw className={`w-3.5 h-3.5 ${isRefetching ? 'animate-spin' : ''}`} />
					Refresh
				</Button>
				<Button size="sm" onClick={onOpenAdd} className="h-8 text-xs gap-1.5 cursor-pointer">
					<Plus className="w-3.5 h-3.5" />
					Add SSH Key
				</Button>
			</div>
		</div>
	);
}
