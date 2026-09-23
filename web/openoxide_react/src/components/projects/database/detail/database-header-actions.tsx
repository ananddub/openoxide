import {useState} from 'react';
import {
	Rocket,
	RefreshCw,
	Play,
	Ban,
	Terminal,
	KeyRound,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {IconTooltip} from '#/components/shared/icon-tooltip';
import type {DatabaseActionType} from './database-deploy-settings-card';

interface DatabaseHeaderActionsProps {
	database: any;
	onAction: (action: DatabaseActionType) => Promise<void>;
	onOpenTerminal?: () => void;
	onOpenEnv?: () => void;
}

/** Icon-only Deploy / Reload / Start-Stop / Terminal buttons. */
export function DatabaseHeaderActions({
	database,
	onAction,
	onOpenTerminal,
	onOpenEnv,
}: DatabaseHeaderActionsProps) {
	const [busy, setBusy] = useState<string | null>(null);

	const st = String(
		database?.database_status || database?.status || '',
	).toUpperCase();
	const isBusyStatus = [
		'PROVISIONING',
		'BUILDING',
		'DEPLOYING',
		'STARTING',
		'PENDING',
		'STOPPING',
		'CANCELLING',
		'RELOADING',
	].includes(st);
	const isRunning = ['RUNNING', 'ACTIVE', 'HEALTHY', 'READY'].includes(st);
	const disabled = isBusyStatus || busy !== null;

	const run = async (action: DatabaseActionType) => {
		setBusy(action);
		try {
			await onAction(action);
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
					disabled={disabled}
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
					disabled={disabled}
					onClick={() => void run('reload')}
					className={iconBtn}>
					<RefreshCw
						className={`size-4 ${busy === 'reload' ? 'animate-spin' : ''}`}
					/>
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
