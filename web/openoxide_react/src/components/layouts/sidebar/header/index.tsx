import * as React from 'react';
import {useOrganizationListByOwner} from 'virtual:openoxide-live';

import {cn} from '#/api/utils';
import {Logo} from '#/components/shared/logo';
import {
	SidebarMenu,
	SidebarMenuButton,
	SidebarMenuItem,
} from '#/components/ui/sidebar';
import {useOrganizationStore} from '#/stores/organization-store';
import {NotificationBell} from './notification';

type Props = {
	isCollapsed: boolean;
	isMobile: boolean;
};

export function HeaderDropdown({isCollapsed, isMobile}: Props) {
	const {data: organizations} = useOrganizationListByOwner();
	const setOrganizations = useOrganizationStore(
		state => state.setOrganizations,
	);

	React.useEffect(() => {
		if (organizations) {
			setOrganizations(organizations as any);
		}
	}, [organizations, setOrganizations]);

	return (
		<SidebarMenu
			className={cn(
				'flex w-full gap-2 p-0',
				isCollapsed ? 'flex-col' : 'flex-row items-center justify-between',
			)}>
			<SidebarMenuItem className="min-w-0 grow">
				<SidebarMenuButton
					size={isCollapsed ? 'sm' : 'lg'}
					className={cn(
						'cursor-default hover:bg-transparent active:bg-transparent',
						isCollapsed &&
							'mx-auto flex h-10 w-10 items-center justify-center rounded-md p-0 group-data-[collapsible=icon]:size-10!',
					)}>
					<div
						className={cn(
							'flex items-center gap-2',
							isCollapsed && 'w-full justify-center',
						)}>
						<Logo
							className={cn(
								'shrink-0 text-primary',
								isCollapsed ? 'size-5' : 'size-8',
							)}
						/>
						<span
							className={cn(
								'text-sm leading-tight font-semibold text-foreground',
								isCollapsed && 'hidden',
							)}>
							OpenOxide
						</span>
					</div>
				</SidebarMenuButton>
			</SidebarMenuItem>
			<SidebarMenuItem
				className={cn('shrink-0', isCollapsed ? 'mx-auto mt-1' : '')}>
				<NotificationBell isCollapsed={isCollapsed} isMobile={isMobile} />
			</SidebarMenuItem>
		</SidebarMenu>
	);
}
