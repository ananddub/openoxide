import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Terminal, Copy, Check} from 'lucide-react';
import {toast} from 'sonner';

interface AuthorizeCommandBlockProps {
	publicKey: string;
}

export function AuthorizeCommandBlock({publicKey}: AuthorizeCommandBlockProps) {
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
		<div className="rounded-lg border border-border/60 bg-muted/20 p-2.5 space-y-1.5 min-w-0 w-full overflow-hidden">
			<div className="flex items-center justify-between">
				<span className="text-xs font-semibold text-foreground flex items-center gap-1.5">
					<Terminal className="w-3.5 h-3.5 text-amber-500" />
					Authorize on Server
				</span>
				<Button
					type="button"
					variant="ghost"
					size="xs"
					onClick={handleCopy}
					className="h-6 text-[11px] gap-1 font-semibold text-primary hover:text-primary p-1"
				>
					{copied ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
					{copied ? 'Copied' : 'Copy'}
				</Button>
			</div>
			<div className="p-2.5 rounded bg-zinc-950 font-mono text-[11px] break-all select-all border border-zinc-800/80 max-h-24 overflow-y-auto leading-relaxed min-w-0 w-full text-zinc-200">
				<span className="text-emerald-400 font-bold">mkdir</span> <span className="text-amber-400">-p</span> <span className="text-cyan-300">~/.ssh</span> <span className="text-zinc-500">&amp;&amp;</span> <span className="text-emerald-400 font-bold">echo</span> <span className="text-sky-300">"{publicKey.trim()}"</span> <span className="text-amber-400">&gt;&gt;</span> <span className="text-cyan-300">~/.ssh/authorized_keys</span> <span className="text-zinc-500">&amp;&amp;</span> <span className="text-emerald-400 font-bold">chmod</span> <span className="text-amber-400">700</span> <span className="text-cyan-300">~/.ssh</span> <span className="text-zinc-500">&amp;&amp;</span> <span className="text-emerald-400 font-bold">chmod</span> <span className="text-amber-400">600</span> <span className="text-cyan-300">~/.ssh/authorized_keys</span>
			</div>
		</div>
	);
}
