import {Card, CardContent} from '#/components/ui/card';
import {HelpCircle, Terminal, CheckCircle2, Shield} from 'lucide-react';

export function SshKeysGuide() {
	return (
		<Card className="bg-card/60 border border-primary/20 rounded-2xl p-5 shadow-sm mt-4">
			<CardContent className="p-0 flex flex-col gap-4">
				<div className="flex items-center gap-2.5 text-sm font-bold text-foreground">
					<HelpCircle className="w-5 h-5 text-primary shrink-0" />
					<span>How to Authorize SSH Keys on Remote Linux Servers</span>
				</div>

				<div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs">
					<div className="flex flex-col gap-1.5 p-3.5 bg-muted/30 border border-border/40 rounded-xl">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<span className="w-5 h-5 rounded-full bg-primary/20 text-primary flex items-center justify-center text-[11px]">1</span>
							<span>Copy Public SSH Key</span>
						</div>
						<p className="text-muted-foreground leading-relaxed">
							Click "Copy Public" on your SSH Key card to copy the public key string (e.g. <code className="font-mono text-primary">ssh-ed25519 AAA...</code>).
						</p>
					</div>

					<div className="flex flex-col gap-1.5 p-3.5 bg-muted/30 border border-border/40 rounded-xl">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<span className="w-5 h-5 rounded-full bg-primary/20 text-primary flex items-center justify-center text-[11px]">2</span>
							<span>Paste to Remote Server</span>
						</div>
						<p className="text-muted-foreground leading-relaxed">
							Append the public key to your remote server's <code className="font-mono text-primary">~/.ssh/authorized_keys</code> file.
						</p>
					</div>

					<div className="flex flex-col gap-1.5 p-3.5 bg-muted/30 border border-border/40 rounded-xl">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<span className="w-5 h-5 rounded-full bg-primary/20 text-primary flex items-center justify-center text-[11px]">3</span>
							<span>Set Strict Permissions</span>
						</div>
						<p className="text-muted-foreground leading-relaxed">
							Run <code className="font-mono text-primary">chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys</code> on your host machine.
						</p>
					</div>
				</div>

				<div className="bg-muted/40 p-3 rounded-xl border border-border/50 flex flex-col gap-1 font-mono text-[11px]">
					<span className="text-muted-foreground font-sans font-semibold text-xs flex items-center gap-1.5">
						<Terminal className="w-3.5 h-3.5 text-primary" /> One-Line Server Setup Command:
					</span>
					<code className="text-foreground bg-background p-2 rounded border border-border/60 break-all select-all">
						mkdir -p ~/.ssh && echo "YOUR_PUBLIC_KEY" &gt;&gt; ~/.ssh/authorized_keys && chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys
					</code>
				</div>
			</CardContent>
		</Card>
	);
}
