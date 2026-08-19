import {Clock, Terminal, XCircle, Trash2, Zap} from 'lucide-react';
import {Button} from '#/components/ui/button';

interface DatabaseDeploymentsListProps {
	events: any[];
	isBuildActive: (e: any) => boolean;
	getStatusBadge: (e: any) => string;
	onOpenLogs: (id: number) => void;
	onCancel: (id: number) => void;
	onDelete: (id: number) => void;
}

export function DatabaseDeploymentsList({
	events,
	isBuildActive,
	getStatusBadge,
	onOpenLogs,
	onCancel,
	onDelete,
}: DatabaseDeploymentsListProps) {
	if (events.length === 0) {
		return (
			<section className="overflow-hidden rounded-xl border border-border bg-card">
				<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground">
					<Zap className="mb-3 h-10 w-10 opacity-30" />
					<p className="text-xs font-semibold">
						No database deployment history recorded yet
					</p>
				</div>
			</section>
		);
	}

	return (
		<section className="overflow-hidden rounded-xl border border-border bg-card">
			<div className="divide-y divide-border/60">
				{events.map((e: any) => {
					const isActive = isBuildActive(e);
					return (
						<div
							key={e.id}
							className="flex items-center justify-between p-4 transition-colors hover:bg-muted/10">
							<div className="flex min-w-0 flex-col gap-0.5">
								<span className="truncate text-xs font-semibold text-foreground">
									{e.title || `Database Deployment #${e.id}`}
								</span>
								{e.description && (
									<span className="truncate text-[11px] text-muted-foreground">
										{e.description}
									</span>
								)}
							</div>

							<div className="flex shrink-0 items-center gap-3">
								<span
									className={`rounded border px-2 py-0.5 text-[10px] font-bold tracking-wider uppercase ${getStatusBadge(e)}`}>
									{e.status || 'DEPLOYED'}
								</span>
								<span className="flex items-center gap-1 text-[10px] font-medium text-muted-foreground">
									<Clock className="h-3 w-3" />
									{e.created_at
										? new Date(e.created_at * 1000).toLocaleDateString()
										: 'Just now'}
								</span>

								<Button
									size="sm"
									variant="outline"
									onClick={() => onOpenLogs(e.id)}
									className="flex h-7 items-center gap-1 rounded-lg border-border px-2 text-xs font-semibold text-foreground hover:bg-muted">
									<Terminal className="h-3 w-3" /> Stream Logs
								</Button>

								{isActive && (
									<Button
										size="sm"
										variant="ghost"
										onClick={() => onCancel(e.id)}
										className="flex h-7 items-center gap-1 rounded-lg px-2 text-xs font-semibold text-destructive hover:bg-destructive/10 hover:text-destructive">
										<XCircle className="h-3 w-3" /> Cancel
									</Button>
								)}

								{!isActive && e.id !== undefined && (
									<Button
										size="sm"
										variant="ghost"
										onClick={() => onDelete(e.id)}
										className="flex h-7 items-center gap-1 rounded-lg px-1.5 text-xs font-semibold text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
										<Trash2 className="h-3.5 w-3.5" />
									</Button>
								)}
							</div>
						</div>
					);
				})}
			</div>
		</section>
	);
}
