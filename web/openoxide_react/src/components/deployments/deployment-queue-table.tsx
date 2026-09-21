import {CheckCircle2, Clock, Terminal, XCircle} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {
	Table,
	TableHeader,
	TableBody,
	TableHead,
	TableRow,
	TableCell,
} from '#/components/ui/table';
import type {Deployment} from '#/hooks/deployments/use-deployments';
import {deploymentType} from './deployment-utils';
import {DeploymentStatusBadge} from './deployment-status-badge';

const HEAD_CLS = 'text-xs font-bold';

/** The Active Queue tab: heading, live table of in-flight builds, empty state. */
export function DeploymentQueueTable({
	queue,
	onViewLogs,
	onCancel,
}: {
	queue: Deployment[];
	onViewLogs: (d: Deployment) => void;
	onCancel: (id: number) => void;
}) {
	return (
		<>
			<div className="flex items-center justify-between">
				<h2 className="text-xs font-bold tracking-wider text-muted-foreground uppercase">
					Live Build Queue Processing
				</h2>
				<span className="text-xs text-muted-foreground">
					{queue.length > 0 ? 'Auto-refreshing every 1s' : 'Queue idle'}
				</span>
			</div>

			{queue.length > 0 ? (
				<div className="animate-in overflow-hidden rounded-xl bg-muted/20 shadow-2xs duration-200 fade-in dark:bg-muted/15">
					<Table>
						<TableHeader className="bg-muted/40">
							<TableRow className="">
								<TableHead className={`w-[80px] ${HEAD_CLS}`}>
									ID
								</TableHead>
								<TableHead className={HEAD_CLS}>
									Deployment Title
								</TableHead>
								<TableHead className={HEAD_CLS}>Type</TableHead>
								<TableHead className={HEAD_CLS}>Status</TableHead>
								<TableHead className={HEAD_CLS}>Time</TableHead>
								<TableHead className={`text-right ${HEAD_CLS}`}>
									Actions
								</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{queue.map(d => (
								<TableRow
									key={String(d.id)}
									className=" hover:bg-muted/20">
									<TableCell className="font-mono text-xs font-semibold text-muted-foreground">
										#{d.id}
									</TableCell>
									<TableCell className="text-xs font-semibold text-foreground">
										<div className="flex flex-col">
											<span>{d.title || `Deployment #${d.id}`}</span>
											{d.description && (
												<span className="max-w-xs truncate text-[11px] font-normal text-muted-foreground">
													{d.description}
												</span>
											)}
										</div>
									</TableCell>
									<TableCell>
										<Badge
											variant="outline"
											className="font-mono text-[10px]">
											{deploymentType(d)}
										</Badge>
									</TableCell>
									<TableCell>
										<DeploymentStatusBadge status={d.status ?? ''} />
									</TableCell>
									<TableCell className="font-mono text-xs text-muted-foreground">
										<span className="flex items-center gap-1">
											<Clock className="h-3 w-3 text-muted-foreground" />
											{new Date(
												Number(d.created_at || 0) * 1000,
											).toLocaleTimeString()}
										</span>
									</TableCell>
									<TableCell className="text-right">
										<div className="flex items-center justify-end gap-1.5">
											<Button
												size="sm"
												variant="outline"
												onClick={() => onViewLogs(d)}
												className="flex h-7 items-center gap-1 rounded-lg  px-2 text-xs font-semibold text-foreground hover:bg-muted">
												<Terminal className="h-3 w-3" /> Stream Logs
											</Button>
											{d.id !== undefined && (
												<Button
													size="sm"
													variant="outline"
													onClick={() => onCancel(Number(d.id))}
													className="flex h-7 items-center gap-1 rounded-lg border-destructive/20 px-2.5 text-xs font-semibold text-destructive hover:bg-destructive/10">
													<XCircle className="h-3.5 w-3.5" /> Cancel Build
												</Button>
											)}
										</div>
									</TableCell>
								</TableRow>
							))}
						</TableBody>
					</Table>
				</div>
			) : (
				<div className="flex flex-col items-center justify-center gap-2 rounded-xl bg-card p-6 text-center text-muted-foreground">
					<CheckCircle2 className="h-8 w-8 text-emerald-500 opacity-80" />
					<p className="text-sm font-semibold text-foreground">
						No Active Builds in Queue
					</p>
					<p className="text-xs">
						All application and compose deployments have completed
						successfully.
					</p>
				</div>
			)}
		</>
	);
}
