import {RefreshCw, AlertCircle, Zap} from 'lucide-react';
import {Button} from '#/components/ui/button';

interface RequestsActivatePanelProps {
	isToggling: boolean;
	onActivate: () => void;
}

/** Empty state shown when Traefik request logging is inactive. */
export function RequestsActivatePanel({
	isToggling,
	onActivate,
}: RequestsActivatePanelProps) {
	return (
		<div className="mx-auto mt-10 mb-auto flex w-full max-w-xl flex-col items-center gap-3.5 py-6 text-center">
			<AlertCircle className="mb-1 size-8 text-muted-foreground/60" />
			<div className="space-y-1">
				<h3 className="text-sm font-bold text-foreground">
					Requests are not activated
				</h3>
				<p className="max-w-md text-xs leading-relaxed text-muted-foreground">
					Activate requests to see incoming traffic statistics and monitor
					your application's usage. After activation, reload Traefik for
					changes to take effect.
				</p>
			</div>
			<Button
				variant="default"
				size="sm"
				onClick={onActivate}
				disabled={isToggling}
				className="mt-2 h-9 cursor-pointer gap-2 px-5 text-xs font-semibold">
				{isToggling ? (
					<RefreshCw className="size-3.5 animate-spin" />
				) : (
					<Zap className="size-3.5 fill-primary-foreground" />
				)}
				Activate Requests
			</Button>
		</div>
	);
}
