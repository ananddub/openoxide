import {useState, useMemo} from 'react';
import {Rocket, RefreshCw, Hammer, Play, Ban, Terminal, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {TerminalModal} from '#/components/projects/common/terminal-modal';
import {useDeploymentList} from 'virtual:openoxide-live';

interface DeploySettingsCardProps {
	app: any;
	handleAction: (action: any) => Promise<void>;
	onUpdated?: () => void;
}

type ActionType = 'deploy' | 'reload' | 'rebuild' | 'start' | 'cancel' | 'stop';

export function DeploySettingsCard({app, handleAction, onUpdated}: DeploySettingsCardProps) {
	const [autoDeploy, setAutoDeploy] = useState(app.auto_deploy !== false);
	const [cleanCache, setCleanCache] = useState(false);
	const [showTerminal, setShowTerminal] = useState(false);
	
	const [confirmAction, setConfirmAction] = useState<ActionType | null>(null);
	const [activeLoading, setActiveLoading] = useState<string | null>(null);

	// Fetch deployments query via live WebSocket hook
	const {data: rawDeployments} = useDeploymentList({
		status: null,
		state: null,
		application_id: BigInt(app?.id || 0),
		compose_id: null,
		database_id: null,
		server_id: null,
		limit: 20n,
		offset: null,
	});

	const events = useMemo(() => (Array.isArray(rawDeployments) ? rawDeployments : []), [rawDeployments]);

	const hasActiveDeployment = (events || []).some((e: any) => {
		if (!e || (e.finished_at && Number(e.finished_at) > 0)) return false;
		const s = (e.status || e.state || '').toUpperCase();
		return ['QUEUED', 'BUILDING', 'STARTING', 'DEPLOYING', 'PREPARING', 'PENDING'].includes(s);
	});

	const rawStatus = (app?.app_status || app?.applicationStatus || app?.application_status || app?.status || '').toLowerCase();
	const isRunning = ['running', 'done', 'healthy', 'deployed', 'success', 'up', 'active', 'ok'].includes(rawStatus);
	const isBuilding = hasActiveDeployment || ['starting', 'building', 'queued', 'preparing'].includes(rawStatus) || activeLoading === 'deploy' || activeLoading === 'rebuild' || activeLoading === 'reload';

	const executeAction = async (action: ActionType) => {
		setConfirmAction(null);
		setActiveLoading(action);
		try {
			await handleAction(action);
			onUpdated?.();
		} finally {
			setActiveLoading(null);
		}
	};

	const getActionTitle = (action: ActionType) => {
		if (action === 'deploy') return 'Deploy Application';
		if (action === 'reload') return 'Reload Application';
		if (action === 'rebuild') return 'Rebuild Application';
		if (action === 'start') return 'Start Application';
		if (action === 'stop') return 'Stop Application';
		return 'Cancel Application Build';
	};

	const getActionDesc = (action: ActionType) => {
		if (action === 'deploy') return 'Are you sure you want to deploy this application? This will pull latest code and trigger a build.';
		if (action === 'reload') return 'Are you sure you want to reload this application without rebuilding?';
		if (action === 'rebuild') return 'Are you sure you want to rebuild this application?';
		if (action === 'start') return 'Are you sure you want to start this application container?';
		if (action === 'stop') return 'Are you sure you want to stop this running application container?';
		return 'Are you sure you want to cancel the active deployment build?';
	};

	const rawAppStatus = (app?.app_status || '').toUpperCase();
	const isStoppingOrCancelling = rawAppStatus === 'STOPPING' || rawAppStatus === 'CANCELLING' || activeLoading === 'stop' || activeLoading === 'cancel';
	const isProcessing = activeLoading !== null || isStoppingOrCancelling;

	return (
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4 shadow-sm">
			<h3 className="text-sm font-bold text-foreground">Deploy Settings</h3>
			<div className="flex flex-wrap items-center justify-between gap-4">
				<div className="flex flex-wrap items-center gap-2">
					{/* Deploy */}
					<Button
						onClick={() => executeAction('deploy')}
						disabled={isProcessing || isBuilding}
						size="sm"
						className="bg-primary hover:bg-primary/90 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{activeLoading === 'deploy' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Rocket className="w-4 h-4" />}
						{activeLoading === 'deploy' ? 'Deploying...' : 'Deploy'}
					</Button>

					{/* Reload */}
					<Button
						onClick={() => executeAction('reload')}
						disabled={isProcessing || isBuilding}
						variant="outline"
						size="sm"
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{activeLoading === 'reload' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <RefreshCw className="w-4 h-4" />}
						{activeLoading === 'reload' ? 'Reloading...' : 'Reload'}
					</Button>

					{/* Rebuild */}
					<Button
						onClick={() => executeAction('rebuild')}
						disabled={isProcessing || isBuilding}
						variant="outline"
						size="sm"
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{activeLoading === 'rebuild' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Hammer className="w-4 h-4" />}
						{activeLoading === 'rebuild' ? 'Rebuilding...' : 'Rebuild'}
					</Button>

					{/* 4-State Action Button: Stopping, Cancelling, Cancel (Building), Stop (Running), Start (Idle/Error/Stopped) */}
					{activeLoading === 'stop' || (app?.app_status || '').toUpperCase() === 'STOPPING' ? (
						<Button
							disabled
							variant="outline"
							size="sm"
							className="border-border text-destructive font-semibold flex items-center gap-1.5 h-9 rounded-lg opacity-80 cursor-not-allowed"
						>
							<RefreshCw className="w-4 h-4 animate-spin text-destructive" />
							Stopping...
						</Button>
					) : activeLoading === 'cancel' || (app?.app_status || '').toUpperCase() === 'CANCELLING' ? (
						<Button
							disabled
							variant="outline"
							size="sm"
							className="border-destructive/50 text-destructive font-bold flex items-center gap-1.5 h-9 rounded-lg px-4 opacity-80 cursor-not-allowed"
						>
							<RefreshCw className="w-4 h-4 animate-spin text-destructive" />
							Cancelling...
						</Button>
					) : isBuilding ? (
						<Button
							onClick={() => executeAction('cancel')}
							disabled={activeLoading === 'cancel'}
							variant="outline"
							size="sm"
							className="border-destructive/50 text-destructive hover:bg-destructive/10 font-bold flex items-center gap-1.5 h-9 rounded-lg px-4 shadow-xs cursor-pointer"
						>
							<RefreshCw className="w-4 h-4 animate-spin text-destructive" />
							{activeLoading === 'cancel' ? 'Cancelling...' : 'Cancel'}
						</Button>
					) : isRunning ? (
						<Button
							onClick={() => executeAction('stop')}
							disabled={isProcessing}
							variant="destructive"
							size="sm"
							className="h-9 px-4 text-xs font-semibold gap-1.5 rounded-lg flex items-center cursor-pointer"
						>
							{activeLoading === 'stop' ? <RefreshCw className="size-4 animate-spin" /> : <Ban className="size-4" />}
							{activeLoading === 'stop' ? 'Stopping...' : 'Stop'}
						</Button>
					) : (
						<Button
							onClick={() => executeAction('start')}
							disabled={isProcessing}
							variant="outline"
							size="sm"
							className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer"
						>
							{activeLoading === 'start' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Play className="w-4 h-4" />}
							{activeLoading === 'start' ? 'Starting...' : 'Start'}
						</Button>
					)}
					{/* Terminal Access Button */}
					<Button
						onClick={() => setShowTerminal(true)}
						disabled={isProcessing || !isRunning}
						variant="outline"
						size="sm"
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
					>
						<Terminal className="w-4 h-4 text-primary" /> Open Terminal
					</Button>
				</div>

				{/* Switches */}
				<div className="flex items-center gap-4 border-l border-border/40 pl-4">
					<label className="flex items-center gap-2 cursor-pointer select-none">
						<button
							type="button"
							onClick={() => setAutoDeploy(!autoDeploy)}
							className={`relative inline-flex h-4.5 w-8 shrink-0 cursor-pointer items-center rounded-full transition-colors ${autoDeploy ? 'bg-primary' : 'bg-muted'}`}
						>
							<span className={`pointer-events-none block w-3 h-3 rounded-full bg-background shadow-lg transition-transform ${autoDeploy ? 'translate-x-4.5' : 'translate-x-0.5'}`} />
						</button>
						<span className="text-[11px] font-bold text-muted-foreground">Autodeploy</span>
					</label>

					<label className="flex items-center gap-2 cursor-pointer select-none">
						<button
							type="button"
							onClick={() => setCleanCache(!cleanCache)}
							className={`relative inline-flex h-4.5 w-8 shrink-0 cursor-pointer items-center rounded-full transition-colors ${cleanCache ? 'bg-primary' : 'bg-muted'}`}
						>
							<span className={`pointer-events-none block w-3 h-3 rounded-full bg-background shadow-lg transition-transform ${cleanCache ? 'translate-x-4.5' : 'translate-x-0.5'}`} />
						</button>
						<span className="text-[11px] font-bold text-muted-foreground">Clean Cache</span>
					</label>
				</div>
			</div>

			{/* Confirmation Modal */}
			{confirmAction && (
				<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
					<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm p-5 flex flex-col gap-4">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<h3 className="text-sm font-bold text-foreground">{getActionTitle(confirmAction)}</h3>
							<Button variant="ghost" size="sm" onClick={() => setConfirmAction(null)} className="h-7 w-7 p-0 text-muted-foreground">
								<X className="w-4 h-4" />
							</Button>
						</div>
						<p className="text-xs text-muted-foreground leading-relaxed">{getActionDesc(confirmAction)}</p>
						<div className="flex justify-end gap-2 border-t border-border/60 pt-3">
							<Button variant="outline" size="sm" onClick={() => setConfirmAction(null)} className="h-8 text-xs font-semibold">
								Cancel
							</Button>
							<Button
								size="sm"
								variant={confirmAction === 'cancel' || confirmAction === 'stop' ? 'destructive' : 'default'}
								onClick={() => executeAction(confirmAction)}
								className="h-8 text-xs font-semibold"
							>
								Confirm
							</Button>
						</div>
					</div>
				</div>
			)}

			{/* Container Terminal Modal */}
			<TerminalModal app={app} open={showTerminal} onClose={() => setShowTerminal(false)} />
		</section>
	);
}
