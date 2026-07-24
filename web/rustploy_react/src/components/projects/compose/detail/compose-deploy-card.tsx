import {useState, useMemo} from 'react';
import {Rocket, RefreshCw, Hammer, Play, Square, StopCircle, Terminal} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {$api} from '#/api/query';
import {TerminalModal} from '#/components/projects/app/detail/terminal-modal';
import {ComposeConfirmDialog, type ActionType} from './cards/compose-confirm-dialog';

interface ComposeDeployCardProps {
	compose: any;
	handleAction: (action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'cancel') => Promise<void>;
	onUpdated?: () => void;
}

const FINAL_STATES = ['DONE', 'DEPLOYED', 'SUCCESS', 'FAILED', 'ERROR', 'CANCELLED', 'STOPPEDBYUSER', 'CRASHED'];

export function ComposeDeployCard({compose, handleAction, onUpdated}: ComposeDeployCardProps) {
	const [autoDeploy, setAutoDeploy] = useState(compose?.auto_deploy !== false);
	const [cleanCache, setCleanCache] = useState(false);
	const [showTerminal, setShowTerminal] = useState(false);
	
	const [confirmAction, setConfirmAction] = useState<ActionType | null>(null);
	const [activeLoading, setActiveLoading] = useState<ActionType | null>(null);

	// Fetch compose deployments query
	const {data: rawEvents = [], refetch} = $api.useQuery(
		'get',
		'/deployments',
		{
			params: {
				query: {
					compose_id: compose?.id || 0,
					limit: 20,
				} as any,
			},
		},
		{
			enabled: !!compose?.id,
			refetchInterval: (query) => {
				const data = query.state.data as any[] | undefined;
				const hasActive = data?.some(e => {
					if (!e || (e.finished_at && Number(e.finished_at) > 0)) return false;
					const s = (e.status || '').toUpperCase();
					const st = (e.state || '').toUpperCase();
					return !FINAL_STATES.includes(s) && !FINAL_STATES.includes(st);
				});
				return hasActive ? 3000 : false;
			},
		}
	);

	const events = useMemo(() => (Array.isArray(rawEvents) ? rawEvents : []), [rawEvents]);

	const isBuilding = events.some(e => {
		if (!e || (e.finished_at && Number(e.finished_at) > 0)) return false;
		const s = (e.status || '').toUpperCase();
		const st = (e.state || '').toUpperCase();
		if (FINAL_STATES.includes(s) || FINAL_STATES.includes(st)) return false;
		const activeKeywords = ['BUILDING', 'PREPARING', 'QUEUE', 'QUEUED', 'STARTING', 'DEPLOYING', 'PENDING'];
		return activeKeywords.some(kw => s.includes(kw) || st.includes(kw));
	});

	const executeAction = async (action: ActionType) => {
		setConfirmAction(null);
		const targetAction = action === 'stop' ? 'cancel' : action;
		setActiveLoading(action);
		try {
			await handleAction(targetAction as any);
			await refetch();
			onUpdated?.();
		} finally {
			setActiveLoading(null);
		}
	};

	const isProcessing = activeLoading !== null;

	return (
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4 shadow-sm">
			<h3 className="text-sm font-bold text-foreground">Deploy Settings</h3>
			<div className="flex flex-wrap items-center justify-between gap-4">
				<div className="flex flex-wrap items-center gap-2">
					<Button
						disabled={isProcessing || isBuilding}
						onClick={() => setConfirmAction('deploy')}
						size="sm"
						className="h-8 text-xs font-semibold flex items-center gap-1.5"
					>
						<Rocket className="w-3.5 h-3.5" /> Deploy Stack
					</Button>

					<Button
						disabled={isProcessing || isBuilding}
						onClick={() => setConfirmAction('reload')}
						variant="outline"
						size="sm"
						className="h-8 text-xs font-semibold flex items-center gap-1.5"
					>
						<RefreshCw className="w-3.5 h-3.5" /> Reload
					</Button>

					<Button
						disabled={isProcessing || isBuilding}
						onClick={() => setConfirmAction('rebuild')}
						variant="outline"
						size="sm"
						className="h-8 text-xs font-semibold flex items-center gap-1.5"
					>
						<Hammer className="w-3.5 h-3.5" /> Rebuild
					</Button>

					<Button
						disabled={isProcessing || isBuilding}
						onClick={() => setConfirmAction('start')}
						variant="outline"
						size="sm"
						className="h-8 text-xs font-semibold flex items-center gap-1.5"
					>
						<Play className="w-3.5 h-3.5 text-emerald-400" /> Start
					</Button>

					<Button
						disabled={isProcessing || isBuilding}
						onClick={() => setConfirmAction('stop')}
						variant="outline"
						size="sm"
						className="h-8 text-xs font-semibold flex items-center gap-1.5 border-rose-500/30 text-rose-400 hover:bg-rose-500/10"
					>
						<Square className="w-3.5 h-3.5" /> Stop
					</Button>

					{isBuilding && (
						<Button
							disabled={isProcessing}
							onClick={() => setConfirmAction('cancel')}
							variant="destructive"
							size="sm"
							className="h-8 text-xs font-semibold flex items-center gap-1.5"
						>
							<StopCircle className="w-3.5 h-3.5" /> Cancel Build
						</Button>
					)}
				</div>

				<Button
					variant="secondary"
					size="sm"
					onClick={() => setShowTerminal(true)}
					className="h-8 text-xs font-semibold flex items-center gap-1.5"
				>
					<Terminal className="w-3.5 h-3.5" /> Open Container Terminal
				</Button>
			</div>

			<div className="flex flex-wrap items-center gap-6 pt-3 border-t border-border/40 text-xs text-muted-foreground">
				<label className="flex items-center gap-2 cursor-pointer font-semibold">
					<input
						type="checkbox"
						checked={autoDeploy}
						onChange={e => setAutoDeploy(e.target.checked)}
						className="accent-primary w-4 h-4 rounded"
					/>
					Auto Deploy on Push
				</label>
				<label className="flex items-center gap-2 cursor-pointer font-semibold">
					<input
						type="checkbox"
						checked={cleanCache}
						onChange={e => setCleanCache(e.target.checked)}
						className="accent-primary w-4 h-4 rounded"
					/>
					Clean Cache Before Build
				</label>
			</div>

			{/* Confirm Action Dialog Component (< 200 lines) */}
			<ComposeConfirmDialog
				confirmAction={confirmAction}
				onClose={() => setConfirmAction(null)}
				onConfirm={executeAction}
			/>

			{/* Terminal Exec Modal Component */}
			{showTerminal && (
				<TerminalModal
					app={compose}
					onClose={() => setShowTerminal(false)}
				/>
			)}
		</section>
	);
}
