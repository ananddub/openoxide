import {useMemo} from 'react';
import {useLocation, useNavigate} from '@tanstack/react-router';
import {Search, PanelLeftClose, PanelLeftOpen} from 'lucide-react';
import {useSidebar, SidebarTrigger} from '#/components/ui/sidebar';
import {useOrganizations} from '#/stores/session-context';
import {useProjectsList} from '#/hooks/projects/use-projects-list';
import {useAccountSwitcher} from '../sidebar/use-account-switcher';
import {AccountOrgDialog} from '../sidebar/account-org-dialog';
import {SETTINGS_TABS} from '#/pages/settings/settings-tabs';
import {SearchDialog, triggerSearch} from './search-bus';
import {TopBarScopeSwitcher} from './topbar-scope-switcher';

/** Route → page title for the bar's center slot. */
const ROUTE_TITLES: Record<string, string> = {
	'/': 'Overview',
	'/projects': 'Projects',
	'/Deployments': 'Deployments',
	'/schedules': 'Schedules',
	'/remote-servers': 'Remote Servers',
	'/infra': 'Web Server',
	'/ssh-keys': 'SSH Keys',
	'/tags': 'Tags',
	'/registry': 'Registry',
	'/vault': 'Vault Providers',
	'/destinations': 'S3 Destinations',
	'/settings': 'Settings',
};

/**
 * Vercel's fixed top bar: the workspace/project scope switcher on the
 * left, the current page title centered, contextual controls on the
 * right. It spans the full window width; the sidebar rail starts
 * *below* it, so the switcher reads as a global chrome element.
 */
export function TopBar() {
	const location = useLocation();
	const navigate = useNavigate();
	const {state, isMobile, toggleSidebar} = useSidebar();
	const {activeOrg} = useOrganizations();
	const {projects} = useProjectsList();
	const ctl = useAccountSwitcher();
	const {organizations, activeId, switchTo, setOpen} = ctl;
	const collapsed = state === 'collapsed' && !isMobile;

	// Scope: a specific project (deep view) or the workspace itself.
	const projectSegments = location.pathname.split('/').filter(Boolean);
	const projectIndex = projectSegments.indexOf('projects');
	const projectId =
		projectIndex !== -1 && projectSegments[projectIndex + 1]
			? Number(projectSegments[projectIndex + 1])
			: null;
	const activeProject = useMemo(
		() =>
			projectId !== null
				? (projects.find(p => Number(p.id) === projectId) ?? null)
				: null,
		[projects, projectId],
	);

	// Center title: route table first, then the settings tab label,
	// then a project detail's name, falling back to the last segment.
	const pageTitle = useMemo(() => {
		if (location.pathname.startsWith('/settings')) {
			const tab = (location.search as Record<string, string>)?.tab;
			return SETTINGS_TABS.find(t => t.id === tab)?.label ?? 'Settings';
		}
		const base = '/' + (projectSegments[0] ?? '');
		if (ROUTE_TITLES[base] && projectSegments.length === 1) {
			return ROUTE_TITLES[base];
		}
		if (activeProject?.name) return activeProject.name;
		return (
			ROUTE_TITLES[base] ??
			(projectSegments.at(-1) != null
				? decodeURIComponent(projectSegments.at(-1)!)
				: 'Overview')
		);
	}, [location.pathname, location.search, projectSegments, activeProject]);

	const selectWorkspace = () => {
		if (projectId !== null) void navigate({to: '/'});
	};

	return (
		<>
			<header
				data-slot="topbar"
				className="fixed inset-x-0 top-0 z-30 flex h-12 shrink-0 items-center justify-between gap-2 border-b border-border/60 bg-background px-2">
				{/* ── Left: scope switcher ──────────────────────────── */}
				<TopBarScopeSwitcher
					activeProject={activeProject}
					activeOrg={activeOrg ?? null}
					projectId={projectId}
					organizations={organizations}
					activeId={activeId}
					switchTo={switchTo}
					setOpen={setOpen}
					onSelectWorkspace={selectWorkspace}
				/>

				{/* ── Center: page title ────────────────────────────── */}
				<span className="pointer-events-none absolute left-1/2 -translate-x-1/2 truncate text-[13px] font-medium text-foreground">
					{pageTitle}
				</span>

				{/* ── Right: search + collapse ──────────────────────── */}
				<div className="flex items-center gap-0.5">
					{/* Mobile: the drawer trigger lives here — the rail is
					    hidden below md and this is its only opener. */}
					<SidebarTrigger className="flex size-8 items-center justify-center rounded-md text-muted-foreground hover:bg-sidebar-accent hover:text-foreground md:hidden" />
					<button
						type="button"
						onClick={triggerSearch}
						aria-label="Search"
						title="Search (Ctrl+K)"
						className="flex size-8 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-sidebar-accent hover:text-foreground">
						<Search className="size-4" />
					</button>
					<button
						type="button"
						onClick={toggleSidebar}
						aria-label="Toggle sidebar"
						title="Toggle sidebar"
						className="hidden size-8 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-sidebar-accent hover:text-foreground md:flex">
						{collapsed ? (
							<PanelLeftOpen className="size-4" />
						) : (
							<PanelLeftClose className="size-4" />
						)}
					</button>
				</div>
			</header>
			{/* Workspace create / rename lives with the switcher's hook. */}
			<AccountOrgDialog ctl={ctl} />
			<SearchDialog />
		</>
	);
}
