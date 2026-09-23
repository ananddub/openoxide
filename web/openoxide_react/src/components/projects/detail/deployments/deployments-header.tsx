import {Activity, RefreshCw, Trash2, XCircle, Zap} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {SectionHeader, Toolbar} from '#/components/settings/list-card';
import {cn} from '#/api/utils';

export interface DeploymentsHeaderProps {
	/** Section heading with a rule under it; omit for the plain toolbar row. */
	title?: string;
	/** One-line note under the heading. */
	description?: string;
	/** Muted status pill on the left; pass null/undefined to hide. */
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

/** Quiet toolbar above the deployment list: status pill left, actions right. */
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
	const pill = pillLabel && (
		<span className="flex items-center gap-1.5 rounded-md bg-foreground/[0.05] px-2 py-1 text-xs font-medium text-muted-foreground">
			<Activity className="size-3.5 animate-spin" />
			{pillLabel}
		</span>
	);

	const actions = (
		<>
			<Button
				variant="ghost"
				size="icon"
				onClick={onClear}
				disabled={clearDisabled}
				title="Clear history"
				className="size-9 text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
				<Trash2 className="size-4" />
			</Button>
			{onRefresh && (
				<Button
					variant="ghost"
					size="icon"
					onClick={onRefresh}
					disabled={isProcessing}
					title="Refresh"
					className="size-9 text-muted-foreground hover:text-foreground">
					<RefreshCw
						className={cn('size-4', refreshSpinning && 'animate-spin')}
					/>
				</Button>
			)}
			{onRedeploy && (
				<Button
					variant="outline"
					onClick={onRedeploy}
					disabled={redeployDisabled}
					className="h-9 gap-1.5">
					<RefreshCw
						className={cn('size-3.5', redeploySpinning && 'animate-spin')}
					/>
					{redeployLabel}
				</Button>
			)}
			{isProcessing ? (
				<Button disabled className="h-9 gap-1.5">
					<RefreshCw className="size-3.5 animate-spin" />
					{processingLabel}
				</Button>
			) : showCancel ? (
				<Button
					variant="destructive"
					onClick={onCancel}
					disabled={cancelDisabled}
					className="h-9 gap-1.5">
					<XCircle className="size-3.5" /> {cancelLabel}
				</Button>
			) : (
				onDeploy && (
					<Button onClick={onDeploy} className="h-9 gap-1.5">
						<Zap className="size-3.5" /> {deployLabel}
					</Button>
				)
			)}
		</>
	);

	if (title)
		return (
			<SectionHeader
				title={title}
				description={description}
				action={actions}>
				{pill}
			</SectionHeader>
		);

	return <Toolbar action={actions}>{pill}</Toolbar>;
}
