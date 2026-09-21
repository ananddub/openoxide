import type {ComponentType} from 'react';
import {
	Bell,
	Bot,
	ClipboardList,
	GitBranch,
	User,
	Users,
	ShieldCheck,
} from 'lucide-react';
import {ProfilePage} from '#/components/profile/profile-page';
import {UsersManagementPage} from '#/components/users/users-management-page';
import {GroupsPage} from './groups';
import {AuditLogsSettingsPage} from './audit-logs';
import {GitProvidersPage} from './git-providers';
import {NotificationsPage} from './notifications';
import {AiSettingsPage} from './ai';
import {CertificatesPage} from './certificates';

export type SettingsTabId =
	| 'account'
	| 'team'
	| 'groups'
	| 'audit'
	| 'git'
	| 'notifications'
	| 'ai'
	| 'certificates';

export type SettingsTab = {
	id: SettingsTabId;
	label: string;
	icon: ComponentType<{className?: string; strokeWidth?: number}>;
	component: ComponentType;
	/** Action gate on the active org's grants; owners/admins hold `*`. */
	permission?: string;
};

export const SETTINGS_TABS: SettingsTab[] = [
	{id: 'account', label: 'Account', icon: User, component: ProfilePage},
	{
		id: 'team',
		label: 'Team',
		icon: Users,
		component: UsersManagementPage,
		permission: 'member:read',
	},
	{
		id: 'groups',
		label: 'Permission Groups',
		icon: ShieldCheck,
		component: GroupsPage,
		permission: 'group:read',
	},
	{
		id: 'audit',
		label: 'Audit Log',
		icon: ClipboardList,
		component: AuditLogsSettingsPage,
		permission: 'audit_log:read',
	},
	{
		id: 'git',
		label: 'Git Providers',
		icon: GitBranch,
		component: GitProvidersPage,
	},
	{
		id: 'notifications',
		label: 'Notifications',
		icon: Bell,
		component: NotificationsPage,
	},
	{id: 'ai', label: 'AI', icon: Bot, component: AiSettingsPage},
	{
		id: 'certificates',
		label: 'Certificates',
		icon: ShieldCheck,
		component: CertificatesPage,
	},
];

export function isSettingsTab(value: unknown): value is SettingsTabId {
	return (
		typeof value === 'string' &&
		SETTINGS_TABS.some(tab => tab.id === value)
	);
}
