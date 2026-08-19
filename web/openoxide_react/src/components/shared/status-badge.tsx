interface StatusBadgeProps {
	status: string;
	isBuilding?: boolean;
	actionLoading?: string | null;
}

export function StatusBadge({
	status,
	isBuilding,
	actionLoading,
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
			<span className="inline-flex items-center gap-1.5 rounded-lg border border-amber-500/30 bg-amber-500/10 px-2.5 py-1 text-xs font-bold text-amber-500 select-none">
				<span className="h-2 w-2 animate-pulse rounded-full bg-amber-500" />
				{label}
			</span>
		);
	}

	if (st === 'STOPPING' || actionLoading === 'stop') {
		return (
			<span className="inline-flex items-center gap-1.5 rounded-lg border border-orange-500/30 bg-orange-500/10 px-2.5 py-1 text-xs font-bold text-orange-500 select-none">
				<span className="h-2 w-2 animate-pulse rounded-full bg-orange-500" />
				STOPPING...
			</span>
		);
	}

	if (st === 'CANCELLING' || String(actionLoading) === 'cancel') {
		return (
			<span className="inline-flex items-center gap-1.5 rounded-lg border border-amber-500/30 bg-amber-500/10 px-2.5 py-1 text-xs font-bold text-amber-500 select-none">
				<span className="h-2 w-2 animate-pulse rounded-full bg-amber-500" />
				CANCELLING...
			</span>
		);
	}

	if (st === 'CANCELLED') {
		return (
			<span className="inline-flex items-center gap-1.5 rounded-lg border border-zinc-500/30 bg-zinc-500/10 px-2.5 py-1 text-xs font-bold text-zinc-400 select-none">
				<span className="h-2 w-2 rounded-full bg-zinc-400" />
				CANCELLED
			</span>
		);
	}

	if (['RUNNING', 'DONE', 'SUCCESS', 'ACTIVE', 'OK'].includes(st)) {
		return (
			<span className="inline-flex items-center gap-1.5 rounded-lg border border-emerald-500/30 bg-emerald-500/10 px-2.5 py-1 text-xs font-bold text-emerald-500 select-none">
				<span className="h-2 w-2 animate-pulse rounded-full bg-emerald-500" />
				RUNNING
			</span>
		);
	}

	if (st === 'ERROR') {
		return (
			<span className="inline-flex items-center gap-1.5 rounded-lg border border-rose-500/30 bg-rose-500/10 px-2.5 py-1 text-xs font-bold text-rose-500 select-none">
				<span className="h-2 w-2 rounded-full bg-rose-500" />
				ERROR
			</span>
		);
	}

	return (
		<span className="inline-flex items-center gap-1.5 rounded-lg border border-rose-500/30 bg-rose-500/10 px-2.5 py-1 text-xs font-bold text-rose-500 select-none">
			<span className="h-2 w-2 rounded-full bg-rose-500" />
			STOPPED
		</span>
	);
}
