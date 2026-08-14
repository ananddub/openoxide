import {Clock, RefreshCw, Terminal, XCircle, Zap} from 'lucide-react';
import {Button} from '#/components/ui/button';

interface ComposeDeploymentsListProps {
	deployments: any[];
	isLoading: boolean;
	onOpenStream: (id: number) => void;
	onCancelBuild?: (id: number) => void;
}

const FINAL_STATES = ['DONE', 'DEPLOYED', 'SUCCESS', 'FAILED', 'ERROR', 'CANCELLED', 'STOPPEDBYUSER', 'CRASHED'];

const isBuildActive = (e: any) => {
	if (!e) return false;
	if (e.finished_at && Number(e.finished_at) > 0) return false;
	const s = (e.status || '').toUpperCase();
	const st = (e.state || '').toUpperCase();
	if (FINAL_STATES.includes(s) || FINAL_STATES.includes(st)) return false;
	const activeKeywords = ['RUNNING', 'BUILDING', 'PREPARING', 'QUEUE', 'QUEUED', 'STARTING', 'DEPLOYING', 'PENDING', 'GIT', 'DOCKER'];
	return activeKeywords.some(kw => s.includes(kw) || st.includes(kw));
};

export function ComposeDeploymentsList({deployments, isLoading, onOpenStream, onCancelBuild}: ComposeDeploymentsListProps) {
	const getStatusBadge = (e: any) => {
		const s = (e.status || e.state || '').toUpperCase();
		if (s === 'DONE' || s === 'HEALTHY' || s === 'SUCCESS' || s === 'DEPLOYED') 
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
		<section className="bg-card border border-border rounded-xl overflow-hidden">
			{isLoading && deployments.length === 0 ? (
				<div className="flex justify-center py-12">
					<RefreshCw className="w-6 h-6 animate-spin text-muted-foreground/45" />
				</div>
			) : deployments.length === 0 ? (
				<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground">
					<Zap className="w-10 h-10 opacity-30 mb-3" />
					<p className="text-xs font-semibold">No deployments registered yet</p>
				</div>
			) : (
				<div className="divide-y divide-border/60">
					{deployments.map((e: any) => {
						const isActive = isBuildActive(e);
						return (
							<div key={e.id} className="flex items-center justify-between p-4 hover:bg-muted/10 transition-colors">
								<div className="min-w-0 flex flex-col gap-0.5">
									<span className="text-xs font-semibold text-foreground truncate">
										{e.title || `Deployment #${e.id}`}
									</span>
									{(e.description || e.message) && (
										<span className="text-[11px] text-muted-foreground truncate">
											{e.description || e.message}
										</span>
									)}
								</div>

								<div className="flex items-center gap-3 shrink-0">
									<span className={`text-[10px] font-bold px-2 py-0.5 rounded border uppercase tracking-wider ${getStatusBadge(e)}`}>
										{e.status || e.state || 'PENDING'}
									</span>
									<span className="text-[10px] text-muted-foreground flex items-center gap-1 font-medium">
										<Clock className="w-3 h-3" />
										{formatTimestamp(e.created_at)}
									</span>

									<Button
										size="sm"
										variant="outline"
										onClick={() => onOpenStream(e.id)}
										className="h-7 text-xs border-border text-foreground hover:bg-muted px-2 rounded-lg font-semibold flex items-center gap-1"
									>
										<Terminal className="w-3 h-3" /> Stream Logs
									</Button>

									{isActive && onCancelBuild && (
										<Button
											size="sm"
											variant="ghost"
											onClick={() => onCancelBuild(e.id)}
											className="h-7 text-xs text-destructive hover:text-destructive hover:bg-destructive/10 px-2 rounded-lg font-semibold flex items-center gap-1"
										>
											<XCircle className="w-3 h-3" /> Cancel
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
