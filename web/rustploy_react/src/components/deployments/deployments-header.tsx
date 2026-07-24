import {RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';

interface DeploymentsHeaderProps {
	refreshing: boolean;
	onRefresh: () => void;
}

export function DeploymentsHeader({refreshing, onRefresh}: DeploymentsHeaderProps) {
	return (
		<div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border/40 pb-6">
			<div>
				<h1 className="text-3xl font-extrabold tracking-tight text-foreground bg-gradient-to-r from-foreground to-foreground/75 bg-clip-text">
					Deployments
				</h1>
				<p className="text-muted-foreground mt-1.5 text-sm">
					Monitor deployment history, execution states, and live streaming console logs.
				</p>
			</div>

			<div className="flex items-center gap-3">
				<Button
					variant="outline"
					onClick={onRefresh}
					disabled={refreshing}
					className="border-border bg-card/40 hover:bg-card/70 font-semibold h-10 px-4 rounded-lg flex items-center gap-2">
					<RefreshCw className={cn('size-4', refreshing && 'animate-spin')} />
					Refresh
				</Button>
			</div>
		</div>
	);
}
