import {useLocation} from '@tanstack/react-router';
import {
	SIDEBAR_CENTER_X,
	SIDEBAR_COLLAPSE,
	SIDEBAR_ICON_CELL,
	SIDEBAR_LABEL,
	SIDEBAR_ROW,
	Sidebar,
	SidebarContent,
	SidebarFooter,
	SidebarGroup,
	SidebarMenu,
	SidebarMenuItem,
	SidebarRail,
	SidebarSeparator,
	useSidebar,
} from '#/components/ui/sidebar';
import {Moon, Search, Sun} from 'lucide-react';
import {toast} from 'sonner';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {useTheme} from '#/hooks/ui/use-theme';
import {useAuthSession} from '#/stores/session-context';
import {useProfile} from '#/api/live';
import {getAvatarInitials, isSolidColorAvatar} from '#/lib/avatar-utils';
import {cn} from '#/api/utils';
import {MENU} from './enum';
import {PageContextNav} from './page-context-nav';
import {NavSection} from './nav-section';
import {AccountSwitcher} from './account-switcher';
import {SidebarNotifications} from './notification-bell';
import {triggerSearch} from '#/components/layouts/topbar/search-bus';

/** True only inside a service detail, settings, or the remote-server
 * surfaces — the rail swaps to that context's rows there. Plain project
 * pages keep the global menu. */
function usePageContextActive(): boolean {
	const location = useLocation();
	const seg = location.pathname.split('/').filter(Boolean);
	return (
		seg[0] === 'settings' ||
		seg[0] === 'infra' ||
		seg[0] === 'Deployments' ||
		// Only a server DETAIL page owns the rail — the plain list keeps the
		// global menu (its "Remote Servers" entry is already there).
		(seg[0] === 'remote-servers' && /^\d+$/.test(seg[1] ?? '')) ||
		(seg[0] === 'projects' && Boolean(seg[1]) && seg.length >= 4)
	);
}

