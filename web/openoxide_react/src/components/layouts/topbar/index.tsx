import {Fragment, useMemo} from 'react';
import {Link, useLocation, useNavigate} from '@tanstack/react-router';
import {useSidebar, SidebarTrigger} from '#/components/ui/sidebar';
import {useProjectsList} from '#/hooks/projects/use-projects-list';
import {SETTINGS_TABS} from '#/pages/settings/settings-tabs';
import {SearchDialog} from './search-bus';
import {TopBarScopeSwitcher} from './topbar-scope-switcher';
import {usePageCrumbs, type Crumb} from './topbar-slots';

/** Route → page title for the bar's center slot (fallback when a page
 * registers no breadcrumb trail). */
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
	'/dns': 'DNS Providers',
	'/destinations': 'S3 Destinations',
	'/settings': 'Settings',
};

/** The centered crumb trail (13px, slash separators, hover pills, active
 * last crumb gets the gray pill — Geist grammar). */
function CenterCrumbs({crumbs}: {crumbs: Crumb[]}) {
	return (
		<nav
			aria-label="Breadcrumb"
			className="pointer-events-auto flex max-w-full flex-wrap items-center gap-1 text-[13px] leading-none font-medium text-muted-foreground">
			{crumbs.map((crumb, index) => {
				const isLast = index === crumbs.length - 1;
				const content = (
					<span className="max-w-[240px] truncate">{crumb.label}</span>
				);
				return (
					<Fragment key={`${index}-${crumb.label}`}>
						{index > 0 && (
							<span
								aria-hidden
								className="px-0.5 text-muted-foreground/50 select-none">
								/
							</span>
						)}
						{crumb.to && !isLast ? (
							<Link
								to={crumb.to as never}
								params={crumb.params as never}
								search={crumb.search as never}
								className="-mx-1 flex items-center gap-1.5 rounded-md px-1.5 py-1 transition-colors outline-none hover:bg-muted hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring/40">
								{content}
							</Link>
						) : (
							<span
								aria-current={isLast ? 'page' : undefined}
								className={
									'-mx-1 flex items-center gap-1.5 rounded-md px-1.5 py-1' +
									(isLast ? ' bg-muted text-foreground' : '')
								}>
								{content}
							</span>
						)}
					</Fragment>
				);
			})}
		</nav>
	);
}

/**
 * Content column's top bar: the project scope switcher on the left, the
 * page's breadcrumb trail CENTERED (pages register it via topbar-slots;
 * plain route title as fallback), the product wordmark on the right.
 * Sits beside the rail (inside the content column), not above it.
 */
export function TopBar() {
	const location = useLocation();
	const navigate = useNavigate();
	const {isMobile} = useSidebar();
	const {projects} = useProjectsList();
	const registeredCrumbs = usePageCrumbs();

	// Scope: a specific project (deep view) or All Projects.
	const pathSegments = location.pathname.split('/').filter(Boolean);
	const projectIndex = pathSegments.indexOf('projects');
	const projectId =
		projectIndex !== -1 && pathSegments[projectIndex + 1]
			? Number(pathSegments[projectIndex + 1])
			: null;
	const activeProject = useMemo(
		() =>
			projectId !== null
				? (projects.find(p => Number(p.id) === projectId) ?? null)
				: null,
		[projects, projectId],
	);

	// Center fallback: route table first, then the settings tab label,
	// then a project detail's name, then the last URL segment.
	const fallbackTitle = useMemo(() => {
		if (location.pathname.startsWith('/settings')) {
			const tab = (location.search as Record<string, string>)?.tab;
			return SETTINGS_TABS.find(t => t.id === tab)?.label ?? 'Settings';
		}
		const base = '/' + (pathSegments[0] ?? '');
		if (ROUTE_TITLES[base] && pathSegments.length === 1) {
			return ROUTE_TITLES[base];
		}
		if (activeProject?.name) return activeProject.name;
		return (
			ROUTE_TITLES[base] ??
			(pathSegments.at(-1) != null
				? decodeURIComponent(pathSegments.at(-1)!)
				: 'Overview')
		);
	}, [location.pathname, location.search, pathSegments, activeProject]);

	return (
		<>
			<header
				data-slot="topbar"
				className="relative flex h-12 shrink-0 items-center justify-between gap-2 border-b border-border bg-background px-2">
				{/* ── Left: project scope switcher ────────────────────── */}
				<TopBarScopeSwitcher
					projects={projects}
					activeProject={activeProject}
					projectId={projectId}
					onAllProjects={() => {
						if (projectId !== null) void navigate({to: '/projects'});
					}}
					onSelectProject={id => {
						void navigate({
							to: '/projects/$id',
							params: {id: String(id)},
						});
					}}
				/>

				{/* ── Center: breadcrumb trail (or plain title) ───────── */}
				<span className="pointer-events-none absolute left-1/2 flex max-w-[60%] -translate-x-1/2 items-center justify-center truncate">
					{registeredCrumbs ? (
						<CenterCrumbs crumbs={registeredCrumbs} />
					) : (
						<span className="truncate text-[13px] font-medium text-foreground">
							{fallbackTitle}
						</span>
					)}
				</span>

				{/* ── Right: product wordmark + mobile drawer trigger ──── */}
				<div className="flex items-center gap-2 pr-2">
					<span className="shrink-0 text-[13px] font-medium tracking-tight text-muted-foreground/80 select-none">
						OpenOxide
					</span>
					{/* Mobile-only: below md the rail is a sheet and this is
					    its only opener. */}
					{isMobile && (
						<SidebarTrigger className="flex size-8 items-center justify-center rounded-md text-muted-foreground hover:bg-sidebar-accent hover:text-foreground" />
					)}
				</div>
			</header>
			{/* Global command palette (Ctrl+K / search icon). */}
			<SearchDialog />
		</>
	);
}
