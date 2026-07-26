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

// Renders a labeled group of nav items with Dokploy signature icon badge boxes.
function NavMenuGroup({label, items, currentPath}: NavMenuGroupProps) {
	return (
		<SidebarGroup>
			<SidebarGroupLabel className="group-data-[collapsible=icon]:hidden">
				{label}
			</SidebarGroupLabel>
			<SidebarMenu className="gap-1">
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
								className={`h-9 text-xs font-medium rounded-lg transition-all gap-2.5 px-2.5 ${
									isActive
										? 'bg-accent/60 text-foreground font-semibold'
										: 'text-muted-foreground hover:text-foreground hover:bg-muted/40'
								}`}
							>
								{/* Dokploy Signature Icon Badge Box */}
								<div
									className={`w-6.5 h-6.5 rounded-md flex items-center justify-center shrink-0 border transition-all ${
										isActive
											? 'bg-primary text-primary-foreground border-primary shadow-xs'
											: 'bg-muted/50 border-border/40 text-muted-foreground group-hover:border-border group-hover:text-foreground'
									}`}
								>
									<item.icon className="w-3.5 h-3.5" />
								</div>
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
		<div className="px-3.5 my-1">
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
			<Sidebar collapsible="icon" variant="floating">
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
										tooltip={item.title}
										className="h-9 text-xs font-medium text-muted-foreground hover:text-foreground hover:bg-muted/40 rounded-lg gap-2.5 px-2.5 transition-all"
									>
										<div className="w-6.5 h-6.5 rounded-md bg-muted/50 border border-border/40 text-muted-foreground flex items-center justify-center shrink-0 group-hover:border-border group-hover:text-foreground transition-all">
											<item.icon className="w-3.5 h-3.5" />
										</div>
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
