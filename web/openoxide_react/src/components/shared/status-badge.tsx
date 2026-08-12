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
		actionLoading === 'deploy'
	) {
		return (
			<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-amber-500/10 text-amber-500 border-amber-500/30">
				<span className="w-2 h-2 rounded-full bg-amber-500 animate-pulse" />
				{st === 'QUEUED' ? 'QUEUED' : 'STARTING...'}
			</span>
		);
	}

	if (
		['STOPPING', 'CANCELLING'].includes(st) ||
		actionLoading === 'stop' ||
		String(actionLoading) === 'cancel'
	) {
		return (
			<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-rose-500/10 text-rose-500 border-rose-500/30">
				<span className="w-2 h-2 rounded-full bg-rose-500 animate-pulse" />
				STOPPING...
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
