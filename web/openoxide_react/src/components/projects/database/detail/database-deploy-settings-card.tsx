import {useState} from 'react';
import {Rocket, RefreshCw, Play, Ban, Terminal, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {TerminalModal} from '#/components/projects/common/terminal-modal';
import type {DatabaseResponse} from '#/types/api-helpers';

export type DatabaseActionType =
	| 'deploy'
	| 'reload'
	| 'start'
	| 'stop'
	| 'cancel';

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
	const [confirmAction, setConfirmAction] =
		useState<DatabaseActionType | null>(null);

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
	const isRunning = [
		'running',
		'done',
		'healthy',
		'deployed',
		'success',
		'up',
		'active',
		'ok',
	].includes(rawStatus);
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
		if (action === 'deploy')
			return 'Are you sure you want to deploy this database? This will provision/update the stack.';
		if (action === 'reload')
			return 'Are you sure you want to reload this database service?';
		if (action === 'start')
			return 'Are you sure you want to start this database container?';
		return 'Are you sure you want to stop this database container?';
	};

	return (
		<div className="flex flex-col gap-5 rounded-2xl border border-border/80 bg-card p-6 shadow-xs">
			<div>
				<h3 className="flex items-center gap-2 text-base font-bold tracking-tight text-foreground">
					<Rocket className="size-4 text-primary" /> Deploy Settings
				</h3>
				<p className="mt-0.5 text-xs text-muted-foreground">
					Control database provisioning, service reload, lifecycle state,
					and open interactive container shell.
				</p>
			</div>

			<div className="flex flex-wrap items-center gap-3 border-t border-border/40 pt-4">
				<Button
					variant="default"
					onClick={() => executeActionClick('deploy')}
					disabled={isProcessing || isBuilding}
					className="h-9 cursor-pointer gap-2 rounded-lg bg-primary px-4 text-xs font-semibold text-primary-foreground shadow-xs hover:bg-primary/90 disabled:cursor-not-allowed disabled:opacity-50">
					{activeLoading === 'deploy' ? (
						<RefreshCw className="size-4 animate-spin" />
					) : (
						<Rocket className="size-4" />
					)}
					{activeLoading === 'deploy' ? 'Deploying...' : 'Deploy'}
				</Button>

				<Button
					variant="secondary"
					onClick={() => executeActionClick('reload')}
					disabled={isProcessing || isBuilding}
					className="h-9 cursor-pointer gap-2 rounded-lg border border-border/80 bg-muted px-4 text-xs font-semibold text-foreground hover:bg-muted/80 disabled:cursor-not-allowed disabled:opacity-50">
					{activeLoading === 'reload' ? (
						<RefreshCw className="size-4 animate-spin" />
					) : (
						<RefreshCw className="size-4" />
					)}
					{activeLoading === 'reload' ? 'Reloading...' : 'Reload'}
				</Button>

				{activeLoading === 'stop' || rawDbStatus === 'STOPPING' ? (
					<Button
						disabled
						variant="outline"
						size="sm"
						className="flex h-9 items-center gap-1.5 rounded-lg border-border font-semibold text-destructive opacity-80">
						<RefreshCw className="h-4 w-4 animate-spin text-destructive" />{' '}
						Stopping...
					</Button>
				) : activeLoading === 'cancel' || rawDbStatus === 'CANCELLING' ? (
					<Button
						disabled
						variant="outline"
						size="sm"
						className="flex h-9 items-center gap-1.5 rounded-lg border-destructive/50 px-4 font-bold text-destructive opacity-80">
						<RefreshCw className="h-4 w-4 animate-spin text-destructive" />{' '}
						Cancelling...
					</Button>
				) : isBuilding ? (
					<Button
						onClick={() => executeActionClick('cancel')}
						disabled={activeLoading === 'cancel'}
						variant="outline"
						size="sm"
						className="flex h-9 cursor-pointer items-center gap-1.5 rounded-lg border-destructive/50 px-4 font-bold text-destructive shadow-xs hover:bg-destructive/10">
						<RefreshCw className="h-4 w-4 animate-spin text-destructive" />
						{activeLoading === 'cancel' ? 'Cancelling...' : 'Cancel'}
					</Button>
				) : isRunning ? (
					<Button
						onClick={() => executeActionClick('stop')}
						disabled={isProcessing}
						variant="destructive"
						size="sm"
						className="flex h-9 cursor-pointer items-center gap-1.5 rounded-lg px-4 text-xs font-semibold">
						{activeLoading === 'stop' ? (
							<RefreshCw className="size-4 animate-spin" />
						) : (
							<Ban className="size-4" />
						)}
						{activeLoading === 'stop' ? 'Stopping...' : 'Stop'}
					</Button>
				) : (
					<Button
						onClick={() => executeActionClick('start')}
						disabled={isProcessing}
						variant="outline"
						size="sm"
						className="flex h-9 cursor-pointer items-center gap-1.5 rounded-lg border-border font-semibold text-foreground hover:bg-muted">
						{activeLoading === 'start' ? (
							<RefreshCw className="size-4 animate-spin" />
						) : (
							<Play className="h-4 w-4" />
						)}
						{activeLoading === 'start' ? 'Starting...' : 'Start'}
					</Button>
				)}

				<Button
					variant="outline"
					onClick={() => setIsTerminalOpen(true)}
					disabled={isProcessing || !isRunning}
					className="h-9 gap-2 rounded-lg border-border px-4 text-xs font-semibold text-foreground hover:bg-accent disabled:cursor-not-allowed disabled:opacity-50">
					<Terminal className="size-4 text-primary" /> Open Terminal
				</Button>
			</div>

			{isTerminalOpen && (
				<TerminalModal
					app={database}
					open={isTerminalOpen}
					onClose={() => setIsTerminalOpen(false)}
				/>
			)}

			{confirmAction && (
				<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm">
					<div className="flex w-full max-w-sm animate-in flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-2xl duration-150 zoom-in-95 fade-in">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<h3 className="text-sm font-bold text-foreground">
								{getActionTitle(confirmAction)}
							</h3>
							<Button
								variant="ghost"
								size="sm"
								onClick={() => setConfirmAction(null)}
								className="h-7 w-7 p-0 text-muted-foreground">
								<X className="h-4 w-4" />
							</Button>
						</div>
						<p className="text-xs leading-relaxed text-muted-foreground">
							{getActionDesc(confirmAction)}
						</p>
						<div className="flex justify-end gap-2 border-t border-border/60 pt-3">
							<Button
								variant="outline"
								size="sm"
								onClick={() => setConfirmAction(null)}
								className="h-8 text-xs font-semibold">
								Cancel
							</Button>
							<Button
								size="sm"
								variant={
									confirmAction === 'stop' ? 'destructive' : 'default'
								}
								onClick={() => executeActionClick(confirmAction)}
								className="h-8 text-xs font-semibold">
								Confirm
							</Button>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
