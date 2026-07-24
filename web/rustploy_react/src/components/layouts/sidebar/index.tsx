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
import {SearchButton} from './search';
import {SearchDialog} from './search/dialog';
import {HeaderDropdown} from './header';
import {AppSidebarFooter} from './footer';

type NavMenuGroupProps = {
	label: string;
	items: NavItem[];
	currentPath: string;
};

// Renders a labeled group of nav items with active-state highlighting.
function NavMenuGroup({label, items, currentPath}: NavMenuGroupProps) {
	return (
		<SidebarGroup>
			<SidebarGroupLabel className="uppercase group-data-[collapsible=icon]:hidden">
				{label}
			</SidebarGroupLabel>
			<SidebarMenu className="gap-1">
				{items.map(item => {
					const isActive = currentPath === item.to;
					return (
						<SidebarMenuItem key={item.title}>
							<SidebarMenuButton
								render={<Link to={item.to as any} />}
								isActive={isActive}
								tooltip={item.title}>
								<item.icon className="size-4" />
								<span>{item.title}</span>
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
		<div className="px-3.5">
			<Separator />
		</div>
	);
}

export function AppSidebar() {
	const location = useLocation();
	const {isMobile, state} = useSidebar();
	const isCollapsed = state == 'collapsed';

	return (
		<>
			<Sidebar collapsible="icon" variant="floating">
				{/* Brand Header */}
				<SidebarHeader className="border-b border-border/40 px-4 py-4 group-data-[collapsible=icon]:p-1.5">
					<HeaderDropdown isCollapsed={isCollapsed} isMobile={isMobile} />
				</SidebarHeader>
				{/* Navigation Content */}
				<SidebarContent className="gap-2 py-2">
					{/* Quick Search */}
					<SearchButton />
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
					<SidebarGroup className="group-data-[collapsible=icon]:hidden">
						<SidebarGroupLabel>Extra</SidebarGroupLabel>
						<SidebarMenu className="gap-1">
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
										tooltip={item.title}>
										<item.icon className="size-4" />
										<span>{item.title}</span>
									</SidebarMenuButton>
								</SidebarMenuItem>
							))}
						</SidebarMenu>
					</SidebarGroup>
				</SidebarContent>
				{/* Footer */}
				<SidebarFooter className="border-t border-border/40 p-4 group-data-[collapsible=icon]:p-2">
					<AppSidebarFooter isCollapsed={isCollapsed} />
				</SidebarFooter>
				<SidebarRail />
			</Sidebar>
			<SearchDialog />
		</>
	);
}
