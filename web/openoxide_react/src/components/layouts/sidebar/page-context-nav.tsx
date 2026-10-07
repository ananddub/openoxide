import {useLocation, useNavigate} from '@tanstack/react-router';
import {
	Activity,
	Archive,
	ChevronLeft,
	Clock,
	Forward,
	GalleryVerticalEnd,
	Globe,
	HardDrive,
	Network,
	Pencil,
	PieChart,
	Rocket,
	ScrollText,
	Server,
	Settings2,
	Trash2,
	Wrench,
	type LucideIcon,
} from 'lucide-react';
import {useState} from 'react';
import {cn} from '#/api/utils';
import {useRemoteServerGet, useSshKeyItems} from '#/api/live';
import {CreateServerModal} from '#/components/remote-servers/create-server-modal';
import {DeleteServerModal} from '#/components/remote-servers/delete-server-modal';
import {ServerSettingsPanel} from '#/components/remote-servers/server-settings-panel';
import {SERVER_TABS} from '#/components/remote-servers/server-tabs';
import type {
	RemoteServerResponse,
	SshKeyResponse,
} from '#/types/api-helpers';
import {
	SIDEBAR_CENTER_X,
	SIDEBAR_COLLAPSE,
	SIDEBAR_ICON_CELL,
	SIDEBAR_LABEL,
	SIDEBAR_ROW,
	SidebarGroup,
	SidebarMenu,
	SidebarMenuItem,
	SidebarSeparator,
} from '#/components/ui/sidebar';
import {useCan} from '#/hooks/permissions/use-permissions';
import {useUrlTab} from '#/hooks/ui/use-url-tab';
import {SETTINGS_TABS} from '#/pages/settings/settings-tabs';

/** Page-aware rail context: the nav swaps to context rows ONLY inside a service detail page (or settings) — plain project pages keep the global menu. The header is the Vercel "< Title" row: a thin chevron plus the centered context name; the whole header is the back control. */

const TAB_ICONS: Record<string, LucideIcon> = {
	General: Settings2,
	Architecture: Network,
	Domains: Globe,
	Deployments: Rocket,
	Schedules: Clock,
	Backups: Archive,
	'Volume Backups': HardDrive,
	Logs: ScrollText,
	Advanced: Wrench,
};

/** Tab lists per detail route family (matches the pages' TABS arrays). */
const COMPOSE_TABS = [
	'General',
	'Architecture',
	'Domains',
	'Deployments',
	'Schedules',
	'Backups',
	'Logs',
	'Advanced',
] as const;

const APP_TABS = [
	'General',
	'Architecture',
	'Domains',
	'Deployments',
	'Volume Backups',
	'Logs',
	'Schedules',
	'Advanced',
] as const;

const DATABASE_BASE_TABS = [
	'General',
	'Architecture',
	'Deployments',
	'Logs',
	'Advanced',
] as const;

function databaseTabs(kind: string | undefined): string[] {
	return kind?.toLowerCase() === 'redis'
		? [...DATABASE_BASE_TABS]
		: [
				...DATABASE_BASE_TABS.slice(0, 3),
				'Backups',
				...DATABASE_BASE_TABS.slice(3),
			];
}

type PageContext =
	| {kind: 'settings'}
	| {kind: 'infra'}
	| {kind: 'deployments'}
	| {kind: 'servers'}
	| {kind: 'server'; id: string}
	| {
			kind: 'service';
			projectId: number;
			tabs: readonly string[];
			label: string;
	  };

/** Resolve which page-context the current URL belongs to. Project
 * detail pages (/projects/:id) keep the global menu — only service
 * detail pages own the rail. */
