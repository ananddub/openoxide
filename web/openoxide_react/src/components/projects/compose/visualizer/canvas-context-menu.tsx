import {useEffect, useRef} from 'react';
import {createPortal} from 'react-dom';
import {Globe, Clock, Database, Terminal, Rocket, SquareTerminal, Trash2, X, Pencil} from 'lucide-react';
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
		console.log('[CanvasContextMenu] MOUNTED for targetType:', targetType, 'targetData:', targetData);
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

	const titleName = targetData?.name || targetData?.domain || targetData?.volume_name || 'Node';

	const handleItemClick = (fn?: (data: any) => void, actionName?: string) => (e: React.MouseEvent) => {
		console.log('[CanvasContextMenu] ITEM CLICKED:', actionName, 'Data:', targetData);
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
				className="fixed inset-0 z-[999998] bg-transparent pointer-events-auto"
				onClick={(e) => {
					e.preventDefault();
					e.stopPropagation();
					if (e.nativeEvent) e.nativeEvent.stopImmediatePropagation();
					onClose();
				}}
				onContextMenu={(e) => {
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
				onClick={(e) => {
					e.stopPropagation();
					if (e.nativeEvent) e.nativeEvent.stopImmediatePropagation();
				}}
				className="w-56 rounded-xl border border-border/80 bg-popover/95 p-1.5 shadow-2xl backdrop-blur-xl animate-in fade-in zoom-in-95 duration-150 pointer-events-auto select-none"
			>
				{/* Menu Header */}
				<div className="flex items-center justify-between border-b border-border/40 px-2.5 py-2 mb-1">
					<div className="flex items-center gap-2 min-w-0">
						<span className={`size-2 rounded-full shrink-0 ${
							targetType === 'domain' ? 'bg-emerald-500' :
							targetType === 'schedule' ? 'bg-purple-400' :
							targetType === 'backup' ? 'bg-sky-400' : 'bg-primary animate-pulse'
						}`} />
						<span className="text-xs font-bold text-foreground truncate">{titleName}</span>
					</div>
					<button
						type="button"
						onClick={handleItemClick(undefined, 'Close')}
						className="text-muted-foreground hover:text-foreground transition-colors p-0.5 rounded-md hover:bg-muted/60 cursor-pointer"
					>
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
								className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-medium text-foreground hover:bg-emerald-500/10 hover:text-emerald-400 rounded-lg transition-colors text-left group cursor-pointer"
							>
								<Globe className="size-3.5 text-emerald-500 group-hover:scale-110 transition-transform" />
								<span>Add Domain</span>
							</button>
						)}

						{onAddSchedule && (
							<button
								type="button"
								onClick={handleItemClick(onAddSchedule, 'Add Schedule')}
								className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-medium text-foreground hover:bg-purple-500/10 hover:text-purple-400 rounded-lg transition-colors text-left group cursor-pointer"
							>
								<Clock className="size-3.5 text-purple-400 group-hover:scale-110 transition-transform" />
								<span>Add Schedule / Cron</span>
							</button>
						)}

						<button
							type="button"
							onClick={handleItemClick(onAddBackup, 'Add Backup')}
							className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-medium text-foreground hover:bg-sky-500/10 hover:text-sky-400 rounded-lg transition-colors text-left group cursor-pointer"
						>
							<Database className="size-3.5 text-sky-400 group-hover:scale-110 transition-transform" />
							<span>Add Volume Backup</span>
						</button>

						<div className="h-px bg-border/40 my-1" />

						<button
							type="button"
							onClick={handleItemClick(onOpenTerminal, 'Terminal')}
							className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-bold text-foreground hover:bg-emerald-500/15 hover:text-emerald-400 rounded-lg transition-colors text-left group cursor-pointer"
						>
							<SquareTerminal className="size-3.5 text-emerald-400 group-hover:scale-110 transition-transform" />
							<span>Terminal</span>
						</button>

						<button
							type="button"
							onClick={handleItemClick(onViewLogs, 'View Logs')}
							className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-medium text-foreground hover:bg-amber-500/10 hover:text-amber-400 rounded-lg transition-colors text-left group cursor-pointer"
						>
							<Terminal className="size-3.5 text-amber-400 group-hover:scale-110 transition-transform" />
							<span>Live Container Logs</span>
						</button>

						{onViewDeployLogs && (
							<button
								type="button"
								onClick={handleItemClick(onViewDeployLogs, 'Deploy Logs')}
								className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-medium text-foreground hover:bg-primary/10 hover:text-primary rounded-lg transition-colors text-left group cursor-pointer"
							>
								<Rocket className="size-3.5 text-primary group-hover:scale-110 transition-transform" />
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
							className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-semibold text-foreground hover:bg-emerald-500/10 hover:text-emerald-400 rounded-lg transition-colors text-left group cursor-pointer"
						>
							<Pencil className="size-3.5 text-emerald-400 group-hover:scale-110 transition-transform" />
							<span>Edit Domain</span>
						</button>
						<button
							type="button"
							onClick={handleItemClick(onDeleteDomain, 'Delete Domain')}
							className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-bold text-destructive hover:bg-destructive/10 rounded-lg transition-colors text-left group cursor-pointer"
						>
							<Trash2 className="size-3.5 text-destructive group-hover:scale-110 transition-transform" />
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
							className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-semibold text-foreground hover:bg-purple-500/10 hover:text-purple-400 rounded-lg transition-colors text-left group cursor-pointer"
						>
							<Pencil className="size-3.5 text-purple-400 group-hover:scale-110 transition-transform" />
							<span>Edit Schedule</span>
						</button>
						<button
							type="button"
							onClick={handleItemClick(onDeleteSchedule, 'Delete Schedule')}
							className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-bold text-destructive hover:bg-destructive/10 rounded-lg transition-colors text-left group cursor-pointer"
						>
							<Trash2 className="size-3.5 text-destructive group-hover:scale-110 transition-transform" />
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
							className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-semibold text-foreground hover:bg-sky-500/10 hover:text-sky-400 rounded-lg transition-colors text-left group cursor-pointer"
						>
							<Pencil className="size-3.5 text-sky-400 group-hover:scale-110 transition-transform" />
							<span>Edit Backup</span>
						</button>
						<button
							type="button"
							onClick={handleItemClick(onDeleteBackup, 'Delete Backup')}
							className="flex items-center gap-2.5 w-full px-2.5 py-1.5 text-xs font-bold text-destructive hover:bg-destructive/10 rounded-lg transition-colors text-left group cursor-pointer"
						>
							<Trash2 className="size-3.5 text-destructive group-hover:scale-110 transition-transform" />
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
