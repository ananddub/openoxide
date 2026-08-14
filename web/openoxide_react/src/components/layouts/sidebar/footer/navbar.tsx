import {
	ChevronsUpDown,
	LogOut,
	User,
	Folder,
	Monitor,
	ChartLine,
	Globe,
	Package,
	Users,
	Key,
	Sun,
	Moon,
} from 'lucide-react';
import {useAuthStore} from '#/stores/auth-store';
import {useNavigate} from '@tanstack/react-router';
import {isSolidColorAvatar} from '#/lib/avatar-utils';
import {useAuthWhoAmI} from 'virtual:openoxide-live';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuGroup,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuSeparator,
} from '#/components/ui/dropdown';
import {SidebarMenuButton} from '#/components/ui/sidebar';
import {useTheme} from '#/hooks/use-theme';

type Props = {
	isCollapsed: boolean;
};

// User account dropdown with quick nav links, theme toggle, and logout.
export function UserNav({isCollapsed}: Props) {
	const user = useAuthStore(state => state.user);
	const logout = useAuthStore(state => state.logout);
	const navigate = useNavigate();
	const {theme, toggleTheme} = useTheme();

	const {data: whoamiData} = useAuthWhoAmI();

	const displayEmail = whoamiData
		? (whoamiData.email ?? '')
		: (user?.email ?? '');
	const displayFirstName = whoamiData
		? (whoamiData.first_name ?? '')
		: (user?.firstName ?? '');
	const displayLastName = whoamiData
		? (whoamiData.last_name ?? '')
		: (user?.lastName ?? '');

	const getInitials = () => {
		if (displayFirstName && displayLastName) {
			return `${displayFirstName[0]}${displayLastName[0]}`.toUpperCase();
		}
		if (displayEmail) {
			return displayEmail[0].toUpperCase();
		}
		return 'U';
	};

	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<SidebarMenuButton
						size="lg"
						className="cursor-pointer data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground group-data-[collapsible=icon]:p-0! group-data-[collapsible=icon]:justify-center! group-data-[collapsible=icon]:mx-auto!"
					/>
				}>
				{whoamiData?.avatar && (whoamiData.avatar.startsWith('data:') || whoamiData.avatar.startsWith('http://') || whoamiData.avatar.startsWith('https://')) ? (
					<img src={whoamiData.avatar} alt="Avatar" className="size-8 rounded-full object-cover shrink-0 select-none group-data-[collapsible=icon]:mx-auto" />
				) : isSolidColorAvatar(whoamiData?.avatar) ? (
					<div
						className="flex size-8 shrink-0 items-center justify-center rounded-full text-xs font-bold text-white select-none group-data-[collapsible=icon]:mx-auto shadow-2xs"
						style={{backgroundColor: whoamiData?.avatar}}>
						{getInitials()}
					</div>
				) : (
					<div className="flex size-8 shrink-0 items-center justify-center rounded-full border border-primary/20 bg-primary/10 text-xs font-bold text-primary select-none group-data-[collapsible=icon]:mx-auto shadow-2xs">
						{getInitials()}
					</div>
				)}

				<div className="grid flex-1 text-left text-sm leading-tight select-none group-data-[collapsible=icon]:hidden">
					<span className="truncate text-xs font-semibold text-foreground">
						{displayFirstName && displayLastName
							? `${displayFirstName} ${displayLastName}`
							: displayEmail || 'Account'}
					</span>
					<span className="truncate text-[10px] text-muted-foreground">
						{displayEmail}
					</span>
				</div>
				<ChevronsUpDown className="ml-auto size-4 group-data-[collapsible=icon]:hidden" />
			</DropdownMenuTrigger>
			<DropdownMenuContent
				className="w-60 rounded-lg"
				side={isCollapsed ? 'right' : 'top'}
				align="end"
				sideOffset={10}>
				<DropdownMenuGroup>
					<div className="flex items-center justify-between px-2 py-1.5">
						<DropdownMenuLabel className="flex flex-col gap-0.5 p-0">
							My Account
							<span className="max-w-36 truncate text-[10px] font-normal text-muted-foreground">
								{displayEmail}
							</span>
						</DropdownMenuLabel>
						<button
							onClick={e => {
								e.stopPropagation();
								toggleTheme();
							}}
							className="flex size-7 cursor-pointer items-center justify-center rounded-md border border-border/50 text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
							title="Toggle Theme">
							{theme === 'dark' ? (
								<Sun className="size-3.5 text-yellow-500" />
							) : (
								<Moon className="size-3.5 text-blue-500" />
							)}
						</button>
					</div>
				</DropdownMenuGroup>
				<DropdownMenuSeparator />
				<DropdownMenuGroup>
					<DropdownMenuItem
						className="flex cursor-pointer items-center gap-2"
						onClick={() => navigate({to: '/settings/profile'})}>
						<User className="size-3.5 text-muted-foreground" />
						Profile
					</DropdownMenuItem>
				</DropdownMenuGroup>
				<DropdownMenuSeparator />
				<DropdownMenuGroup>
					<DropdownMenuLabel className="px-2 py-1 text-[9px] font-semibold text-muted-foreground uppercase select-none">
						Platform
					</DropdownMenuLabel>
					<DropdownMenuItem
						className="flex cursor-pointer items-center gap-2"
						onClick={() => navigate({to: '/projects'})}>
						<Folder className="size-3.5 text-muted-foreground" />
						Projects
					</DropdownMenuItem>
					<DropdownMenuItem
						className="flex cursor-pointer items-center gap-2"
						onClick={() => navigate({to: '/monitoring'})}>
						<ChartLine className="size-3.5 text-muted-foreground" />
						Monitoring
					</DropdownMenuItem>
					<DropdownMenuItem
						className="flex cursor-pointer items-center gap-2"
						onClick={() => navigate({to: '/docker'})}>
						<Globe className="size-3.5 text-muted-foreground" />
						Traefik
					</DropdownMenuItem>
					<DropdownMenuItem
						className="flex cursor-pointer items-center gap-2"
						onClick={() => navigate({to: '/docker'})}>
						<Package className="size-3.5 text-muted-foreground" />
						Docker
					</DropdownMenuItem>
				</DropdownMenuGroup>
				<DropdownMenuSeparator />
				<DropdownMenuGroup>
					<DropdownMenuLabel className="px-2 py-1 text-[9px] font-semibold text-muted-foreground uppercase select-none">
						Administration
					</DropdownMenuLabel>
					<DropdownMenuItem
						className="flex cursor-pointer items-center gap-2"
						onClick={() => navigate({to: '/remote-servers' as any})}>
						<Monitor className="size-3.5 text-muted-foreground" />
						Remote Servers
					</DropdownMenuItem>
					<DropdownMenuItem
						className="flex cursor-pointer items-center gap-2"
						onClick={() => navigate({to: '/settings/users' as any})}>
						<Users className="size-3.5 text-muted-foreground" />
						Users
					</DropdownMenuItem>
					<DropdownMenuItem
						className="flex cursor-pointer items-center gap-2"
						onClick={() => navigate({to: '/ssh-keys' as any})}>
						<Key className="size-3.5 text-muted-foreground" />
						SSH Keys
					</DropdownMenuItem>
				</DropdownMenuGroup>
				<DropdownMenuSeparator />
				<DropdownMenuItem
					className="flex cursor-pointer items-center gap-2 text-destructive focus:bg-destructive/10 focus:text-destructive"
					onClick={() => logout()}>
					<LogOut className="size-3.5" />
					Log out
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}
