import {
	Clock,
	RefreshCw,
	Terminal,
	XCircle,
	Zap,
	Trash2,
} from 'lucide-react';
import {Button} from '#/components/ui/button';

interface ComposeDeploymentsListProps {
	deployments: any[];
	isLoading: boolean;
	onOpenStream: (id: number) => void;
	onCancelBuild?: (id: number) => void;
	onDeleteDeployment?: (id: number) => void;
}

const FINAL_STATES = [
	'DONE',
	'DEPLOYED',
	'SUCCESS',
	'FAILED',
	'ERROR',
	'CANCELLED',
	'STOPPEDBYUSER',
	'CRASHED',
];

const isBuildActive = (e: any) => {
	if (!e) return false;
	if (e.finished_at && Number(e.finished_at) > 0) return false;
	const s = (e.status || '').toUpperCase();
	const st = (e.state || '').toUpperCase();
	if (FINAL_STATES.includes(s) || FINAL_STATES.includes(st)) return false;
	const activeKeywords = [
		'RUNNING',
		'BUILDING',
		'PREPARING',
		'QUEUE',
		'QUEUED',
		'STARTING',
		'DEPLOYING',
		'PENDING',
		'GIT',
		'DOCKER',
	];
	return activeKeywords.some(kw => s.includes(kw) || st.includes(kw));
};

export function ComposeDeploymentsList({
	deployments,
	isLoading,
	onOpenStream,
	onCancelBuild,
	onDeleteDeployment,
}: ComposeDeploymentsListProps) {
	const getStatusBadge = (e: any) => {
		const s = (e.status || e.state || '').toUpperCase();
		if (
			s === 'DONE' ||
			s === 'HEALTHY' ||
			s === 'SUCCESS' ||
			s === 'DEPLOYED'
		)
			return 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20';
		if (s === 'ERROR' || s === 'FAILED' || s === 'CRASHED')
			return 'text-rose-500 bg-rose-500/10 border-rose-500/20';
		if (s === 'CANCELLING')
			return 'text-amber-500 bg-amber-500/10 border-amber-500/30 animate-pulse';
		if (s === 'CANCELLED')
			return 'text-zinc-400 bg-zinc-500/10 border-zinc-500/20';
		if (isBuildActive(e))
			return 'text-amber-500 bg-amber-500/10 border-amber-500/30 animate-pulse';
		return 'text-muted-foreground bg-muted border-border';
	};

	const formatTimestamp = (raw: any) => {
		if (!raw) return 'N/A';
		const num = Number(raw);
		if (isNaN(num)) return String(raw);
		const ms = num < 1e11 ? num * 1000 : num;
		return new Date(ms).toLocaleDateString();
	};

	return (
		<section className="overflow-hidden rounded-xl border border-border bg-card">
			{isLoading && deployments.length === 0 ? (
				<div className="flex justify-center py-12">
					<RefreshCw className="h-6 w-6 animate-spin text-muted-foreground/45" />
				</div>
			) : deployments.length === 0 ? (
				<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground">
					<Zap className="mb-3 h-10 w-10 opacity-30" />
					<p className="text-xs font-semibold">
						No deployments registered yet
					</p>
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
										{e.title || `Deployment #${e.id}`}
									</span>
									{(e.description || e.message) && (
										<span className="truncate text-[11px] text-muted-foreground">
											{e.description || e.message}
										</span>
									)}
								</div>

								<div className="flex shrink-0 items-center gap-3">
									<span
										className={`rounded border px-2 py-0.5 text-[10px] font-bold tracking-wider uppercase ${getStatusBadge(e)}`}>
										{e.status || e.state || 'PENDING'}
									</span>
									<span className="flex items-center gap-1 text-[10px] font-medium text-muted-foreground">
										<Clock className="h-3 w-3" />
										{formatTimestamp(e.created_at)}
									</span>

									<Button
										size="sm"
										variant="outline"
										onClick={() => onOpenStream(e.id)}
										className="flex h-7 items-center gap-1 rounded-lg border-border px-2 text-xs font-semibold text-foreground hover:bg-muted">
										<Terminal className="h-3 w-3" /> Stream Logs
									</Button>

									{isActive && onCancelBuild && (
										<Button
											size="sm"
											variant="ghost"
											onClick={() => onCancelBuild(e.id)}
											className="flex h-7 items-center gap-1 rounded-lg px-2 text-xs font-semibold text-destructive hover:bg-destructive/10 hover:text-destructive">
											<XCircle className="h-3 w-3" /> Cancel
										</Button>
									)}

									{!isActive && onDeleteDeployment && (
										<Button
											size="sm"
											variant="ghost"
											onClick={() => onDeleteDeployment(e.id)}
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
