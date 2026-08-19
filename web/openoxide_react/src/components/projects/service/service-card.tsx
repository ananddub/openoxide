import React, {useState, useEffect, useMemo} from 'react';
import {
	Box,
	Layers2,
	Database as DbIcon,
	Play,
	Square,
	Rocket,
	Trash2,
	Eye,
	MoreVertical,
	XCircle,
} from 'lucide-react';
import {useNavigate, Link} from '@tanstack/react-router';
import {cn} from '#/api/utils';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
	MongodbIcon,
	RedisIcon,
	LibsqlIcon,
} from '#/components/icons/db-icons';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
} from '#/components/ui/dropdown';
import {DeleteServiceDialog} from './delete-service-dialog';

interface ServiceCardProps {
	projectId: number;
	type: 'APP' | 'COMPOSE' | 'DATABASE';
	id: number;
	name: string;
	subtitle: string;
	status: string;
	createdAt: number;
	dbKind?: string;
	onDeleted?: () => void;
}

export function ServiceCard({
	projectId,
	type,
	id,
	name,
	subtitle,
	status,
	createdAt,
	dbKind,
	onDeleted,
}: ServiceCardProps) {
	const navigate = useNavigate();
	const [contextMenu, setContextMenu] = useState<{
		x: number;
		y: number;
	} | null>(null);
	const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);

	const routeConfig = useMemo(() => {
		if (type === 'APP') {
			return {
				to: '/projects/$id/app/$appId' as const,
				params: {id: String(projectId), appId: String(id)},
			};
		}
		if (type === 'COMPOSE') {
			return {
				to: '/projects/$id/compose/$composeId' as const,
				params: {id: String(projectId), composeId: String(id)},
			};
		}
		return {
			to: '/projects/$id/database/$dbId' as const,
			params: {id: String(projectId), dbId: String(id)},
			search: {kind: dbKind || 'postgres'},
		};
	}, [type, projectId, id, dbKind]);

	// Close context menu on outside click or scroll
	useEffect(() => {
		const handleClose = () => setContextMenu(null);
		if (contextMenu) {
			window.addEventListener('click', handleClose);
			window.addEventListener('scroll', handleClose, true);
		}
		return () => {
			window.removeEventListener('click', handleClose);
			window.removeEventListener('scroll', handleClose, true);
		};
	}, [contextMenu]);

	// Status Flags
	const s = status?.toLowerCase() || '';

	const isStopping = s.includes('stopping') || s.includes('cancelling');

	const isStartingOrDeploying =
		!isStopping &&
		(s.includes('starting') ||
			s.includes('deploying') ||
			s.includes('building') ||
			s.includes('loading'));

	const isRunning =
		!isStopping &&
		!isStartingOrDeploying &&
		(s.includes('running') ||
			s.includes('active') ||
			s.includes('healthy') ||
			s.includes('up'));

	const isStopped = !isStopping && !isStartingOrDeploying && !isRunning;

	// App Mutations
	const appStart = $api.useMutation('post', '/applications/{id}/start');
	const appStop = $api.useMutation('post', '/applications/{id}/stop');
	const appDeploy = $api.useMutation('post', '/applications/{id}/deploy');
	const appCancel = $api.useMutation('post', '/applications/{id}/cancel');
	const appDelete = $api.useMutation('delete', '/applications/{id}');

	// Compose Mutations
	const composeStart = $api.useMutation('post', '/compose/{id}/start');
	const composeStop = $api.useMutation('post', '/compose/{id}/stop');
	const composeDeploy = $api.useMutation('post', '/compose/{id}/deploy');
	const composeCancel = $api.useMutation('post', '/compose/{id}/cancel');
	const composeDelete = $api.useMutation('delete', '/compose/{id}');

	// Database Mutations
	const getDbKindPath = () => {
		const k = (dbKind || '').toLowerCase();
		if (k.includes('mysql')) return 'mysql';
		if (k.includes('mariadb')) return 'mariadb';
		if (k.includes('mongo')) return 'mongo';
		if (k.includes('redis')) return 'redis';
		if (k.includes('libsql')) return 'libsql';
		return 'postgres';
	};

	const dbKindPath = getDbKindPath();
	const dbStart = $api.useMutation(
		'post',
		`/${dbKindPath}/{id}/start` as any,
	);
	const dbStop = $api.useMutation(
		'post',
		`/${dbKindPath}/{id}/stop` as any,
	);
	const dbDeploy = $api.useMutation(
		'post',
		`/${dbKindPath}/{id}/deploy` as any,
	);
	const dbCancel = $api.useMutation(
		'post',
		`/${dbKindPath}/{id}/cancel` as any,
	);
	const dbDelete = $api.useMutation(
		'delete',
		`/${dbKindPath}/{id}` as any,
	);

	const getStatusDotColor = (status: string) => {
		if (isStopping) {
			return 'bg-orange-500 animate-pulse';
		}
		if (isRunning) {
			return 'bg-emerald-500';
		}
		if (
			s.includes('error') ||
			s.includes('fail') ||
			s.includes('unhealthy') ||
			s.includes('crash')
		) {
			return 'bg-rose-500';
		}
		if (isStartingOrDeploying) {
			return 'bg-amber-500 animate-pulse';
		}
		return 'bg-muted-foreground/40';
	};

	const getIcon = () => {
		if (type === 'APP') {
			return <Box className="size-4 text-foreground" />;
		}
		if (type === 'COMPOSE') {
			return <Layers2 className="size-4 text-foreground" />;
		}
		const kind = (dbKind || '').toLowerCase();
		if (kind.includes('postgres'))
			return <PostgresqlIcon className="size-5 shrink-0" />;
		if (kind.includes('mysql'))
			return <MysqlIcon className="size-5 shrink-0" />;
		if (kind.includes('mariadb'))
			return <MariadbIcon className="size-5 shrink-0" />;
		if (kind.includes('mongo'))
			return <MongodbIcon className="size-5 shrink-0" />;
		if (kind.includes('redis'))
			return <RedisIcon className="size-5 shrink-0" />;
		if (kind.includes('libsql'))
			return <LibsqlIcon className="size-5 shrink-0" />;

		return <DbIcon className="size-4 text-foreground" />;
	};

	const handleNavigate = () => {
		if (type === 'APP') {
			navigate({
				to: '/projects/$id/app/$appId',
				params: {id: String(projectId), appId: String(id)},
			});
		} else if (type === 'COMPOSE') {
			navigate({
				to: '/projects/$id/compose/$composeId',
				params: {id: String(projectId), composeId: String(id)},
			});
		} else {
			navigate({
				to: '/projects/$id/database/$dbId',
				params: {id: String(projectId), dbId: String(id)},
				search: {kind: dbKind || 'postgres'} as any,
			});
		}
	};

	const handleContextMenu = (e: React.MouseEvent) => {
		e.preventDefault();
		e.stopPropagation();
		setContextMenu({x: e.clientX, y: e.clientY});
	};

	const handleStart = async (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		if (isRunning || isStartingOrDeploying || isStopping) return;
		try {
			if (type === 'APP') {
				await appStart.mutateAsync({params: {path: {id}}});
			} else if (type === 'COMPOSE') {
				await composeStart.mutateAsync({params: {path: {id}}});
			} else {
				await dbStart.mutateAsync({params: {path: {id}}});
			}
			toast.success(`Starting ${name}...`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const handleStop = async (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		if (isStopped || isStopping) return;
		try {
			if (type === 'APP') {
				await appStop.mutateAsync({params: {path: {id}}});
			} else if (type === 'COMPOSE') {
				await composeStop.mutateAsync({params: {path: {id}}});
			} else {
				await dbStop.mutateAsync({params: {path: {id}}});
			}
			toast.success(`Stopping ${name}...`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const handleCancel = async (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		if (isStopping) return;
		try {
			if (type === 'APP') {
				await appCancel.mutateAsync({params: {path: {id}}});
			} else if (type === 'COMPOSE') {
				await composeCancel.mutateAsync({params: {path: {id}}});
			} else {
				await dbCancel.mutateAsync({params: {path: {id}}});
			}
			toast.success(`Cancelling ${name}...`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const handleDeploy = async (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		if (isStartingOrDeploying || isStopping) return;
		try {
			if (type === 'APP') {
				await appDeploy.mutateAsync({params: {path: {id}}});
			} else if (type === 'COMPOSE') {
				await composeDeploy.mutateAsync({params: {path: {id}}});
			} else {
				await dbDeploy.mutateAsync({params: {path: {id}}});
			}
			toast.success(`Deploying ${name}...`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const handleDelete = (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		setIsDeleteDialogOpen(true);
	};

	const confirmDeleteService = async () => {
		try {
			if (type === 'APP') {
				await appDelete.mutateAsync({params: {path: {id}}});
			} else if (type === 'COMPOSE') {
				await composeDelete.mutateAsync({params: {path: {id}}});
			} else {
				await dbDelete.mutateAsync({params: {path: {id}}});
			}
			toast.success(`Deleted ${name}`);
			onDeleted?.();
		} catch (err) {
			toast.error(formatApiError(err));
			throw err;
		}
	};

	return (
		<>
			<div
				onContextMenu={handleContextMenu}
				className="group relative block flex w-full cursor-pointer flex-col justify-between gap-3.5 overflow-hidden rounded-xl border border-border bg-card p-4 text-left shadow-2xs transition-all duration-200 hover:border-primary/40">
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
						<div className="relative flex size-9 shrink-0 items-center justify-center rounded-lg border border-border/40 bg-muted/40 text-foreground">
							{getIcon()}
							<span
								className={cn(
									'absolute -top-0.5 -right-0.5 size-2 rounded-full border-2 border-card',
									getStatusDotColor(status),
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
								className="w-40 border border-border bg-popover/95 shadow-lg backdrop-blur-md">
								<DropdownMenuItem
									onClick={handleNavigate}
									className="flex cursor-pointer items-center gap-2 py-1.5 text-xs font-medium text-foreground">
									<Eye className="size-3.5 text-foreground" />
									View details
								</DropdownMenuItem>

								{/* Start Option: Disabled if running, starting/deploying, or stopping */}
								<DropdownMenuItem
									disabled={
										isRunning || isStartingOrDeploying || isStopping
									}
									onClick={handleStart}
									className={cn(
										'flex cursor-pointer items-center gap-2 py-1.5 text-xs font-medium text-foreground',
										(isRunning || isStartingOrDeploying || isStopping) &&
											'cursor-not-allowed opacity-40',
									)}>
									<Play className="size-3.5 text-foreground" />
									Start
								</DropdownMenuItem>

								{/* Stop/Cancel Option */}
								{isStartingOrDeploying ? (
									<DropdownMenuItem
										onClick={handleCancel}
										className="flex cursor-pointer items-center gap-2 py-1.5 text-xs font-medium text-foreground">
										<XCircle className="size-3.5 text-foreground" />
										Cancel
									</DropdownMenuItem>
								) : (
									<DropdownMenuItem
										disabled={isStopped || isStopping}
										onClick={handleStop}
										className={cn(
											'flex cursor-pointer items-center gap-2 py-1.5 text-xs font-medium text-foreground',
											(isStopped || isStopping) &&
												'cursor-not-allowed opacity-40',
										)}>
										<Square className="size-3.5 text-foreground" />
										Stop
									</DropdownMenuItem>
								)}

								{/* Deploy Option */}
								<DropdownMenuItem
									disabled={isStartingOrDeploying || isStopping}
									onClick={handleDeploy}
									className={cn(
										'flex cursor-pointer items-center gap-2 py-1.5 text-xs font-medium text-foreground',
										(isStartingOrDeploying || isStopping) &&
											'cursor-not-allowed opacity-40',
									)}>
									<Rocket className="size-3.5 text-foreground" />
									Deploy
								</DropdownMenuItem>

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

				<div className="pointer-events-none relative z-10 flex items-center justify-between border-t border-border/40 pt-2.5">
					<span className="flex items-center gap-1.5 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
						<span
							className={cn(
								'size-1.5 rounded-full',
								getStatusDotColor(status),
							)}
						/>
						{status?.toLowerCase() || 'idle'}
					</span>
					<span className="font-mono text-[10px] text-muted-foreground/70">
						{new Date(createdAt * 1000).toLocaleDateString(undefined, {
							day: '2-digit',
							month: 'short',
						})}
					</span>
				</div>
			</div>

			{/* Custom Right-Click Context Menu */}
			{contextMenu && (
				<div
					style={{top: `${contextMenu.y}px`, left: `${contextMenu.x}px`}}
					className="fixed z-50 w-44 animate-in rounded-lg border border-border bg-popover/95 p-1 text-foreground shadow-xl backdrop-blur-md duration-100 fade-in"
					onClick={e => e.stopPropagation()}>
					<button
						onClick={handleNavigate}
						className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-muted">
						<Eye className="size-3.5 text-foreground" />
						View details
					</button>

					{/* Start */}
					<button
						disabled={isRunning || isStartingOrDeploying || isStopping}
						onClick={handleStart}
						className={cn(
							'flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-muted',
							(isRunning || isStartingOrDeploying || isStopping) &&
								'cursor-not-allowed opacity-40 hover:bg-transparent',
						)}>
						<Play className="size-3.5 text-foreground" />
						Start
					</button>

					{/* Stop / Cancel */}
					{isStartingOrDeploying ? (
						<button
							onClick={handleCancel}
							className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-muted">
							<XCircle className="size-3.5 text-foreground" />
							Cancel
						</button>
					) : (
						<button
							disabled={isStopped || isStopping}
							onClick={handleStop}
							className={cn(
								'flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-muted',
								(isStopped || isStopping) &&
									'cursor-not-allowed opacity-40 hover:bg-transparent',
							)}>
							<Square className="size-3.5 text-foreground" />
							Stop
						</button>
					)}

					{/* Deploy */}
					<button
						disabled={isStartingOrDeploying || isStopping}
						onClick={handleDeploy}
						className={cn(
							'flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-muted',
							(isStartingOrDeploying || isStopping) &&
								'cursor-not-allowed opacity-40 hover:bg-transparent',
						)}>
						<Rocket className="size-3.5 text-foreground" />
						Deploy
					</button>

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
