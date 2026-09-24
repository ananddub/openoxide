import {useLocation} from '@tanstack/react-router';
import {
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
import {useNavigate} from '@tanstack/react-router';
import {
	LogOut,
	Moon,
	PanelLeftClose,
	PanelLeftOpen,
	Search,
	Settings,
	Sun,
} from 'lucide-react';
import {toast} from 'sonner';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
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

/** True only inside a service detail (or settings) — the rail swaps to
 * that context's rows there. Plain project pages keep the global menu. */
function usePageContextActive(): boolean {
	const location = useLocation();
	const seg = location.pathname.split('/').filter(Boolean);
	return (
		seg[0] === 'settings' ||
		seg[0] === 'infra' ||
		seg[0] === 'Deployments' ||
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
	const navigate = useNavigate();
	const {state, isMobile, toggleSidebar} = useSidebar();
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
				{/* Workspace band: an h-12 box with a bottom hairline — the exact
     same construction as the topbar (h-12 + border-b). Both are border-box,
     so their lines land on the same row and read as one continuous,
     full-width hairline across the sidebar and the main area. */}
				<div className="h-12 shrink-0 border-b border-sidebar-border px-2">
					<AccountSwitcher collapsed={collapsed} />
				</div>

				{/* ── Nav sections ───────────────────────────────────── */}
				<SidebarContent className="relative min-h-0 flex-1 overflow-hidden px-0">
					<nav className="h-full overflow-y-auto pt-4 pb-6">
						{/* ── Search trigger ────────────────────────────────
						    Same row grammar as the nav items; opens the global
						    command palette (Ctrl/Cmd+K). */}
						<SidebarGroup className="px-2 py-1">
							<SidebarMenu className="gap-px">
								<SidebarMenuItem>
									<button
										type="button"
										onClick={triggerSearch}
										title={collapsed ? 'Search (Ctrl+K)' : undefined}
										className={cn(
											'relative flex w-full items-center rounded-md px-2 py-1.5 text-[13px] transition-colors outline-none',
											collapsed ? 'justify-center' : 'gap-2',
											'font-normal text-muted-foreground hover:bg-sidebar-accent hover:text-foreground',
										)}>
										<Search
											className="size-4 shrink-0"
											strokeWidth={1.7}
										/>
										{!collapsed && (
											<span className="flex-1 truncate text-start">
												Search
											</span>
										)}
										{!collapsed && (
											<kbd className="rounded border border-border/60 bg-sidebar-accent/50 px-1.5 py-0.5 text-[10px] font-normal text-muted-foreground">
												⌘K
											</kbd>
										)}
									</button>
								</SidebarMenuItem>
							</SidebarMenu>
						</SidebarGroup>
						{/* ── Service/settings context swaps the rail; otherwise
						    the global menu (project pages keep it too). ── */}
						<PageContextNav collapsed={collapsed} />
						{!contextActive && (
							<>
								<NavSection
									items={MENU.main}
									currentPath={location.pathname}
									currentTab={currentTab}
									collapsed={collapsed}
								/>
								<SidebarSeparator className="mx-3 my-2 ml-4" />
								<NavSection
									items={MENU.infrastructure}
									currentPath={location.pathname}
									currentTab={currentTab}
									collapsed={collapsed}
								/>
								<SidebarSeparator className="mx-3 my-2 ml-4" />
								<NavSection
									items={MENU.resources}
									currentPath={location.pathname}
									currentTab={currentTab}
									collapsed={collapsed}
								/>
								<SidebarSeparator className="mx-3 my-2 ml-4" />
								<NavSection
									items={MENU.system}
									currentPath={location.pathname}
									currentTab={currentTab}
									collapsed={collapsed}
								/>
							</>
						)}
					</nav>
					{/* Scroll fade: ramps from the rail surface color. */}
					<div className="pointer-events-none absolute inset-x-0 bottom-0 h-14 bg-gradient-to-t from-sidebar to-transparent" />
				</SidebarContent>

				{/* ── Controls (foot): account menu on the user row, alerts + theme right. ── */}
				<SidebarFooter className="mt-auto gap-0 border-t border-sidebar-border p-0">
					<div
						className={cn(
							'flex items-center gap-2',
							collapsed ? 'flex-col px-2 py-2' : 'px-3 py-2.5',
						)}>
						<DropdownMenu>
							<DropdownMenuTrigger
								render={
									<button
										type="button"
										aria-label="Account options"
										title={displayName}
										className={cn(
											'-mx-1 flex min-w-0 flex-1 cursor-pointer items-center gap-2 rounded-md px-1 py-0.5 text-start transition-colors hover:bg-sidebar-accent',
											collapsed && 'mx-0 px-0',
										)}>
										{isSolidColorAvatar(avatarValue) ? (
											<span
												className="size-6 shrink-0 rounded-full ring-1 ring-border/50 ring-inset"
												style={{backgroundColor: avatarValue}}
											/>
										) : avatarValue ? (
											<img
												src={avatarValue}
												alt={displayName}
												className="size-6 shrink-0 rounded-full bg-muted object-cover"
											/>
										) : (
											<span className="flex size-6 shrink-0 items-center justify-center rounded-full bg-foreground/[0.06] text-[10px] font-semibold text-foreground uppercase">
												{initials}
											</span>
										)}
										{!collapsed && (
											<span className="min-w-0 flex-1 truncate text-[13px] leading-tight font-medium text-foreground">
												{displayName}
											</span>
										)}
									</button>
								}
							/>
							<DropdownMenuContent
								side="top"
								align={collapsed ? 'start' : 'end'}
								className="min-w-44 rounded-lg">
								<DropdownMenuItem
									className="text-[13px]"
									onClick={toggleSidebar}>
									{collapsed ? (
										<PanelLeftOpen className="size-4" />
									) : (
										<PanelLeftClose className="size-4" />
									)}
									Collapse sidebar
								</DropdownMenuItem>
								<DropdownMenuSeparator />
								<DropdownMenuItem
									className="text-[13px]"
									onClick={() => void navigate({to: '/settings'})}>
									<Settings className="size-4" /> Settings
								</DropdownMenuItem>
								<DropdownMenuItem
									className="text-[13px]"
									onClick={() => {
										logout();
										toast.success('Signed out');
									}}>
									<LogOut className="size-4" /> Log out
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
