import {useMemo} from 'react';
import {Play, Square, Rocket, Eye, XCircle} from 'lucide-react';
import {useNavigate} from '@tanstack/react-router';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useApplicationCancel,
	useApplicationDelete,
	useApplicationDeploy,
	useApplicationStart,
	useApplicationStop,
	useComposeCancel,
	useComposeDelete,
	useComposeDeploy,
	useComposeStart,
	useComposeStop,
	useDatabaseCancel,
	useDatabaseDelete,
	useDatabaseDeploy,
	useDatabaseStart,
	useDatabaseStop,
	parseDatabaseKind,
} from '#/api/live';
import {
	type ServiceAction,
	type ServiceCardProps,
} from './service-card-types';

export type {ServiceCardProps, ServiceAction} from './service-card-types';

/** Owns every service-card behaviour: type-aware routing, lifecycle status flags, per-kind mutations and the derived action menu descriptor list. */
export function useServiceCard({
	projectId,
	type,
	id,
	name,
	status,
	dbKind,
	onDeleted,
}: ServiceCardProps) {
	const navigate = useNavigate();

	const routeConfig = useMemo(() => {
		if (type === 'APP')
			return {
				to: '/projects/$id/app/$appId' as const,
				params: {id: String(projectId), appId: String(id)},
			};
		if (type === 'COMPOSE')
			return {
				to: '/projects/$id/compose/$composeId' as const,
				params: {id: String(projectId), composeId: String(id)},
			};
		return {
			to: '/projects/$id/database/$dbId' as const,
			params: {id: String(projectId), dbId: String(id)},
			search: {kind: dbKind || 'postgres'},
		};
	}, [type, projectId, id, dbKind]);

	const s = status?.toLowerCase() || '';
	const has = (...terms: string[]) => terms.some(t => s.includes(t));
	const isStopping = has('stopping', 'cancelling');
	const isStartingOrDeploying =
		!isStopping && has('starting', 'deploying', 'building', 'loading');
	const isRunning =
		!isStopping &&
		!isStartingOrDeploying &&
		has('running', 'active', 'healthy', 'up');
	const isStopped = !isStopping && !isStartingOrDeploying && !isRunning;

	const app = {
		start: useApplicationStart(),
		stop: useApplicationStop(),
		deploy: useApplicationDeploy(),
		cancel: useApplicationCancel(),
		del: useApplicationDelete(),
	};
	const compose = {
		start: useComposeStart(),
		stop: useComposeStop(),
		deploy: useComposeDeploy(),
		cancel: useComposeCancel(),
		del: useComposeDelete(),
	};
	const dbKindEnum = parseDatabaseKind(dbKind || 'postgres');
	const db = {
		start: useDatabaseStart(),
		stop: useDatabaseStop(),
		deploy: useDatabaseDeploy(),
		cancel: useDatabaseCancel(),
		del: useDatabaseDelete(),
	};

	const run =
		(action: 'start' | 'stop' | 'deploy' | 'cancel', verb: string) =>
		async (e?: React.MouseEvent) => {
			e?.stopPropagation();
			try {
				if (type === 'APP') await app[action].mutateAsync(id);
				else if (type === 'COMPOSE') await compose[action].mutateAsync(id);
				else await db[action].mutateAsync({kind: dbKindEnum, id});
				toast.success(`${verb} ${name}...`);
			} catch (err) {
				toast.error(formatApiError(err));
			}
		};

	const handleStart = run('start', 'Starting');
	const handleStop = run('stop', 'Stopping');
	const handleCancel = run('cancel', 'Cancelling');
	const handleDeploy = run('deploy', 'Deploying');

	const confirmDeleteService = async () => {
		try {
			if (type === 'APP') await app.del.mutateAsync(id);
			else if (type === 'COMPOSE') await compose.del.mutateAsync(id);
			else await db.del.mutateAsync({kind: dbKindEnum, id});
			toast.success(`Deleted ${name}`);
			onDeleted?.();
		} catch (err) {
			toast.error(formatApiError(err));
			throw err;
		}
	};

	const handleNavigate = () => navigate(routeConfig as any);

	const actions: ServiceAction[] = [
		{
			id: 'view',
			label: 'View details',
			Icon: Eye,
			onClick: handleNavigate,
		},
		{
			id: 'start',
			label: 'Start',
			Icon: Play,
			onClick: handleStart,
			disabled: isRunning || isStartingOrDeploying || isStopping,
		},
		isStartingOrDeploying
			? {
					id: 'cancel',
					label: 'Cancel',
					Icon: XCircle,
					onClick: handleCancel,
				}
			: {
					id: 'stop',
					label: 'Stop',
					Icon: Square,
					onClick: handleStop,
					disabled: isStopped || isStopping,
				},
		{
			id: 'deploy',
			label: 'Deploy',
			Icon: Rocket,
			onClick: handleDeploy,
			disabled: isStartingOrDeploying || isStopping,
		},
	];

	return {
		routeConfig,
		isStopping,
		isRunning,
		isStartingOrDeploying,
		isStopped,
		statusKey: s,
		actions,
		confirmDeleteService,
		handleNavigate,
	};
}
