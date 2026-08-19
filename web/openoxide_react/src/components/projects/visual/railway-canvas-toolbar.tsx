import {Button} from '#/components/ui/button';
import {
	LayoutGrid,
	List,
	Maximize2,
	Sparkles,
	ZoomIn,
	ZoomOut,
} from 'lucide-react';

interface RailwayCanvasToolbarProps {
	viewMode: 'list' | 'canvas';
	onViewModeChange: (mode: 'list' | 'canvas') => void;
	onAutoLayout: () => void;
	onZoomIn: () => void;
	onZoomOut: () => void;
	onFitView: () => void;
}

export function RailwayCanvasToolbar({
	viewMode,
	onViewModeChange,
	onAutoLayout,
	onZoomIn,
	onZoomOut,
	onFitView,
}: RailwayCanvasToolbarProps) {
	return (
		<div className="flex items-center justify-between gap-3 rounded-xl border border-border/80 bg-card/90 p-1.5 font-sans shadow-xs backdrop-blur-md">
			{/* View Mode Switcher */}
			<div className="flex items-center gap-1 rounded-lg border border-border/40 bg-muted/40 p-1">
				<button
					type="button"
					onClick={() => onViewModeChange('list')}
					className={`flex cursor-pointer items-center gap-1.5 rounded-md px-3 py-1 text-xs font-semibold transition-all ${
						viewMode === 'list'
							? 'border border-border bg-background text-foreground shadow-xs'
							: 'text-muted-foreground hover:text-foreground'
					}`}>
					<List className="h-3.5 w-3.5" />
					<span>List View</span>
				</button>
				<button
					type="button"
					onClick={() => onViewModeChange('canvas')}
					className={`flex cursor-pointer items-center gap-1.5 rounded-md px-3 py-1 text-xs font-semibold transition-all ${
						viewMode === 'canvas'
							? 'border border-border bg-background text-foreground shadow-xs'
							: 'text-muted-foreground hover:text-foreground'
					}`}>
					<LayoutGrid className="h-3.5 w-3.5 text-primary" />
					<span>Architecture Diagram</span>
				</button>
			</div>

			{/* Canvas Controls */}
			{viewMode === 'canvas' && (
				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={onAutoLayout}
						title="Auto-align diagram nodes"
						className="h-7 cursor-pointer gap-1.5 border-border/60 bg-muted/20 text-xs font-semibold hover:bg-muted/50">
						<Sparkles className="h-3.5 w-3.5 text-amber-500" />
						<span>Auto Layout</span>
					</Button>

					<div className="flex items-center gap-0.5 rounded-lg border border-border/40 bg-muted/20 p-0.5">
						<button
							type="button"
							onClick={onZoomIn}
							title="Zoom In"
							className="cursor-pointer rounded-md p-1 text-muted-foreground transition-colors hover:bg-muted/50 hover:text-foreground">
							<ZoomIn className="h-3.5 w-3.5" />
						</button>
						<button
							type="button"
							onClick={onZoomOut}
							title="Zoom Out"
							className="cursor-pointer rounded-md p-1 text-muted-foreground transition-colors hover:bg-muted/50 hover:text-foreground">
							<ZoomOut className="h-3.5 w-3.5" />
						</button>
						<button
							type="button"
							onClick={onFitView}
							title="Fit View"
							className="cursor-pointer rounded-md p-1 text-muted-foreground transition-colors hover:bg-muted/50 hover:text-foreground">
							<Maximize2 className="h-3.5 w-3.5" />
						</button>
					</div>
				</div>
			)}
		</div>
	);
}
