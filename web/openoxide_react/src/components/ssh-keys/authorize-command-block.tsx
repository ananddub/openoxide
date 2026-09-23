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
				<span className="flex items-center gap-1.5 text-[13px] font-medium text-foreground">
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
			<div className="max-h-24 w-full min-w-0 overflow-y-auto rounded-md border border-border/60 bg-muted/40 p-2.5 font-mono text-[11px] leading-relaxed break-all select-all">
				<span className="font-semibold text-emerald-500">mkdir</span>{' '}
				<span className="text-amber-500">-p</span>{' '}
				<span className="text-sky-500">~/.ssh</span>{' '}
				<span className="text-muted-foreground">&amp;&amp;</span>{' '}
				<span className="font-semibold text-emerald-500">echo</span>{' '}
				<span className="text-foreground">"{publicKey.trim()}"</span>{' '}
				<span className="text-amber-500">&gt;&gt;</span>{' '}
				<span className="text-sky-500">~/.ssh/authorized_keys</span>{' '}
				<span className="text-muted-foreground">&amp;&amp;</span>{' '}
				<span className="font-semibold text-emerald-500">chmod</span>{' '}
				<span className="text-amber-500">700</span>{' '}
				<span className="text-sky-500">~/.ssh</span>{' '}
				<span className="text-muted-foreground">&amp;&amp;</span>{' '}
				<span className="font-semibold text-emerald-500">chmod</span>{' '}
				<span className="text-amber-500">600</span>{' '}
				<span className="text-sky-500">~/.ssh/authorized_keys</span>
			</div>
		</div>
	);
}
