type Tone = 'amber' | 'orange' | 'emerald' | 'rose' | 'zinc';

const TONES: Record<Tone, {badge: string; dot: string}> = {
	amber: {badge: 'bg-amber-500/10 text-amber-500', dot: 'bg-amber-500'},
	orange: {
		badge: 'bg-orange-500/10 text-orange-500',
		dot: 'bg-orange-500',
	},
	emerald: {
		badge: 'bg-emerald-500/10 text-emerald-500',
		dot: 'bg-emerald-500',
	},
	rose: {badge: 'bg-rose-500/10 text-rose-500', dot: 'bg-rose-500'},
	zinc: {badge: 'bg-zinc-500/10 text-zinc-400', dot: 'bg-zinc-400'},
};

const RULES: {
	keys: string[];
	tone: Tone;
	label: string;
	pulse?: boolean;
}[] = [
	{keys: ['queued'], tone: 'amber', label: 'Queued', pulse: true},
	{keys: ['cancelling'], tone: 'amber', label: 'Cancelling', pulse: true},
	{keys: ['stopping'], tone: 'orange', label: 'Stopping', pulse: true},
	{keys: ['starting'], tone: 'amber', label: 'Starting', pulse: true},
	{
		keys: ['deploying', 'building', 'loading'],
		tone: 'amber',
		label: 'Deploying',
		pulse: true,
	},
	{keys: ['cancelled'], tone: 'zinc', label: 'Cancelled'},
	{
		keys: ['running', 'active', 'healthy', 'up', 'done'],
		tone: 'emerald',
		label: 'Running',
	},
	{
		keys: ['err', 'fail', 'unhealthy', 'crash'],
		tone: 'rose',
		label: 'Error',
	},
];

/** Data-driven service status pill (was 9 near-identical JSX blocks). */
export function ServiceStatusBadge({status}: {status: string}) {
	const s = (status || 'done').toLowerCase();
	const rule = RULES.find(r => r.keys.some(k => s.includes(k)));
	const tone = TONES[rule ? rule.tone : 'zinc'];
	const label = rule ? rule.label : 'Stopped';

	return (
		<div
			className={`inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-semibold ${tone.badge}`}>
			<span
				className={`size-2 shrink-0 rounded-full ${tone.dot} ${rule?.pulse ? 'animate-pulse' : ''}`}
			/>
			<span>{label}</span>
		</div>
	);
}
