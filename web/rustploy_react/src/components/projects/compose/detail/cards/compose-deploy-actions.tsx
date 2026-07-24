import {Rocket, RefreshCw, Hammer, Play, Square, StopCircle, Terminal} from 'lucide-react';
import {Button} from '#/components/ui/button';

type ActionType = 'deploy' | 'reload' | 'rebuild' | 'start' | 'stop' | 'cancel';

interface ComposeDeployActionsProps {
	isProcessing: boolean;
	isBuilding: boolean;
	isRunning: boolean;
	activeLoading: ActionType | null;
	autoDeploy: boolean;
	setAutoDeploy: (v: boolean) => void;
	cleanCache: boolean;
	setCleanCache: (v: boolean) => void;
	setConfirmAction: (action: ActionType) => void;
	onOpenTerminal?: () => void;
}

export function ComposeDeployActions({
	isProcessing,
	isBuilding,
	isRunning,
	activeLoading,
	autoDeploy,
	setAutoDeploy,
	cleanCache,
	setCleanCache,
	setConfirmAction,
	onOpenTerminal,
}: ComposeDeployActionsProps) {
	return (
		<div className="flex flex-wrap items-center justify-between gap-4">
			<div className="flex flex-wrap items-center gap-2">
				{/* Deploy */}
				<Button
					onClick={() => setConfirmAction('deploy')}
					disabled={isProcessing || isBuilding}
					size="sm"
					className="bg-primary hover:bg-primary/90 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg"
				>
					{activeLoading === 'deploy' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Rocket className="w-4 h-4" />}
					{activeLoading === 'deploy' ? 'Deploying...' : 'Deploy'}
				</Button>

				{/* Reload */}
				<Button
					onClick={() => setConfirmAction('reload')}
					disabled={isProcessing || isBuilding}
					variant="outline"
					size="sm"
					className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg"
				>
					{activeLoading === 'reload' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <RefreshCw className="w-4 h-4" />}
					{activeLoading === 'reload' ? 'Reloading...' : 'Reload'}
				</Button>

				{/* Rebuild */}
				<Button
					onClick={() => setConfirmAction('rebuild')}
					disabled={isProcessing || isBuilding}
					variant="outline"
					size="sm"
					className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg"
				>
					{activeLoading === 'rebuild' ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Hammer className="w-4 h-4" />}
					{activeLoading === 'rebuild' ? 'Rebuilding...' : 'Rebuild'}
				</Button>

				{/* 3-State Action Button: Cancel (Building), Stop (Running), Start (Stopped) */}
				{isBuilding ? (
					<Button
						onClick={() => setConfirmAction('cancel')}
						disabled={isProcessing}
						variant="outline"
						size="sm"
						className="border-border text-destructive hover:bg-destructive/10 font-semibold flex items-center gap-1.5 h-9 rounded-lg"
					>
						{activeLoading === 'cancel' ? <RefreshCw className="w-4 h-4 animate-spin text-destructive" /> : <Square className="w-4 h-4" />}
						{activeLoading === 'cancel' ? 'Cancelling...' : 'Cancel'}
					</Button>
				) : isRunning ? (
					<Button
						onClick={() => setConfirmAction('stop')}
						disabled={isProcessing}
						variant="outline"
						size="sm"
						className="border-border text-destructive hover:bg-destructive/10 font-semibold flex items-center gap-1.5 h-9 rounded-lg"
					>
						{activeLoading === 'stop' ? <RefreshCw className="w-4 h-4 animate-spin text-destructive" /> : <StopCircle className="w-4 h-4" />}
						{activeLoading === 'stop' ? 'Stopping...' : 'Stop'}
					</Button>
				) : (
					<Button
						onClick={() => setConfirmAction('start')}
						disabled={isProcessing}
						variant="outline"
						size="sm"
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg"
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
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg"
					>
						<Terminal className="w-4 h-4 text-primary" /> Open Terminal
					</Button>
				)}
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
	);
}
