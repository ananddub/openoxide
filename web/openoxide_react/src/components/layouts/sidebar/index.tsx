import {useLocation} from '@tanstack/react-router';
import {
	Sidebar,
	SidebarContent,
	SidebarFooter,
	SidebarRail,
	SidebarSeparator,
	useSidebar,
} from '#/components/ui/sidebar';
import {
	LogOut,
	Moon,
	MoreHorizontal,
	PanelLeftClose,
	PanelLeftOpen,
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
import {useTheme} from '#/hooks/use-theme';
import {useAuthSession} from '#/stores/session-context';
import {cn} from '#/api/utils';
import {MENU} from './enum';
import {NavSection} from './nav-section';
import {AccountSwitcher} from './account-switcher';

/**
 * The panel rail, matching the Vercel dashboard: workspace switcher +
 * New Project at the top, then separated nav sections, theme/collapse at
 * the foot. White rail (Background 1) with a single hairline edge.
 */
export function AppSidebar() {
	const location = useLocation();
	// TanStack parses `?tab=` reactively; the parsed shape is an object.
	const currentTab =
		typeof location.search === 'object' && location.search
			? (location.search as Record<string, string>).tab
			: undefined;
	const {state, isMobile, toggleSidebar} = useSidebar();
	const collapsed = state === 'collapsed' && !isMobile;
	const {theme, toggleTheme} = useTheme();
	const {user, logout} = useAuthSession();

	return (
		<>
			<Sidebar collapsible="icon" variant="sidebar">
				{/* ── Workspace switcher (top, no header) ──────────────
     Plain div: SidebarContent's flex-1/basis-0 base would collapse this
     block to zero height and spill its content over the nav below. */}
				<div className="shrink-0 px-2 pt-3 pb-1">
					<AccountSwitcher collapsed={collapsed} />
				</div>
				{/* Inset separators: clear of the rail's right edge, deeper on
     the left so the line reads centered inside the row padding. */}
				<SidebarSeparator className="mx-3 ml-4" />

				{/* ── Nav sections ───────────────────────────────────── */}
				<SidebarContent className="relative min-h-0 flex-1 overflow-hidden px-0">
					<nav className="h-full overflow-y-auto pt-1 pb-6">
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
							items={MENU.settings}
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
					</nav>
					{/* Scroll fade: ramps from the rail surface color. */}
					<div className="pointer-events-none absolute inset-x-0 bottom-0 h-14 bg-gradient-to-t from-sidebar to-transparent" />
				</SidebarContent>

				{/* ── Controls (foot): user row with the ••• menu ────── */}
				<SidebarFooter className="mt-auto gap-0 border-t border-sidebar-border p-0">
					<div
						className={cn(
							'flex items-center gap-2',
							collapsed ? 'flex-col px-2 py-2' : 'px-3 py-2.5',
						)}>
						<button
							onClick={toggleTheme}
							aria-label="Toggle theme"
							title="Toggle theme"
							className="flex size-7 shrink-0 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-sidebar-accent hover:text-foreground">
							{/* Icon shows the CURRENT theme; click toggles light/dark. */}
							{theme === 'light' ? (
								<Sun className="size-4" />
							) : (
								<Moon className="size-4" />
							)}
						</button>
						{!collapsed && (
							<div className="min-w-0 flex-1 text-[13px] leading-tight">
								<p className="truncate font-medium text-foreground">
									{[user?.firstName, user?.lastName]
										.filter(Boolean)
										.join(' ') ||
										user?.email?.split('@')[0] ||
										'User'}
								</p>
							</div>
						)}
						<DropdownMenu>
							<DropdownMenuTrigger
								render={
									<button
										type="button"
										aria-label="Account options"
										title="Account options"
										className="flex size-7 shrink-0 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-sidebar-accent hover:text-foreground">
										<MoreHorizontal className="size-4" />
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
									onClick={() => {
										logout();
										toast.success('Signed out');
									}}>
									<LogOut className="size-4" /> Log out
								</DropdownMenuItem>
							</DropdownMenuContent>
						</DropdownMenu>
					</div>
				</SidebarFooter>

				<SidebarRail />
			</Sidebar>
		</>
	);
}
