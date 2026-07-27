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
			return <Badge variant="default" className="text-[10px] uppercase font-bold tracking-wider">SUCCESS</Badge>;
		}
		if (status === 'ERROR' || status === 'FAILED' || status === 'CRASHED') {
			return <Badge variant="destructive" className="text-[10px] uppercase font-bold tracking-wider">FAILED</Badge>;
		}
		if (isRunningOrQueued) {
			return <Badge variant="outline" className="text-[10px] uppercase font-bold tracking-wider text-amber-500 border-amber-500/30 bg-amber-500/10 animate-pulse">{status}</Badge>;
		}
		return <Badge variant="secondary" className="text-[10px] uppercase font-bold tracking-wider">{status}</Badge>;
	};

	const formatTimestamp = (raw: any) => {
		if (!raw) return 'N/A';
		const num = Number(raw);
		if (isNaN(num)) return String(raw);
		const ms = num < 1e11 ? num * 1000 : num;
		return new Date(ms).toLocaleDateString();
	};

	return (
		<div className="p-4 hover:bg-muted/10 transition-colors flex items-center justify-between gap-4">
			{/* Left Details */}
			<div className="flex items-start gap-3 min-w-0">
				<div className="w-8 h-8 rounded-lg bg-muted flex items-center justify-center text-muted-foreground shrink-0 border border-border/50 mt-0.5">
					{hasApp ? (
						<Rocket className="w-4 h-4" />
					) : hasCompose ? (
						<Boxes className="w-4 h-4" />
					) : (
						<Database className="w-4 h-4" />
					)}
				</div>

				<div className="min-w-0 flex flex-col gap-0.5">
					<div className="flex items-center gap-2 flex-wrap">
						<span className="text-xs font-bold text-foreground truncate">
							{d.title || `Deployment #${d.id}`}
						</span>
						<Badge variant="outline" className="text-[10px] font-mono font-medium">
							#{d.id} • {type}
						</Badge>
					</div>

					{d.description && (
						<span className="text-[11px] text-muted-foreground truncate">
							{d.description}
						</span>
					)}

					{d.error_message && (
						<button
							onClick={e => {
								e.stopPropagation();
								onViewError(d);
							}}
							className="text-[11px] text-destructive hover:underline font-medium flex items-center gap-1 text-left mt-0.5">
							<AlertCircle className="w-3 h-3 shrink-0" />
							<span className="truncate max-w-md">Error: {d.error_message}</span>
						</button>
					)}
				</div>
			</div>

			{/* Right Actions & Status */}
			<div className="flex items-center gap-3 shrink-0">
				{getStatusBadge()}

				<span className="text-[10px] text-muted-foreground flex items-center gap-1 font-medium">
					<Clock className="w-3 h-3" />
					{formatTimestamp(d.created_at)}
				</span>

				<Button
					size="sm"
					variant="outline"
					onClick={() => onViewLogs(d)}
					className="h-7 text-xs border-border text-foreground hover:bg-muted px-2 rounded-lg font-semibold flex items-center gap-1">
					<Terminal className="w-3 h-3" /> Stream Logs
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
		</div>
	);
}
