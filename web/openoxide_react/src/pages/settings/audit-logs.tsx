import {createFileRoute, Navigate} from '@tanstack/react-router';
import {AuditLogPage} from '#/components/settings/audit/audit-log-page';

export const Route = createFileRoute('/_app/settings/audit-logs')({
	// The audit feed lives in the /settings tab shell now; old links redirect.
	component: () => (
		<Navigate to="/settings" search={{tab: 'audit'}} replace />
	),
});

export function AuditLogsSettingsPage() {
	return <AuditLogPage />;
}
