import {Card, CardContent} from '#/components/ui/card';
import {HelpCircle, Server, ShieldCheck, Cpu} from 'lucide-react';

export function RemoteServersGuide() {
	return (
		<Card className="mt-4 rounded-2xl border border-primary/20 bg-card/60 p-5 shadow-sm">
			<CardContent className="flex flex-col gap-4 p-0">
				<div className="flex items-center gap-2.5 text-sm font-bold text-foreground">
					<HelpCircle className="h-5 w-5 shrink-0 text-primary" />
					<span>How Remote Servers Work in OpenOxide</span>
				</div>

				<div className="grid grid-cols-1 gap-4 text-xs md:grid-cols-3">
					<div className="flex flex-col gap-1.5 rounded-xl border border-border/40 bg-muted/30 p-3.5">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<Server className="h-4 w-4 shrink-0 text-primary" />
							<span>1. Connect Remote Node</span>
						</div>
						<p className="leading-relaxed text-muted-foreground">
							Add external VPS/Linux host IP, SSH port, and select an SSH
							Key credential.
						</p>
					</div>

					<div className="flex flex-col gap-1.5 rounded-xl border border-border/40 bg-muted/30 p-3.5">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<ShieldCheck className="h-4 w-4 shrink-0 text-primary" />
							<span>2. Test & Audit Connection</span>
						</div>
						<p className="leading-relaxed text-muted-foreground">
							Click "Setup & Audit" to run automated SSH checks, audit
							system RAM, CPU cores, and OS.
						</p>
					</div>

					<div className="flex flex-col gap-1.5 rounded-xl border border-border/40 bg-muted/30 p-3.5">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<Cpu className="h-4 w-4 shrink-0 text-primary" />
							<span>3. Deploy Remote Clusters</span>
						</div>
						<p className="leading-relaxed text-muted-foreground">
							Deploy applications, Docker Compose stacks, and databases
							directly onto your remote server node.
						</p>
					</div>
				</div>
			</CardContent>
		</Card>
	);
}
