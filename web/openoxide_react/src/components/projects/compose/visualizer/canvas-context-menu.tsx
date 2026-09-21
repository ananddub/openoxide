import {useEffect, useRef} from 'react';
import {createPortal} from 'react-dom';
import {X} from 'lucide-react';
import {
	buildMenu,
	type CanvasContextMenuProps,
} from './context-menu-items';

export type {CanvasContextMenuProps} from './context-menu-items';

export function CanvasContextMenu(props: CanvasContextMenuProps) {
	const {x, y, targetType, targetData, onClose} = props;
	const ref = useRef<HTMLDivElement>(null);

	useEffect(() => {
		const handleEscape = (e: KeyboardEvent) => {
			if (e.key === 'Escape') onClose();
		};
		window.addEventListener('keydown', handleEscape);
		return () => window.removeEventListener('keydown', handleEscape);
	}, [onClose]);

	const screenW = typeof window !== 'undefined' ? window.innerWidth : 1000;
	const screenH = typeof window !== 'undefined' ? window.innerHeight : 800;
	const adjustedX = Math.min(Math.max(10, x), screenW - 240);
	const adjustedY = Math.min(Math.max(10, y), screenH - 310);

	const titleName =
		targetData?.name ||
		targetData?.domain ||
		targetData?.volume_name ||
		'Node';

	const dismiss = (e: React.MouseEvent) => {
		e.preventDefault();
		e.stopPropagation();
		e.nativeEvent?.stopImmediatePropagation();
		onClose();
	};

	const select = (fn?: (data: any) => void) => (e: React.MouseEvent) => {
		e.preventDefault();
		e.stopPropagation();
		e.nativeEvent?.stopImmediatePropagation();
		fn?.(targetData);
		onClose();
	};

	const statusDot =
		targetType === 'domain'
			? 'bg-emerald-500'
			: targetType === 'schedule'
				? 'bg-purple-400'
				: targetType === 'backup'
					? 'bg-sky-400'
					: 'animate-pulse bg-primary';

	const items = buildMenu(targetType, props);

	const menuContent = (
		<>
			<div
				className="pointer-events-auto fixed inset-0 z-[999998] bg-transparent"
				onClick={dismiss}
				onContextMenu={dismiss}
			/>
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
					e.nativeEvent?.stopImmediatePropagation();
				}}
				className="pointer-events-auto w-56 animate-in rounded-xl bg-popover/95 p-1.5 shadow-lg backdrop-blur-xl duration-150 select-none zoom-in-95 fade-in">
				<div className="mb-1 flex items-center justify-between border-b  px-2.5 py-2">
					<div className="flex min-w-0 items-center gap-2">
						<span
							className={`size-2 shrink-0 rounded-full ${statusDot}`}
						/>
						<span className="truncate text-xs font-bold text-foreground">
							{titleName}
						</span>
					</div>
					<button
						type="button"
						onClick={select()}
						className="cursor-pointer rounded-md p-0.5 text-muted-foreground transition-colors hover:bg-muted/60 hover:text-foreground">
						<X className="size-3.5" />
					</button>
				</div>

				<div className="flex flex-col gap-0.5">
					{items.map(entry => {
						const Icon = entry.icon;
						const weightClass =
							entry.weight === 'bold'
								? 'font-bold'
								: entry.weight === 'semibold'
									? 'font-semibold'
									: 'font-medium';
						return (
							<div key={entry.label}>
								{entry.dividerBefore && (
									<div className="my-1 h-px bg-border/40" />
								)}
								<button
									type="button"
									onClick={select(entry.action)}
									className={`group flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left text-xs ${weightClass} ${entry.color} transition-colors ${entry.hover}`}>
									<Icon className="size-3.5 transition-transform group-hover:scale-110" />
									<span>{entry.label}</span>
								</button>
							</div>
						);
					})}
				</div>
			</div>
		</>
	);

	if (typeof document === 'undefined') return null;
	return createPortal(menuContent, document.body);
}
