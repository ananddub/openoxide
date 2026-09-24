import {useEffect} from 'react';
import {useNavigate, useSearch} from '@tanstack/react-router';
import {useDeployments} from './use-deployments';

/** Page-level state for /Deployments (Logs & History): the data hook plus the ?id= deep link that opens a deployment's log stream. */
export function useDeploymentsPage() {
	const dp = useDeployments();
	const navigate = useNavigate();
	const {id: deepLinkId} = useSearch({from: '/_app/Deployments'});

	// Deep link: /Deployments?id=<id> opens that deployment's log stream and clears the param, so a refresh doesn't re-open the dialog.
	useEffect(() => {
		if (!deepLinkId) return;
		const match = dp.deployments.find(d => String(d.id) === deepLinkId);
		if (match) dp.setSelectedDeployment(match);
		void navigate({to: '/Deployments', search: {}, replace: true});
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [deepLinkId, dp.deployments, navigate]);

	return dp;
}
