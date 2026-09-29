import {useState} from 'react';
import {toast} from 'sonner';
import {
	Rocket,
	RefreshCw,
	Hammer,
	Play,
	Ban,
	Terminal,
	KeyRound,
	GitBranch,
	X,
} from 'lucide-react';
import {
	useComposeDeploy,
	useComposeRedeploy,
	useComposeReload,
	useComposeStart,
	useComposeStop,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {IconTooltip} from '#/components/shared/icon-tooltip';
import {
	useComposeDeployState,
	type ComposeActionType,
} from './use-compose-deploy-state';

interface ComposeHeaderActionsProps {
	compose: any;
	onUpdated?: () => void;
	onOpenTerminal?: () => void;
	onOpenEnv?: () => void;
	/** Source & Provider sheet, opened from the card header. */
	onOpenSource?: () => void;
}

/** Icon-only Deploy / Reload / Rebuild / Start-Stop / Terminal buttons. */
export function ComposeHeaderActions({
	compose,
	onUpdated,
	onOpenTerminal,
	onOpenEnv,
	onOpenSource,
}: ComposeHeaderActionsProps) {
	const [activeLoading, setActiveLoading] =
		useState<ComposeActionType | null>(null);

	const {setActionInFlight, pendingMutationAction, isBuilding, isRunning} =
		useComposeDeployState(compose);

	const deployMutation = useComposeDeploy();
	const reloadMutation = useComposeReload();
	const rebuildMutation = useComposeRedeploy();
	const startMutation = useComposeStart();
	const stopMutation = useComposeStop();

	const executeAction = async (action: ComposeActionType) => {
		setActiveLoading(action);
		if (['start', 'deploy', 'rebuild', 'reload'].includes(action)) {
			setActionInFlight(action);
		} else {
			setActionInFlight(null);
		}
		try {
			const id = compose?.id;
			if (action === 'deploy') {
				await deployMutation.mutateAsync(id);
				toast.success('Compose stack deployment initiated');
			} else if (action === 'reload') {
				await reloadMutation.mutateAsync(id);
				toast.success('Compose stack reloaded');
			} else if (action === 'rebuild') {
				await rebuildMutation.mutateAsync(id);
				toast.success('Compose stack rebuild initiated');
			} else if (action === 'start') {
				await startMutation.mutateAsync(id);
				toast.success('Compose stack started');
			} else if (action === 'stop' || action === 'cancel') {
				await stopMutation.mutateAsync(id);
				toast.success(
					action === 'stop'
						? 'Compose stack stopped'
						: 'Compose deployment cancelled',
				);
			}
			onUpdated?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
			setActionInFlight(null);
		} finally {
			setActiveLoading(null);
		}
	};

	const st = (
		compose?.compose_status ||
		compose?.status ||
		''
	).toUpperCase();
	const isProcessing = activeLoading !== null || !!pendingMutationAction;
	const isStoppingOrCancelling =
		st === 'STOPPING' ||
		st === 'CANCELLING' ||
		activeLoading === 'stop' ||
		activeLoading === 'cancel';
	const disabled = isProcessing || isBuilding || isStoppingOrCancelling;

	return (
		<div className="flex shrink-0 items-center gap-1">
			<IconTooltip label="Deploy">
				<Button
					variant="ghost"
					size="icon"
					aria-label="Deploy"
					disabled={disabled}
					onClick={() => executeAction('deploy')}
					className="size-8 text-muted-foreground hover:text-foreground">
					{activeLoading === 'deploy' ? (
						<RefreshCw className="size-4 animate-spin" />
					) : (
						<Rocket className="size-4" />
					)}
				</Button>
			</IconTooltip>
			<IconTooltip label="Rebuild">
				<Button
					variant="ghost"
					size="icon"
					aria-label="Rebuild"
					disabled={disabled}
					onClick={() => executeAction('rebuild')}
					className="size-8 text-muted-foreground hover:text-foreground">
					{activeLoading === 'rebuild' ? (
						<RefreshCw className="size-4 animate-spin" />
					) : (
						<Hammer className="size-4" />
					)}
				</Button>
			</IconTooltip>
			{isStoppingOrCancelling ? (
				<IconTooltip
					label={st === 'CANCELLING' ? 'Cancelling' : 'Stopping'}>
					<Button
						variant="ghost"
						size="icon"
						aria-label={st === 'CANCELLING' ? 'Cancelling' : 'Stopping'}
						disabled
						className="size-8 text-red-500 dark:text-red-400">
						<RefreshCw className="size-4 animate-spin" />
					</Button>
				</IconTooltip>
			) : isBuilding ? (
				<IconTooltip label="Cancel build">
					<Button
						variant="ghost"
						size="icon"
						aria-label="Cancel build"
						onClick={() => executeAction('cancel')}
						className="size-8 text-red-500 transition-colors hover:bg-red-500/10 hover:text-red-600 dark:text-red-400 dark:hover:bg-red-400/15 dark:hover:text-red-300">
						<X className="size-4" />
					</Button>
				</IconTooltip>
			) : isRunning ? (
				<IconTooltip label="Stop">
					<Button
						variant="ghost"
						size="icon"
						aria-label="Stop"
						disabled={disabled}
						onClick={() => executeAction('stop')}
						className="size-8 text-red-500 transition-colors hover:bg-red-500/10 hover:text-red-600 dark:text-red-400 dark:hover:bg-red-400/15 dark:hover:text-red-300">
						<Ban className="size-4" />
					</Button>
				</IconTooltip>
			) : (
				<IconTooltip label="Start">
					<Button
						variant="ghost"
						size="icon"
						aria-label="Start"
						disabled={disabled}
						onClick={() => executeAction('start')}
						className="size-8 text-muted-foreground hover:text-foreground">
						{activeLoading === 'start' ? (
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
						className="size-8 text-muted-foreground hover:text-foreground">
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
						className="size-8 text-muted-foreground hover:text-foreground">
						<KeyRound className="size-4" />
					</Button>
				</IconTooltip>
			)}
			{onOpenSource && (
				<IconTooltip label="Source configuration">
					<Button
						variant="ghost"
						size="icon"
						aria-label="Source configuration"
						onClick={onOpenSource}
						className="size-8 text-muted-foreground hover:text-foreground">
						<GitBranch className="size-4" />
					</Button>
				</IconTooltip>
			)}
		</div>
	);
}
