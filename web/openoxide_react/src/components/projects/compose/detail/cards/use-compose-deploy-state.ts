import {useEffect, useMemo, useState} from 'react';
import {useMutationState} from '@tanstack/react-query';
import {useDeploymentItems} from '#/api/live';

export type ComposeActionType =
	| 'deploy'
	| 'reload'
	| 'rebuild'
	| 'start'
	| 'stop'
	| 'cancel';

const ACTIVE_STATUSES = [
	'QUEUED',
	'RUNNING',
	'BUILDING',
	'STARTING',
	'DEPLOYING',
	'PREPARING',
	'PENDING',
];

/** Derives compose deployment/lifecycle state from the live deployments feed plus any in-flight TanStack mutations, and mirrors the backend transition by clearing the local `actionInFlight` flag once it is observable. */
export function useComposeDeployState(compose: any) {
	const [actionInFlight, setActionInFlight] =
		useState<ComposeActionType | null>(null);

	const {data: storeDeployments = []} = useDeploymentItems();
	const events = useMemo(
		() =>
			storeDeployments.filter(
				(d: any) => String(d.compose_id) === String(compose?.id),
			),
		[storeDeployments, compose?.id],
	);

	const deploymentRevision = useMemo(
		() =>
			events
				.map(
					(e: any) =>
						`${e.id}:${e.status}:${e.state}:${e.last_state_at ?? ''}`,
				)
				.join('|'),
		[events],
	);

	const pendingMutationAction = useMutationState({
		filters: {status: 'pending'},
		select: mutation =>
			(mutation.state.variables as any)?.actionName as
				| ComposeActionType
				| undefined,
	})[0];

	const hasActiveDeployment = events.some((e: any) => {
		if (!e || (e.finished_at && Number(e.finished_at) > 0)) return false;
		return ACTIVE_STATUSES.includes(
			(e.status || e.state || '').toUpperCase(),
		);
	});

	const rawStatus = (
		compose?.compose_status ||
		compose?.status ||
		''
	).toLowerCase();
	const isRunning = [
		'running',
		'deployed',
		'done',
		'success',
		'active',
		'ok',
	].includes(rawStatus);
	const isBuilding =
		!!actionInFlight ||
		!!pendingMutationAction ||
		hasActiveDeployment ||
		['starting', 'building', 'queued', 'preparing'].includes(rawStatus);

	useEffect(() => {
		if (
			hasActiveDeployment ||
			['running', 'starting', 'error'].includes(rawStatus)
		) {
			setActionInFlight(null);
		}
	}, [hasActiveDeployment, rawStatus]);

	useEffect(() => {
		console.debug('[Compose Deployments Live]', {
			composeId: compose?.id,
			count: events.length,
		});
	}, [compose?.id, deploymentRevision]);

	return {
		actionInFlight,
		setActionInFlight,
		pendingMutationAction,
		isBuilding,
		isRunning,
	};
}
