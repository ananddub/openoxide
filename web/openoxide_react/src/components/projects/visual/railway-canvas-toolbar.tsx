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
		<div className="flex items-center justify-between gap-3 p-1.5 bg-card/90 backdrop-blur-md border border-border/80 rounded-xl shadow-xs font-sans">
			{/* View Mode Switcher */}
			<div className="flex items-center gap-1 bg-muted/40 p-1 rounded-lg border border-border/40">
				<button
					type="button"
					onClick={() => onViewModeChange('list')}
					className={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-semibold transition-all cursor-pointer ${
						viewMode === 'list'
							? 'bg-background text-foreground shadow-xs border border-border'
							: 'text-muted-foreground hover:text-foreground'
					}`}
				>
					<List className="w-3.5 h-3.5" />
					<span>List View</span>
				</button>
				<button
					type="button"
					onClick={() => onViewModeChange('canvas')}
					className={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-semibold transition-all cursor-pointer ${
						viewMode === 'canvas'
							? 'bg-background text-foreground shadow-xs border border-border'
							: 'text-muted-foreground hover:text-foreground'
					}`}
				>
					<LayoutGrid className="w-3.5 h-3.5 text-primary" />
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
						className="h-7 text-xs font-semibold gap-1.5 border-border/60 bg-muted/20 hover:bg-muted/50 cursor-pointer"
					>
						<Sparkles className="w-3.5 h-3.5 text-amber-500" />
						<span>Auto Layout</span>
					</Button>

					<div className="flex items-center gap-0.5 border border-border/40 rounded-lg p-0.5 bg-muted/20">
						<button
							type="button"
							onClick={onZoomIn}
							title="Zoom In"
							className="p-1 text-muted-foreground hover:text-foreground rounded-md hover:bg-muted/50 transition-colors cursor-pointer"
						>
							<ZoomIn className="w-3.5 h-3.5" />
						</button>
						<button
							type="button"
							onClick={onZoomOut}
							title="Zoom Out"
							className="p-1 text-muted-foreground hover:text-foreground rounded-md hover:bg-muted/50 transition-colors cursor-pointer"
						>
							<ZoomOut className="w-3.5 h-3.5" />
						</button>
						<button
							type="button"
							onClick={onFitView}
							title="Fit View"
							className="p-1 text-muted-foreground hover:text-foreground rounded-md hover:bg-muted/50 transition-colors cursor-pointer"
						>
							<Maximize2 className="w-3.5 h-3.5" />
						</button>
					</div>
				</div>
			)}
		</div>
	);
}
