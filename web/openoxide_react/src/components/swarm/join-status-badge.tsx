import {Badge} from '#/components/ui/badge';
import {RefreshCw, CheckCircle2} from 'lucide-react';

/** Status pill for the selected remote server's cluster membership. */
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
				className="gap-1 px-1.5 py-0 text-[10px] text-muted-foreground">
				<RefreshCw className="h-3 w-3 animate-spin" /> Checking…
			</Badge>
		);
	}
	if (isConnected) {
		return (
			<Badge variant="secondary" className="gap-1 px-1.5 py-0 text-[10px]">
				<CheckCircle2 className="h-3 w-3 text-emerald-500" /> Already in
				this cluster
			</Badge>
		);
	}
	return (
		<Badge
			variant="outline"
			className="gap-1 px-1.5 py-0 text-[10px] text-muted-foreground">
			Not in this cluster
		</Badge>
	);
}
