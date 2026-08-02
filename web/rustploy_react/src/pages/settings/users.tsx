import {createFileRoute} from '@tanstack/react-router';
import {UsersManagementPage} from '#/components/users/users-management-page';

export const Route = createFileRoute('/_app/settings/users')({
	component: UsersSettingsPage,
});

function UsersSettingsPage() {
	return <UsersManagementPage />;
}
