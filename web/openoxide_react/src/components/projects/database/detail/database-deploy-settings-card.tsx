import {useState} from 'react';
import {Rocket, RefreshCw, Play, Ban, Terminal, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {TerminalModal} from '#/components/projects/common/terminal-modal';
import type {DatabaseResponse} from '#/types/api-helpers';

export type DatabaseActionType = 'deploy' | 'reload' | 'start' | 'stop' | 'cancel';

interface DatabaseDeploySettingsCardProps {
	database: DatabaseResponse | null;
	actionLoading?: string | null;
	onAction: (action: DatabaseActionType) => Promise<void>;
	onUpdated?: () => void;
}

export function DatabaseDeploySettingsCard({
	database,
	actionLoading: propActionLoading,
	onAction,
	onUpdated,
}: DatabaseDeploySettingsCardProps) {
	const [isTerminalOpen, setIsTerminalOpen] = useState(false);
	const [confirmAction, setConfirmAction] = useState<DatabaseActionType | null>(null);

	const dbStatusStr = database?.status || database?.app_status || '';
	const rawDbStatus = dbStatusStr.toUpperCase();
	const isStoppingOrCancelling =
		rawDbStatus === 'STOPPING' ||
		rawDbStatus === 'CANCELLING' ||
		propActionLoading === 'stop' ||
		propActionLoading === 'cancel';
	const activeLoading = propActionLoading || null;
	const isProcessing = activeLoading !== null || isStoppingOrCancelling;

	const rawStatus = dbStatusStr.toLowerCase();
	const isRunning = ['running', 'done', 'healthy', 'deployed', 'success', 'up', 'active', 'ok'].includes(rawStatus);
	const isBuilding =
		['starting', 'building', 'queued', 'preparing'].includes(rawStatus) ||
		activeLoading === 'deploy' ||
		activeLoading === 'reload';

	const executeActionClick = async (action: DatabaseActionType) => {
		setConfirmAction(null);
		try {
			await onAction(action);
			if (onUpdated) onUpdated();
		} catch {}
	};

	const getActionTitle = (action: DatabaseActionType) => {
		if (action === 'deploy') return 'Deploy Database';
		if (action === 'reload') return 'Reload Database';
		if (action === 'start') return 'Start Database';
		return 'Stop Database';
	};

	const getActionDesc = (action: DatabaseActionType) => {
		if (action === 'deploy') return 'Are you sure you want to deploy this database? This will provision/update the stack.';
		if (action === 'reload') return 'Are you sure you want to reload this database service?';
		if (action === 'start') return 'Are you sure you want to start this database container?';
		return 'Are you sure you want to stop this database container?';
	};

	return (
		<div className="bg-card border border-border/80 rounded-2xl p-6 shadow-xs flex flex-col gap-5">
			<div>
				<h3 className="text-base font-bold text-foreground tracking-tight flex items-center gap-2">
					<Rocket className="size-4 text-primary" /> Deploy Settings
				</h3>
				<p className="text-xs text-muted-foreground mt-0.5">
					Control database provisioning, service reload, lifecycle state, and open interactive container shell.
				</p>
			</div>

			<div className="flex items-center gap-3 flex-wrap border-t border-border/40 pt-4">
				<Button
					variant="default"
					onClick={() => executeActionClick('deploy')}
					disabled={isProcessing || isBuilding}
					className="h-9 px-4 text-xs font-semibold bg-primary hover:bg-primary/90 text-primary-foreground shadow-xs gap-2 rounded-lg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{activeLoading === 'deploy' ? <RefreshCw className="size-4 animate-spin" /> : <Rocket className="size-4" />}
					{activeLoading === 'deploy' ? 'Deploying...' : 'Deploy'}
				</Button>

				<Button
					variant="secondary"
					onClick={() => executeActionClick('reload')}
					disabled={isProcessing || isBuilding}
					className="h-9 px-4 text-xs font-semibold bg-muted hover:bg-muted/80 text-foreground border border-border/80 gap-2 rounded-lg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{activeLoading === 'reload' ? <RefreshCw className="size-4 animate-spin" /> : <RefreshCw className="size-4" />}
					{activeLoading === 'reload' ? 'Reloading...' : 'Reload'}
				</Button>

				{activeLoading === 'stop' || rawDbStatus === 'STOPPING' ? (
					<Button disabled variant="outline" size="sm" className="border-border text-destructive font-semibold flex items-center gap-1.5 h-9 rounded-lg opacity-80">
						<RefreshCw className="w-4 h-4 animate-spin text-destructive" /> Stopping...
					</Button>
				) : activeLoading === 'cancel' || rawDbStatus === 'CANCELLING' ? (
					<Button disabled variant="outline" size="sm" className="border-destructive/50 text-destructive font-bold flex items-center gap-1.5 h-9 rounded-lg px-4 opacity-80">
						<RefreshCw className="w-4 h-4 animate-spin text-destructive" /> Cancelling...
					</Button>
				) : isBuilding ? (
					<Button
						onClick={() => executeActionClick('cancel')}
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
						onClick={() => executeActionClick('stop')}
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
						onClick={() => executeActionClick('start')}
						disabled={isProcessing}
						variant="outline"
						size="sm"
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer"
					>
						{activeLoading === 'start' ? <RefreshCw className="size-4 animate-spin" /> : <Play className="w-4 h-4" />}
						{activeLoading === 'start' ? 'Starting...' : 'Start'}
					</Button>
				)}

				<Button
					variant="outline"
					onClick={() => setIsTerminalOpen(true)}
					disabled={isProcessing || !isRunning}
					className="h-9 px-4 text-xs font-semibold border-border hover:bg-accent text-foreground gap-2 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
				>
					<Terminal className="size-4 text-primary" /> Open Terminal
				</Button>
			</div>

			{isTerminalOpen && <TerminalModal app={database} open={isTerminalOpen} onClose={() => setIsTerminalOpen(false)} />}

			{confirmAction && (
				<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
					<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm p-5 flex flex-col gap-4 animate-in fade-in zoom-in-95 duration-150">
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
								variant={confirmAction === 'stop' ? 'destructive' : 'default'}
								onClick={() => executeActionClick(confirmAction)}
								className="h-8 text-xs font-semibold"
							>
								Confirm
							</Button>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
