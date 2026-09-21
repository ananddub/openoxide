import {createFileRoute, useNavigate, useSearch} from '@tanstack/react-router';
import type {ComponentType} from 'react';
import {
	Bell,
	Bot,
	ClipboardList,
	GitBranch,
	Settings as SettingsIcon,
	ShieldCheck,
	User,
	Users,
} from 'lucide-react';
import {usePermissions} from '#/hooks/use-permissions';
import {useAuthSession} from '#/stores/session-context';
import {ProfilePage} from '#/components/profile/profile-page';
import {UsersManagementPage} from '#/components/users/users-management-page';
import {GroupsPage} from './groups';
import {AuditLogsSettingsPage} from './audit-logs';
import {GitProvidersPage} from './git-providers';
import {NotificationsPage} from './notifications';
import {AiSettingsPage} from './ai';
import {CertificatesPage} from './certificates';

/**
 * Settings — Openship's layout: primary content on the left, a sticky
 * tab rail on the right (`lg:grid-cols-[1fr_280px]`), URL-driven via
 * `?tab=` so every section deep-links. Below `lg` the rail becomes a
 * horizontal pill strip above the content.
 *
 * The rail is OpenShip's exact visual language: an identity card (icon
 * tile + title + account email) above a quiet card of rounded pill rows
 * — inline icon, label only, active row a soft foreground wash. No
 * dividers, no boxed icons, no per-row subtitles.
 *
 * Resource-management surfaces (SSH Keys, Tags, Registry, Vault, S3,
 * Web Server) stay top-level in the sidebar — they are workspaces, not
 * settings.
 */

type SettingsTabId =
	| 'account'
	| 'team'
	| 'groups'
	| 'audit'
	| 'git'
	| 'notifications'
	| 'ai'
	| 'certificates';

type SettingsTab = {
	id: SettingsTabId;
	label: string;
	icon: ComponentType<{className?: string; strokeWidth?: number}>;
	component: ComponentType;
	/** Action gate on the active org's grants; owners/admins hold `*`. */
	permission?: string;
};

export const Route = createFileRoute('/_app/settings')({
	component: SettingsPage,
	validateSearch: (
		search: Record<string, unknown>,
	): {tab?: SettingsTabId} => ({
		tab: isSettingsTab(search.tab) ? search.tab : undefined,
	}),
});

const SETTINGS_TABS: SettingsTab[] = [
	{
		id: 'account',
		label: 'Account',
		icon: User,
		component: ProfilePage,
	},
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
	{
		id: 'ai',
		label: 'AI',
		icon: Bot,
		component: AiSettingsPage,
	},
	{
		id: 'certificates',
		label: 'Certificates',
		icon: ShieldCheck,
		component: CertificatesPage,
	},
];

function isSettingsTab(value: unknown): value is SettingsTabId {
	return (
		typeof value === 'string' &&
		SETTINGS_TABS.some(tab => tab.id === value)
	);
}

function SettingsPage() {
	const {tab} = useSearch({from: '/_app/settings'});
	const navigate = useNavigate();
	const {can} = usePermissions();
	const {user} = useAuthSession();

	const visibleTabs = SETTINGS_TABS.filter(
		t => !t.permission || can(t.permission),
	);
	const active = visibleTabs.find(t => t.id === tab) ?? visibleTabs[0];
	const Active = active.component;

	const select = (id: SettingsTabId) =>
		void navigate({
			to: '/settings',
			search: {tab: id === visibleTabs[0].id ? undefined : id},
			replace: true,
		});

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			<div className="mb-2 flex flex-col gap-1">
				<h1 className="text-2xl font-medium tracking-tight text-foreground/80">
					Settings
				</h1>
				<p className="text-sm text-muted-foreground/70">
					Account, team, and platform preferences
				</p>
			</div>

			{/* Mobile: pill strip (OpenShip SettingsMobileTabs) */}
			<div className="overflow-x-auto lg:hidden">
				<div className="inline-flex items-center gap-1 rounded-xl bg-card p-1">
					{visibleTabs.map(t => (
						<button
							key={t.id}
							type="button"
							onClick={() => select(t.id)}
							className={`inline-flex items-center gap-2 whitespace-nowrap rounded-lg px-3 py-2 text-sm font-medium transition-colors ${
								t.id === active.id
									? 'bg-foreground/[0.07] text-foreground'
									: 'text-muted-foreground hover:bg-foreground/[0.04] hover:text-foreground'
							}`}>
							<t.icon className="size-[15px]" strokeWidth={1.7} />
							{t.label}
						</button>
					))}
				</div>
			</div>

			<div className="grid min-w-0 grid-cols-1 gap-6 lg:grid-cols-[1fr_280px]">
				{/* Active tab content (left, primary) */}
				<div className="min-w-0">
					<Active />
				</div>

				{/* Rail (right, sticky): identity card + pill nav card */}
				<aside className="hidden space-y-3 lg:sticky lg:top-6 lg:block lg:self-start">
					<div className="rounded-2xl bg-card p-4">
						<div className="flex items-center gap-3">
							<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted">
								<SettingsIcon
									className="size-4 text-foreground"
									strokeWidth={1.7}
								/>
							</div>
							<div className="min-w-0 flex-1">
								<p className="truncate text-sm font-medium text-foreground">
									Settings
								</p>
								{user?.email && (
									<p className="truncate text-xs text-muted-foreground">
										{user.email}
									</p>
								)}
							</div>
						</div>
					</div>

					<div className="rounded-2xl bg-card p-3">
						<div className="space-y-1">
							{visibleTabs.map(t => {
								const isActive = t.id === active.id;
								return (
									<button
										key={t.id}
										type="button"
										onClick={() => select(t.id)}
										className={`flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-[14px] font-medium transition-colors ${
											isActive
												? 'bg-foreground/[0.07] text-foreground'
												: 'text-muted-foreground hover:bg-foreground/[0.04] hover:text-foreground'
										}`}>
										<t.icon
											className="size-[17px] shrink-0"
											strokeWidth={1.7}
										/>
										{t.label}
									</button>
								);
							})}
						</div>
					</div>
				</aside>
			</div>
		</div>
	);
}
