import {RefreshCw, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';

interface DeploymentsHeaderProps {
	refreshing: boolean;
	onRefresh: () => void;
	onClearAll?: () => void;
}

export function DeploymentsHeader({
	refreshing,
	onRefresh,
	onClearAll,
}: DeploymentsHeaderProps) {
	return (
		<div className="flex flex-col justify-between gap-4 border-b border-border/30 pb-5 md:flex-row md:items-center">
			<div>
				<h1 className="text-3xl font-extrabold tracking-tight text-foreground">
					Deployments
				</h1>
				<p className="mt-1 text-xs font-medium text-muted-foreground">
					Monitor deployment history, execution states, and live streaming
					console logs
				</p>
			</div>

			<div className="flex items-center gap-3">
				{onClearAll && (
					<Button
						variant="outline"
						onClick={onClearAll}
						className="flex h-9 items-center gap-2 rounded-lg border-destructive/30 bg-destructive/10 px-3.5 text-xs font-semibold text-destructive shadow-2xs hover:bg-destructive/20">
						<Trash2 className="size-3.5" />
						Clear History
					</Button>
				)}
				<Button
					variant="outline"
					onClick={onRefresh}
					disabled={refreshing}
					className="flex h-9 items-center gap-2 rounded-lg border-border bg-card px-3.5 text-xs font-semibold shadow-2xs hover:bg-muted/50">
					<RefreshCw
						className={cn('size-3.5', refreshing && 'animate-spin')}
					/>
					Refresh
				</Button>
			</div>
		</div>
	);
}
