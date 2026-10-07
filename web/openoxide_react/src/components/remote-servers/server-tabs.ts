import {
	Activity,
	Archive,
	Container,
	Globe,
	HardDrive,
	Image,
	LayoutGrid,
	Network,
	Radar,
	Settings,
	Terminal as TerminalIcon,
} from 'lucide-react';

/** Tabs of one remote server: the sidebar rail, the titlebar switch and the route search param all read this list. */
export const SERVER_TABS = [
	{id: 'overview', label: 'Overview', icon: LayoutGrid},
	{id: 'network', label: 'Private Network', icon: Network},
	{id: 'containers', label: 'Containers', icon: Container},
	{id: 'images', label: 'Images', icon: Image},
	{id: 'volumes', label: 'Volumes', icon: Archive},
	{id: 'networks', label: 'Networks', icon: Globe},
	{id: 'disk', label: 'Disk Usage', icon: HardDrive},
	{id: 'agent', label: 'Agent', icon: Radar},
	{id: 'management', label: 'Actions', icon: Activity},
	{id: 'settings', label: 'Settings', icon: Settings},
	{id: 'terminal', label: 'Terminal', icon: TerminalIcon},
] as const;

export type ServerTabId = (typeof SERVER_TABS)[number]['id'];

export const SERVER_TAB_IDS: readonly string[] = SERVER_TABS.map(
	tab => tab.id,
);

/** Segmented labels stay short where the rail label would overflow the switch. */
export const SERVER_TAB_SWITCH_LABELS: Record<ServerTabId, string> = {
	overview: 'Overview',
	network: 'Network',
	containers: 'Containers',
	images: 'Images',
	volumes: 'Volumes',
	networks: 'Networks',
	disk: 'Disk',
	agent: 'Agent',
	management: 'Actions',
	settings: 'Settings',
	terminal: 'Terminal',
};

export function isServerTabId(value: unknown): value is ServerTabId {
	return typeof value === 'string' && SERVER_TAB_IDS.includes(value);
}