/** The panel rail: global menu by default; inside a service detail (or settings) it swaps to that context's rows (Vercel behavior). */
export function AppSidebar() {
	const location = useLocation();
	const contextActive = usePageContextActive();
	// TanStack parses `?tab=` reactively; the parsed shape is an object.
	const currentTab =
		typeof location.search === 'object' && location.search
			? (location.search as Record<string, string>).tab
			: undefined;
	const {state, isMobile} = useSidebar();
	const collapsed = state === 'collapsed' && !isMobile;
	const {theme, toggleTheme} = useTheme();
	const {user: authUser, logout} = useAuthSession();
	const {data: profile} = useProfile();
	const displayName =
		profile?.name ||
		[authUser?.firstName, authUser?.lastName].filter(Boolean).join(' ') ||
		profile?.email?.split('@')[0] ||
		authUser?.email?.split('@')[0] ||
		'User';
	const avatarValue = profile?.avatar || '';
	const initials = getAvatarInitials(displayName);

	return (
		<>
			<Sidebar collapsible="icon" variant="sidebar">
				{/* Workspace band: the topbar's h-12 + border-b construction, so the two hairlines read as one. */}
				<div
					className={cn(
						'h-12 shrink-0 border-b border-sidebar-border transition-[padding]',
						SIDEBAR_COLLAPSE,
						collapsed ? 'px-0' : 'px-2',
					)}>
					<AccountSwitcher collapsed={collapsed} />
				</div>

				{/* ── Nav sections ───────────────────────────────────── */}
				<SidebarContent className="relative min-h-0 flex-1 overflow-hidden px-0">
					<nav className="relative h-full overflow-y-auto pt-4 pb-6">
						{/* Search trigger — one row of the shared nav grammar; opens the global command palette. */}
						<SidebarGroup className="gap-1 px-2 py-1">
							<SidebarMenu className="gap-1">
								<SidebarMenuItem>
									<button
										type="button"
										onClick={triggerSearch}
										title={collapsed ? 'Search (Ctrl+K)' : undefined}
										className={cn(
											SIDEBAR_ROW,
											SIDEBAR_COLLAPSE,
											'rounded-lg border border-foreground/15 bg-muted',
											collapsed && SIDEBAR_CENTER_X,
											'text-muted-foreground',
										)}>
										<span className={SIDEBAR_ICON_CELL}>
											<Search className="size-4" strokeWidth={1.7} />
										</span>
										<span
											className={cn(
												SIDEBAR_LABEL,
												collapsed && 'opacity-0',
											)}>
											Search
										</span>
										{!collapsed && (
											<kbd className="mr-2 rounded border border-border/60 bg-sidebar-accent/50 px-1.5 py-0.5 text-[10px] font-normal text-muted-foreground">
												⌘K
											</kbd>
										)}
									</button>
								</SidebarMenuItem>
							</SidebarMenu>
						</SidebarGroup>
						{/* ── Service/settings context swaps the rail; otherwise
						    the global menu (project pages keep it too). The key
						    remounts on swap so the rail cross-fades. ── */}
						<div
							key={contextActive ? 'context' : 'global'}
							className="animate-in duration-200 ease-out fade-in-0">
							{contextActive ? (
								<PageContextNav collapsed={collapsed} />
							) : (
								<>
									<NavSection
										items={MENU.main}
										currentPath={location.pathname}
										currentTab={currentTab}
										collapsed={collapsed}
									/>
									<SidebarSeparator className="my-1" />
									<NavSection
										items={MENU.infrastructure}
										currentPath={location.pathname}
										currentTab={currentTab}
										collapsed={collapsed}
									/>
									<SidebarSeparator className="my-1" />
									<NavSection
										items={MENU.resources}
										currentPath={location.pathname}
										currentTab={currentTab}
										collapsed={collapsed}
									/>
									<SidebarSeparator className="my-1" />
									<NavSection
										items={MENU.system}
										currentPath={location.pathname}
										currentTab={currentTab}
										collapsed={collapsed}
									/>
								</>
							)}
						</div>
					</nav>
					{/* Scroll fade stays inside the nav's own pb-6 gutter so it can never veil a row's fill. */}
					<div className="pointer-events-none absolute inset-x-0 bottom-0 h-6 bg-gradient-to-t from-sidebar to-transparent" />
				</SidebarContent>

				{/* ── Controls (foot): account menu on the user row, alerts + theme right. ── */}
				<SidebarFooter className="mt-auto gap-0 border-t border-sidebar-border p-0">
					<div
						className={cn(
							'flex items-center gap-2 px-2 py-2',
							collapsed && 'flex-col px-0',
						)}>
						<DropdownMenu>
							<DropdownMenuTrigger
								render={
									<button
										type="button"
										aria-label="Account options"
										title={displayName}
										className={cn(
											SIDEBAR_ROW,
											SIDEBAR_COLLAPSE,
											'min-w-0',
											collapsed ? SIDEBAR_CENTER_X : 'flex-1',
										)}>
										<span className={SIDEBAR_ICON_CELL}>
											{isSolidColorAvatar(avatarValue) ? (
												<span
													className="size-6 rounded-full ring-1 ring-border/50 ring-inset"
													style={{backgroundColor: avatarValue}}
												/>
											) : avatarValue ? (
												<img
													src={avatarValue}
													alt={displayName}
													className="size-6 rounded-full bg-muted object-cover"
												/>
											) : (
												<span className="flex size-6 items-center justify-center rounded-full bg-foreground/[0.06] text-[10px] font-semibold text-foreground uppercase">
													{initials}
												</span>
											)}
										</span>
										<span
											className={cn(
												SIDEBAR_LABEL,
												'text-foreground',
												collapsed && 'opacity-0',
											)}>
											{displayName}
										</span>
									</button>
								}
							/>
							<DropdownMenuContent
								side="top"
								align={collapsed ? 'start' : 'end'}
								className="min-w-44 rounded-xl p-1.5">
								<DropdownMenuItem
									variant="destructive"
									className="cursor-pointer gap-2 rounded-lg px-2.5 py-1.5 text-[13px] font-medium"
									onClick={() => {
										logout();
										toast.success('Signed out');
									}}>
									Log out
								</DropdownMenuItem>
							</DropdownMenuContent>
						</DropdownMenu>

						<div
							className={cn(
								'flex shrink-0 items-center gap-1',
								!collapsed && 'ml-auto',
							)}>
							<button
								onClick={toggleTheme}
								aria-label="Toggle theme"
								title="Toggle theme"
								className="flex size-6 shrink-0 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-sidebar-accent hover:text-foreground">
								{/* Icon shows the CURRENT theme; click toggles light/dark. */}
								{theme === 'light' ? (
									<Sun className="size-4" />
								) : (
									<Moon className="size-4" />
								)}
							</button>
							<SidebarNotifications collapsed={collapsed} />
						</div>
					</div>
				</SidebarFooter>

				<SidebarRail />
			</Sidebar>
		</>
	);
}
