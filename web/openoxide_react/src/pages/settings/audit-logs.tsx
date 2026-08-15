import {createFileRoute} from '@tanstack/react-router';
import {ClipboardList, ShieldCheck, UserCheck} from 'lucide-react';
import {Badge} from '#/components/ui/badge';

export const Route = createFileRoute('/_app/settings/audit-logs')({
	component: AuditLogsSettingsPage,
});

function AuditLogsSettingsPage() {
	return (
		<div className="p-6 flex flex-col gap-6 max-w-7xl mx-auto w-full animate-in fade-in duration-200">
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold text-foreground tracking-tight">Audit Logs</h1>
				<p className="text-xs text-muted-foreground">
					Security event history, user authentication logs, and infrastructure mutations
				</p>
			</div>

			<div className="border rounded-xl bg-card overflow-hidden">
				<div className="p-4 border-b border-border/40 flex items-center justify-between">
					<div className="flex items-center gap-2">
						<ClipboardList className="size-4 text-primary" />
						<span className="text-xs font-bold text-foreground">Recent Security Events</span>
					</div>
					<Badge variant="outline" className="text-[10px]">Realtime Audit Log</Badge>
				</div>
				<div className="divide-y divide-border/40">
					<div className="p-4 flex items-center justify-between text-xs">
						<div className="flex items-center gap-3">
							<ShieldCheck className="size-4 text-emerald-500" />
							<div>
								<p className="font-semibold text-foreground">Session Authenticated</p>
								<p className="text-[10px] text-muted-foreground font-mono">User login token validated successfully</p>
							</div>
						</div>
						<span className="text-[10px] text-muted-foreground font-mono">Just now</span>
					</div>
					<div className="p-4 flex items-center justify-between text-xs">
						<div className="flex items-center gap-3">
							<UserCheck className="size-4 text-primary" />
							<div>
								<p className="font-semibold text-foreground">Permission Verified</p>
								<p className="text-[10px] text-muted-foreground font-mono">Organization claims verified for active workspace</p>
							</div>
						</div>
						<span className="text-[10px] text-muted-foreground font-mono">2 mins ago</span>
					</div>
				</div>
			</div>
		</div>
	);
}
