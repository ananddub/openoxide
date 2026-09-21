import {
	Clock,
	RefreshCw,
	Terminal,
	Trash2,
	XCircle,
	Zap,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	formatTimestamp,
	getStatusBadge,
	isBuildActive,
} from './build-status';

export interface DeploymentsListProps {
	deployments: any[];
	isLoading?: boolean;
	emptyLabel?: string;
	/** Prefix for the synthetic title of rows without one. */
	titlePrefix?: string;
	/** Status shown when a row carries neither status nor state. */
	defaultStatus?: string;
	onOpenStream: (id: number) => void;
	onCancel?: (id: number) => void;
	onDelete?: (id: number) => void;
}

export function DeploymentsList({
	deployments,
	isLoading,
	emptyLabel = 'No deployments registered yet',
	titlePrefix = 'Deployment',
	defaultStatus = 'PENDING',
	onOpenStream,
	onCancel,
	onDelete,
}: DeploymentsListProps) {
	return (
		<section className="overflow-hidden rounded-xl bg-card">
			{isLoading && deployments.length === 0 ? (
				<div className="flex justify-center py-12">
					<RefreshCw className="h-6 w-6 animate-spin text-muted-foreground/45" />
				</div>
			) : deployments.length === 0 ? (
				<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground">
					<Zap className="mb-3 h-10 w-10 opacity-30" />
					<p className="text-xs font-semibold">{emptyLabel}</p>
				</div>
			) : (
				<div className="divide-y divide-border/60">
					{deployments.map((e: any) => {
						const isActive = isBuildActive(e);
						return (
							<div
								key={e.id}
								className="flex items-center justify-between p-4 transition-colors hover:bg-muted/10">
								<div className="flex min-w-0 flex-col gap-0.5">
									<span className="truncate text-xs font-semibold text-foreground">
										{e.title || `${titlePrefix} #${e.id}`}
									</span>
									{(e.description || e.message) && (
										<span className="truncate text-[11px] text-muted-foreground">
											{e.description || e.message}
										</span>
									)}
								</div>

								<div className="flex shrink-0 items-center gap-3">
									<span
										className={`rounded px-2 py-0.5 text-[10px] font-bold tracking-wider uppercase ${getStatusBadge(e)}`}>
										{e.status || e.state || defaultStatus}
									</span>
									<span className="flex items-center gap-1 text-[10px] font-medium text-muted-foreground">
										<Clock className="h-3 w-3" />
										{formatTimestamp(e.created_at)}
									</span>

									<Button
										size="sm"
										variant="outline"
										onClick={() => onOpenStream(e.id)}
										className="flex h-7 items-center gap-1 rounded-lg px-2 text-xs font-semibold text-foreground hover:bg-muted">
										<Terminal className="h-3 w-3" /> Stream Logs
									</Button>

									{isActive && onCancel && (
										<Button
											size="sm"
											variant="ghost"
											onClick={() => onCancel(e.id)}
											className="flex h-7 items-center gap-1 rounded-lg px-2 text-xs font-semibold text-destructive hover:bg-destructive/10 hover:text-destructive">
											<XCircle className="h-3 w-3" /> Cancel
										</Button>
									)}

									{!isActive && onDelete && e.id !== undefined && (
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
			)}
		</section>
	);
}