function resolveContext(
	pathname: string,
	search: Record<string, unknown>,
): PageContext | null {
	const seg = pathname.split('/').filter(Boolean);

	// /settings?tab=… — the settings surface owns the rail.
	if (seg[0] === 'settings') return {kind: 'settings'};

	// /infra?tab=… — Web Server / Traefik / Swarm / Requests own the rail.
	if (seg[0] === 'infra') return {kind: 'infra'};

	// /Deployments and /Deployments/queue — the two views own the rail.
	if (seg[0] === 'Deployments') return {kind: 'deployments'};

	// /remote-servers (+ /new) show the list rail; /remote-servers/:id the server rail.
	if (seg[0] === 'remote-servers') {
		if (seg.length === 1 || seg[1] === 'new') return {kind: 'servers'};
		if (seg[1] && /^\d+$/.test(seg[1]))
			return {kind: 'server', id: seg[1]};
		return {kind: 'servers'};
	}

	// Only /projects/:id/family/:resourceId (service detail) — NOT the plain project page.
	if (seg[0] !== 'projects' || !seg[1] || seg.length < 4) return null;
	const projectId = Number(seg[1]);
	if (Number.isNaN(projectId)) return null;

	switch (seg[2]) {
		case 'app':
			return {
				kind: 'service',
				projectId,
				tabs: APP_TABS,
				label: 'Service',
			};
		case 'compose':
			return {
				kind: 'service',
				projectId,
				tabs: COMPOSE_TABS,
				label: 'Compose',
			};
		case 'database':
			return {
				kind: 'service',
				projectId,
				tabs: databaseTabs(search.kind as string | undefined),
				label: 'Database',
			};
	}
	return null;
}

/** Context header — the whole bar is the back control: chevron cell + centered title. */
function ContextHeader({
	label,
	collapsed,
	onBack,
}: {
	label: string;
	collapsed: boolean;
	onBack: () => void;
}) {
	return (
		<button
			type="button"
			onClick={onBack}
			title={collapsed ? 'Back' : undefined}
			aria-label={`Back — ${label}`}
			className={cn(
				SIDEBAR_ROW,
				SIDEBAR_COLLAPSE,
				'focus-visible:ring-2 focus-visible:ring-ring/40',
				collapsed && SIDEBAR_CENTER_X,
			)}>
			<span className={SIDEBAR_ICON_CELL}>
				<ChevronLeft
					className="size-4 text-muted-foreground"
					strokeWidth={2}
				/>
			</span>
			<span
				className={cn(
					SIDEBAR_LABEL,
					'text-center text-sm font-semibold text-foreground',
					collapsed ? 'opacity-0' : 'pr-9',
				)}>
				{label}
			</span>
		</button>
	);
}

/** Shared row grammar — matches the global nav rows. */
function ContextRow({
	icon: Icon,
	label,
	active,
	collapsed,
	onClick,
}: {
	icon: React.ComponentType<{className?: string; strokeWidth?: number}>;
	label: string;
	active: boolean;
	collapsed: boolean;
	onClick: () => void;
}) {
	return (
		<SidebarMenuItem>
			<button
				type="button"
				onClick={onClick}
				title={collapsed ? label : undefined}
				className={cn(
					SIDEBAR_ROW,
					SIDEBAR_COLLAPSE,
					collapsed && SIDEBAR_CENTER_X,
					active
						? 'bg-sidebar-accent text-foreground'
						: 'text-muted-foreground',
				)}>
				<span className={SIDEBAR_ICON_CELL}>
					<Icon className="size-4" strokeWidth={1.7} />
				</span>
				<span className={cn(SIDEBAR_LABEL, collapsed && 'opacity-0')}>
					{label}
				</span>
			</button>
		</SidebarMenuItem>
	);
}

/* ── Settings context ──────────────────────────────────────────────── */

function SettingsContextNav({collapsed}: {collapsed: boolean}) {
	const location = useLocation();
	const navigate = useNavigate();
	const can = useCan();
	const currentTab =
		typeof location.search === 'object' && location.search
			? (location.search as Record<string, string>).tab
			: undefined;

	const tabs = SETTINGS_TABS.filter(
		tab => !tab.permission || can(tab.permission),
	);

	return (
		<SidebarGroup className="gap-1 px-2 py-1">
			<ContextHeader
				label="Settings"
				collapsed={collapsed}
				onBack={() => void navigate({to: '/'})}
			/>
			<SidebarMenu className="gap-1">
				{tabs.map(tab => {
					const active = (currentTab ?? 'account') === tab.id;
					return (
						<ContextRow
							key={tab.id}
							icon={tab.icon}
							label={tab.label}
							active={active}
							collapsed={collapsed}
							onClick={() =>
								void navigate({
									to: '/settings',
									search: {tab: tab.id} as never,
								})
							}
						/>
					);
				})}
			</SidebarMenu>
		</SidebarGroup>
	);
}

/* ── Infra (Web Server) context ────────────────────────────────────── */

const INFRA_NAV = [
	{id: 'web-server', label: 'Web Server', icon: Activity},
	{id: 'traefik', label: 'Traefik', icon: GalleryVerticalEnd},
	{id: 'swarm', label: 'Swarm', icon: PieChart},
	{id: 'requests', label: 'Requests', icon: Forward},
] as const;

