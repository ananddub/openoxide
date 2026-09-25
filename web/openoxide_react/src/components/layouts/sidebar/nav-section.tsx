import {Link} from '@tanstack/react-router';
import {ChevronRight} from 'lucide-react';
import {
	SIDEBAR_CENTER_X,
	SIDEBAR_COLLAPSE,
	SIDEBAR_ICON_CELL,
	SIDEBAR_LABEL,
	SIDEBAR_ROW,
	SidebarGroup,
	SidebarMenu,
	SidebarMenuItem,
} from '#/components/ui/sidebar';
import {useCan} from '#/hooks/permissions/use-permissions';
import {cn} from '#/api/utils';
import {type NavItem, type SingleNavItem} from './enum';

type NavSectionProps = {
	items: NavItem[];
	currentPath: string;
	currentTab: string | undefined;
	collapsed: boolean;
};

/** One section of the rail, Geist style: flat rows, no section titles — separators between groups carry the rhythm. Selected = gray-alpha fill with gray-1000 text; hover = one step lighter. The Vercel dashboard nav grammar. */
export function NavSection({
	items,
	currentPath,
	currentTab,
	collapsed,
}: NavSectionProps) {
	const can = useCan();
	const visibleItems = items.filter(item => {
		if ('isSingle' in item && item.isSingle === false) return true;
		return !item.permission || can(item.permission);
	});
	if (!visibleItems.length) return null;

	return (
		<SidebarGroup className="gap-1 px-2 py-1">
			<SidebarMenu className="gap-1">
				{visibleItems.map(item => {
					if ('isSingle' in item && item.isSingle === false) return null;
					const single = item as SingleNavItem;
					// ?tab= deep links: match the path, then the tab. A missing tab param means the page fell back to its first tab (Account), so the Account row stays lit on bare /settings too. currentTab is TanStack's reactive search param — window.location would go stale.
					const [basePath, query] = single.to.split('?');
					const queryTab = query
						? new URLSearchParams(query).get('tab')
						: null;
					// Bare route (no ?tab=) renders the page's first tab — light that row instead: /settings → account, /infra → web-server.
					const defaultTab =
						basePath === '/infra' ? 'web-server' : 'account';
					const active =
						basePath === '/'
							? currentPath === '/'
							: queryTab
								? currentPath === basePath &&
									(currentTab ?? defaultTab) === queryTab
								: currentPath.startsWith(basePath);
					return (
						<SidebarMenuItem key={single.to}>
							<Link
								to={basePath as '/'}
								search={
									query
										? (Object.fromEntries(
												new URLSearchParams(query),
											) as never)
										: undefined
								}
								title={collapsed ? single.title : undefined}
								className={cn(
									SIDEBAR_ROW,
									SIDEBAR_COLLAPSE,
									collapsed && SIDEBAR_CENTER_X,
									active
										? 'bg-sidebar-accent text-foreground'
										: 'text-muted-foreground',
								)}>
								<span className={SIDEBAR_ICON_CELL}>
									<single.icon className="size-4" strokeWidth={1.7} />
								</span>
								<span
									className={cn(
										SIDEBAR_LABEL,
										collapsed && 'opacity-0',
									)}>
									{single.title}
								</span>
								{/* Marks a route that swaps the rail to its own context rows. */}
								{!collapsed && single.railContext && (
									<span className="mr-1.5 grid size-6 shrink-0 place-content-center opacity-50">
										<ChevronRight className="size-4" strokeWidth={2} />
									</span>
								)}
							</Link>
						</SidebarMenuItem>
					);
				})}
			</SidebarMenu>
		</SidebarGroup>
	);
}
