import {Button} from '#/components/ui/button';
import {KeyRound, Plus, Sparkles, RefreshCw} from 'lucide-react';

interface SshKeysHeaderProps {
	onOpenAdd: () => void;
	onOpenGenerate: () => void;
	onRefresh: () => void;
	isRefetching: boolean;
}

export function SshKeysHeader({
	onOpenAdd,
	onOpenGenerate,
	onRefresh,
	isRefetching,
}: SshKeysHeaderProps) {
	return (
		<div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 pb-4 border-b border-border/40">
			<div>
				<h1 className="text-xl font-bold tracking-tight text-foreground flex items-center gap-2.5">
					<KeyRound className="w-6 h-6 text-primary" />
					<span>SSH Keys</span>
				</h1>
				<p className="text-xs text-muted-foreground mt-1">
					Manage SSH key pairs for authenticating with remote server nodes and private Git repositories.
				</p>
			</div>

			<div className="flex items-center gap-2">
				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isRefetching}
					className="h-9 text-xs font-semibold gap-1.5"
				>
					<RefreshCw className={`w-3.5 h-3.5 ${isRefetching ? 'animate-spin' : ''}`} />
					Reload
				</Button>

				<Button
					variant="outline"
					size="sm"
					onClick={onOpenGenerate}
					className="h-9 text-xs font-semibold gap-1.5 border-primary/40 text-primary hover:bg-primary/10"
				>
					<Sparkles className="w-3.5 h-3.5 text-primary" />
					Generate Pair
				</Button>

				<Button
					size="sm"
					onClick={onOpenAdd}
					className="h-9 text-xs font-semibold gap-1.5 px-4 shadow-sm"
				>
					<Plus className="w-4 h-4" />
					Add SSH Key
				</Button>
			</div>
		</div>
	);
}
