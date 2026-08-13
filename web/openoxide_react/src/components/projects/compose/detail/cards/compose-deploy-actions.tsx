import {Rocket, RefreshCw, Hammer, Play, Ban, Terminal} from 'lucide-react';
import {Button} from '#/components/ui/button';

type ActionType = 'deploy' | 'reload' | 'rebuild' | 'start' | 'stop' | 'cancel';

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
	const isStoppingOrCancelling = st === 'STOPPING' || st === 'CANCELLING' || activeLoading === 'stop' || activeLoading === 'cancel';
	const isDisabled = isProcessing || isBuilding || isStoppingOrCancelling;

	return (
		<div className="flex flex-wrap items-center justify-between gap-4">
			<div className="flex flex-wrap items-center gap-2">
				{/* Deploy */}
				<Button
					onClick={() => onAction('deploy')}
					disabled={isDisabled}
					size="sm"
					className="bg-primary hover:bg-primary/90 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{activeLoading === 'deploy' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Rocket className="w-4 h-4" />}
					{activeLoading === 'deploy' ? 'Deploying...' : 'Deploy'}
				</Button>

				{/* Reload */}
				<Button
					onClick={() => onAction('reload')}
					disabled={isDisabled}
					variant="outline"
					size="sm"
					className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{activeLoading === 'reload' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <RefreshCw className="w-4 h-4" />}
					{activeLoading === 'reload' ? 'Reloading...' : 'Reload'}
				</Button>

				{/* Rebuild */}
				<Button
					onClick={() => onAction('rebuild')}
					disabled={isDisabled}
					variant="outline"
					size="sm"
					className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{activeLoading === 'rebuild' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Hammer className="w-4 h-4" />}
					{activeLoading === 'rebuild' ? 'Rebuilding...' : 'Rebuild'}
				</Button>

				{/* 4-State Dynamic Action Button: Stopping, Cancelling, Stop (Running), Start (Stopped) */}
				{activeLoading === 'stop' || st === 'STOPPING' ? (
					<Button
						disabled
						variant="outline"
						size="sm"
						className="border-border text-destructive font-semibold flex items-center gap-1.5 h-9 rounded-lg opacity-80 cursor-not-allowed"
					>
						<RefreshCw className="w-4 h-4 animate-spin text-destructive" />
						Stopping...
					</Button>
				) : activeLoading === 'cancel' || st === 'CANCELLING' ? (
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
						onClick={() => onAction('cancel')}
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
						onClick={() => onAction('stop')}
						disabled={isDisabled}
						variant="destructive"
						size="sm"
						className="h-9 px-4 text-xs font-semibold gap-1.5 rounded-lg flex items-center cursor-pointer"
					>
						{activeLoading === 'stop' ? <RefreshCw className="size-4 animate-spin" /> : <Ban className="size-4" />}
						{activeLoading === 'stop' ? 'Stopping...' : 'Stop'}
					</Button>
				) : (
					<Button
						onClick={() => onAction('start')}
						disabled={isDisabled}
						variant="outline"
						size="sm"
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer"
					>
						{activeLoading === 'start' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Play className="w-4 h-4" />}
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
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg ml-auto sm:ml-0 disabled:opacity-50 disabled:cursor-not-allowed"
					>
						<Terminal className="w-4 h-4" />
						Terminal
					</Button>
				)}
			</div>
		</div>
	);
}
