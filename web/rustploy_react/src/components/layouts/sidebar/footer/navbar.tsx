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

	// Derives initials from name if available, otherwise falls back to email first char.
	const getInitials = () => {
		if (!user) return '?';
		if (user.firstName && user.lastName) {
			return `${user.firstName[0]}${user.lastName[0]}`.toUpperCase();
		}
		return user.email[0].toUpperCase();
	};

	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<SidebarMenuButton
						size="lg"
						className="cursor-pointer data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
					/>
				}>
				<div className="flex size-8 shrink-0 items-center justify-center rounded-lg border border-border/40 bg-muted text-xs font-semibold text-muted-foreground select-none">
					{getInitials()}
				</div>
				<div className="grid flex-1 text-left text-sm leading-tight select-none group-data-[collapsible=icon]:hidden">
					<span className="truncate text-xs font-semibold text-foreground">
						{user?.firstName && user?.lastName
							? `${user.firstName} ${user.lastName}`
							: 'Account'}
					</span>
					<span className="truncate text-[10px] text-muted-foreground">
						{user?.email}
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
								{user?.email}
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
						onClick={() => navigate({to: '/settings/profile' as any})}>
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
						onClick={() => navigate({to: '/projects' as any})}>
						<Folder className="size-3.5 text-muted-foreground" />
						Projects
					</DropdownMenuItem>
					<DropdownMenuItem
						className="flex cursor-pointer items-center gap-2"
						onClick={() => navigate({to: '/monitoring' as any})}>
						<ChartLine className="size-3.5 text-muted-foreground" />
						Monitoring
					</DropdownMenuItem>
					<DropdownMenuItem
						className="flex cursor-pointer items-center gap-2"
						onClick={() => navigate({to: '/traefik' as any})}>
						<Globe className="size-3.5 text-muted-foreground" />
						Traefik
					</DropdownMenuItem>
					<DropdownMenuItem
						className="flex cursor-pointer items-center gap-2"
						onClick={() => navigate({to: '/docker' as any})}>
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
