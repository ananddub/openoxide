import {
	Rocket,
	Boxes,
	Terminal,
	XCircle,
	AlertCircle,
	Database,
	Calendar,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';
import type {Deployment} from '#/hooks/deployments/use-deployments';

interface DeploymentItemProps {
	deployment: Deployment;
	onViewLogs: (d: Deployment) => void;
	onViewError: (d: Deployment) => void;
	onCancel: (id: number) => void;
}

// Colors accents for statuses
const statusColorMap: Record<
	string,
	{border: string; text: string; bg: string; dot: string}
> = {
	DONE: {
		border: 'border-l-emerald-500/80 hover:border-emerald-500/50',
		text: 'text-emerald-500',
		bg: 'bg-emerald-500/5',
		dot: 'bg-emerald-500',
	},
	SUCCESS: {
		border: 'border-l-emerald-500/80 hover:border-emerald-500/50',
		text: 'text-emerald-500',
		bg: 'bg-emerald-500/5',
		dot: 'bg-emerald-500',
	},
	RUNNING: {
		border: 'border-l-blue-500/80 hover:border-blue-500/50',
		text: 'text-blue-500',
		bg: 'bg-blue-500/5',
		dot: 'bg-blue-500 animate-ping',
	},
	QUEUED: {
		border: 'border-l-zinc-500/80 hover:border-zinc-500/50',
		text: 'text-zinc-400',
		bg: 'bg-zinc-500/5',
		dot: 'bg-zinc-500',
	},
	ERROR: {
		border: 'border-l-rose-500/80 hover:border-rose-500/50',
		text: 'text-rose-500',
		bg: 'bg-rose-500/5',
		dot: 'bg-rose-500',
	},
};

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
	const isRunningOrQueued = d.status.toUpperCase() === 'RUNNING' || d.status.toUpperCase() === 'QUEUED';
	const status = d.status.toUpperCase();

	const colors = statusColorMap[status] || {
		border: 'border-l-zinc-500/80 hover:border-zinc-500/50',
		text: 'text-zinc-400',
		bg: 'bg-zinc-500/5',
		dot: 'bg-zinc-500',
	};

	return (
		<div
			className={cn(
				'group border border-border border-l-4 bg-card/30 hover:bg-card/65 rounded-xl p-4.5 transition-all duration-200 shadow-sm flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4',
				colors.border,
			)}>
			{/* Left Section: Icon and Details */}
			<div className="flex items-start gap-3.5 min-w-0">
				<div className="p-2 rounded-lg bg-muted text-muted-foreground shrink-0 mt-0.5">
					{hasApp ? (
						<Rocket className="size-4.5" />
					) : hasCompose ? (
						<Boxes className="size-4.5" />
					) : (
						<Database className="size-4.5" />
					)}
				</div>
				<div className="min-w-0 flex-1">
					<div className="flex items-center gap-2 flex-wrap">
						<span className="font-semibold text-sm text-foreground group-hover:text-primary transition-colors">
							{d.title}
						</span>
						<span className="text-[10px] font-mono bg-muted/65 text-muted-foreground px-1.5 py-0.5 rounded border border-border/40">
							#{d.id}
						</span>
						<span className="text-[10px] text-muted-foreground/80 font-medium">
							• {type}
						</span>
					</div>
					<p className="text-xs text-muted-foreground mt-1 line-clamp-2 leading-relaxed">
						{d.description}
					</p>
					{d.error_message && (
						<button
							onClick={e => {
								e.stopPropagation();
								onViewError(d);
							}}
							className="text-[11px] text-rose-500/90 hover:text-rose-400 font-medium mt-1.5 flex items-center gap-1 bg-rose-500/5 hover:bg-rose-500/10 border border-rose-500/10 hover:border-rose-500/25 px-2 py-0.5 rounded cursor-pointer max-w-sm transition-colors text-left">
							<AlertCircle className="size-3 shrink-0" />
							<span className="truncate block">
								Error: {d.error_message}
							</span>
						</button>
					)}
				</div>
			</div>

			{/* Right Section: Status, Time, and Actions */}
			<div className="flex flex-row sm:flex-col md:flex-row items-center justify-between sm:justify-end gap-4 w-full sm:w-auto shrink-0 border-t border-border/10 sm:border-t-0 pt-3 sm:pt-0">
				<div className="flex flex-col items-start sm:items-end gap-1.5">
					{/* Status Badge */}
					<div className="flex items-center gap-1.5">
						<span className={cn('size-1.5 rounded-full shrink-0', colors.dot)} />
						{status === 'RUNNING' && (
							<span className="size-1.5 bg-blue-500 rounded-full shrink-0 absolute opacity-75 animate-ping" />
						)}
						<span className={cn('text-[10px] font-bold tracking-wider uppercase', colors.text)}>
							{d.status}
						</span>
					</div>
					{/* Date & Time */}
					<span className="text-[10px] text-muted-foreground/85 flex items-center gap-1">
						<Calendar className="size-3" />
						{new Date(d.created_at * 1000).toLocaleString()}
					</span>
				</div>

				{/* Action buttons */}
				<div className="flex items-center gap-1.5">
					<Button
						variant="ghost"
						size="sm"
						onClick={() => onViewLogs(d)}
						className="h-8 px-2.5 rounded-lg hover:bg-muted text-xs font-semibold flex items-center gap-1.5 text-muted-foreground hover:text-foreground border border-transparent hover:border-border/30">
						<Terminal className="size-3.5" />
						Logs
					</Button>

					{isRunningOrQueued && d.id !== undefined && (
						<Button
							variant="ghost"
							size="icon"
							onClick={() => onCancel(d.id!)}
							className="size-8 rounded-lg hover:bg-rose-500/5 hover:text-rose-500 text-muted-foreground/60 border border-transparent hover:border-rose-500/10">
							<XCircle className="size-4" />
						</Button>
					)}
				</div>
			</div>
		</div>
	);
}
