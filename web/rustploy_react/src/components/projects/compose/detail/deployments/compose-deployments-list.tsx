import {Clock, RefreshCw, Terminal, XCircle, Activity} from 'lucide-react';
import {Button} from '#/components/ui/button';

interface ComposeDeploymentsListProps {
	deployments: any[];
	isLoading: boolean;
	onOpenStream: (id: number) => void;
}

export function ComposeDeploymentsList({deployments, isLoading, onOpenStream}: ComposeDeploymentsListProps) {
	const getStatusBadge = (statusStr?: string, stateStr?: string) => {
		const s = (statusStr || '').toUpperCase();
		const st = (stateStr || '').toUpperCase();

		if (s.includes('DEPLOYED') || s.includes('SUCCESS') || st.includes('SUCCESS')) {
			return (
				<span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10px] font-bold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
					<span className="h-1.5 w-1.5 rounded-full bg-emerald-400" />
					DEPLOYED
				</span>
			);
		}

		if (s.includes('FAIL') || s.includes('ERROR') || st.includes('FAIL') || st.includes('ERROR')) {
			return (
				<span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10px] font-bold bg-rose-500/10 text-rose-400 border border-rose-500/20">
					<XCircle className="w-3 h-3" />
					FAILED
				</span>
			);
		}

		if (s.includes('CANCEL') || st.includes('CANCEL') || s.includes('STOPPED')) {
			return (
				<span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10px] font-bold bg-amber-500/10 text-amber-400 border border-amber-500/20">
					CANCELLED
				</span>
			);
		}

		return (
			<span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10px] font-bold bg-sky-500/10 text-sky-400 border border-sky-500/20 animate-pulse">
				<Activity className="w-3 h-3 animate-spin" />
				{s || st || 'BUILDING'}
			</span>
		);
	};

	const formatTimestamp = (raw: any) => {
		if (!raw) return 'N/A';
		const num = Number(raw);
		if (isNaN(num)) return String(raw);
		const ms = num < 1e11 ? num * 1000 : num;
		return new Date(ms).toLocaleString();
	};

	return (
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4 shadow-sm">
			{isLoading && deployments.length === 0 ? (
				<div className="flex items-center justify-center h-48 text-xs text-muted-foreground gap-2">
					<RefreshCw className="w-4 h-4 animate-spin text-primary" /> Loading deployment logs...
				</div>
			) : deployments.length === 0 ? (
				<div className="flex flex-col items-center justify-center h-48 text-muted-foreground gap-2 text-xs">
					<Clock className="w-8 h-8 opacity-40" />
					<p>No compose stack deployments found.</p>
				</div>
			) : (
				<div className="flex flex-col gap-3">
					{deployments.map((d: any) => (
						<div
							key={d.id}
							className="border border-border/80 rounded-lg p-4 bg-muted/20 hover:bg-muted/40 transition-colors flex items-center justify-between gap-4 flex-wrap"
						>
							<div className="flex items-start gap-3">
								<div className="w-8 h-8 rounded-lg bg-secondary flex items-center justify-center text-foreground font-mono font-bold text-xs shrink-0 border border-border/40">
									#{d.id}
								</div>
								<div className="flex flex-col gap-1">
									<div className="flex items-center gap-2 flex-wrap">
										<span className="text-xs font-bold text-foreground">
											{d.type || 'Manual Deploy'}
										</span>
										{getStatusBadge(d.status, d.state)}
									</div>
									<p className="text-xs text-muted-foreground">
										{d.description || d.message || 'Triggered via Compose Stack Manager'}
									</p>
									<span className="text-[11px] text-muted-foreground/70 font-mono">
										Started: {formatTimestamp(d.created_at)}
									</span>
								</div>
							</div>

							<div className="flex items-center gap-2">
								<Button
									variant="outline"
									size="sm"
									onClick={() => onOpenStream(d.id)}
									className="h-8 text-xs font-semibold border-border hover:bg-muted flex items-center gap-1.5"
								>
									<Terminal className="w-3.5 h-3.5 text-primary" /> Stream Logs
								</Button>
							</div>
						</div>
					))}
				</div>
			)}
		</section>
	);
}
