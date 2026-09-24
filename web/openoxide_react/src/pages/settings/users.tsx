import {createFileRoute, Navigate} from '@tanstack/react-router';
import {UsersManagementPage} from '#/components/users/users-management-page';

export const Route = createFileRoute('/_app/settings/users')({
	// Team management lives in the /settings tab shell now; keep old links working by redirecting into it.
	component: () => (
		<Navigate to="/settings" search={{tab: 'team'}} replace />
	),
});

export function UsersSettingsPage() {
	return <UsersManagementPage />;
}
