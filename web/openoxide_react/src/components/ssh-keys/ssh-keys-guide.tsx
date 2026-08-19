import {Card, CardContent} from '#/components/ui/card';
import {HelpCircle, Terminal} from 'lucide-react';

export function SshKeysGuide() {
	return (
		<Card className="mt-4 rounded-2xl border border-primary/20 bg-card/60 p-5 shadow-sm">
			<CardContent className="flex flex-col gap-4 p-0">
				<div className="flex items-center gap-2.5 text-sm font-bold text-foreground">
					<HelpCircle className="h-5 w-5 shrink-0 text-primary" />
					<span>How to Authorize SSH Keys on Remote Linux Servers</span>
				</div>

				<div className="grid grid-cols-1 gap-4 text-xs md:grid-cols-3">
					<div className="flex flex-col gap-1.5 rounded-xl border border-border/40 bg-muted/30 p-3.5">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<span className="flex h-5 w-5 items-center justify-center rounded-full bg-primary/20 text-[11px] text-primary">
								1
							</span>
							<span>Copy Public SSH Key</span>
						</div>
						<p className="leading-relaxed text-muted-foreground">
							Click "Copy Public" on your SSH Key card to copy the public
							key string (e.g.{' '}
							<code className="font-mono text-primary">
								ssh-ed25519 AAA...
							</code>
							).
						</p>
					</div>

					<div className="flex flex-col gap-1.5 rounded-xl border border-border/40 bg-muted/30 p-3.5">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<span className="flex h-5 w-5 items-center justify-center rounded-full bg-primary/20 text-[11px] text-primary">
								2
							</span>
							<span>Paste to Remote Server</span>
						</div>
						<p className="leading-relaxed text-muted-foreground">
							Append the public key to your remote server's{' '}
							<code className="font-mono text-primary">
								~/.ssh/authorized_keys
							</code>{' '}
							file.
						</p>
					</div>

					<div className="flex flex-col gap-1.5 rounded-xl border border-border/40 bg-muted/30 p-3.5">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<span className="flex h-5 w-5 items-center justify-center rounded-full bg-primary/20 text-[11px] text-primary">
								3
							</span>
							<span>Set Strict Permissions</span>
						</div>
						<p className="leading-relaxed text-muted-foreground">
							Run{' '}
							<code className="font-mono text-primary">
								chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys
							</code>{' '}
							on your host machine.
						</p>
					</div>
				</div>

				<div className="flex flex-col gap-1 rounded-xl border border-border/50 bg-muted/40 p-3 font-mono text-[11px]">
					<span className="flex items-center gap-1.5 font-sans text-xs font-semibold text-muted-foreground">
						<Terminal className="h-3.5 w-3.5 text-primary" /> One-Line
						Server Setup Command:
					</span>
					<code className="rounded border border-border/60 bg-background p-2 break-all text-foreground select-all">
						mkdir -p ~/.ssh && echo "YOUR_PUBLIC_KEY" &gt;&gt;
						~/.ssh/authorized_keys && chmod 700 ~/.ssh && chmod 600
						~/.ssh/authorized_keys
					</code>
				</div>
			</CardContent>
		</Card>
	);
}
