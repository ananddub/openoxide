import {Badge} from '#/components/ui/badge';
import {cn} from '#/api/utils';
import {CheckCircle2, RefreshCw} from 'lucide-react';

/** Cluster-membership pill for the engine picked in the New Node sheet. */
export function JoinStatusBadge({
	isChecking,
	isConnected,
}: {
	isChecking: boolean;
	isConnected: boolean;
}) {
	if (isChecking) {
		return (
			<Badge
				variant="outline"
				className="shrink-0 gap-1.5 px-1.5 py-0 text-[10px] font-medium text-muted-foreground">
				<RefreshCw className="size-3 animate-spin" />
				Checking…
			</Badge>
		);
	}

	return (
		<Badge
			variant="outline"
			className={cn(
				'shrink-0 gap-1.5 px-1.5 py-0 text-[10px] font-medium',
				isConnected
					? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-500'
					: 'text-muted-foreground',
			)}>
			{isConnected ? (
				<CheckCircle2 className="size-3" />
			) : (
				<span className="size-1.5 rounded-full bg-muted-foreground/50" />
			)}
			{isConnected ? 'In this cluster' : 'Ready to join'}
		</Badge>
	);
}
