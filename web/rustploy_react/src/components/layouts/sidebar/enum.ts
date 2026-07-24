import {
	House,
	FolderOpen,
	Zap,
	ChartLine,
	Calendar,
	Globe,
	Package,
	Globe2,
	Link,
	Cpu,
	User,
	Server,
	Users,
	FileText,
	Key,
	Settings,
	Tag,
	GitBranch,
	Database,
	Layers,
	Shield,
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
		{title: 'Projects', icon: FolderOpen, to: '/projects'},
		{title: 'Deployments', icon: Zap, to: '/Deployments'},
		{title: 'Monitoring', icon: ChartLine, to: '/monitoring'},
		{title: 'Schedules', icon: Calendar, to: '/schedules'},
		{title: 'Traefik', icon: Globe, to: '/traefik'},
		{title: 'Docker', icon: Package, to: '/docker'},
		{title: 'Swarm', icon: Globe2, to: '/swarm'},
		{title: 'Requests', icon: Link, to: '/requests'},
	],
	settings: [
		{title: 'Web Server', icon: Cpu, to: '/settings/server'},
		{title: 'Profile', icon: User, to: '/settings/profile'},
		{title: 'Servers', icon: Server, to: '/settings/servers'},
		{title: 'Remote Servers', icon: Cpu, to: '/remote-servers'},
		{title: 'Users', icon: Users, to: '/settings/users'},
		{title: 'Audit Logs', icon: FileText, to: '/settings/audit-logs'},
		{title: 'SSH Keys', icon: Key, to: '/ssh-keys'},
		{title: 'Organization', icon: Settings, to: '/settings'},
		{title: 'Tags', icon: Tag, to: '/settings/tags'},
		{
			title: 'Git Providers',
			icon: GitBranch,
			to: '/settings/git-providers',
		},
		{title: 'Registry', icon: Database, to: '/settings/registry'},
		{title: 'S3 Destinations', icon: Layers, to: '/settings/destinations'},
		{title: 'Certificates', icon: Shield, to: '/settings/certificates'},
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
