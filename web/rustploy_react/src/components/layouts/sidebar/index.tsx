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

// Renders a labeled group of nav items with 100% aligned full-width selection background.
function NavMenuGroup({label, items, currentPath}: NavMenuGroupProps) {
	return (
		<SidebarGroup className="py-1 px-0">
			<SidebarGroupLabel className="text-[10px] font-mono tracking-widest text-muted-foreground/60 uppercase px-3 mb-1 group-data-[collapsible=icon]:hidden">
				{label}
			</SidebarGroupLabel>
			<SidebarMenu className="gap-0.5 w-full">
				{items.map(item => {
					const isActive = (() => {
						if (!item.to) return false;
						if (item.to === '/') return currentPath === '/';
						if (item.to === '/settings') return currentPath === '/settings';
						if (item.to === '/settings/server') return currentPath === '/settings/server';
						return currentPath.startsWith(item.to);
					})();
					return (
						<SidebarMenuItem key={item.title} className="w-full">
							<SidebarMenuButton
								render={<Link to={item.to as any} />}
								isActive={isActive}
								tooltip={item.title}
								className={`h-8.5 w-full text-xs font-medium rounded-none transition-all px-3 flex items-center justify-start gap-2.5 ${
									isActive
										? 'bg-primary/10 text-primary font-semibold'
										: 'text-muted-foreground hover:text-foreground hover:bg-muted/40'
								}`}
							>
								<item.icon className={`size-4 shrink-0 ${isActive ? 'text-primary' : 'text-muted-foreground/70'}`} />
								<span className="truncate">{item.title}</span>
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
		<div className="px-3 my-1">
			<Separator className="bg-border/30" />
		</div>
	);
}

export function AppSidebar() {
	const location = useLocation();
	const {isMobile, state} = useSidebar();
	const isCollapsed = state === 'collapsed';

	return (
		<>
			<Sidebar collapsible="icon" variant="sidebar" className="rounded-none border-r border-border/40 bg-card/60 backdrop-blur-xl">
				{/* Brand Header */}
				<SidebarHeader className="border-b border-border/30 px-3 py-3 group-data-[collapsible=icon]:p-1.5">
					<HeaderDropdown isCollapsed={isCollapsed} isMobile={isMobile} />
				</SidebarHeader>

				{/* Navigation Content */}
				<SidebarContent className="gap-1 px-0 py-2">
					{/* Quick Search */}
					<div className="px-3">
						<SearchButton />
					</div>

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
					<SidebarGroup className="py-1 px-0 group-data-[collapsible=icon]:hidden">
						<SidebarGroupLabel className="text-[10px] font-mono tracking-widest text-muted-foreground/60 uppercase px-3 mb-1">
							Extra
						</SidebarGroupLabel>
						<SidebarMenu className="gap-0.5 w-full">
							{MENU.help.map(item => (
								<SidebarMenuItem key={item.title} className="w-full">
									<SidebarMenuButton
										render={
											<a
												href={item.href}
												target="_blank"
												rel="noopener noreferrer"
											/>
										}
										tooltip={item.title}
										className="h-8.5 w-full text-xs font-medium text-muted-foreground hover:text-foreground hover:bg-muted/40 rounded-none px-3 flex items-center justify-start gap-2.5 transition-all"
									>
										<item.icon className="size-4 shrink-0 text-muted-foreground/70" />
										<span className="truncate">{item.title}</span>
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
