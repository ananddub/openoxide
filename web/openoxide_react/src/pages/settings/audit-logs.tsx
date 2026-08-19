import {createFileRoute} from '@tanstack/react-router';
import {ClipboardList, ShieldCheck, UserCheck} from 'lucide-react';
import {Badge} from '#/components/ui/badge';

export const Route = createFileRoute('/_app/settings/audit-logs')({
	component: AuditLogsSettingsPage,
});

function AuditLogsSettingsPage() {
	return (
		<div className="mx-auto flex w-full max-w-7xl animate-in flex-col gap-6 p-6 duration-200 fade-in">
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold tracking-tight text-foreground">
					Audit Logs
				</h1>
				<p className="text-xs text-muted-foreground">
					Security event history, user authentication logs, and
					infrastructure mutations
				</p>
			</div>

			<div className="overflow-hidden rounded-xl border bg-card">
				<div className="flex items-center justify-between border-b border-border/40 p-4">
					<div className="flex items-center gap-2">
						<ClipboardList className="size-4 text-primary" />
						<span className="text-xs font-bold text-foreground">
							Recent Security Events
						</span>
					</div>
					<Badge variant="outline" className="text-[10px]">
						Realtime Audit Log
					</Badge>
				</div>
				<div className="divide-y divide-border/40">
					<div className="flex items-center justify-between p-4 text-xs">
						<div className="flex items-center gap-3">
							<ShieldCheck className="size-4 text-emerald-500" />
							<div>
								<p className="font-semibold text-foreground">
									Session Authenticated
								</p>
								<p className="font-mono text-[10px] text-muted-foreground">
									User login token validated successfully
								</p>
							</div>
						</div>
						<span className="font-mono text-[10px] text-muted-foreground">
							Just now
						</span>
					</div>
					<div className="flex items-center justify-between p-4 text-xs">
						<div className="flex items-center gap-3">
							<UserCheck className="size-4 text-primary" />
							<div>
								<p className="font-semibold text-foreground">
									Permission Verified
								</p>
								<p className="font-mono text-[10px] text-muted-foreground">
									Organization claims verified for active workspace
								</p>
							</div>
						</div>
						<span className="font-mono text-[10px] text-muted-foreground">
							2 mins ago
						</span>
					</div>
				</div>
			</div>
		</div>
	);
}
