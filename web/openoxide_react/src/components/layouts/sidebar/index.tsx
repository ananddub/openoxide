import {Link, useLocation} from '@tanstack/react-router';
import {
	Sidebar,
	SidebarContent,
	SidebarFooter,
	SidebarGroup,
	SidebarHeader,
	SidebarMenu,
	SidebarMenuItem,
	SidebarRail,
	useSidebar,
} from '#/components/ui/sidebar';
import {
	Moon,
	PanelLeftClose,
	PanelLeftOpen,
	Plus,
	Sun,
	SunMoon,
} from 'lucide-react';
import {useCan} from '#/hooks/use-permissions';
import {useTheme} from '#/hooks/use-theme';
import {useOrganizations} from '#/stores/session-context';
import {cn} from '#/api/utils';
import {type NavItem, MENU} from './enum';
import {Logo} from '#/components/shared/logo';
import {AnimatedShinyText} from '#/components/ui/animated-shiny-text';
import {SearchDialog} from './search/dialog';
import {AccountSwitcher} from './account-switcher';

type NavSectionProps = {
	label: string;
	items: NavItem[];
	currentPath: string;
	collapsed: boolean;
	className?: string;
};

/**
 * One labeled section of the rail, rendered in the OpenShip style: a tiny
 * uppercase tracking label, then roomy rounded-xl rows whose active state
 * is a soft foreground wash (not the old filled accent).
 */
function NavSection({
	label,
	items,
	currentPath,
	collapsed,
	className,
}: NavSectionProps) {
	const can = useCan();
	const visibleItems = items.filter(item => {
		if ('isSingle' in item && item.isSingle === false) return true;
		return !item.permission || can(item.permission);
	});
	if (!visibleItems.length) return null;

	return (
		<SidebarGroup className={cn('py-1', className)}>
			{!collapsed && (
				<p className="mb-1.5 px-2 text-[10px] font-semibold tracking-widest text-muted-foreground/60 uppercase">
					{label}
				</p>
			)}
			{collapsed && <div className="mx-2 my-3 h-px bg-border/60" />}
			<SidebarMenu className="gap-1">
				{visibleItems.map(item => {
					if ('isSingle' in item && item.isSingle === false) return null;
					const single = item as Extract<
						NavItem,
						{title: string; to: string}
					>;
					const active =
						single.to === '/'
							? currentPath === '/'
							: currentPath.startsWith(single.to);
					return (
						<SidebarMenuItem key={single.to}>
							<Link
								to={single.to as '/'}
								title={collapsed ? single.title : undefined}
								className={cn(
									'flex items-center rounded-xl px-2.5 py-2 text-[13px] font-medium transition-colors',
									collapsed ? 'justify-center' : 'gap-2.5',
									active
										? 'bg-foreground/[0.07] text-foreground'
										: 'text-muted-foreground hover:bg-foreground/[0.04] hover:text-foreground',
								)}>
								<single.icon
									className="size-4 shrink-0"
									strokeWidth={1.7}
								/>
								{!collapsed && (
									<span className="flex-1 truncate">{single.title}</span>
								)}
							</Link>
						</SidebarMenuItem>
					);
				})}
			</SidebarMenu>
		</SidebarGroup>
	);
}

/**
 * The panel rail, laid out like the OpenShip dashboard sidebar: a floating
 * rounded card with logo + theme/collapse controls in the header, scrollable
 * nav sections with a bottom fade, a gradient primary CTA, and the account /
 * organization switcher pinned at the foot.
 */
