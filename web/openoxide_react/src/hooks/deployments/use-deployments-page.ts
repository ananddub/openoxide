import {useEffect, useMemo, useState} from 'react';
import {useNavigate, useSearch} from '@tanstack/react-router';
import {useDeployments} from './use-deployments';
import type {DeploymentsTabId} from '#/components/deployments/deployments-tabs';

/**
 * Page-level state for /Deployments: the data hook plus tab routing,
 * fleet stats, the ?id= deep link and the cancel-build confirmation.
 */
export function useDeploymentsPage() {
	const dp = useDeployments();
	const [cancelingId, setCancelingId] = useState<number | null>(null);
	const navigate = useNavigate();
	const {id: deepLinkId, tab: tabParam} = useSearch({
		from: '/_app/Deployments',
	});
	const active: DeploymentsTabId = tabParam ?? 'history';

	const select = (id: DeploymentsTabId) =>
		void navigate({
			to: '/Deployments',
			search: {tab: id === 'history' ? undefined : id},
			replace: true,
		});

	// OpenShip-style fleet stats from the full deployment list.
	const stats = useMemo(() => {
		let success = 0;
		let failed = 0;
		let inProgress = 0;
		for (const d of dp.deployments) {
			const s = (d.status || '').toUpperCase();
			if (['DONE', 'SUCCESS', 'DEPLOYED', 'HEALTHY'].includes(s))
				success++;
			else if (['ERROR', 'FAILED', 'CRASHED', 'CANCELLED'].includes(s))
				failed++;
			else if (d && !['IDLE', 'STOPPED', 'STOPPING'].includes(s))
				inProgress++;
		}
		return {
			total: dp.deployments.length,
			success,
			failed,
			inProgress,
		};
	}, [dp.deployments]);

	// Deep link: /Deployments?id=<id> opens that deployment's log stream and
	// clears the param, so a refresh doesn't re-open the dialog.
	useEffect(() => {
		if (!deepLinkId) return;
		const match = dp.deployments.find(d => String(d.id) === deepLinkId);
		if (match) dp.setSelectedDeployment(match);
		void navigate({to: '/Deployments', search: {}, replace: true});
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [deepLinkId, dp.deployments, navigate]);

	const confirmCancel = async () => {
		if (cancelingId !== null) {
			await dp.handleCancelDeployment(cancelingId);
			setCancelingId(null);
		}
	};

	return {
		dp,
		active,
		select,
		stats,
		cancelingId,
		setCancelingId,
		confirmCancel,
	};
}
