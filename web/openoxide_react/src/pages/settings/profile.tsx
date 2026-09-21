import {createFileRoute, Navigate} from '@tanstack/react-router';

export const Route = createFileRoute('/_app/settings/profile')({
	component: () => (
		<Navigate to="/settings" search={{tab: 'account'}} replace />
	),
});
