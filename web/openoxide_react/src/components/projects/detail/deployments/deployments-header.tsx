import {Activity, RefreshCw, Trash2, XCircle, Zap} from 'lucide-react';
import {Button} from '#/components/ui/button';

export interface DeploymentsHeaderProps {
	title: string;
	description: string;
	/** Amber status pill next to the title; pass null to hide. */
	pillLabel?: string | null;
	onClear: () => void;
	clearDisabled?: boolean;
	/** Refresh button is only rendered when a handler is provided. */
	onRefresh?: () => void;
	refreshSpinning?: boolean;
	redeployLabel?: string;
	onRedeploy?: () => void;
	redeployDisabled?: boolean;
	redeploySpinning?: boolean;
	/** While true the primary button is a disabled "Processing…" state. */
	isProcessing?: boolean;
	processingLabel?: string;
	/** Show the destructive Cancel button instead of the Deploy button. */
	showCancel?: boolean;
	onCancel?: () => void;
	cancelLabel?: string;
	cancelDisabled?: boolean;
	onDeploy?: () => void;
	deployLabel?: string;
}

export function DeploymentsHeader({
	title,
	description,
	pillLabel,
	onClear,
	clearDisabled,
	onRefresh,
	refreshSpinning,
	redeployLabel = 'Redeploy',
	onRedeploy,
	redeployDisabled,
	redeploySpinning,
	isProcessing,
	processingLabel = 'Processing...',
	showCancel,
	onCancel,
	cancelLabel = 'Cancel Build',
	cancelDisabled,
	onDeploy,
	deployLabel = 'Deploy',
}: DeploymentsHeaderProps) {
	return (
		<section className="flex items-center justify-between rounded-xl bg-card p-5">
			<div>
				<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
					{title}
					{pillLabel && (
						<span className="flex animate-pulse items-center gap-1 rounded-full bg-amber-500/10 px-2 py-0.5 text-[10px] font-semibold text-amber-500">
							<Activity className="h-3 w-3 animate-spin" /> {pillLabel}
						</span>
					)}
				</h3>
				<p className="mt-0.5 text-xs text-muted-foreground">
					{description}
				</p>
			</div>

			<div className="flex items-center gap-2">
				<Button
					variant="outline"
					size="sm"
					onClick={onClear}
					disabled={clearDisabled}
					className="flex h-8 items-center gap-1.5 border-destructive/30 bg-destructive/10 text-xs font-semibold text-destructive hover:bg-destructive/20">
					<Trash2 className="h-3.5 w-3.5" /> Clear History
				</Button>
				{onRefresh && (
					<Button
						variant="outline"
						size="sm"
						onClick={onRefresh}
						disabled={isProcessing}
						className="flex h-8 items-center gap-1.5 text-xs font-semibold text-foreground hover:bg-muted">
						<RefreshCw
							className={`h-3.5 w-3.5 ${refreshSpinning ? 'animate-spin' : ''}`}
						/>{' '}
						Refresh
					</Button>
				)}
				{onRedeploy && (
					<Button
						variant="outline"
						size="sm"
						onClick={onRedeploy}
						disabled={redeployDisabled}
						className="flex h-8 items-center gap-1.5 text-xs font-semibold text-foreground hover:bg-muted disabled:opacity-50">
						<RefreshCw
							className={`h-3.5 w-3.5 ${redeploySpinning ? 'animate-spin' : ''}`}
						/>{' '}
						{redeployLabel}
					</Button>
				)}
				{isProcessing ? (
					<Button
						disabled
						size="sm"
						className="flex h-8 items-center gap-1.5 bg-primary/80 text-xs font-semibold text-primary-foreground">
						<RefreshCw className="h-3.5 w-3.5 animate-spin" />{' '}
						{processingLabel}
					</Button>
				) : showCancel ? (
					<Button
						onClick={onCancel}
						disabled={cancelDisabled}
						size="sm"
						variant="destructive"
						className="flex h-8 items-center gap-1.5 text-xs font-semibold">
						<XCircle className="h-3.5 w-3.5" /> {cancelLabel}
					</Button>
				) : (
					<Button
						onClick={onDeploy}
						size="sm"
						className="flex h-8 items-center gap-1.5 bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/90">
						<Zap className="h-3.5 w-3.5" /> {deployLabel}
					</Button>
				)}
			</div>
		</section>
	);
}
