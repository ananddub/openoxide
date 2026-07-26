import {
	House,
	FolderKanban,
	Zap,
	Activity,
	Clock,
	Globe,
	Container,
	Network,
	Link,
	Cpu,
	User,
	Server,
	Users,
	ClipboardList,
	KeyRound,
	Settings,
	Tag,
	GitBranch,
	Box,
	HardDrive,
	ShieldCheck,
	Bell,
	BookIcon,
	CircleHelp,
} from 'lucide-react';
import type {LucideIcon} from 'lucide-react';

// A leaf nav item with a direct route link.
export type SingleNavItem = {
	isSingle?: true;
	title: string;
	to: string;
	icon: LucideIcon;
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

// External link item used in the Help/Extra group.
export type ExternalLink = {
	title: string;
	href: string;
	icon: LucideIcon;
};

// Top-level menu structure split into platform, settings, and help sections.
export type Menu = {
	platform: NavItem[];
	settings: NavItem[];
	help: ExternalLink[];
};

export const MENU: Menu = {
	platform: [
		{title: 'Home', icon: House, to: '/'},
		{title: 'Projects', icon: FolderKanban, to: '/projects'},
		{title: 'Deployments', icon: Zap, to: '/Deployments'},
		{title: 'Monitoring', icon: Activity, to: '/monitoring'},
		{title: 'Schedules', icon: Clock, to: '/schedules'},
		{title: 'Traefik', icon: Globe, to: '/traefik'},
		{title: 'Docker', icon: Container, to: '/docker'},
		{title: 'Swarm', icon: Network, to: '/swarm'},
		{title: 'Requests', icon: Link, to: '/requests'},
	],
	settings: [
		{title: 'Web Server', icon: Cpu, to: '/settings/server'},
		{title: 'Profile', icon: User, to: '/settings/profile'},
		{title: 'Remote Servers', icon: Server, to: '/remote-servers'},
		{title: 'Users', icon: Users, to: '/settings/users'},
		{title: 'Audit Logs', icon: ClipboardList, to: '/settings/audit-logs'},
		{title: 'SSH Keys', icon: KeyRound, to: '/ssh-keys'},
		{title: 'Organization', icon: Settings, to: '/settings'},
		{title: 'Tags', icon: Tag, to: '/settings/tags'},
		{
			title: 'Git Providers',
			icon: GitBranch,
			to: '/settings/git-providers',
		},
		{title: 'Registry', icon: Box, to: '/registry'},
		{title: 'S3 Destinations', icon: HardDrive, to: '/destinations'},
		{title: 'Certificates', icon: ShieldCheck, to: '/settings/certificates'},
		{title: 'Notifications', icon: Bell, to: '/settings/notifications'},
	],
	help: [
		{
			title: 'Documentation',
			href: 'https://github.com/vajra-labs/goploy',
			icon: BookIcon,
		},
		{
			title: 'Support',
			href: 'https://github.com/vajra-labs/goploy/issues',
			icon: CircleHelp,
		},
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
	...MENU.platform.map(item => ({
		label: item.title,
		path: item.to,
		icon: item.icon,
		group: 'Navigation',
	})),
	...MENU.settings.map(item => ({
		label: item.title,
		path: item.to,
		icon: item.icon,
		group: 'Settings',
	})),
];
