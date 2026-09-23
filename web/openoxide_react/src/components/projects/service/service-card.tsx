import React, {useMemo, useState, useEffect} from 'react';
import {MoreVertical, Trash2, Server, Layers} from 'lucide-react';
import {Link} from '@tanstack/react-router';
import {cn} from '#/api/utils';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
} from '#/components/ui/dropdown';
import {
	Sheet,
	SheetContent,
	SheetHeader,
	SheetTitle,
	SheetDescription,
} from '#/components/ui/sheet';
import {
	useEnvironmentListByProject,
	type EnvironmentPayload,
} from '#/api/live/environments';
import {DeleteServiceDialog} from './delete-service-dialog';
import {ServiceTypeIcon, getStatusDotColor} from './service-card-icon';
import {useServiceCard, type ServiceCardProps} from './use-service-card';

export type {ServiceCardProps} from './use-service-card';

/** Variable names from a raw .env-style string (values stay hidden). */
function envVarKeys(raw?: string): string[] {
	return (raw || '')
		.split('\n')
		.map(line => line.trim())
		.filter(line => line && !line.startsWith('#') && line.includes('='))
		.map(line => line.slice(0, line.indexOf('=')).trim());
}

function EnvSection({
	envs,
	onPick,
}: {
	envs: EnvironmentPayload[];
	onPick: (env: EnvironmentPayload) => void;
}) {
	if (envs.length === 0) return null;
	return (
		<>
			<div className="px-2 py-1 text-[10px] font-semibold tracking-wider text-muted-foreground uppercase">
				Environments
			</div>
			{envs.map(env => (
				<DropdownMenuItem
					key={String(env.id)}
					onClick={() => onPick(env)}
					className="flex cursor-pointer items-center gap-2 py-1.5 text-xs font-medium text-foreground">
					<Layers className="size-3.5 shrink-0 text-muted-foreground" />
					<span className="truncate">
						{env.name}
						{env.is_default && (
							<span className="text-muted-foreground">
								{' '}
								(Default)
							</span>
						)}
					</span>
					<span className="ml-auto shrink-0 font-mono text-[10px] text-muted-foreground">
						{envVarKeys(env.env_var).length} vars
					</span>
				</DropdownMenuItem>
			))}
		</>
	);
}

export function ServiceCard(props: ServiceCardProps) {
	const {projectId, type, name, subtitle, createdAt, dbKind, serverName} =
		props;
	const {routeConfig, actions, statusKey, confirmDeleteService, ...flags} =
		useServiceCard(props);

	const [contextMenu, setContextMenu] = useState<{
		x: number;
		y: number;
	} | null>(null);
	const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
	const [envSheet, setEnvSheet] = useState<EnvironmentPayload | null>(null);

	const projectIdBig = useMemo(
		() => BigInt(Number(projectId) || 0),
		[projectId],
	);
	const {data: envs = []} = useEnvironmentListByProject(projectIdBig);
	const openEnvSheet = (env: EnvironmentPayload) => {
		setContextMenu(null);
		setEnvSheet(env);
	};

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
									<button
										title={`${name} actions`}
										aria-label={`${name} actions`}
										className="flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-md text-muted-foreground opacity-0 transition-opacity group-hover:opacity-100 hover:bg-muted/80 hover:text-foreground focus:outline-none">
										<MoreVertical className="size-4 text-foreground" />
									</button>
								}
							/>
							<DropdownMenuContent
								align="end"
								className="w-52 bg-popover/95 shadow-lg backdrop-blur-md">
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
								<EnvSection envs={envs} onPick={openEnvSheet} />
								{envs.length > 0 && (
									<DropdownMenuSeparator className="bg-border/60" />
								)}
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
					{envs.length > 0 && (
						<>
							<div className="px-2.5 py-1 text-[10px] font-semibold tracking-wider text-muted-foreground uppercase">
								Environments
							</div>
							{envs.map(env => (
								<button
									key={String(env.id)}
									onClick={() => openEnvSheet(env)}
									className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-muted">
									<Layers className="size-3.5 shrink-0 text-muted-foreground" />
									<span className="truncate">{env.name}</span>
									<span className="ml-auto shrink-0 font-mono text-[10px] text-muted-foreground">
										{envVarKeys(env.env_var).length}
									</span>
								</button>
							))}
							<div className="my-1 h-px bg-border/60" />
						</>
					)}
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

			{/* Environment basics sidebar — keeps the menu itself clean */}
			<Sheet
				open={!!envSheet}
				onOpenChange={open => {
					if (!open) setEnvSheet(null);
				}}>
				<SheetContent side="right">
					<SheetHeader>
						<SheetTitle className="flex items-center gap-2">
							{envSheet?.name}
							{envSheet?.is_default && (
								<span className="rounded-full border border-border px-2 py-0.5 text-[10px] font-semibold text-muted-foreground">
									Default
								</span>
							)}
						</SheetTitle>
						<SheetDescription>
							{envSheet?.description?.trim() ||
								'Basic environment details'}
						</SheetDescription>
					</SheetHeader>

					<div className="flex-1 overflow-y-auto px-6 py-6">
						<div className="flex items-center justify-between border-b border-border pb-3">
							<span className="text-xs font-semibold tracking-wider text-muted-foreground uppercase">
								Variables
							</span>
							<span className="font-mono text-xs text-muted-foreground">
								{envVarKeys(envSheet?.env_var).length} keys
							</span>
						</div>
						{envVarKeys(envSheet?.env_var).length > 0 ? (
							<ul className="divide-y divide-border/60">
								{envVarKeys(envSheet?.env_var).map(key => (
									<li
										key={key}
										className="flex items-center py-2">
										<span className="truncate font-mono text-xs text-foreground">
											{key}
										</span>
									</li>
								))}
							</ul>
						) : (
							<p className="py-4 text-xs text-muted-foreground">
								No variables defined.
							</p>
						)}
						{envSheet && (
							<p className="mt-4 text-[11px] text-muted-foreground">
								Edited from Projects → Environments. Values
								are hidden here to keep this view quick to
								scan.
							</p>
						)}
					</div>
				</SheetContent>
			</Sheet>
		</>
	);
}
