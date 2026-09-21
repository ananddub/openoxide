import {AlertCircle} from 'lucide-react';
import {
	Tooltip,
	TooltipContent,
	TooltipTrigger,
} from '#/components/ui/tooltip';
import {AnimatedShinyText} from '#/components/ui/animated-shiny-text';

interface StatusBadgeProps {
	status: string;
	error?: string | null;
	isBuilding?: boolean;
	actionLoading?: string | null;
}

/* Shared status pill chrome: semantic status tokens only (no raw palette),
 * 6px radius (shape lock), 2px dot = real semantic state. */
const pill =
	'inline-flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium select-none';

export function StatusBadge({
	status,
	isBuilding,
	actionLoading,
	error,
}: StatusBadgeProps) {
	const st = (status || '').toUpperCase();

	if (
		[
			'QUEUED',
			'STARTING',
			'BUILDING',
			'DEPLOYING',
			'REBUILDING',
			'REDEPLOYING',
		].includes(st) ||
		isBuilding ||
		actionLoading === 'start' ||
		actionLoading === 'deploy' ||
		actionLoading === 'redeploy'
	) {
		const label =
			st === 'QUEUED'
				? 'QUEUED'
				: st === 'BUILDING'
					? 'BUILDING...'
					: st === 'DEPLOYING'
						? 'DEPLOYING...'
						: st === 'REBUILDING' || st === 'REDEPLOYING'
							? 'REDEPLOYING...'
							: actionLoading === 'deploy' || actionLoading === 'redeploy'
								? 'DEPLOYING...'
								: 'STARTING...';

		return (
			<span className={`${pill} bg-warning-bg text-warning`}>
				<span className="h-2 w-2 animate-pulse rounded-full bg-warning-solid" />
				<AnimatedShinyText className="text-warning/70">
					{label}
				</AnimatedShinyText>
			</span>
		);
	}

	if (st === 'STOPPING' || actionLoading === 'stop') {
		return (
			<span className={`${pill} bg-warning-bg text-warning`}>
				<span className="h-2 w-2 animate-pulse rounded-full bg-warning-solid" />
				STOPPING...
			</span>
		);
	}

	if (st === 'CANCELLING' || String(actionLoading) === 'cancel') {
		return (
			<span className={`${pill} bg-warning-bg text-warning`}>
				<span className="h-2 w-2 animate-pulse rounded-full bg-warning-solid" />
				CANCELLING...
			</span>
		);
	}

	if (st === 'CANCELLED') {
		return (
			<span className={`${pill} bg-neutral-bg text-neutral`}>
				<span className="h-2 w-2 rounded-full bg-neutral-solid" />
				CANCELLED
			</span>
		);
	}

	if (['RUNNING', 'DONE', 'SUCCESS', 'ACTIVE', 'OK'].includes(st)) {
		return (
			<span className={`${pill} bg-success-bg text-success`}>
				<span className="h-2 w-2 animate-pulse rounded-full bg-success-solid" />
				RUNNING
			</span>
		);
	}

	if (st === 'ERROR') {
		const badge = (
			<span className={`${pill} bg-danger-bg text-danger`}>
				<span className="h-2 w-2 rounded-full bg-danger-solid" />
				ERROR
				{error && <AlertCircle className="size-3.5" />}
			</span>
		);
		if (!error) return badge;
		return (
			<Tooltip>
				<TooltipTrigger render={badge} />
				<TooltipContent className="max-w-sm text-xs whitespace-pre-wrap">
					{error}
				</TooltipContent>
			</Tooltip>
		);
	}

	return (
		<span className={`${pill} bg-danger-bg text-danger`}>
			<span className="h-2 w-2 rounded-full bg-danger-solid" />
			STOPPED
		</span>
	);
}
