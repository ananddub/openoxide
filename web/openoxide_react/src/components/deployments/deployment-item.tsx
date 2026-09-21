import {
	Rocket,
	Boxes,
	Terminal,
	XCircle,
	AlertCircle,
	Database,
	Clock,
	Trash2,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {TableRow, TableCell} from '#/components/ui/table';
import type {Deployment} from '#/hooks/deployments/use-deployments';
import {
	deploymentType,
	isBusyStatus,
	formatDeploymentDate,
} from './deployment-utils';
import {DeploymentStatusBadge} from './deployment-status-badge';

interface DeploymentItemProps {
	deployment: Deployment;
	onViewLogs: (d: Deployment) => void;
	onViewError: (d: Deployment) => void;
	onCancel: (id: number) => void;
	onDelete?: (id: number) => void;
}

export function DeploymentItem({
	deployment,
	onViewLogs,
	onViewError,
	onCancel,
	onDelete,
}: DeploymentItemProps) {
	const d = deployment;
	const hasApp = d.application_id != null;
	const hasCompose = d.compose_id != null;
	const status = (d.status || '').toUpperCase();
	const busy = isBusyStatus(status);

	return (
		<TableRow className=" transition-colors hover:bg-muted/30">
			{/* ID */}
			<TableCell className="w-[80px] font-mono text-xs font-semibold text-muted-foreground">
				#{d.id}
			</TableCell>

			{/* Title & Info */}
			<TableCell>
				<div className="flex min-w-0 items-start gap-2.5">
					<div className="mt-0.5 flex h-7 w-7 shrink-0 items-center justify-center rounded-md bg-muted/40">
						{hasApp ? (
							<Rocket className="h-3.5 w-3.5 text-primary" />
						) : hasCompose ? (
							<Boxes className="h-3.5 w-3.5 text-amber-500" />
						) : (
							<Database className="h-3.5 w-3.5 text-emerald-500" />
						)}
					</div>
					<div className="flex min-w-0 flex-col gap-0.5">
						<span className="truncate text-xs font-bold text-foreground">
							{d.title || `Deployment #${d.id}`}
						</span>
						{d.description && (
							<span className="max-w-md truncate text-[11px] text-muted-foreground">
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
								className="mt-0.5 flex h-auto items-center justify-start gap-1 p-0 text-left text-[11px] font-medium text-destructive hover:underline">
								<AlertCircle className="h-3 w-3 shrink-0" />
								<span className="max-w-md truncate">
									Error: {d.error_message}
								</span>
							</Button>
						)}
					</div>
				</div>
			</TableCell>

			{/* Type */}
			<TableCell>
				<Badge
					variant="outline"
					className=" font-mono text-[10px] font-medium text-muted-foreground">
					{deploymentType(d)}
				</Badge>
			</TableCell>

			{/* Status */}
			<TableCell>
				<DeploymentStatusBadge status={status} />
			</TableCell>

			{/* Date */}
			<TableCell className="font-mono text-xs text-muted-foreground">
				<span className="flex items-center gap-1">
					<Clock className="h-3 w-3 text-muted-foreground/70" />
					{formatDeploymentDate(d.created_at)}
				</span>
			</TableCell>

			{/* Actions */}
			<TableCell className="text-right">
				<div className="flex items-center justify-end gap-1.5">
					<Button
						size="sm"
						variant="outline"
						onClick={() => onViewLogs(d)}
						className="flex h-7 items-center gap-1 rounded-lg  bg-muted/20 px-2 text-xs font-semibold text-foreground shadow-2xs hover:bg-muted/40 dark:bg-muted/15">
						<Terminal className="h-3 w-3 text-primary" /> Logs
					</Button>
					{busy && d.id !== undefined && (
						<Button
							size="sm"
							variant="ghost"
							onClick={() => onCancel(d.id!)}
							className="flex h-7 items-center gap-1 rounded-lg px-2 text-xs font-semibold text-destructive hover:bg-destructive/10 hover:text-destructive">
							<XCircle className="h-3 w-3" /> Cancel
						</Button>
					)}
					{!busy && d.id !== undefined && (
						<Button
							size="sm"
							variant="ghost"
							onClick={() => onDelete?.(d.id!)}
							className="flex h-7 items-center gap-1 rounded-lg px-1.5 text-xs font-semibold text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
							<Trash2 className="h-3 w-3" />
						</Button>
					)}
				</div>
			</TableCell>
		</TableRow>
	);
}
