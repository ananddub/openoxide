import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Terminal, Copy, Check} from 'lucide-react';
import {toast} from 'sonner';

interface AuthorizeCommandBlockProps {
	publicKey: string;
}

export function AuthorizeCommandBlock({
	publicKey,
}: AuthorizeCommandBlockProps) {
	const [copied, setCopied] = useState(false);

	if (!publicKey?.trim()) return null;

	const cmd = `mkdir -p ~/.ssh && echo "${publicKey.trim()}" >> ~/.ssh/authorized_keys && chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys`;

	const handleCopy = () => {
		navigator.clipboard.writeText(cmd);
		setCopied(true);
		toast.success('Server authorization command copied!');
		setTimeout(() => setCopied(false), 2000);
	};

	return (
		<div className="w-full min-w-0 space-y-1.5 overflow-hidden rounded-lg border border-border/60 bg-muted/20 p-2.5">
			<div className="flex items-center justify-between">
				<span className="flex items-center gap-1.5 text-xs font-semibold text-foreground">
					<Terminal className="h-3.5 w-3.5 text-amber-500" />
					Authorize on Server
				</span>
				<Button
					type="button"
					variant="ghost"
					size="xs"
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
			<div className="max-h-24 w-full min-w-0 overflow-y-auto rounded border border-zinc-800/80 bg-zinc-950 p-2.5 font-mono text-[11px] leading-relaxed break-all text-zinc-200 select-all">
				<span className="font-bold text-emerald-400">mkdir</span>{' '}
				<span className="text-amber-400">-p</span>{' '}
				<span className="text-cyan-300">~/.ssh</span>{' '}
				<span className="text-zinc-500">&amp;&amp;</span>{' '}
				<span className="font-bold text-emerald-400">echo</span>{' '}
				<span className="text-sky-300">"{publicKey.trim()}"</span>{' '}
				<span className="text-amber-400">&gt;&gt;</span>{' '}
				<span className="text-cyan-300">~/.ssh/authorized_keys</span>{' '}
				<span className="text-zinc-500">&amp;&amp;</span>{' '}
				<span className="font-bold text-emerald-400">chmod</span>{' '}
				<span className="text-amber-400">700</span>{' '}
				<span className="text-cyan-300">~/.ssh</span>{' '}
				<span className="text-zinc-500">&amp;&amp;</span>{' '}
				<span className="font-bold text-emerald-400">chmod</span>{' '}
				<span className="text-amber-400">600</span>{' '}
				<span className="text-cyan-300">~/.ssh/authorized_keys</span>
			</div>
		</div>
	);
}
