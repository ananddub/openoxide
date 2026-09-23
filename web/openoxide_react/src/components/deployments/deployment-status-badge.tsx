import {Badge} from '#/components/ui/badge';
import {cn} from '#/api/utils';
import {isBusyStatus} from './deployment-utils';

type Tone = 'emerald' | 'rose' | 'amber' | 'zinc';

const TONES: Record<Tone, string> = {
	emerald: 'border-emerald-500/30 bg-emerald-500/10 text-emerald-500',
	rose: 'border-rose-500/30 bg-rose-500/10 text-rose-500',
	amber: 'border-amber-500/30 bg-amber-500/10 text-amber-500',
	zinc: 'border-border bg-muted/60 text-muted-foreground',
};

const DOTS: Record<Tone, string> = {
	emerald: 'bg-emerald-500',
	rose: 'bg-rose-500',
	amber: 'bg-amber-500',
	zinc: 'bg-muted-foreground/60',
};

const RULES: {
	keys: string[];
	tone: Tone;
	label: string;
	pulse?: boolean;
}[] = [
	{
		keys: ['CANCELLING'],
		tone: 'amber',
		label: 'CANCELLING...',
		pulse: true,
	},
	{keys: ['CANCELLED'], tone: 'zinc', label: 'CANCELLED'},
	{
		keys: ['DONE', 'HEALTHY', 'SUCCESS', 'DEPLOYED'],
		tone: 'emerald',
		label: 'SUCCESS',
	},
	{keys: ['ERROR', 'FAILED', 'CRASHED'], tone: 'rose', label: 'FAILED'},
];

/** Colored dot-badge for a deployment status (shared by history + queue). */
export function DeploymentStatusBadge({status}: {status: string}) {
	const s = (status || '').toUpperCase();
	const rule = RULES.find(r => r.keys.includes(s));
	const busy = !rule && isBusyStatus(s);

	if (!rule && !busy) {
		return (
			<Badge
				variant="secondary"
				className="gap-1.5 py-0.5 text-[10px] font-bold tracking-wider uppercase">
				<span className="size-1.5 rounded-full bg-muted-foreground/50" />
				{s}
			</Badge>
		);
	}

	const tone = rule?.tone ?? 'amber';
	return (
		<Badge
			variant="outline"
			className={cn(
				'gap-1.5 py-0.5 text-[10px] font-bold tracking-wider uppercase',
				TONES[tone],
				busy && 'animate-pulse',
			)}>
			<span
				className={cn(
					'size-1.5 rounded-full',
					DOTS[tone],
					busy && 'animate-ping',
					rule?.pulse && 'animate-pulse',
				)}
			/>
			{rule?.label ?? s}
		</Badge>
	);
}
