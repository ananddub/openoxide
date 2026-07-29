import {Link, useLocation} from '@tanstack/react-router';
import {
	Sidebar,
	SidebarContent,
	SidebarFooter,
	SidebarHeader,
	SidebarMenu,
	SidebarMenuButton,
	SidebarMenuItem,
	SidebarGroup,
	SidebarGroupLabel,
	SidebarRail,
	useSidebar,
} from '#/components/ui/sidebar';
import {type NavItem, MENU} from './enum';
import {Separator} from '#/components/ui/separator';
import {SearchDialog} from './search/dialog';
import {HeaderDropdown} from './header';
import {AppSidebarFooter} from './footer';

type NavMenuGroupProps = {
	label: string;
	items: NavItem[];
	currentPath: string;
};

// Renders a labeled group of nav items with high-contrast Dokploy color styling.
function NavMenuGroup({label, items, currentPath}: NavMenuGroupProps) {
	return (
		<SidebarGroup className="py-1">
			<SidebarGroupLabel className="text-[11px] font-mono tracking-wider text-muted-foreground font-semibold uppercase px-2 mb-1 group-data-[collapsible=icon]:hidden">
				{label}
			</SidebarGroupLabel>
			<SidebarMenu className="gap-0.5">
				{items.map(item => {
					const isActive = (() => {
						if (!item.to) return false;
						if (item.to === '/') return currentPath === '/';
						if (item.to === '/settings') return currentPath === '/settings';
						if (item.to === '/settings/server') return currentPath === '/settings/server';
						return currentPath.startsWith(item.to);
					})();
					return (
						<SidebarMenuItem key={item.title}>
							<SidebarMenuButton
								render={<Link to={item.to as any} />}
								isActive={isActive}
								tooltip={item.title}
								className={`h-8.5 text-xs font-medium rounded-lg transition-colors px-2.5 group-data-[collapsible=icon]:px-0! group-data-[collapsible=icon]:justify-center! ${
									isActive
										? 'bg-primary/15 text-primary font-semibold'
										: 'text-foreground/90 hover:text-foreground hover:bg-accent/60'
								}`}
							>
								<item.icon className={`size-4 shrink-0 ${isActive ? 'text-primary' : 'text-foreground/80'}`} />
								<span className="group-data-[collapsible=icon]:hidden">{item.title}</span>
							</SidebarMenuButton>
						</SidebarMenuItem>
					);
				})}
			</SidebarMenu>
		</SidebarGroup>
	);
}

// Thin horizontal rule with consistent horizontal padding between nav groups.
function SidebarSeparator() {
	return (
		<div className="px-3.5 my-1">
			<Separator className="bg-border/40" />
		</div>
	);
}

export function AppSidebar() {
	const location = useLocation();
	const {isMobile, state} = useSidebar();
	const isCollapsed = state === 'collapsed';

	return (
		<>
			<Sidebar collapsible="icon" variant="floating">
				{/* Brand Header */}
				<SidebarHeader className="border-b border-border/30 px-3 py-3 group-data-[collapsible=icon]:p-1.5">
					<HeaderDropdown isCollapsed={isCollapsed} isMobile={isMobile} />
				</SidebarHeader>

				{/* Navigation Content */}
				<SidebarContent className="gap-1 px-1.5 py-2 group-data-[collapsible=icon]:px-0 group-data-[collapsible=icon]:items-center">
					{/* Platform Group */}
					<NavMenuGroup
						label="Platform"
						items={MENU.platform}
						currentPath={location.pathname}
					/>

					<SidebarSeparator />

					{/* Settings Group */}
					<NavMenuGroup
						label="Settings"
						items={MENU.settings}
						currentPath={location.pathname}
					/>

					<SidebarSeparator />

					{/* Extra / Help Group */}
					<SidebarGroup className="py-1 group-data-[collapsible=icon]:hidden">
						<SidebarGroupLabel className="text-[11px] font-mono tracking-wider text-muted-foreground font-semibold uppercase px-2 mb-1">
							Extra
						</SidebarGroupLabel>
						<SidebarMenu className="gap-0.5">
							{MENU.help.map(item => (
								<SidebarMenuItem key={item.title}>
									<SidebarMenuButton
										render={
											<a
												href={item.href}
												target="_blank"
												rel="noopener noreferrer"
											/>
										}
										tooltip={item.title}
										className="h-8.5 text-xs font-medium text-foreground/90 hover:text-foreground hover:bg-accent/60 rounded-lg px-2.5 transition-colors"
									>
										<item.icon className="size-4 shrink-0 text-foreground/80" />
										<span>{item.title}</span>
									</SidebarMenuButton>
								</SidebarMenuItem>
							))}
						</SidebarMenu>
					</SidebarGroup>
				</SidebarContent>

				{/* Footer */}
				<SidebarFooter className="border-t border-border/30 p-3 group-data-[collapsible=icon]:p-2">
					<AppSidebarFooter isCollapsed={isCollapsed} />
				</SidebarFooter>

				<SidebarRail />
			</Sidebar>
			<SearchDialog />
		</>
	);
}
