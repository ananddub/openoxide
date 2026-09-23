import {useState, useMemo} from 'react';
import {Terminal, Rocket} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {TerminalCard} from '#/components/shared/terminal-card';
import {ToggleSwitch} from '#/components/shared/toggle-switch';
import {
	useApplicationUpdateBuildConfig,
	useDeploymentItems,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {isBuildActive} from '#/components/projects/detail/deployments/build-status';
import {
	ACTION_DESCRIPTIONS,
	ACTION_TITLES,
	ACTIVE_DEPLOYMENT_STATUSES,
	isAppProcessing,
	rawAppStatus,
	type ActionType,
} from './deploy-settings-card/actions';
import {ConfirmActionDialog} from './deploy-settings-card/confirm-action-dialog';
import {PrimaryActionButtons} from './deploy-settings-card/primary-action-buttons';
import {StatusActionButton} from './deploy-settings-card/status-action-button';

interface DeploySettingsCardProps {
	app: any;
	handleAction: (action: any) => Promise<void>;
	onUpdated?: () => void;
	/** Render inside another card (no own panel background/shadow). */
	embedded?: boolean;
}

export function DeploySettingsCard({
	app,
	handleAction,
	onUpdated,
	embedded = false,
}: DeploySettingsCardProps) {
	const [autoDeploy, setAutoDeploy] = useState(app.auto_deploy !== false);
	const [cleanCache, setCleanCache] = useState(!!app.clean_cache);
	const [showTerminal, setShowTerminal] = useState(false);
	const [confirmAction, setConfirmAction] = useState<ActionType | null>(
		null,
	);
	const [activeLoading, setActiveLoading] = useState<string | null>(null);

	// Autodeploy + clean-cache ride the dedicated UpdateBuildConfig RPC —
	// they were local-only state, so toggles silently reset on refetch.
	const patchBuildConfig = useApplicationUpdateBuildConfig();
	const persistFlag = async (
		flag: 'auto_deploy' | 'clean_cache',
		value: boolean,
	) => {
		if (flag === 'auto_deploy') setAutoDeploy(value);
		else setCleanCache(value);
		try {
			await patchBuildConfig.mutateAsync({id: app.id, [flag]: value});
		} catch (err: unknown) {
			if (flag === 'auto_deploy') setAutoDeploy(!value);
			else setCleanCache(!value);
			toast.error(formatApiError(err));
		}
	};

	// Live deployments query filtered to this app; the table-watch stream
	// (DEPLOYMENTS table) invalidates ['deployments'] while mounted.
	const {data: storeDeployments = []} = useDeploymentItems();
	const hasActiveDeployment = useMemo(
		() =>
			(Array.isArray(storeDeployments) ? storeDeployments : []).some(
				(e: any) =>
					String(e?.application_id) === String(app?.id) &&
					isBuildActive(e),
			),
		[storeDeployments, app?.id],
	);

	const status = rawAppStatus(app);
	const isRunning = [
		'running',
		'done',
		'healthy',
		'deployed',
		'success',
		'up',
		'active',
		'ok',
	].includes(status);
	// Autodeploy is a webhook-driven service_git flag; it is meaningless
	// for Docker/Drop sources (they have no repository to push to).
	const isGitSource = !['DOCKER', 'DROP', 'NONE', ''].includes(
		(app?.source_type || '').toUpperCase(),
	);
	const isBuilding =
		ACTIVE_DEPLOYMENT_STATUSES.some(s => status === s.toLowerCase()) ||
		['starting', 'building', 'queued', 'preparing'].includes(status) ||
		hasActiveDeployment ||
		activeLoading === 'deploy' ||
		activeLoading === 'rebuild' ||
		activeLoading === 'reload';

	const executeAction = async (action: ActionType) => {
		setConfirmAction(null);
		setActiveLoading(action);
		try {
			await handleAction(action);
			onUpdated?.();
		} finally {
			setActiveLoading(null);
		}
	};

	const isProcessing =
		activeLoading !== null || isAppProcessing(app) || hasActiveDeployment;

	return (
		<section
			className={
				embedded
					? 'flex flex-col gap-3'
					: 'flex flex-col gap-4 rounded-xl bg-card p-5 shadow-sm'
			}>
			<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
				<Rocket className="size-3.5 text-primary" />
				Deploy Settings
			</h3>
			<div className="flex flex-wrap items-center justify-between gap-4">
				{!embedded && (
					<div className="flex flex-wrap items-center gap-2">
						<PrimaryActionButtons
						isBuilding={isBuilding}
						isProcessing={isProcessing}
						activeLoading={activeLoading}
						onExecute={executeAction}
					/>

					<StatusActionButton
						app={app}
						isBuilding={isBuilding}
						isRunning={isRunning}
						activeLoading={activeLoading}
						disabled={isProcessing}
						onExecute={executeAction}
					/>

					{/* Terminal Access Button */}
					<Button
						onClick={() => setShowTerminal(true)}
						disabled={isProcessing || !isRunning}
						variant="outline"
						size="sm"
						className="flex h-9 items-center gap-1.5 rounded-lg  font-semibold text-foreground hover:bg-muted disabled:cursor-not-allowed disabled:opacity-50">
						<Terminal className="h-4 w-4 text-primary" /> Open Terminal
						</Button>
					</div>
				)}

				{/* Switches */}
				<div className="flex items-center gap-4 border-l  pl-4">
					<label className="flex cursor-pointer items-center gap-2 select-none">
						<ToggleSwitch
							checked={autoDeploy && isGitSource}
							disabled={!isGitSource}
							title={
								isGitSource
									? undefined
									: 'Autodeploy applies to Git repository sources only'
							}
							onCheckedChange={v => void persistFlag('auto_deploy', v)}
						/>
						<span className="text-[11px] font-bold text-muted-foreground">
							Autodeploy
						</span>
					</label>

					<label className="flex cursor-pointer items-center gap-2 select-none">
						<ToggleSwitch
							checked={cleanCache}
							onCheckedChange={v => void persistFlag('clean_cache', v)}
						/>
						<span className="text-[11px] font-bold text-muted-foreground">
							Clean Cache
						</span>
					</label>
				</div>
			</div>

			{/* Confirmation Modal */}
			{confirmAction && (
				<ConfirmActionDialog
					title={ACTION_TITLES[confirmAction]}
					description={ACTION_DESCRIPTIONS[confirmAction]}
					destructive={
						confirmAction === 'cancel' || confirmAction === 'stop'
					}
					onCancel={() => setConfirmAction(null)}
					onConfirm={() => void executeAction(confirmAction)}
				/>
			)}

			{/* Container Terminal — inline card */}
			{showTerminal && (
				<TerminalCard
					app={app}
					enabled={true}
					onClose={() => setShowTerminal(false)}
				/>
			)}
		</section>
	);
}
