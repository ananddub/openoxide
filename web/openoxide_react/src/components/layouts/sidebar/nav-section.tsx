import {Link} from '@tanstack/react-router';
import {
	SidebarGroup,
	SidebarMenu,
	SidebarMenuItem,
} from '#/components/ui/sidebar';
import {useCan} from '#/hooks/use-permissions';
import {cn} from '#/api/utils';
import {type NavItem} from './enum';

type NavSectionProps = {
	items: NavItem[];
	currentPath: string;
	currentTab: string | undefined;
	collapsed: boolean;
};

/**
 * One section of the rail, Geist style: flat rows, no section titles —
 * separators between groups carry the rhythm. Selected = gray-alpha fill
 * with gray-1000 text; hover = one step lighter. The Vercel dashboard
 * nav grammar.
 */
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
		<SidebarGroup className="px-2 py-1">
			<SidebarMenu className="gap-px">
				{visibleItems.map(item => {
					if ('isSingle' in item && item.isSingle === false) return null;
					const single = item as Extract<
						NavItem,
						{title: string; to: string}
					>;
					// ?tab= deep links: match the path, then the tab. A missing tab
					// param means the page fell back to its first tab (Account), so
					// the Account row stays lit on bare /settings too. currentTab is
					// TanStack's reactive search param — window.location would go stale.
					const [basePath, query] = single.to.split('?');
					const queryTab = query
						? new URLSearchParams(query).get('tab')
						: null;
					const active =
						basePath === '/'
							? currentPath === '/'
							: queryTab
								? currentPath === basePath &&
									(currentTab ?? 'account') === queryTab
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
									'relative flex items-center rounded-md px-2 py-1.5 text-[13px] transition-colors outline-none',
									collapsed ? 'justify-center' : 'gap-2',
									// Geist: selected = Color 1 fill + gray-1000 text;
									// hover = one step lighter. No left-edge bar.
									active
										? 'bg-sidebar-active font-medium text-foreground'
										: 'font-normal text-muted-foreground hover:bg-sidebar-accent hover:text-foreground',
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
