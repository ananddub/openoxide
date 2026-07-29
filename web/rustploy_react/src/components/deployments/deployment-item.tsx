import {
	Rocket,
	Boxes,
	Terminal,
	XCircle,
	AlertCircle,
	Database,
	Clock,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {TableRow, TableCell} from '#/components/ui/table';
import type {Deployment} from '#/hooks/deployments/use-deployments';

interface DeploymentItemProps {
	deployment: Deployment;
	onViewLogs: (d: Deployment) => void;
	onViewError: (d: Deployment) => void;
	onCancel: (id: number) => void;
}

export function DeploymentItem({
	deployment,
	onViewLogs,
	onViewError,
	onCancel,
}: DeploymentItemProps) {
	const d = deployment;
	const hasApp = d.application_id !== null && d.application_id !== undefined;
	const hasCompose = d.compose_id !== null && d.compose_id !== undefined;
	const hasDatabase = d.database_id !== null && d.database_id !== undefined;
	const type = hasApp ? 'Application' : hasCompose ? 'Compose' : hasDatabase ? 'Database' : 'Generic';
	const status = (d.status || '').toUpperCase();
	const isRunningOrQueued = status === 'RUNNING' || status === 'QUEUED' || status.includes('BUILD') || status.includes('PEND');

	const getStatusBadge = () => {
		if (status === 'DONE' || status === 'HEALTHY' || status === 'SUCCESS' || status === 'DEPLOYED') {
			return (
				<Badge variant="outline" className="text-[10px] uppercase font-bold tracking-wider text-emerald-500 border-emerald-500/30 bg-emerald-500/10 gap-1.5 py-0.5">
					<span className="size-1.5 rounded-full bg-emerald-500" />
					SUCCESS
				</Badge>
			);
		}
		if (status === 'ERROR' || status === 'FAILED' || status === 'CRASHED') {
			return (
				<Badge variant="outline" className="text-[10px] uppercase font-bold tracking-wider text-rose-500 border-rose-500/30 bg-rose-500/10 gap-1.5 py-0.5">
					<span className="size-1.5 rounded-full bg-rose-500" />
					FAILED
				</Badge>
			);
		}
		if (isRunningOrQueued) {
			return (
				<Badge variant="outline" className="text-[10px] uppercase font-bold tracking-wider text-amber-500 border-amber-500/30 bg-amber-500/10 animate-pulse gap-1.5 py-0.5">
					<span className="size-1.5 rounded-full bg-amber-500 animate-ping" />
					{status}
				</Badge>
			);
		}
		return (
			<Badge variant="secondary" className="text-[10px] uppercase font-bold tracking-wider gap-1.5 py-0.5">
				<span className="size-1.5 rounded-full bg-muted-foreground/50" />
				{status}
			</Badge>
		);
	};

	const formatTimestamp = (raw: any) => {
		if (!raw) return 'N/A';
		const num = Number(raw);
		if (isNaN(num)) return String(raw);
		const ms = num < 1e11 ? num * 1000 : num;
		return new Date(ms).toLocaleDateString();
	};

	return (
		<TableRow className="border-border/30 hover:bg-muted/30 transition-colors">
			{/* ID */}
			<TableCell className="font-mono text-xs font-semibold text-muted-foreground w-[80px]">
				#{d.id}
			</TableCell>

			{/* Title & Info */}
			<TableCell>
				<div className="flex items-start gap-2.5 min-w-0">
					<div className="w-7 h-7 rounded-md bg-muted/40 flex items-center justify-center shrink-0 border border-border/40 mt-0.5">
						{hasApp ? (
							<Rocket className="w-3.5 h-3.5 text-primary" />
						) : hasCompose ? (
							<Boxes className="w-3.5 h-3.5 text-amber-500" />
						) : (
							<Database className="w-3.5 h-3.5 text-emerald-500" />
						)}
					</div>
					<div className="flex flex-col gap-0.5 min-w-0">
						<span className="text-xs font-bold text-foreground truncate">
							{d.title || `Deployment #${d.id}`}
						</span>
						{d.description && (
							<span className="text-[11px] text-muted-foreground truncate max-w-md">
								{d.description}
							</span>
						)}
						{d.error_message && (
							<Button
								variant="link"
								size="xs"
								onClick={e => {
									e.stopPropagation();
									onViewError(d);
								}}
								className="text-[11px] text-destructive hover:underline font-medium p-0 h-auto justify-start flex items-center gap-1 text-left mt-0.5">
								<AlertCircle className="w-3 h-3 shrink-0" />
								<span className="truncate max-w-md">Error: {d.error_message}</span>
							</Button>
						)}
					</div>
				</div>
			</TableCell>

			{/* Type */}
			<TableCell>
				<Badge variant="outline" className="text-[10px] font-mono font-medium border-border/40 text-muted-foreground">
					{type}
				</Badge>
			</TableCell>

			{/* Status */}
			<TableCell>
				{getStatusBadge()}
			</TableCell>

			{/* Date */}
			<TableCell className="text-xs text-muted-foreground font-mono">
				<span className="flex items-center gap-1">
					<Clock className="w-3 h-3 text-muted-foreground/70" />
					{formatTimestamp(d.created_at)}
				</span>
			</TableCell>

			{/* Actions */}
			<TableCell className="text-right">
				<div className="flex items-center justify-end gap-1.5">
					<Button
						size="sm"
						variant="outline"
						onClick={() => onViewLogs(d)}
						className="h-7 text-xs border-border/40 bg-muted/20 dark:bg-muted/15 text-foreground hover:bg-muted/40 px-2 rounded-lg font-semibold flex items-center gap-1 shadow-2xs">
						<Terminal className="w-3 h-3 text-primary" /> Logs
					</Button>
					{isRunningOrQueued && d.id !== undefined && (
						<Button
							size="sm"
							variant="ghost"
							onClick={() => onCancel(d.id!)}
							className="h-7 text-xs text-destructive hover:text-destructive hover:bg-destructive/10 px-2 rounded-lg font-semibold flex items-center gap-1">
							<XCircle className="w-3 h-3" /> Cancel
						</Button>
					)}
				</div>
			</TableCell>
		</TableRow>
	);
}