function InfraContextNav({collapsed}: {collapsed: boolean}) {
	const location = useLocation();
	const navigate = useNavigate();
	const currentTab =
		typeof location.search === 'object' && location.search
			? (location.search as Record<string, string>).tab
			: undefined;

	return (
		<SidebarGroup className="gap-1 px-2 py-1">
			<ContextHeader
				label="Web Server"
				collapsed={collapsed}
				onBack={() => void navigate({to: '/'})}
			/>
			<SidebarMenu className="gap-1">
				{INFRA_NAV.map(item => {
					const active = (currentTab ?? 'web-server') === item.id;
					return (
						<ContextRow
							key={item.id}
							icon={item.icon}
							label={item.label}
							active={active}
							collapsed={collapsed}
							onClick={() =>
								void navigate({
									to: '/infra',
									search: {tab: item.id} as never,
								})
							}
						/>
					);
				})}
			</SidebarMenu>
		</SidebarGroup>
	);
}

/* ── Remote Servers context ────────────────────────────────────────── */

const SERVERS_NAV = [
	{
		id: 'list',
		label: 'All Servers',
		icon: Server,
		to: '/remote-servers',
	},
] as const;

function ServersContextNav({collapsed}: {collapsed: boolean}) {
	const location = useLocation();
	const navigate = useNavigate();
	const path = location.pathname.replace(/\/+$/, '') || '/';

	return (
		<SidebarGroup className="gap-1 px-2 py-1">
			<ContextHeader
				label="Remote Servers"
				collapsed={collapsed}
				onBack={() => void navigate({to: '/'})}
			/>
			<SidebarMenu className="gap-1">
				{SERVERS_NAV.map(item => (
					<ContextRow
						key={item.id}
						icon={item.icon}
						label={item.label}
						active={path === item.to}
						collapsed={collapsed}
						onClick={() => void navigate({to: item.to as never})}
					/>
				))}
			</SidebarMenu>
		</SidebarGroup>
	);
}

/* ── Remote Server detail context ───────────────────────────────────── */

function ServerDetailContextNav({
	id,
	collapsed,
}: {
	id: string;
	collapsed: boolean;
}) {
	const location = useLocation();
	const navigate = useNavigate();
	const currentTab =
		typeof location.search === 'object' && location.search
			? (location.search as Record<string, string>).tab
			: undefined;
	const activeTab = currentTab ?? 'overview';
	const {data: server} = useRemoteServerGet(BigInt(id));
	const {data: sshKeysData} = useSshKeyItems();
	const [monitoringOpen, setMonitoringOpen] = useState(false);
	const [editOpen, setEditOpen] = useState(false);
	const [deleteOpen, setDeleteOpen] = useState(false);

	const row = (server ?? null) as unknown as RemoteServerResponse | null;
	const sshKeys = (sshKeysData ?? []) as unknown as SshKeyResponse[];

	const actions = [
		{
			id: 'monitoring',
			label: 'Monitoring',
			icon: Activity,
			on: monitoringOpen,
			onClick: () => setMonitoringOpen(true),
		},
		{
			id: 'edit',
			label: 'Edit Server',
			icon: Pencil,
			on: editOpen,
			onClick: () => setEditOpen(true),
		},
		{
			id: 'remove',
			label: 'Remove Server',
			icon: Trash2,
			on: deleteOpen,
			onClick: () => setDeleteOpen(true),
		},
	];

	return (
		<>
			<SidebarGroup className="gap-1 px-2 py-1">
				<ContextHeader
					label={server?.name || 'Server'}
					collapsed={collapsed}
					onBack={() => void navigate({to: '/remote-servers'})}
				/>
				<SidebarMenu className="gap-1">
					{SERVER_TABS.map(item => (
						<ContextRow
							key={item.id}
							icon={item.icon}
							label={item.label}
							active={activeTab === item.id}
							collapsed={collapsed}
							onClick={() =>
								void navigate({
									to: '/remote-servers/$id',
									params: {id},
									search: {
										tab: item.id === 'overview' ? undefined : item.id,
									} as never,
									replace: true,
								})
							}
						/>
					))}
				</SidebarMenu>
				<SidebarSeparator className="my-1" />
				<SidebarMenu className="gap-1">
					{actions.map(item => (
						<ContextRow
							key={item.id}
							icon={item.icon}
							label={item.label}
							active={item.on}
							collapsed={collapsed}
							onClick={item.onClick}
						/>
					))}
				</SidebarMenu>
			</SidebarGroup>
			{row && (
				<>
					<ServerSettingsPanel
						isOpen={monitoringOpen}
						server={row}
						onClose={() => setMonitoringOpen(false)}
					/>
					<CreateServerModal
						isOpen={editOpen}
						sshKeys={sshKeys}
						editingServer={row}
						onClose={() => setEditOpen(false)}
						onSuccess={() => setEditOpen(false)}
					/>
					<DeleteServerModal
						isOpen={deleteOpen}
						server={row}
						onClose={() => setDeleteOpen(false)}
						onSuccess={() => {
							setDeleteOpen(false);
							void navigate({to: '/remote-servers'});
						}}
					/>
				</>
			)}
		</>
	);
}

