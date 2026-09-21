import {
	House,
	// LayoutDashboard, // used by disabled Overview sidebar entry
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
	// User, Users, ClipboardList — moved into /settings tabs
	KeyRound,
	Settings,
	Tags,
	// GitBranch, Bot, ShieldCheck, Bell — moved into /settings tabs
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

// (External help links section removed — the rail carries navigation only.)

/**
 * Nav structure mirrors the OpenShip dashboard rail: a short "main" work
 * section, an "infrastructure" section for host/proxy surfaces, and a long
 * "settings" section for everything else. Empty sections filter out at
 * render time, so permission-gated rows collapse cleanly.
 */
export type Menu = {
	main: NavItem[];
	infrastructure: NavItem[];
	settings: NavItem[];
};

export const MENU: Menu = {
	main: [
		{title: 'Home', icon: House, to: '/'},
		{title: 'Projects', icon: Folder, to: '/projects'},
		// Disabled: Overview duplicates the home dashboard; kept on disk for re-enable.
		// {title: 'Overview', icon: LayoutDashboard, to: '/overview'},
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
		// Settings is one surface now: /settings carries Account/Team/Groups/
		// Audit/Git/Notifications/AI/Certificates as URL tabs with a right rail
		// (OpenShip pattern). The old Organization placeholder page is gone —
		// the active workspace lives in the sidebar switcher. Resource tools
		// below stay top-level: they are workspaces, not settings.
		{title: 'Settings', icon: Settings, to: '/settings'},
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
];