export function AppSidebar() {
	const location = useLocation();
	const {state, isMobile, toggleSidebar} = useSidebar();
	const collapsed = state === 'collapsed' && !isMobile;
	const {theme, toggleTheme} = useTheme();
	const {organizations} = useOrganizations();

	return (
		<>
			<Sidebar
				collapsible="icon"
				variant="floating"
				className="[&_[data-slot=sidebar-inner]]:rounded-2xl! [&_[data-slot=sidebar-inner]]:ring-border/50">
				{/* ── Header ─────────────────────────────────────────── */}
				<SidebarHeader
					className={cn(
						'px-5 py-6',
						collapsed
							? 'flex-col items-center gap-3 pb-3'
							: 'flex-row items-center justify-between',
					)}>
					<div className="flex min-w-0 items-center gap-2.5">
						<Logo className="size-6 shrink-0" />
						{!collapsed && (
							<AnimatedShinyText className="truncate text-sm font-semibold tracking-tight text-foreground/80">
								OpenOxide
							</AnimatedShinyText>
						)}
					</div>
					<div
						className={cn(
							'flex items-center',
							collapsed ? 'flex-col gap-1' : 'gap-1',
						)}>
						<button
							onClick={toggleTheme}
							aria-label="Toggle theme"
							title="Toggle theme"
							className="flex size-8 items-center justify-center rounded-xl text-muted-foreground transition-colors hover:bg-foreground/[0.06] hover:text-foreground">
							{/* Icon shows the CURRENT theme; clicking cycles light → dim → dark. */}
							{theme === 'light' ? (
								<Sun className="size-4" />
							) : theme === 'dim' ? (
								<SunMoon className="size-4" />
							) : (
								<Moon className="size-4" />
							)}
						</button>
						<button
							type="button"
							onClick={toggleSidebar}
							aria-label={
								collapsed ? 'Expand sidebar' : 'Collapse sidebar'
							}
							className="flex size-8 items-center justify-center rounded-xl text-muted-foreground transition-colors hover:bg-foreground/[0.06] hover:text-foreground">
							{collapsed ? (
								<PanelLeftOpen className="size-4" />
							) : (
								<PanelLeftClose className="size-4" />
							)}
						</button>
					</div>
				</SidebarHeader>

				<div className="mx-3 h-px bg-border/60" />

				{/* ── Nav sections ───────────────────────────────────── */}
				<SidebarContent className="relative min-h-0 overflow-hidden px-0">
					<nav className="h-full overflow-y-auto px-3 pt-3 pb-12">
						<NavSection
							label="Main"
							items={MENU.main}
							currentPath={location.pathname}
							collapsed={collapsed}
						/>
						<NavSection
							label="Infrastructure"
							items={MENU.infrastructure}
							currentPath={location.pathname}
							collapsed={collapsed}
							className="mt-5"
						/>
						<NavSection
							label="Settings"
							items={MENU.settings}
							currentPath={location.pathname}
							collapsed={collapsed}
							className="mt-5"
						/>
					</nav>
					{/* Scroll fade: --th-card-on-page is the opaque composite of
					    the rail surface over the page, so the ramp matches in
					    every theme. */}
					<div className="pointer-events-none absolute inset-x-0 bottom-0 h-14 bg-gradient-to-t from-[var(--th-card-on-page)] to-transparent" />
				</SidebarContent>

				{/* ── Primary action + account ───────────────────────── */}
				<SidebarFooter className="gap-0 p-0">
					{organizations.length > 0 && (
						<div className="px-3 pb-2">
							<Link
								to="/projects"
								search={{create: 1} as never}
								title={collapsed ? 'New Project' : undefined}
								className={cn(
									'relative flex items-center justify-center gap-2.5 overflow-hidden rounded-xl bg-gradient-to-r from-violet-500/90 via-primary/90 to-blue-500/90 px-3 py-2 text-[13px] font-semibold text-white shadow-sm shadow-primary/20 transition-all hover:shadow-md hover:shadow-primary/30 hover:brightness-110',
									'dim:from-[hsl(86_84%_74%)]! dim:via-[hsl(82_80%_64%)]! dim:to-[hsl(74_74%_54%)]! dim:text-[#0c1206]! dim:shadow-lime-400/25 dim:hover:shadow-lime-400/40',
									'dark:from-amber-400/90 dark:via-orange-500/90 dark:to-rose-500/90 dark:shadow-orange-500/20 dark:hover:shadow-orange-500/30',
								)}>
								<span className="absolute inset-0 bg-[radial-gradient(circle_at_30%_50%,rgba(255,255,255,0.15),transparent_70%)]" />
								<Plus className="relative size-4" strokeWidth={2.5} />
								{!collapsed && (
									<span className="relative">New Project</span>
								)}
							</Link>
						</div>
					)}
					<AccountSwitcher collapsed={collapsed} />
				</SidebarFooter>

				<SidebarRail />
			</Sidebar>
			<SearchDialog />
		</>
	);
}
