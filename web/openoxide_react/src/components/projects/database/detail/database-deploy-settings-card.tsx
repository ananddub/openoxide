import {useState} from 'react';
import {Rocket, RefreshCw, Play, Ban, Terminal} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {TerminalCard} from '#/components/shared/terminal-card';
import {DatabaseActionConfirmDialog} from './database-action-confirm-dialog';
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
	/** Render inside another card (no own panel background/shadow). */
	embedded?: boolean;
}

export function DatabaseDeploySettingsCard({
	database,
	actionLoading: propActionLoading,
	onAction,
	onUpdated,
	embedded = false,
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

	return (
		<div
			className={
				embedded
					? 'flex flex-col gap-3'
					: 'flex flex-col gap-5 rounded-xl bg-card p-6 shadow-xs'
			}>
			<div>
				<h3 className="flex items-center gap-2 text-base font-bold tracking-tight text-foreground">
					<Rocket className="size-4 text-primary" /> Deploy Settings
				</h3>
				<p className="mt-0.5 text-xs text-muted-foreground">
					Control database provisioning, service reload, lifecycle state,
					and open interactive container shell.
				</p>
			</div>

			<div className="flex flex-wrap items-center gap-3 border-t  pt-4">
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
					className="h-9 cursor-pointer gap-2 rounded-lg bg-muted px-4 text-xs font-semibold text-foreground hover:bg-muted/80 disabled:cursor-not-allowed disabled:opacity-50">
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
						className="flex h-9 items-center gap-1.5 rounded-lg  font-semibold text-destructive opacity-80">
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
						className="flex h-9 cursor-pointer items-center gap-1.5 rounded-lg  font-semibold text-foreground hover:bg-muted">
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
					className="h-9 gap-2 rounded-lg  px-4 text-xs font-semibold text-foreground hover:bg-accent disabled:cursor-not-allowed disabled:opacity-50">
					<Terminal className="size-4 text-primary" /> Open Terminal
				</Button>
			</div>

			{isTerminalOpen && database && (
				<TerminalCard
					app={database}
					enabled={true}
					onClose={() => setIsTerminalOpen(false)}
				/>
			)}

			{confirmAction && (
				<DatabaseActionConfirmDialog
					action={confirmAction}
					onClose={() => setConfirmAction(null)}
					onConfirm={() => executeActionClick(confirmAction)}
				/>
			)}
		</div>
	);
}
