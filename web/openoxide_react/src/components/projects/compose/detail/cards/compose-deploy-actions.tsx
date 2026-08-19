import {
	Rocket,
	RefreshCw,
	Hammer,
	Play,
	Ban,
	Terminal,
} from 'lucide-react';
import {Button} from '#/components/ui/button';

type ActionType =
	| 'deploy'
	| 'reload'
	| 'rebuild'
	| 'start'
	| 'stop'
	| 'cancel';

interface ComposeDeployActionsProps {
	isProcessing: boolean;
	isBuilding: boolean;
	isRunning: boolean;
	composeStatus?: string;
	activeLoading: any;
	onAction: (action: ActionType) => void;
	onOpenTerminal?: () => void;
}

export function ComposeDeployActions({
	isProcessing,
	isBuilding,
	isRunning,
	composeStatus,
	activeLoading,
	onAction,
	onOpenTerminal,
}: ComposeDeployActionsProps) {
	const st = (composeStatus || '').toUpperCase();
	const isStoppingOrCancelling =
		st === 'STOPPING' ||
		st === 'CANCELLING' ||
		activeLoading === 'stop' ||
		activeLoading === 'cancel';
	const isDisabled = isProcessing || isBuilding || isStoppingOrCancelling;

	return (
		<div className="flex flex-wrap items-center justify-between gap-4">
			<div className="flex flex-wrap items-center gap-2">
				{/* Deploy */}
				<Button
					onClick={() => onAction('deploy')}
					disabled={isDisabled}
					size="sm"
					className="flex h-9 cursor-pointer items-center gap-1.5 rounded-lg bg-primary font-semibold text-primary-foreground hover:bg-primary/90 disabled:cursor-not-allowed disabled:opacity-50">
					{activeLoading === 'deploy' ? (
						<RefreshCw className="h-4 w-4 animate-spin" />
					) : (
						<Rocket className="h-4 w-4" />
					)}
					{activeLoading === 'deploy' ? 'Deploying...' : 'Deploy'}
				</Button>

				{/* Reload */}
				<Button
					onClick={() => onAction('reload')}
					disabled={isDisabled}
					variant="outline"
					size="sm"
					className="flex h-9 cursor-pointer items-center gap-1.5 rounded-lg border-border font-semibold text-foreground hover:bg-muted disabled:cursor-not-allowed disabled:opacity-50">
					{activeLoading === 'reload' ? (
						<RefreshCw className="h-4 w-4 animate-spin" />
					) : (
						<RefreshCw className="h-4 w-4" />
					)}
					{activeLoading === 'reload' ? 'Reloading...' : 'Reload'}
				</Button>

				{/* Rebuild */}
				<Button
					onClick={() => onAction('rebuild')}
					disabled={isDisabled}
					variant="outline"
					size="sm"
					className="flex h-9 cursor-pointer items-center gap-1.5 rounded-lg border-border font-semibold text-foreground hover:bg-muted disabled:cursor-not-allowed disabled:opacity-50">
					{activeLoading === 'rebuild' ? (
						<RefreshCw className="h-4 w-4 animate-spin" />
					) : (
						<Hammer className="h-4 w-4" />
					)}
					{activeLoading === 'rebuild' ? 'Rebuilding...' : 'Rebuild'}
				</Button>

				{/* 4-State Dynamic Action Button: Stopping, Cancelling, Stop (Running), Start (Stopped) */}
				{activeLoading === 'stop' || st === 'STOPPING' ? (
					<Button
						disabled
						variant="outline"
						size="sm"
						className="flex h-9 cursor-not-allowed items-center gap-1.5 rounded-lg border-border font-semibold text-destructive opacity-80">
						<RefreshCw className="h-4 w-4 animate-spin text-destructive" />
						Stopping...
					</Button>
				) : activeLoading === 'cancel' || st === 'CANCELLING' ? (
					<Button
						disabled
						variant="outline"
						size="sm"
						className="flex h-9 cursor-not-allowed items-center gap-1.5 rounded-lg border-destructive/50 px-4 font-bold text-destructive opacity-80">
						<RefreshCw className="h-4 w-4 animate-spin text-destructive" />
						Cancelling...
					</Button>
				) : isBuilding ? (
					<Button
						onClick={() => onAction('cancel')}
						disabled={activeLoading === 'cancel'}
						variant="outline"
						size="sm"
						className="flex h-9 cursor-pointer items-center gap-1.5 rounded-lg border-destructive/50 px-4 font-bold text-destructive shadow-xs hover:bg-destructive/10">
						<RefreshCw className="h-4 w-4 animate-spin text-destructive" />
						{activeLoading === 'cancel' ? 'Cancelling...' : 'Cancel'}
					</Button>
				) : isRunning ? (
					<Button
						onClick={() => onAction('stop')}
						disabled={isDisabled}
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
						onClick={() => onAction('start')}
						disabled={isDisabled}
						variant="outline"
						size="sm"
						className="flex h-9 cursor-pointer items-center gap-1.5 rounded-lg border-border font-semibold text-foreground hover:bg-muted">
						{activeLoading === 'start' ? (
							<RefreshCw className="h-4 w-4 animate-spin" />
						) : (
							<Play className="h-4 w-4" />
						)}
						{activeLoading === 'start' ? 'Starting...' : 'Start'}
					</Button>
				)}

				{/* Terminal Access Button */}
				{onOpenTerminal && (
					<Button
						onClick={onOpenTerminal}
						disabled={isDisabled || !isRunning}
						variant="outline"
						size="sm"
						className="ml-auto flex h-9 items-center gap-1.5 rounded-lg border-border font-semibold text-foreground hover:bg-muted disabled:cursor-not-allowed disabled:opacity-50 sm:ml-0">
						<Terminal className="h-4 w-4" />
						Terminal
					</Button>
				)}
			</div>
		</div>
	);
}
