import {createFileRoute} from '@tanstack/react-router';
import {UsersManagementPage} from '#/components/users/users-management-page';

export const Route = createFileRoute('/_app/users')({
	component: UsersPage,
});

function UsersPage() {
	return <UsersManagementPage />;
}
