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
			<section className="bg-card border border-border rounded-xl overflow-hidden">
				<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground">
					<Zap className="w-10 h-10 opacity-30 mb-3" />
					<p className="text-xs font-semibold">No database deployment history recorded yet</p>
				</div>
			</section>
		);
	}

	return (
		<section className="bg-card border border-border rounded-xl overflow-hidden">
			<div className="divide-y divide-border/60">
				{events.map((e: any) => {
					const isActive = isBuildActive(e);
					return (
						<div key={e.id} className="flex items-center justify-between p-4 hover:bg-muted/10 transition-colors">
							<div className="min-w-0 flex flex-col gap-0.5">
								<span className="text-xs font-semibold text-foreground truncate">{e.title || `Database Deployment #${e.id}`}</span>
								{e.description && <span className="text-[11px] text-muted-foreground truncate">{e.description}</span>}
							</div>

							<div className="flex items-center gap-3 shrink-0">
								<span className={`text-[10px] font-bold px-2 py-0.5 rounded border uppercase tracking-wider ${getStatusBadge(e)}`}>
									{e.status || 'DEPLOYED'}
								</span>
								<span className="text-[10px] text-muted-foreground flex items-center gap-1 font-medium">
									<Clock className="w-3 h-3" />
									{e.created_at ? new Date(e.created_at * 1000).toLocaleDateString() : 'Just now'}
								</span>

								<Button
									size="sm"
									variant="outline"
									onClick={() => onOpenLogs(e.id)}
									className="h-7 text-xs border-border text-foreground hover:bg-muted px-2 rounded-lg font-semibold flex items-center gap-1"
								>
									<Terminal className="w-3 h-3" /> Stream Logs
								</Button>

								{isActive && (
									<Button
										size="sm"
										variant="ghost"
										onClick={() => onCancel(e.id)}
										className="h-7 text-xs text-destructive hover:text-destructive hover:bg-destructive/10 px-2 rounded-lg font-semibold flex items-center gap-1"
									>
										<XCircle className="w-3 h-3" /> Cancel
									</Button>
								)}

								{!isActive && e.id !== undefined && (
									<Button
										size="sm"
										variant="ghost"
										onClick={() => onDelete(e.id)}
										className="h-7 text-xs text-muted-foreground hover:text-destructive hover:bg-destructive/10 px-1.5 rounded-lg font-semibold flex items-center gap-1"
									>
										<Trash2 className="w-3.5 h-3.5" />
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
