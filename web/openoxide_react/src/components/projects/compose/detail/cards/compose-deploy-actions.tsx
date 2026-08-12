import {Rocket, RefreshCw, Hammer, Play, Ban, Terminal} from 'lucide-react';
import {Button} from '#/components/ui/button';

type ActionType = 'deploy' | 'reload' | 'rebuild' | 'start' | 'stop' | 'cancel';

interface ComposeDeployActionsProps {
	isProcessing: boolean;
	isBuilding: boolean;
	isRunning: boolean;
	activeLoading: any;
	onAction: (action: ActionType) => void;
	onOpenTerminal?: () => void;
}

export function ComposeDeployActions({
	isProcessing,
	isBuilding,
	isRunning,
	activeLoading,
	onAction,
	onOpenTerminal,
}: ComposeDeployActionsProps) {
	return (
		<div className="flex flex-wrap items-center justify-between gap-4">
			<div className="flex flex-wrap items-center gap-2">
				{/* Deploy */}
				<Button
					onClick={() => onAction('deploy')}
					disabled={isProcessing || isBuilding}
					size="sm"
					className="bg-primary hover:bg-primary/90 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer"
				>
					{activeLoading === 'deploy' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Rocket className="w-4 h-4" />}
					{activeLoading === 'deploy' ? 'Deploying...' : 'Deploy'}
				</Button>

				{/* Reload */}
				<Button
					onClick={() => onAction('reload')}
					disabled={isProcessing || isBuilding}
					variant="outline"
					size="sm"
					className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer"
				>
					{activeLoading === 'reload' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <RefreshCw className="w-4 h-4" />}
					{activeLoading === 'reload' ? 'Reloading...' : 'Reload'}
				</Button>

				{/* Rebuild */}
				<Button
					onClick={() => onAction('rebuild')}
					disabled={isProcessing || isBuilding}
					variant="outline"
					size="sm"
					className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer"
				>
					{activeLoading === 'rebuild' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Hammer className="w-4 h-4" />}
					{activeLoading === 'rebuild' ? 'Rebuilding...' : 'Rebuild'}
				</Button>

				{/* 4-State Dynamic Action Button: Stopping (Stop Loading), Cancel (Deploy Building), Stop (Running), Start (Stopped) */}
				{activeLoading === 'stop' ? (
					<Button
						disabled
						variant="outline"
						size="sm"
						className="border-border text-destructive font-semibold flex items-center gap-1.5 h-9 rounded-lg opacity-80"
					>
						<RefreshCw className="w-4 h-4 animate-spin text-destructive" />
						Stopping...
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
						onClick={() => onAction('start')}
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
				{onOpenTerminal && (
					<Button
						onClick={onOpenTerminal}
						variant="outline"
						size="sm"
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg ml-auto sm:ml-0"
					>
						<Terminal className="w-4 h-4" />
						Terminal
					</Button>
				)}
			</div>
		</div>
	);
}
