import {RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';

interface DeploymentsHeaderProps {
	refreshing: boolean;
	onRefresh: () => void;
}

export function DeploymentsHeader({refreshing, onRefresh}: DeploymentsHeaderProps) {
	return (
		<div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border/30 pb-5">
			<div>
				<h1 className="text-3xl font-extrabold tracking-tight text-foreground">
					Deployments
				</h1>
				<p className="text-muted-foreground mt-1 text-xs font-medium">
					Monitor deployment history, execution states, and live streaming console logs
				</p>
			</div>

			<div className="flex items-center gap-3">
				<Button
					variant="outline"
					onClick={onRefresh}
					disabled={refreshing}
					className="border-border bg-card hover:bg-muted/50 font-semibold h-9 px-3.5 text-xs rounded-lg flex items-center gap-2 shadow-2xs">
					<RefreshCw className={cn('size-3.5', refreshing && 'animate-spin')} />
					Refresh
				</Button>
			</div>
		</div>
	);
}
