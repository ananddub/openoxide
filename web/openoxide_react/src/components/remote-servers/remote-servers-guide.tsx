import {Card, CardContent} from '#/components/ui/card';
import {HelpCircle, Server, ShieldCheck, Cpu} from 'lucide-react';

export function RemoteServersGuide() {
	return (
		<Card className="bg-card/60 border border-primary/20 rounded-2xl p-5 shadow-sm mt-4">
			<CardContent className="p-0 flex flex-col gap-4">
				<div className="flex items-center gap-2.5 text-sm font-bold text-foreground">
					<HelpCircle className="w-5 h-5 text-primary shrink-0" />
					<span>How Remote Servers Work in OpenOxide</span>
				</div>

				<div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs">
					<div className="flex flex-col gap-1.5 p-3.5 bg-muted/30 border border-border/40 rounded-xl">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<Server className="w-4 h-4 text-primary shrink-0" />
							<span>1. Connect Remote Node</span>
						</div>
						<p className="text-muted-foreground leading-relaxed">
							Add external VPS/Linux host IP, SSH port, and select an SSH Key credential.
						</p>
					</div>

					<div className="flex flex-col gap-1.5 p-3.5 bg-muted/30 border border-border/40 rounded-xl">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<ShieldCheck className="w-4 h-4 text-primary shrink-0" />
							<span>2. Test & Audit Connection</span>
						</div>
						<p className="text-muted-foreground leading-relaxed">
							Click "Setup & Audit" to run automated SSH checks, audit system RAM, CPU cores, and OS.
						</p>
					</div>

					<div className="flex flex-col gap-1.5 p-3.5 bg-muted/30 border border-border/40 rounded-xl">
						<div className="flex items-center gap-2 font-semibold text-foreground">
							<Cpu className="w-4 h-4 text-primary shrink-0" />
							<span>3. Deploy Remote Clusters</span>
						</div>
						<p className="text-muted-foreground leading-relaxed">
							Deploy applications, Docker Compose stacks, and databases directly onto your remote server node.
						</p>
					</div>
				</div>
			</CardContent>
		</Card>
	);
}
