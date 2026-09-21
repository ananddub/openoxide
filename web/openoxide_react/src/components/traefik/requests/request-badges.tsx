import {Badge} from '#/components/ui/badge';

/** HTTP status → colored outline badge. */
export function getStatusBadge(code: number) {
	if (code >= 200 && code < 300) {
		return (
			<Badge
				variant="outline"
				className="border-emerald-500/20 bg-emerald-500/10 px-1.5 py-0 font-mono text-[11px] font-semibold text-emerald-500">
				{code} OK
			</Badge>
		);
	}
	if (code >= 300 && code < 400) {
		return (
			<Badge
				variant="outline"
				className="border-blue-500/20 bg-blue-500/10 px-1.5 py-0 font-mono text-[11px] font-semibold text-blue-400">
				{code} Redirect
			</Badge>
		);
	}
	if (code >= 400 && code < 500) {
		return (
			<Badge
				variant="outline"
				className="border-amber-500/20 bg-amber-500/10 px-1.5 py-0 font-mono text-[11px] font-semibold text-amber-500">
				{code} Error
			</Badge>
		);
	}
	return (
		<Badge
			variant="outline"
			className="border-rose-500/20 bg-rose-500/10 px-1.5 py-0 font-mono text-[11px] font-semibold text-rose-500">
			{code} Fatal
		</Badge>
	);
}

/** HTTP method → colored monospace chip. */
export function getMethodBadge(method: string) {
	const m = method.toUpperCase();
	switch (m) {
		case 'GET':
			return (
				<span className="rounded bg-emerald-500/15 px-2 py-0.5 font-mono text-[10px] font-bold text-emerald-400">
					GET
				</span>
			);
		case 'POST':
			return (
				<span className="rounded bg-blue-500/15 px-2 py-0.5 font-mono text-[10px] font-bold text-blue-400">
					POST
				</span>
			);
		case 'PUT':
			return (
				<span className="rounded bg-amber-500/15 px-2 py-0.5 font-mono text-[10px] font-bold text-amber-400">
					PUT
				</span>
			);
		case 'DELETE':
			return (
				<span className="rounded bg-rose-500/15 px-2 py-0.5 font-mono text-[10px] font-bold text-rose-400">
					DEL
				</span>
			);
		default:
			return (
				<span className="rounded bg-muted px-2 py-0.5 font-mono text-[10px] font-bold text-muted-foreground">
					{m}
				</span>
			);
	}
}
