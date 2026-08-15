import React, { useState, useEffect } from 'react';
import { Box, Layers2, Database as DbIcon, Play, Square, Rocket, Trash2, Eye, MoreVertical, XCircle } from 'lucide-react';
import { useNavigate } from '@tanstack/react-router';
import { cn } from '#/api/utils';
import { $api } from '#/api/query';
import { toast } from 'sonner';
import { formatApiError } from '#/api/utils';
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
	const [contextMenu, setContextMenu] = useState<{ x: number; y: number } | null>(null);

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
	const isStartingOrDeploying =
		s.includes('starting') ||
		s.includes('deploying') ||
		s.includes('building') ||
		s.includes('loading');

	const isRunning =
		!isStartingOrDeploying &&
		(s.includes('running') ||
			s.includes('active') ||
			s.includes('healthy') ||
			s.includes('up'));

	const isStopped = !isStartingOrDeploying && !isRunning;

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
	const dbStart = $api.useMutation('post', `/${dbKindPath}/{id}/start` as any);
	const dbStop = $api.useMutation('post', `/${dbKindPath}/{id}/stop` as any);
	const dbDeploy = $api.useMutation('post', `/${dbKindPath}/{id}/deploy` as any);
	const dbCancel = $api.useMutation('post', `/${dbKindPath}/{id}/cancel` as any);
	const dbDelete = $api.useMutation('delete', `/${dbKindPath}/{id}` as any);

	const getStatusDotColor = (status: string) => {
		if (s.includes('stopping') || s.includes('cancelling')) {
			return 'bg-orange-500 animate-pulse';
		}
		if (isRunning) {
			return 'bg-emerald-500';
		}
		if (s.includes('error') || s.includes('fail') || s.includes('unhealthy') || s.includes('crash')) {
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
		if (kind.includes('postgres')) return <PostgresqlIcon className="size-5 shrink-0" />;
		if (kind.includes('mysql')) return <MysqlIcon className="size-5 shrink-0" />;
		if (kind.includes('mariadb')) return <MariadbIcon className="size-5 shrink-0" />;
		if (kind.includes('mongo')) return <MongodbIcon className="size-5 shrink-0" />;
		if (kind.includes('redis')) return <RedisIcon className="size-5 shrink-0" />;
		if (kind.includes('libsql')) return <LibsqlIcon className="size-5 shrink-0" />;

		return <DbIcon className="size-4 text-foreground" />;
	};

	const handleNavigate = () => {
		if (type === 'APP') {
			navigate({ to: '/projects/$id/app/$appId', params: { id: String(projectId), appId: String(id) } });
		} else if (type === 'COMPOSE') {
			navigate({ to: '/projects/$id/compose/$composeId', params: { id: String(projectId), composeId: String(id) } });
		} else {
			navigate({
				to: '/projects/$id/database/$dbId',
				params: { id: String(projectId), dbId: String(id) },
				search: { kind: dbKind || 'postgres' } as any,
			});
		}
	};

	const handleContextMenu = (e: React.MouseEvent) => {
		e.preventDefault();
		e.stopPropagation();
		setContextMenu({ x: e.clientX, y: e.clientY });
	};

	const handleStart = async (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		if (isRunning || isStartingOrDeploying) return;
		try {
			if (type === 'APP') {
				await appStart.mutateAsync({ params: { path: { id } } });
			} else if (type === 'COMPOSE') {
				await composeStart.mutateAsync({ params: { path: { id } } });
			} else {
				await dbStart.mutateAsync({ params: { path: { id } } });
			}
			toast.success(`Starting ${name}...`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const handleStop = async (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		if (isStopped) return;
		try {
			if (type === 'APP') {
				await appStop.mutateAsync({ params: { path: { id } } });
			} else if (type === 'COMPOSE') {
				await composeStop.mutateAsync({ params: { path: { id } } });
			} else {
				await dbStop.mutateAsync({ params: { path: { id } } });
			}
			toast.success(`Stopping ${name}...`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const handleCancel = async (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		try {
			if (type === 'APP') {
				await appCancel.mutateAsync({ params: { path: { id } } });
			} else if (type === 'COMPOSE') {
				await composeCancel.mutateAsync({ params: { path: { id } } });
			} else {
				await dbCancel.mutateAsync({ params: { path: { id } } });
			}
			toast.success(`Cancelling ${name}...`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const handleDeploy = async (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		if (isStartingOrDeploying) return;
		try {
			if (type === 'APP') {
				await appDeploy.mutateAsync({ params: { path: { id } } });
			} else if (type === 'COMPOSE') {
				await composeDeploy.mutateAsync({ params: { path: { id } } });
			} else {
				await dbDeploy.mutateAsync({ params: { path: { id } } });
			}
			toast.success(`Deploying ${name}...`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const handleDelete = async (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		if (!confirm(`Are you sure you want to delete "${name}"?`)) return;
		try {
			if (type === 'APP') {
				await appDelete.mutateAsync({ params: { path: { id } } });
			} else if (type === 'COMPOSE') {
				await composeDelete.mutateAsync({ params: { path: { id } } });
			} else {
				await dbDelete.mutateAsync({ params: { path: { id } } });
			}
			toast.success(`Deleted ${name}`);
			onDeleted?.();
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<>
			<div
				onClick={handleNavigate}
				onContextMenu={handleContextMenu}
				className="w-full bg-card border border-border hover:border-primary/40 transition-all duration-200 rounded-xl p-4 flex flex-col justify-between gap-3.5 cursor-pointer group shadow-2xs block text-left relative overflow-hidden"
			>
				<div className="flex items-center justify-between gap-3">
					<div className="flex items-center gap-3 min-w-0">
						<div className="size-9 rounded-lg bg-muted/40 flex items-center justify-center shrink-0 relative border border-border/40 text-foreground">
							{getIcon()}
							<span className={cn('absolute -top-0.5 -right-0.5 size-2 rounded-full border-2 border-card', getStatusDotColor(status))} />
						</div>
						<div className="min-w-0">
							<p className="font-bold text-xs text-foreground truncate group-hover:text-primary transition-colors leading-snug">{name}</p>
							<p className="text-[10px] text-muted-foreground font-mono uppercase tracking-wider">{subtitle}</p>
						</div>
					</div>

					<DropdownMenu>
						<DropdownMenuTrigger
							render={
								<button
									onClick={(e) => e.stopPropagation()}
									className="size-7 flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded-md shrink-0 focus:outline-none opacity-0 group-hover:opacity-100 transition-opacity"
								>
									<MoreVertical className="size-4 text-foreground" />
								</button>
							}
						/>
						<DropdownMenuContent
							align="end"
							className="w-40 border border-border bg-popover/95 backdrop-blur-md shadow-lg"
							onClick={(e) => e.stopPropagation()}
						>
							<DropdownMenuItem
								onClick={handleNavigate}
								className="flex items-center gap-2 cursor-pointer text-xs font-medium py-1.5 text-foreground"
							>
								<Eye className="size-3.5 text-foreground" />
								View details
							</DropdownMenuItem>

							{/* Start Option: Disabled if running or starting */}
							<DropdownMenuItem
								disabled={isRunning || isStartingOrDeploying}
								onClick={handleStart}
								className={cn(
									'flex items-center gap-2 cursor-pointer text-xs font-medium py-1.5 text-foreground',
									(isRunning || isStartingOrDeploying) && 'opacity-40 cursor-not-allowed'
								)}
							>
								<Play className="size-3.5 text-foreground" />
								Start
							</DropdownMenuItem>

							{/* Stop/Cancel Option: Shows Cancel when starting/deploying; Stop when running */}
							{isStartingOrDeploying ? (
								<DropdownMenuItem
									onClick={handleCancel}
									className="flex items-center gap-2 cursor-pointer text-xs font-medium py-1.5 text-foreground"
								>
									<XCircle className="size-3.5 text-foreground" />
									Cancel
								</DropdownMenuItem>
							) : (
								<DropdownMenuItem
									disabled={isStopped}
									onClick={handleStop}
									className={cn(
										'flex items-center gap-2 cursor-pointer text-xs font-medium py-1.5 text-foreground',
										isStopped && 'opacity-40 cursor-not-allowed'
									)}
								>
									<Square className="size-3.5 text-foreground" />
									Stop
								</DropdownMenuItem>
							)}

							{/* Deploy Option: Disabled while starting/deploying */}
							<DropdownMenuItem
								disabled={isStartingOrDeploying}
								onClick={handleDeploy}
								className={cn(
									'flex items-center gap-2 cursor-pointer text-xs font-medium py-1.5 text-foreground',
									isStartingOrDeploying && 'opacity-40 cursor-not-allowed'
								)}
							>
								<Rocket className="size-3.5 text-foreground" />
								Deploy
							</DropdownMenuItem>

							<DropdownMenuSeparator className="bg-border/60" />
							<DropdownMenuItem
								onClick={handleDelete}
								className="flex items-center gap-2 cursor-pointer text-xs font-medium py-1.5 text-destructive hover:bg-destructive/10 focus:text-destructive focus:bg-destructive/10"
							>
								<Trash2 className="size-3.5 text-destructive" />
								Delete
							</DropdownMenuItem>
						</DropdownMenuContent>
					</DropdownMenu>
				</div>

				<div className="flex items-center justify-between border-t border-border/40 pt-2.5">
					<span className="flex items-center gap-1.5 text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						<span className={cn('size-1.5 rounded-full', getStatusDotColor(status))} />
						{status?.toLowerCase() || 'idle'}
					</span>
					<span className="text-[10px] text-muted-foreground/70 font-mono">
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
					style={{ top: `${contextMenu.y}px`, left: `${contextMenu.x}px` }}
					className="fixed z-50 w-44 rounded-lg border border-border bg-popover/95 backdrop-blur-md shadow-xl p-1 animate-in fade-in duration-100 text-foreground"
					onClick={(e) => e.stopPropagation()}
				>
					<button
						onClick={handleNavigate}
						className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-xs font-medium text-foreground hover:bg-muted transition-colors text-left"
					>
						<Eye className="size-3.5 text-foreground" />
						View details
					</button>

					{/* Start */}
					<button
						disabled={isRunning || isStartingOrDeploying}
						onClick={handleStart}
						className={cn(
							'flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-xs font-medium text-foreground hover:bg-muted transition-colors text-left',
							(isRunning || isStartingOrDeploying) && 'opacity-40 cursor-not-allowed hover:bg-transparent'
						)}
					>
						<Play className="size-3.5 text-foreground" />
						Start
					</button>

					{/* Stop / Cancel */}
					{isStartingOrDeploying ? (
						<button
							onClick={handleCancel}
							className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-xs font-medium text-foreground hover:bg-muted transition-colors text-left"
						>
							<XCircle className="size-3.5 text-foreground" />
							Cancel
						</button>
					) : (
						<button
							disabled={isStopped}
							onClick={handleStop}
							className={cn(
								'flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-xs font-medium text-foreground hover:bg-muted transition-colors text-left',
								isStopped && 'opacity-40 cursor-not-allowed hover:bg-transparent'
							)}
						>
							<Square className="size-3.5 text-foreground" />
							Stop
						</button>
					)}

					{/* Deploy */}
					<button
						disabled={isStartingOrDeploying}
						onClick={handleDeploy}
						className={cn(
							'flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-xs font-medium text-foreground hover:bg-muted transition-colors text-left',
							isStartingOrDeploying && 'opacity-40 cursor-not-allowed hover:bg-transparent'
						)}
					>
						<Rocket className="size-3.5 text-foreground" />
						Deploy
					</button>

					<div className="my-1 h-px bg-border/60" />
					<button
						onClick={handleDelete}
						className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-xs font-medium text-destructive hover:bg-destructive/10 transition-colors text-left"
					>
						<Trash2 className="size-3.5 text-destructive" />
						Delete
					</button>
				</div>
			)}
		</>
	);
}
