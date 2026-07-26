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

// Pure standard Shadcn UI NavMenuGroup implementation.
function NavMenuGroup({label, items, currentPath}: NavMenuGroupProps) {
	return (
		<SidebarGroup>
			<SidebarGroupLabel className="group-data-[collapsible=icon]:hidden">
				{label}
			</SidebarGroupLabel>
			<SidebarMenu>
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
	const isCollapsed = state === 'collapsed';

	return (
		<>
			<Sidebar collapsible="icon" variant="sidebar">
				{/* Brand Header */}
				<SidebarHeader>
					<HeaderDropdown isCollapsed={isCollapsed} isMobile={isMobile} />
				</SidebarHeader>

				{/* Navigation Content */}
				<SidebarContent>
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
						<SidebarMenu>
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
				<SidebarFooter>
					<AppSidebarFooter isCollapsed={isCollapsed} />
				</SidebarFooter>

				<SidebarRail />
			</Sidebar>
			<SearchDialog />
		</>
	);
}
