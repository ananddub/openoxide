import {UserNav} from './navbar';
import {UpdateServerButton} from './update';
import {SidebarMenu, SidebarMenuItem} from '#/components/ui/sidebar';

type Props = {
	isCollapsed: boolean;
};

// Sidebar footer: update banner, user nav dropdown, and version label.
export function AppSidebarFooter({isCollapsed}: Props) {
	return (
		<div className="flex flex-col gap-3">
			{/* Server Update Banner */}
			<UpdateServerButton isCollapsed={isCollapsed} />
			{/* User Account / Navigation */}
			<SidebarMenu>
				<SidebarMenuItem>
					<UserNav isCollapsed={isCollapsed} />
				</SidebarMenuItem>
			</SidebarMenu>
			{/* Version Footer */}
			{!isCollapsed && (
				<div className="flex flex-col gap-0.5 px-3 text-center text-[10px] text-muted-foreground/60 select-none">
					<div>OpenOxide Self-Hosted</div>
					<div className="text-[9px] opacity-70">Version v0.1.0</div>
				</div>
			)}
		</div>
	);
}
