import {useId} from 'react';
import {cn} from '#/api/utils';

/** Deep, saturated palette per usage band; the ring stroke and its halo share one tone. */
const TONES = {
	ok: {
		from: '#2563eb',
		to: '#0284c7',
		glow: 'rgba(37,99,235,0.35)',
	},
	warm: {
		from: '#d97706',
		to: '#ea580c',
		glow: 'rgba(217,119,6,0.35)',
	},
	hot: {
		from: '#dc2626',
		to: '#b91c1c',
		glow: 'rgba(220,38,38,0.4)',
	},
};

function toneFor(percent: number) {
	if (percent >= 85) return TONES.hot;
	if (percent >= 60) return TONES.warm;
	return TONES.ok;
}

/** Geist bolt glyph shown in the gauge centre instead of the raw number. */
function GeistBolt() {
	return (
		<svg
			viewBox="0 0 16 16"
			className="size-2.5"
			fill="currentColor"
			aria-hidden>
			<path
				fillRule="evenodd"
				d="M5.51 3.62 3.76 8.35a1 1 0 0 1-.93.65H0V7.5h2.48l2.09-5.64a1 1 0 0 1 1.87-.01l4.07 10.6 1.73-4.32a1 1 0 0 1 .93-.63H16V9h-2.49l-2.08 5.19a1 1 0 0 1-1.86-.02z"
				clipRule="evenodd"
			/>
		</svg>
	);
}

/** MagicUI-style animated circular gauge — value arc grows clockwise from the top, the leftover arc sits after a moving gap, gradient stroke + glow, bolt glyph in the centre. */
export function UsageRing({
	percent,
	ready,
	title,
}: {
	percent: number;
	ready: boolean;
	title: string;
}) {
	const C = 2 * Math.PI * 45;
	const percentPx = C / 100;
	const GAP = 5; // gap-percent — 18° of breathing room between the two arcs
	const pct = Math.round(Math.min(100, Math.max(0, percent)));
	const current = ready ? pct : 0;
	const tone = toneFor(pct);
	const gradientId = useId();
	const origin = {transformOrigin: '50px 50px'};
	return (
		<div className="relative size-8 shrink-0" title={title}>
			<svg viewBox="0 0 100 100" className="size-full" fill="none">
				<defs>
					<linearGradient id={gradientId} x1="0" y1="0" x2="1" y2="1">
						<stop offset="0%" stopColor={tone.from} />
						<stop offset="100%" stopColor={tone.to} />
					</linearGradient>
				</defs>
				{current < 90 && (
					<circle
						cx="50"
						cy="50"
						r="45"
						strokeWidth="10"
						strokeLinecap="round"
						strokeDasharray={`${(90 - current) * percentPx} ${C}`}
						className={cn(
							'stroke-muted-foreground/30 transition-all duration-700 ease-out',
							!ready && 'opacity-30',
						)}
						style={{
							...origin,
							transform: `rotate(${360 - 90 - GAP * 3.6}deg) scaleY(-1)`,
						}}
					/>
				)}
				<circle
					cx="50"
					cy="50"
					r="45"
					strokeWidth="10"
					strokeLinecap="round"
					strokeDasharray={`${current * percentPx} ${C}`}
					stroke={`url(#${gradientId})`}
					style={{
						...origin,
						transform: 'rotate(-90deg)',
						filter: `drop-shadow(0 0 3px ${tone.glow})`,
					}}
					className={cn(
						'transition-all duration-700 ease-out',
						!ready && 'opacity-30',
					)}
				/>
			</svg>
			<span
				className={cn(
					'absolute inset-0 flex items-center justify-center',
					ready ? 'text-foreground' : 'text-muted-foreground',
				)}>
				<GeistBolt />
			</span>
		</div>
	);
}
