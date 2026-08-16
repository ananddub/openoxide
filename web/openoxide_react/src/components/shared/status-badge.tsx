interface StatusBadgeProps {
	status: string;
	isBuilding?: boolean;
	actionLoading?: string | null;
}

export function StatusBadge({status, isBuilding, actionLoading}: StatusBadgeProps) {
	const st = (status || '').toUpperCase();

	if (
		['QUEUED', 'STARTING', 'BUILDING', 'DEPLOYING', 'REBUILDING', 'REDEPLOYING'].includes(st) ||
		isBuilding ||
		actionLoading === 'start' ||
		actionLoading === 'deploy' ||
		actionLoading === 'redeploy'
	) {
		const label = st === 'QUEUED' ? 'QUEUED' 
			: st === 'BUILDING' ? 'BUILDING...'
			: st === 'DEPLOYING' ? 'DEPLOYING...'
			: st === 'REBUILDING' || st === 'REDEPLOYING' ? 'REDEPLOYING...'
			: actionLoading === 'deploy' || actionLoading === 'redeploy' ? 'DEPLOYING...'
			: 'STARTING...';

		return (
			<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-amber-500/10 text-amber-500 border-amber-500/30">
				<span className="w-2 h-2 rounded-full bg-amber-500 animate-pulse" />
				{label}
			</span>
		);
	}

	if (
		st === 'STOPPING' ||
		actionLoading === 'stop'
	) {
		return (
			<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-orange-500/10 text-orange-500 border-orange-500/30">
				<span className="w-2 h-2 rounded-full bg-orange-500 animate-pulse" />
				STOPPING...
			</span>
		);
	}

	if (
		st === 'CANCELLING' ||
		String(actionLoading) === 'cancel'
	) {
		return (
			<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-amber-500/10 text-amber-500 border-amber-500/30">
				<span className="w-2 h-2 rounded-full bg-amber-500 animate-pulse" />
				CANCELLING...
			</span>
		);
	}

	if (st === 'CANCELLED') {
		return (
			<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-zinc-500/10 text-zinc-400 border-zinc-500/30">
				<span className="w-2 h-2 rounded-full bg-zinc-400" />
				CANCELLED
			</span>
		);
	}

	if (['RUNNING', 'DONE', 'SUCCESS', 'ACTIVE', 'OK'].includes(st)) {
		return (
			<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-emerald-500/10 text-emerald-500 border-emerald-500/30">
				<span className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
				RUNNING
			</span>
		);
	}

	if (st === 'ERROR') {
		return (
			<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-rose-500/10 text-rose-500 border-rose-500/30">
				<span className="w-2 h-2 rounded-full bg-rose-500" />
				ERROR
			</span>
		);
	}

	return (
		<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-rose-500/10 text-rose-500 border-rose-500/30">
			<span className="w-2 h-2 rounded-full bg-rose-500" />
			STOPPED
		</span>
	);
}
