import {useState} from 'react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {ComposeConfirmDialog} from './cards/compose-confirm-dialog';
import {ComposeDeployActions} from './cards/compose-deploy-actions';

interface ComposeDeployCardProps {
	compose: any;
	onUpdated?: () => void;
	onOpenTerminal?: () => void;
}

type ActionType = 'deploy' | 'reload' | 'rebuild' | 'start' | 'stop' | 'cancel';

const FINAL_STATES = ['DEPLOYED', 'SUCCESS', 'FAILED', 'CANCELLED', 'ERROR'];

export function ComposeDeployCard({compose, onUpdated, onOpenTerminal}: ComposeDeployCardProps) {
	const [confirmAction, setConfirmAction] = useState<ActionType | null>(null);
	const [activeLoading, setActiveLoading] = useState<ActionType | null>(null);
	const [autoDeploy, setAutoDeploy] = useState<boolean>(compose?.auto_deploy ?? true);
	const [cleanCache, setCleanCache] = useState<boolean>(compose?.clean_cache ?? false);

	// Fetch compose deployments query
	const {data: rawEvents = [], refetch} = $api.useQuery(
		'get',
		'/deployments',
		{
			params: {
				query: {
					compose_id: compose?.id || 0,
					limit: 20,
				} as any,
			},
		},
		{
			enabled: !!compose?.id,
			refetchInterval: (query) => {
				const data = query.state.data as any[] | undefined;
				const hasActive = data?.some(e => {
					if (!e || (e.finished_at && Number(e.finished_at) > 0)) return false;
					const s = (e.status || '').toUpperCase();
					const st = (e.state || '').toUpperCase();
					return !FINAL_STATES.includes(s) && !FINAL_STATES.includes(st);
				});
				return hasActive ? 3000 : false;
			},
		}
	);

	const events = Array.isArray(rawEvents) ? rawEvents : [];

	const isBuilding = events.some(e => {
		if (!e || (e.finished_at && Number(e.finished_at) > 0)) return false;
		const s = (e.status || '').toUpperCase();
		const st = (e.state || '').toUpperCase();
		return !FINAL_STATES.includes(s) && !FINAL_STATES.includes(st);
	});

	// Mutations
	const deployMutation = $api.useMutation('post', '/compose/{id}/deploy');
	const reloadMutation = $api.useMutation('post', '/compose/{id}/reload');
	const rebuildMutation = $api.useMutation('post', '/compose/{id}/rebuild');
	const startMutation = $api.useMutation('post', '/compose/{id}/start');
	const stopMutation = $api.useMutation('post', '/compose/{id}/stop');

	const executeAction = async (action: ActionType) => {
		setConfirmAction(null);
		setActiveLoading(action);
		try {
			const path = {id: compose?.id};
			if (action === 'deploy') {
				await deployMutation.mutateAsync({params: {path}});
				toast.success('Compose stack deployment initiated');
			} else if (action === 'reload') {
				await reloadMutation.mutateAsync({params: {path}});
				toast.success('Compose stack reloaded');
			} else if (action === 'rebuild') {
				await rebuildMutation.mutateAsync({params: {path}});
				toast.success('Compose stack rebuild initiated');
			} else if (action === 'start') {
				await startMutation.mutateAsync({params: {path}});
				toast.success('Compose stack started');
			} else if (action === 'stop') {
				await stopMutation.mutateAsync({params: {path}});
				toast.success('Compose stack stopped');
			}
			refetch();
			onUpdated?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setActiveLoading(null);
		}
	};

	const isProcessing = activeLoading !== null;
	const rawStatus = (compose?.compose_status || compose?.status || '').toLowerCase();
	const isRunning = ['running', 'deployed', 'done', 'success', 'active', 'ok'].includes(rawStatus);

	return (
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4 shadow-sm">
			<h3 className="text-sm font-bold text-foreground">Deploy Settings</h3>
			
			<ComposeDeployActions
				isProcessing={isProcessing}
				isBuilding={isBuilding}
				isRunning={isRunning}
				activeLoading={activeLoading}
				autoDeploy={autoDeploy}
				setAutoDeploy={setAutoDeploy}
				cleanCache={cleanCache}
				setCleanCache={setCleanCache}
				setConfirmAction={setConfirmAction}
				onOpenTerminal={onOpenTerminal}
			/>

			<ComposeConfirmDialog
				action={confirmAction}
				onClose={() => setConfirmAction(null)}
				onConfirm={executeAction}
			/>
		</section>
	);
}
