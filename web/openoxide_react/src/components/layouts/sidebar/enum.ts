import {
	House,
	Folder,
	// BarChartHorizontal — retired with the global Monitoring page (per-server overview carries it now)
	Clock,
	GalleryVerticalEnd,
	// Blocks, // used by disabled Docker sidebar entry
	Rocket,
	PieChart,
	Forward,
	Activity,
	Server,
	User,
	Users,
	ShieldCheck,
	ClipboardList,
	GitBranch,
	Bell,
	Bot,
	FileKey,
	KeyRound,
	Tags,
	Database,
	Package,
	// BookIcon, CircleHelp — removed with the sidebar Extra/help section
	// Globe, // used by disabled DNS Providers sidebar entry
} from 'lucide-react';
import type {LucideIcon} from 'lucide-react';

// A leaf nav item with a direct route link.
export type SingleNavItem = {
	isSingle?: true;
	title: string;
	to: string;
	icon: LucideIcon;
	// Optional action gate ("group:read"); hidden when the active
	// organization's grants lack it. Owners/admins hold the * wildcard.
	permission?: string;
};

// Nav item — either a single leaf or a collapsible group with children.
export type NavItem =
	| SingleNavItem
	| {
			isSingle: false;
			title: string;
			icon: LucideIcon;
			items: SingleNavItem[];
			to: string;
	  };

/**
 * Nav structure: a short "main" work section, host/proxy surfaces, the full
 * Settings surface (every /settings?tab= entry lives here so the section is
 * reachable without opening the page first), and resource workspaces.
 * Empty sections filter out at render time, so permission-gated rows
 * collapse cleanly.
 */
export type Menu = {
	main: NavItem[];
	infrastructure: NavItem[];
	settings: NavItem[];
	resources: NavItem[];
};

export const MENU: Menu = {
	main: [
		{title: 'Overview', icon: House, to: '/'},
		{title: 'Projects', icon: Folder, to: '/projects'},
		{title: 'Deployments', icon: Rocket, to: '/Deployments'},
		{title: 'Schedules', icon: Clock, to: '/schedules'},
	],
	infrastructure: [
		{title: 'Remote Servers', icon: Server, to: '/remote-servers'},
		// Web Server / Traefik / Swarm / Requests are underline tabs of one
		// page now (pages/infra.tsx); ?tab= deep-links from the search palette.
		{title: 'Web Server', icon: Activity, to: '/infra'},
		// Disabled: host-wide Docker view leaks org-unmanaged containers into org context.
		// Re-enable (or move under Remote Servers) when decided.
		// {title: 'Docker', icon: Blocks, to: '/docker'},
	],
	settings: [
		// The full /settings surface, one row per URL tab. Permissions mirror
		// the page's SETTINGS_TABS gates so the rail and the page agree.
		{title: 'Account', icon: User, to: '/settings?tab=account'},
		{
			title: 'Team',
			icon: Users,
			to: '/settings?tab=team',
			permission: 'member:read',
		},
		{
			title: 'Permission Groups',
			icon: ShieldCheck,
			to: '/settings?tab=groups',
			permission: 'group:read',
		},
		{
			title: 'Audit Log',
			icon: ClipboardList,
			to: '/settings?tab=audit',
			permission: 'audit_log:read',
		},
		{title: 'Git Providers', icon: GitBranch, to: '/settings?tab=git'},
		{
			title: 'Notifications',
			icon: Bell,
			to: '/settings?tab=notifications',
		},
		{title: 'AI', icon: Bot, to: '/settings?tab=ai'},
		{
			title: 'Certificates',
			icon: FileKey,
			to: '/settings?tab=certificates',
		},
	],
	resources: [
		// Resource workspaces — not settings, so they stay their own section.
		{title: 'SSH Keys', icon: KeyRound, to: '/ssh-keys'},
		{title: 'Tags', icon: Tags, to: '/tags'},
		{title: 'Registry', icon: Package, to: '/registry'},
		{title: 'Vault Providers', icon: KeyRound, to: '/vault'},
		// Disabled: DNS Providers (page kept on disk for future re-enable).
		// {title: 'DNS Providers', icon: Globe, to: '/dns'},
		{title: 'S3 Destinations', icon: Database, to: '/destinations'},
	],
};

export type RouteItem = {
	label: string;
	path: string;
	icon: LucideIcon;
	group: string;
};

/**
 * ROUTES is derived from MENU — single source of truth.
 * Used by SearchDialog for command palette navigation.
 */
export const ROUTES: RouteItem[] = [
	...MENU.main.map(item => ({
		label: item.title,
		path: item.to,
		icon: item.icon,
		group: 'Main',
	})),
	...MENU.infrastructure.map(item => ({
		label: item.title,
		path: item.to,
		icon: item.icon,
		group: 'Infrastructure',
	})),
	// The consolidated infra page's other tabs — reachable from the palette
	// even though the sidebar shows only the "Web Server" entry.
	{
		label: 'Traefik',
		path: '/infra?tab=traefik',
		icon: GalleryVerticalEnd,
		group: 'Infrastructure',
	},
	{
		label: 'Swarm',
		path: '/infra?tab=swarm',
		icon: PieChart,
		group: 'Infrastructure',
	},
	{
		label: 'Requests',
		path: '/infra?tab=requests',
		icon: Forward,
		group: 'Infrastructure',
	},
	...MENU.settings.map(item => ({
		label: item.title,
		path: item.to,
		icon: item.icon,
		group: 'Settings',
	})),
	...MENU.resources.map(item => ({
		label: item.title,
		path: item.to,
		icon: item.icon,
		group: 'Resources',
	})),
];
