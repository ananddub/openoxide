import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {KeyRound, Copy, Check} from 'lucide-react';
import {useState} from 'react';
import {toast} from 'sonner';

interface JoinTokensModalProps {
	isOpen: boolean;
	tokens: {worker?: string; manager?: string} | null;
	isLoading: boolean;
	onClose: () => void;
}

function HighlightedJoinCommand({command}: {command: string}) {
	if (!command || command.startsWith('Loading') || command === 'N/A') {
		return <span className="text-muted-foreground">{command}</span>;
	}

	const tokenPrefix = 'docker swarm join --token ';
	if (command.startsWith(tokenPrefix)) {
		const tokenValue = command.slice(tokenPrefix.length);
		return (
			<span className="font-mono text-[11px] leading-relaxed">
				<span className="text-emerald-500 font-bold">docker swarm join</span>{' '}
				<span className="text-amber-500 font-medium">--token</span>{' '}
				<span className="text-foreground font-medium break-all">{tokenValue}</span>
			</span>
		);
	}

	return <span className="font-mono text-[11px] text-foreground">{command}</span>;
}

export function JoinTokensModal({
	isOpen,
	tokens,
	isLoading,
	onClose,
}: JoinTokensModalProps) {
	const [copiedKey, setCopiedKey] = useState<string | null>(null);

	const handleCopy = (type: 'worker' | 'manager', token?: string) => {
		if (!token) {
			toast.error('Token not available');
			return;
		}
		const cmd = `docker swarm join --token ${token}`;
		navigator.clipboard.writeText(cmd);
		setCopiedKey(type);
		toast.success(`Copied ${type} join command to clipboard`);
		setTimeout(() => setCopiedKey(null), 2000);
	};

	const workerCmd = isLoading
		? 'Loading worker token...'
		: tokens?.worker
			? `docker swarm join --token ${tokens.worker}`
			: 'N/A';

	const managerCmd = isLoading
		? 'Loading manager token...'
		: tokens?.manager
			? `docker swarm join --token ${tokens.manager}`
			: 'N/A';

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-xl w-full bg-card border-border p-6 shadow-2xl rounded-2xl">
				<DialogHeader className="pb-4 border-b border-border/50">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2.5">
						<div className="p-2 rounded-xl bg-primary/10 text-primary">
							<KeyRound className="w-5 h-5" />
						</div>
						Swarm Join Tokens
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Use these command tokens on worker or manager nodes to join this cluster
					</DialogDescription>
				</DialogHeader>

				<div className="flex flex-col gap-4 mt-3 text-xs">
					{/* Worker Token */}
					<div className="flex flex-col gap-1.5">
						<div className="flex items-center justify-between">
							<span className="font-semibold text-foreground">Worker Join Command</span>
							<Button
								variant="ghost"
								size="sm"
								onClick={() => handleCopy('worker', tokens?.worker)}
								className="h-7 text-xs font-medium gap-1 px-2"
							>
								{copiedKey === 'worker' ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
								{copiedKey === 'worker' ? 'Copied' : 'Copy'}
							</Button>
						</div>
						<div className="bg-zinc-950 dark:bg-zinc-950/80 p-3 rounded-xl border border-zinc-800 shadow-inner overflow-x-auto select-all">
							<HighlightedJoinCommand command={workerCmd} />
						</div>
					</div>

					{/* Manager Token */}
					<div className="flex flex-col gap-1.5">
						<div className="flex items-center justify-between">
							<span className="font-semibold text-foreground">Manager Join Command</span>
							<Button
								variant="ghost"
								size="sm"
								onClick={() => handleCopy('manager', tokens?.manager)}
								className="h-7 text-xs font-medium gap-1 px-2"
							>
								{copiedKey === 'manager' ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
								{copiedKey === 'manager' ? 'Copied' : 'Copy'}
							</Button>
						</div>
						<div className="bg-zinc-950 dark:bg-zinc-950/80 p-3 rounded-xl border border-zinc-800 shadow-inner overflow-x-auto select-all">
							<HighlightedJoinCommand command={managerCmd} />
						</div>
					</div>

					<div className="flex justify-end pt-3 border-t border-border/50">
						<Button variant="outline" onClick={onClose} className="h-9 text-xs font-semibold px-4">
							Close
						</Button>
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
