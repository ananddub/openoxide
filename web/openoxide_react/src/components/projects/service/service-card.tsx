import React, {useState, useEffect} from 'react';
import {MoreVertical, Trash2, Server} from 'lucide-react';
import {Link} from '@tanstack/react-router';
import {cn} from '#/api/utils';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
} from '#/components/ui/dropdown';
import {DeleteServiceDialog} from './delete-service-dialog';
import {ServiceTypeIcon, getStatusDotColor} from './service-card-icon';
import {useServiceCard, type ServiceCardProps} from './use-service-card';

export type {ServiceCardProps} from './use-service-card';

export function ServiceCard(props: ServiceCardProps) {
	const {type, name, subtitle, createdAt, dbKind, serverName} = props;
	const {routeConfig, actions, statusKey, confirmDeleteService, ...flags} =
		useServiceCard(props);

	const [contextMenu, setContextMenu] = useState<{
		x: number;
		y: number;
	} | null>(null);
	const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);

	// Close context menu on outside click or scroll
	useEffect(() => {
		if (!contextMenu) return;
		const handleClose = () => setContextMenu(null);
		window.addEventListener('click', handleClose);
		window.addEventListener('scroll', handleClose, true);
		return () => {
			window.removeEventListener('click', handleClose);
			window.removeEventListener('scroll', handleClose, true);
		};
	}, [contextMenu]);

	const dotColor = getStatusDotColor({...flags, statusKey});
	const closeMenu = () => setContextMenu(null);
	const openContextMenu = (e: React.MouseEvent) => {
		e.preventDefault();
		e.stopPropagation();
		setContextMenu({x: e.clientX, y: e.clientY});
	};
	const handleDelete = (e?: React.MouseEvent) => {
		e?.stopPropagation();
		closeMenu();
		setIsDeleteDialogOpen(true);
	};

	return (
		<>
			<div
				onContextMenu={openContextMenu}
				className="group relative block flex w-full cursor-pointer flex-col justify-between gap-3.5 overflow-hidden rounded-xl bg-card p-4 text-left shadow-2xs transition-all duration-200 ">
				{/* Clickable Card Overlay */}
				<Link
					to={routeConfig.to as any}
					params={routeConfig.params as any}
					search={(routeConfig as any).search}
					preload="intent"
					className="absolute inset-0 z-0 rounded-xl"
					aria-label={`Open ${name}`}
				/>

				<div className="pointer-events-none relative z-10 flex items-center justify-between gap-3">
					<div className="flex min-w-0 items-center gap-3">
						<div className="relative flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted/40 text-foreground">
							<ServiceTypeIcon type={type} dbKind={dbKind} />
							<span
								className={cn(
									'absolute -top-0.5 -right-0.5 size-2 rounded-full border-2 border-card',
									dotColor,
								)}
							/>
						</div>
						<div className="min-w-0">
							<p className="truncate text-xs leading-snug font-bold text-foreground transition-colors group-hover:text-primary">
								{name}
							</p>
							<p className="font-mono text-[10px] tracking-wider text-muted-foreground uppercase">
								{subtitle}
							</p>
						</div>
					</div>

					<div className="pointer-events-auto shrink-0">
						<DropdownMenu>
							<DropdownMenuTrigger
								render={
									<button className="flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-md text-muted-foreground opacity-0 transition-opacity group-hover:opacity-100 hover:bg-muted/80 hover:text-foreground focus:outline-none">
										<MoreVertical className="size-4 text-foreground" />
									</button>
								}
							/>
							<DropdownMenuContent
								align="end"
								className="w-40 bg-popover/95 shadow-lg backdrop-blur-md">
								{actions.map(({id, label, Icon, onClick, disabled}) => (
									<DropdownMenuItem
										key={id}
										disabled={disabled}
										onClick={e => onClick(e)}
										className={cn(
											'flex cursor-pointer items-center gap-2 py-1.5 text-xs font-medium text-foreground',
											disabled && 'cursor-not-allowed opacity-40',
										)}>
										<Icon className="size-3.5 text-foreground" />
										{label}
									</DropdownMenuItem>
								))}
								<DropdownMenuSeparator className="bg-border/60" />
								<DropdownMenuItem
									onClick={handleDelete}
									className="flex cursor-pointer items-center gap-2 py-1.5 text-xs font-medium text-destructive hover:bg-destructive/10 focus:bg-destructive/10 focus:text-destructive">
									<Trash2 className="size-3.5 text-destructive" />
									Delete
								</DropdownMenuItem>
							</DropdownMenuContent>
						</DropdownMenu>
					</div>
				</div>

				<div className="pointer-events-none relative z-10 flex flex-col gap-2 border-t  pt-2.5">
					<div
						className="flex min-w-0 items-center gap-1.5 text-[10px] text-muted-foreground"
						title="Deployment server">
						<Server className="size-3 shrink-0" />
						<span className="shrink-0">Server</span>
						<span className="truncate font-semibold text-foreground">
							{serverName || 'Localhost (Main server)'}
						</span>
					</div>
					<div className="flex items-center justify-between">
						<span className="flex items-center gap-1.5 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
							<span className={cn('size-1.5 rounded-full', dotColor)} />
							{statusKey || 'idle'}
						</span>
						<span className="font-mono text-[10px] text-muted-foreground/70">
							{new Date(Number(createdAt) * 1000).toLocaleDateString(
								undefined,
								{day: '2-digit', month: 'short'},
							)}
						</span>
					</div>
				</div>
			</div>

			{/* Custom Right-Click Context Menu */}
			{contextMenu && (
				<div
					style={{top: `${contextMenu.y}px`, left: `${contextMenu.x}px`}}
					className="fixed z-50 w-44 animate-in rounded-lg bg-popover/95 p-1 text-foreground shadow-md backdrop-blur-md duration-100 fade-in"
					onClick={e => e.stopPropagation()}>
					{actions.map(({id, label, Icon, onClick, disabled}) => (
						<button
							key={id}
							disabled={disabled}
							onClick={onClick}
							className={cn(
								'flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-muted',
								disabled &&
									'cursor-not-allowed opacity-40 hover:bg-transparent',
							)}>
							<Icon className="size-3.5 text-foreground" />
							{label}
						</button>
					))}
					<div className="my-1 h-px bg-border/60" />
					<button
						onClick={handleDelete}
						className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs font-medium text-destructive transition-colors hover:bg-destructive/10">
						<Trash2 className="size-3.5 text-destructive" />
						Delete
					</button>
				</div>
			)}

			<DeleteServiceDialog
				isOpen={isDeleteDialogOpen}
				onClose={() => setIsDeleteDialogOpen(false)}
				serviceName={name}
				serviceType={type === 'DATABASE' ? dbKind || 'Database' : type}
				onConfirm={confirmDeleteService}
			/>
		</>
	);
}
