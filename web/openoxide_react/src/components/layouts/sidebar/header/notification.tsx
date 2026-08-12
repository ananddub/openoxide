import * as React from 'react';
import {
	Bell,
	AlertCircle,
	AlertTriangle,
	Check,
	Info,
	Trash2,
} from 'lucide-react';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuLabel,
	DropdownMenuGroup,
} from '#/components/ui/dropdown';
import {cn} from '#/api/utils';

type Props = {
	isCollapsed: boolean;
	isMobile: boolean;
};

type NotificationItem = {
	id: string;
	type: 'success' | 'warning' | 'error' | 'info';
	title: string;
	description: string;
	time: string;
	read: boolean;
};

// Bell icon with unread badge. Dropdown lists notifications with per-item read/dismiss actions.
export function NotificationBell({isCollapsed, isMobile}: Props) {
	// Mock real-world PaaS notifications
	const [notifications, setNotifications] = React.useState<
		NotificationItem[]
	>([
		{
			id: 'notif-1',
			type: 'error',
			title: 'Deployment Failed',
			description: "Project 'goploy-api' build failed on commit #8fa23.",
			time: '5m ago',
			read: false,
		},
		{
			id: 'notif-2',
			type: 'warning',
			title: 'High CPU Usage',
			description: "Server 'VPS-1' CPU utilization exceeded 90%.",
			time: '20m ago',
			read: false,
		},
		{
			id: 'notif-3',
			type: 'success',
			title: 'Backup Successful',
			description: "Daily backup for 'Postgres-Prod' uploaded to S3.",
			time: '1h ago',
			read: true,
		},
		{
			id: 'notif-4',
			type: 'info',
			title: 'SSL Auto-Renewed',
			description: "SSL certificate for 'api.goploy.dev' renewed.",
			time: '3h ago',
			read: true,
		},
	]);

	const unreadCount = notifications.filter(n => !n.read).length;

	const handleMarkAsRead = (id: string, e: React.MouseEvent) => {
		e.stopPropagation();
		setNotifications(prev =>
			prev.map(n => (n.id === id ? {...n, read: true} : n)),
		);
	};

	const handleMarkAllAsRead = (e: React.MouseEvent) => {
		e.stopPropagation();
		setNotifications(prev => prev.map(n => ({...n, read: true})));
	};

	const handleClearAll = (e: React.MouseEvent) => {
		e.stopPropagation();
		setNotifications([]);
	};

	// Maps notification type to its corresponding colored icon.
	const getIcon = (type: NotificationItem['type']) => {
		switch (type) {
			case 'error':
				return (
					<AlertCircle className="size-4 shrink-0 text-destructive" />
				);
			case 'warning':
				return (
					<AlertTriangle className="size-4 shrink-0 text-yellow-500" />
				);
			case 'success':
				return <Check className="size-4 shrink-0 text-green-500" />;
			case 'info':
				return <Info className="size-4 shrink-0 text-blue-500" />;
		}
	};

	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<button
						className={cn(
							'relative flex shrink-0 cursor-pointer items-center justify-center rounded-md bg-transparent text-muted-foreground transition-colors hover:bg-muted/40 hover:text-foreground',
							isCollapsed ? 'size-8' : 'size-9',
						)}
						title="Notifications"
					/>
				}>
				<Bell className="size-4" />
				{unreadCount > 0 && (
					<span className="absolute -top-1 -right-1 flex h-4 min-w-4 items-center justify-center rounded-full bg-blue-500 px-1 text-[9px] font-medium text-white">
						{unreadCount}
					</span>
				)}
			</DropdownMenuTrigger>
			<DropdownMenuContent
				align="end"
				side={isMobile ? 'bottom' : 'right'}
				sideOffset={4}
				className="flex max-h-[min(70vh,28rem)] w-80 flex-col p-0">
				<DropdownMenuGroup>
					<div className="flex shrink-0 items-center justify-between border-b border-border/50 px-3 py-2">
						<DropdownMenuLabel className="p-0 text-xs font-semibold text-muted-foreground">
							Notifications
						</DropdownMenuLabel>
						{notifications.length > 0 && (
							<div className="flex gap-2">
								<button
									onClick={handleMarkAllAsRead}
									className="cursor-pointer text-[10px] text-primary hover:underline">
									Mark all read
								</button>
								<span className="text-[10px] text-muted-foreground">
									|
								</span>
								<button
									onClick={handleClearAll}
									className="flex cursor-pointer items-center gap-0.5 text-[10px] text-destructive hover:underline">
									<Trash2 className="size-2.5" />
									Clear
								</button>
							</div>
						)}
					</div>
					<div className="flex min-h-0 flex-col divide-y divide-border/30 overflow-y-auto">
						{notifications.length > 0 ? (
							notifications.map(notif => (
								<div
									key={notif.id}
									className={cn(
										'flex gap-2.5 p-3 transition-colors',
										notif.read ? 'opacity-65' : 'bg-blue-500/5',
									)}>
									{getIcon(notif.type)}
									<div className="flex min-w-0 flex-1 flex-col gap-0.5">
										<div className="flex items-center justify-between gap-1.5">
											<span className="truncate text-xs font-semibold text-foreground">
												{notif.title}
											</span>
											<span className="shrink-0 text-[9px] text-muted-foreground">
												{notif.time}
											</span>
										</div>
										<p className="text-[10px] leading-normal wrap-break-word text-muted-foreground">
											{notif.description}
										</p>
									</div>
									{!notif.read && (
										<button
											onClick={e => handleMarkAsRead(notif.id, e)}
											className="shrink-0 cursor-pointer self-start rounded-sm p-1 text-muted-foreground hover:bg-muted hover:text-foreground"
											title="Mark as read">
											<Check className="size-3" />
										</button>
									)}
								</div>
							))
						) : (
							<div className="p-6 text-center text-xs text-muted-foreground">
								No notifications
							</div>
						)}
					</div>
				</DropdownMenuGroup>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}
