import {createFileRoute} from '@tanstack/react-router';
import {Activity, Cpu, HardDrive, Network, Server} from 'lucide-react';
import {Card, CardHeader, CardTitle, CardDescription, CardContent} from '#/components/ui/card';
import {Badge} from '#/components/ui/badge';

export const Route = createFileRoute('/_app/settings/server')({
	component: WebServerSettingsPage,
});

function WebServerSettingsPage() {
	return (
		<div className="p-6 flex flex-col gap-6 max-w-7xl mx-auto w-full animate-in fade-in duration-200">
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold text-foreground tracking-tight">Web Server Settings</h1>
				<p className="text-xs text-muted-foreground">
					Manage host web server configuration, Traefik reverse proxy, and core runtime status
				</p>
			</div>

			<div className="grid grid-cols-1 md:grid-cols-3 gap-4">
				<div className="p-5 border rounded-xl bg-card flex flex-col gap-3">
					<div className="flex items-center gap-3">
						<div className="size-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
							<Activity className="size-5" />
						</div>
						<div>
							<h3 className="text-sm font-bold text-foreground">Rust Engine</h3>
							<p className="text-[10px] text-muted-foreground">Axum Async Tokio Worker</p>
						</div>
					</div>
					<div className="flex items-center justify-between border-t border-border/40 pt-3">
						<span className="text-xs text-muted-foreground">Status</span>
						<Badge variant="default" className="bg-emerald-500 text-[10px] font-bold">ONLINE</Badge>
					</div>
				</div>

				<div className="p-5 border rounded-xl bg-card flex flex-col gap-3">
					<div className="flex items-center gap-3">
						<div className="size-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
							<Server className="size-5" />
						</div>
						<div>
							<h3 className="text-sm font-bold text-foreground">Reverse Proxy</h3>
							<p className="text-[10px] text-muted-foreground">Traefik v3 Dynamic Proxy</p>
						</div>
					</div>
					<div className="flex items-center justify-between border-t border-border/40 pt-3">
						<span className="text-xs text-muted-foreground">Status</span>
						<Badge variant="default" className="bg-emerald-500 text-[10px] font-bold">ACTIVE</Badge>
					</div>
				</div>

				<div className="p-5 border rounded-xl bg-card flex flex-col gap-3">
					<div className="flex items-center gap-3">
						<div className="size-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
							<HardDrive className="size-5" />
						</div>
						<div>
							<h3 className="text-sm font-bold text-foreground">Database Storage</h3>
							<p className="text-[10px] text-muted-foreground">SQLite WAL Mode</p>
						</div>
					</div>
					<div className="flex items-center justify-between border-t border-border/40 pt-3">
						<span className="text-xs text-muted-foreground">Status</span>
						<Badge variant="default" className="bg-emerald-500 text-[10px] font-bold">HEALTHY</Badge>
					</div>
				</div>
			</div>
		</div>
	);
}
