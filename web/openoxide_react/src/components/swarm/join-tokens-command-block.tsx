import {Button} from '#/components/ui/button';
import {Copy, Check} from 'lucide-react';
import {useState} from 'react';
import {toast} from 'sonner';

/** Copyable docker join script block for the Manual tab. */
export function CommandBlock({
	label,
	command,
	isLoading,
}: {
	label: string;
	command: string;
	isLoading: boolean;
}) {
	const [copied, setCopied] = useState(false);
	const disabled = isLoading || !command;

	const handleCopy = () => {
		if (!command) return;
		navigator.clipboard.writeText(command);
		setCopied(true);
		toast.success(
			`${label} join script copied — paste it on the target server.`,
		);
		setTimeout(() => setCopied(false), 2000);
	};

	return (
		<div className="w-full min-w-0 space-y-1.5 rounded-lg bg-muted/20 p-2.5">
			<div className="flex items-center justify-between">
				<span className="text-xs font-semibold text-foreground">
					{label}
				</span>
				<Button
					type="button"
					variant="ghost"
					size="xs"
					disabled={disabled}
					onClick={handleCopy}
					className="h-6 gap-1 p-1 text-[11px] font-semibold text-primary hover:text-primary">
					{copied ? (
						<Check className="h-3 w-3 text-emerald-500" />
					) : (
						<Copy className="h-3 w-3" />
					)}
					{copied ? 'Copied' : 'Copy'}
				</Button>
			</div>
			<div className="max-h-24 w-full min-w-0 overflow-y-auto rounded border border-border bg-background p-2.5 font-mono text-[11px] leading-relaxed break-all text-foreground select-all">
				{isLoading ? (
					<span className="text-muted-foreground">Loading token…</span>
				) : command ? (
					command
				) : (
					<span className="text-muted-foreground">Token unavailable</span>
				)}
			</div>
		</div>
	);
}
