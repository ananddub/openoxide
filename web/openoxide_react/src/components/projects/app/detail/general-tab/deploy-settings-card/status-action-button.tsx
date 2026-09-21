import {RefreshCw, Play, Ban} from 'lucide-react';
import {Button} from '#/components/ui/button';
import type {ActionType} from './actions';

/**
 * 4-state action button: Stopping / Cancelling / Cancel (building) /
 * Stop (running) / Start (idle, error or stopped).
 */
export function StatusActionButton({
	app,
	isBuilding,
	isRunning,
	activeLoading,
	disabled,
	onExecute,
}: {
	app: any;
	isBuilding: boolean;
	isRunning: boolean;
	activeLoading: string | null;
	disabled: boolean;
	onExecute: (action: ActionType) => void;
}) {
	const rawStatus = (app?.app_status || '').toUpperCase();
	if (activeLoading === 'stop' || rawStatus === 'STOPPING') {
		return (
			<Button
				disabled
				variant="outline"
				size="sm"
				className="flex h-9 cursor-not-allowed items-center gap-1.5 rounded-lg  font-semibold text-destructive opacity-80">
				<RefreshCw className="h-4 w-4 animate-spin text-destructive" />
				Stopping...
			</Button>
		);
	}
	if (activeLoading === 'cancel' || rawStatus === 'CANCELLING') {
		return (
			<Button
				disabled
				variant="outline"
				size="sm"
				className="flex h-9 cursor-not-allowed items-center gap-1.5 rounded-lg border-destructive/50 px-4 font-bold text-destructive opacity-80">
				<RefreshCw className="h-4 w-4 animate-spin text-destructive" />
				Cancelling...
			</Button>
		);
	}
	if (isBuilding) {
		return (
			<Button
				onClick={() => onExecute('cancel')}
				disabled={activeLoading === 'cancel'}
				variant="outline"
				size="sm"
				className="flex h-9 cursor-pointer items-center gap-1.5 rounded-lg border-destructive/50 px-4 font-bold text-destructive shadow-xs hover:bg-destructive/10">
				<RefreshCw className="h-4 w-4 animate-spin text-destructive" />
				{activeLoading === 'cancel' ? 'Cancelling...' : 'Cancel'}
			</Button>
		);
	}
	if (isRunning) {
		return (
			<Button
				onClick={() => onExecute('stop')}
				disabled={disabled}
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
		);
	}
	return (
		<Button
			onClick={() => onExecute('start')}
			disabled={disabled}
			variant="outline"
			size="sm"
			className="flex h-9 cursor-pointer items-center gap-1.5 rounded-lg  font-semibold text-foreground hover:bg-muted">
			{activeLoading === 'start' ? (
				<RefreshCw className="h-4 w-4 animate-spin" />
			) : (
				<Play className="h-4 w-4" />
			)}
			{activeLoading === 'start' ? 'Starting...' : 'Start'}
		</Button>
	);
}