/* ── Service context ───────────────────────────────────────────────── */

/* ── Deployments context ─────────────────────────────────────────── */

const DEPLOYMENTS_NAV = [
	{
		id: 'history',
		label: 'Logs & History',
		icon: ScrollText,
		to: '/Deployments',
	},
	{
		id: 'queue',
		label: 'Active Queue',
		icon: Activity,
		to: '/Deployments/queue',
	},
] as const;

function DeploymentsContextNav({collapsed}: {collapsed: boolean}) {
	const location = useLocation();
	const navigate = useNavigate();
	const path = location.pathname.replace(/\/+$/, '') || '/';

	return (
		<SidebarGroup className="gap-1 px-2 py-1">
			<ContextHeader
				label="Deployments"
				collapsed={collapsed}
				onBack={() => void navigate({to: '/'})}
			/>
			<SidebarMenu className="gap-1">
				{DEPLOYMENTS_NAV.map(item => (
					<ContextRow
						key={item.id}
						icon={item.icon}
						label={item.label}
						active={path === item.to}
						collapsed={collapsed}
						onClick={() => void navigate({to: item.to as never})}
					/>
				))}
			</SidebarMenu>
		</SidebarGroup>
	);
}

function ServiceContextNav({
	projectId,
	tabs,
	label,
	collapsed,
}: {
	projectId: number;
	tabs: readonly string[];
	label: string;
	collapsed: boolean;
}) {
	const navigate = useNavigate();
	const [activeTab, setActiveTab] = useUrlTab('General');

	// Stale param guard: arriving from a service with a tab this one lacks (app Logs → compose without Logs) would leave nothing selected.
	const effective = tabs.includes(activeTab) ? activeTab : 'General';
	if (effective !== activeTab) {
		queueMicrotask(() => setActiveTab(effective));
	}

	return (
		<SidebarGroup className="gap-1 px-2 py-1">
			<ContextHeader
				label={label}
				collapsed={collapsed}
				onBack={() =>
					void navigate({
						to: '/projects/$id',
						params: {id: String(projectId)},
					})
				}
			/>
			<SidebarMenu className="gap-1">
				{tabs.map(tab => {
					const Icon = TAB_ICONS[tab] ?? Wrench;
					return (
						<ContextRow
							key={tab}
							icon={Icon}
							label={tab}
							active={tab === effective}
							collapsed={collapsed}
							onClick={() => setActiveTab(tab)}
						/>
					);
				})}
			</SidebarMenu>
		</SidebarGroup>
	);
}

/* ── Entry ─────────────────────────────────────────────────────────── */

export function PageContextNav({collapsed}: {collapsed: boolean}) {
	const location = useLocation();
	const search = (location.search ?? {}) as Record<string, unknown>;
	const ctx = resolveContext(location.pathname, search);

	if (!ctx) return null;
	switch (ctx.kind) {
		case 'settings':
			return <SettingsContextNav collapsed={collapsed} />;
		case 'infra':
			return <InfraContextNav collapsed={collapsed} />;
		case 'deployments':
			return <DeploymentsContextNav collapsed={collapsed} />;
		case 'servers':
			return <ServersContextNav collapsed={collapsed} />;
		case 'server':
			return <ServerDetailContextNav id={ctx.id} collapsed={collapsed} />;
		case 'service':
			return (
				<ServiceContextNav
					projectId={ctx.projectId}
					tabs={ctx.tabs}
					label={ctx.label}
					collapsed={collapsed}
				/>
			);
	}
}
