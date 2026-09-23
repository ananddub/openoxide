import {useState} from 'react';
import {
	Rocket,
	RefreshCw,
	Hammer,
	Play,
	Ban,
	Terminal,
	KeyRound,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {IconTooltip} from '#/components/shared/icon-tooltip';
import {
	rawAppStatus,
	isAppProcessing,
} from './deploy-settings-card/actions';

interface AppHeaderActionsProps {
	app: any;
	handleAction: (action: any) => Promise<void>;
	onOpenTerminal?: () => void;
	onOpenEnv?: () => void;
}

/** Icon-only Deploy / Reload / Rebuild / Start-Stop / Terminal buttons. */
export function AppHeaderActions({
	app,
	handleAction,
	onOpenTerminal,
	onOpenEnv,
}: AppHeaderActionsProps) {
	const [busy, setBusy] = useState<string | null>(null);

	const st = rawAppStatus(app).toUpperCase();
	const isBusyStatus = [
		'BUILDING',
		'DEPLOYING',
		'QUEUED',
		'PENDING',
		'PREPARING',
		'STOPPING',
		'CANCELLING',
	].includes(st);
	const isRunning = ['RUNNING', 'ACTIVE', 'HEALTHY', 'DEPLOYED'].includes(
		st,
	);
	const disabled = isAppProcessing(app) || busy !== null;

	const run = async (action: string) => {
		setBusy(action);
		try {
			await handleAction(action);
		} finally {
			setBusy(null);
		}
	};

	const iconBtn = 'size-8 text-muted-foreground hover:text-foreground';

	return (
		<div className="flex shrink-0 items-center gap-1">
			<IconTooltip label="Deploy">
				<Button
					variant="ghost"
					size="icon"
					aria-label="Deploy"
					disabled={disabled || isBusyStatus}
					onClick={() => void run('deploy')}
					className={iconBtn}>
					{busy === 'deploy' ? (
						<RefreshCw className="size-4 animate-spin" />
					) : (
						<Rocket className="size-4" />
					)}
				</Button>
			</IconTooltip>
			<IconTooltip label="Reload">
				<Button
					variant="ghost"
					size="icon"
					aria-label="Reload"
					disabled={disabled || isBusyStatus}
					onClick={() => void run('reload')}
					className={iconBtn}>
					<RefreshCw
						className={`size-4 ${busy === 'reload' ? 'animate-spin' : ''}`}
					/>
				</Button>
			</IconTooltip>
			<IconTooltip label="Rebuild">
				<Button
					variant="ghost"
					size="icon"
					aria-label="Rebuild"
					disabled={disabled || isBusyStatus}
					onClick={() => void run('rebuild')}
					className={iconBtn}>
					{busy === 'rebuild' ? (
						<RefreshCw className="size-4 animate-spin" />
					) : (
						<Hammer className="size-4" />
					)}
				</Button>
			</IconTooltip>
			{st === 'STOPPING' || busy === 'stop' ? (
				<IconTooltip label="Stopping">
					<Button
						variant="ghost"
						size="icon"
						aria-label="Stopping"
						disabled
						className="size-8 text-destructive">
						<RefreshCw className="size-4 animate-spin" />
					</Button>
				</IconTooltip>
			) : st === 'CANCELLING' || busy === 'cancel' ? (
				<IconTooltip label="Cancelling">
					<Button
						variant="ghost"
						size="icon"
						aria-label="Cancelling"
						disabled
						className="size-8 text-destructive">
						<RefreshCw className="size-4 animate-spin" />
					</Button>
				</IconTooltip>
			) : isBusyStatus ? (
				<IconTooltip label="Cancel build">
					<Button
						variant="ghost"
						size="icon"
						aria-label="Cancel build"
						disabled={disabled}
						onClick={() => void run('cancel')}
						className="size-8 text-destructive hover:text-destructive">
						<Ban className="size-4" />
					</Button>
				</IconTooltip>
			) : isRunning ? (
				<IconTooltip label="Stop">
					<Button
						variant="ghost"
						size="icon"
						aria-label="Stop"
						disabled={disabled}
						onClick={() => void run('stop')}
						className="size-8 text-muted-foreground hover:text-destructive">
						{busy === 'stop' ? (
							<RefreshCw className="size-4 animate-spin" />
						) : (
							<Ban className="size-4" />
						)}
					</Button>
				</IconTooltip>
			) : (
				<IconTooltip label="Start">
					<Button
						variant="ghost"
						size="icon"
						aria-label="Start"
						disabled={disabled}
						onClick={() => void run('start')}
						className={iconBtn}>
						{busy === 'start' ? (
							<RefreshCw className="size-4 animate-spin" />
						) : (
							<Play className="size-4" />
						)}
					</Button>
				</IconTooltip>
			)}
			{onOpenTerminal && (
				<IconTooltip label="Terminal">
					<Button
						variant="ghost"
						size="icon"
						aria-label="Terminal"
						disabled={disabled || !isRunning}
						onClick={onOpenTerminal}
						className={iconBtn}>
						<Terminal className="size-4" />
					</Button>
				</IconTooltip>
			)}
			{onOpenEnv && (
				<IconTooltip label="Environment variables">
					<Button
						variant="ghost"
						size="icon"
						aria-label="Environment variables"
						onClick={onOpenEnv}
						className={iconBtn}>
						<KeyRound className="size-4" />
					</Button>
				</IconTooltip>
			)}
		</div>
	);
}
