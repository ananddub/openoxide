import {useEffect, useRef} from 'react';
import {createPortal} from 'react-dom';
import {
	Globe,
	Clock,
	Database,
	Terminal,
	Rocket,
	SquareTerminal,
	Trash2,
	X,
	Pencil,
} from 'lucide-react';
import type {ComposeService} from './compose-visualizer';

export interface CanvasContextMenuProps {
	x: number;
	y: number;
	targetType: 'service' | 'domain' | 'schedule' | 'backup';
	targetData: any;
	onClose: () => void;
	onAddDomain?: (service: ComposeService) => void;
	onAddSchedule?: (service: ComposeService) => void;
	onAddBackup?: (service: ComposeService) => void;
	onOpenTerminal?: (service: ComposeService) => void;
	onViewLogs?: (service: ComposeService) => void;
	onViewDeployLogs?: (service: ComposeService) => void;
	onEditDomain?: (domainData: any) => void;
	onDeleteDomain?: (domainData: any) => void;
	onEditSchedule?: (scheduleData: any) => void;
	onDeleteSchedule?: (scheduleData: any) => void;
	onEditBackup?: (backupData: any) => void;
	onDeleteBackup?: (backupData: any) => void;
}

export function CanvasContextMenu({
	x,
	y,
	targetType,
	targetData,
	onClose,
	onAddDomain,
	onAddSchedule,
	onAddBackup,
	onOpenTerminal,
	onViewLogs,
	onViewDeployLogs,
	onEditDomain,
	onDeleteDomain,
	onEditSchedule,
	onDeleteSchedule,
	onEditBackup,
	onDeleteBackup,
}: CanvasContextMenuProps) {
	const ref = useRef<HTMLDivElement>(null);

	useEffect(() => {
		console.log(
			'[CanvasContextMenu] MOUNTED for targetType:',
			targetType,
			'targetData:',
			targetData,
		);
		const handleEscape = (e: KeyboardEvent) => {
			if (e.key === 'Escape') onClose();
		};
		window.addEventListener('keydown', handleEscape);
		return () => window.removeEventListener('keydown', handleEscape);
	}, [targetType, targetData, onClose]);

	const screenW = typeof window !== 'undefined' ? window.innerWidth : 1000;
	const screenH = typeof window !== 'undefined' ? window.innerHeight : 800;
	const adjustedX = Math.min(Math.max(10, x), screenW - 240);
	const adjustedY = Math.min(Math.max(10, y), screenH - 310);

	const titleName =
		targetData?.name ||
		targetData?.domain ||
		targetData?.volume_name ||
		'Node';

	const handleItemClick =
		(fn?: (data: any) => void, actionName?: string) =>
		(e: React.MouseEvent) => {
			console.log(
				'[CanvasContextMenu] ITEM CLICKED:',
				actionName,
				'Data:',
				targetData,
			);
			e.preventDefault();
			e.stopPropagation();
			if (e.nativeEvent) {
				e.nativeEvent.stopImmediatePropagation();
			}
			if (fn) {
				console.log('[CanvasContextMenu] EXECUTING FN FOR:', actionName);
				fn(targetData);
			} else {
				console.warn('[CanvasContextMenu] NO FN DEFINED FOR:', actionName);
			}
			onClose();
		};

	const menuContent = (
		<>
			{/* Transparent backdrop overlay to dismiss menu on click outside */}
			<div
				className="pointer-events-auto fixed inset-0 z-[999998] bg-transparent"
				onClick={e => {
					e.preventDefault();
					e.stopPropagation();
					if (e.nativeEvent) e.nativeEvent.stopImmediatePropagation();
					onClose();
				}}
				onContextMenu={e => {
					e.preventDefault();
					e.stopPropagation();
					if (e.nativeEvent) e.nativeEvent.stopImmediatePropagation();
					onClose();
				}}
			/>

			{/* Floating Menu Container */}
			<div
				ref={ref}
				style={{
					position: 'fixed',
					left: adjustedX,
					top: adjustedY,
					zIndex: 999999,
				}}
				onClick={e => {
					e.stopPropagation();
					if (e.nativeEvent) e.nativeEvent.stopImmediatePropagation();
				}}
				className="pointer-events-auto w-56 animate-in rounded-xl border border-border/80 bg-popover/95 p-1.5 shadow-2xl backdrop-blur-xl duration-150 select-none zoom-in-95 fade-in">
				{/* Menu Header */}
				<div className="mb-1 flex items-center justify-between border-b border-border/40 px-2.5 py-2">
					<div className="flex min-w-0 items-center gap-2">
						<span
							className={`size-2 shrink-0 rounded-full ${
								targetType === 'domain'
									? 'bg-emerald-500'
									: targetType === 'schedule'
										? 'bg-purple-400'
										: targetType === 'backup'
											? 'bg-sky-400'
											: 'animate-pulse bg-primary'
							}`}
						/>
						<span className="truncate text-xs font-bold text-foreground">
							{titleName}
						</span>
					</div>
					<button
						type="button"
						onClick={handleItemClick(undefined, 'Close')}
						className="cursor-pointer rounded-md p-0.5 text-muted-foreground transition-colors hover:bg-muted/60 hover:text-foreground">
						<X className="size-3.5" />
					</button>
				</div>

				{/* Options for Service Node */}
				{targetType === 'service' && (
					<div className="flex flex-col gap-0.5">
						{onAddDomain && (
							<button
								type="button"
								onClick={handleItemClick(onAddDomain, 'Add Domain')}
								className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-emerald-500/10 hover:text-emerald-400">
								<Globe className="size-3.5 text-emerald-500 transition-transform group-hover:scale-110" />
								<span>Add Domain</span>
							</button>
						)}

						{onAddSchedule && (
							<button
								type="button"
								onClick={handleItemClick(onAddSchedule, 'Add Schedule')}
								className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-purple-500/10 hover:text-purple-400">
								<Clock className="size-3.5 text-purple-400 transition-transform group-hover:scale-110" />
								<span>Add Schedule / Cron</span>
							</button>
						)}

						<button
							type="button"
							onClick={handleItemClick(onAddBackup, 'Add Backup')}
							className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-sky-500/10 hover:text-sky-400">
							<Database className="size-3.5 text-sky-400 transition-transform group-hover:scale-110" />
							<span>Add Volume Backup</span>
						</button>

						<div className="my-1 h-px bg-border/40" />

						<button
							type="button"
							onClick={handleItemClick(onOpenTerminal, 'Terminal')}
							className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-bold text-foreground transition-colors hover:bg-emerald-500/15 hover:text-emerald-400">
							<SquareTerminal className="size-3.5 text-emerald-400 transition-transform group-hover:scale-110" />
							<span>Terminal</span>
						</button>

						<button
							type="button"
							onClick={handleItemClick(onViewLogs, 'View Logs')}
							className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-amber-500/10 hover:text-amber-400">
							<Terminal className="size-3.5 text-amber-400 transition-transform group-hover:scale-110" />
							<span>Live Container Logs</span>
						</button>

						{onViewDeployLogs && (
							<button
								type="button"
								onClick={handleItemClick(onViewDeployLogs, 'Deploy Logs')}
								className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-primary/10 hover:text-primary">
								<Rocket className="size-3.5 text-primary transition-transform group-hover:scale-110" />
								<span>Deployment Logs</span>
							</button>
						)}
					</div>
				)}

				{/* Options for Domain Node */}
				{targetType === 'domain' && (
					<div className="flex flex-col gap-0.5">
						<button
							type="button"
							onClick={handleItemClick(onEditDomain, 'Edit Domain')}
							className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-semibold text-foreground transition-colors hover:bg-emerald-500/10 hover:text-emerald-400">
							<Pencil className="size-3.5 text-emerald-400 transition-transform group-hover:scale-110" />
							<span>Edit Domain</span>
						</button>
						<button
							type="button"
							onClick={handleItemClick(onDeleteDomain, 'Delete Domain')}
							className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-bold text-destructive transition-colors hover:bg-destructive/10">
							<Trash2 className="size-3.5 text-destructive transition-transform group-hover:scale-110" />
							<span>Delete Domain</span>
						</button>
					</div>
				)}

				{/* Options for Schedule Node */}
				{targetType === 'schedule' && (
					<div className="flex flex-col gap-0.5">
						<button
							type="button"
							onClick={handleItemClick(onEditSchedule, 'Edit Schedule')}
							className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-semibold text-foreground transition-colors hover:bg-purple-500/10 hover:text-purple-400">
							<Pencil className="size-3.5 text-purple-400 transition-transform group-hover:scale-110" />
							<span>Edit Schedule</span>
						</button>
						<button
							type="button"
							onClick={handleItemClick(
								onDeleteSchedule,
								'Delete Schedule',
							)}
							className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-bold text-destructive transition-colors hover:bg-destructive/10">
							<Trash2 className="size-3.5 text-destructive transition-transform group-hover:scale-110" />
							<span>Delete Schedule</span>
						</button>
					</div>
				)}

				{/* Options for Backup Node */}
				{targetType === 'backup' && (
					<div className="flex flex-col gap-0.5">
						<button
							type="button"
							onClick={handleItemClick(onEditBackup, 'Edit Backup')}
							className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-semibold text-foreground transition-colors hover:bg-sky-500/10 hover:text-sky-400">
							<Pencil className="size-3.5 text-sky-400 transition-transform group-hover:scale-110" />
							<span>Edit Backup</span>
						</button>
						<button
							type="button"
							onClick={handleItemClick(onDeleteBackup, 'Delete Backup')}
							className="group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs font-bold text-destructive transition-colors hover:bg-destructive/10">
							<Trash2 className="size-3.5 text-destructive transition-transform group-hover:scale-110" />
							<span>Delete Backup</span>
						</button>
					</div>
				)}
			</div>
		</>
	);

	if (typeof document === 'undefined') return null;
	return createPortal(menuContent, document.body);
}
