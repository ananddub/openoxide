import {useRef, type ComponentType, type KeyboardEvent} from 'react';
import {cn} from '#/api/utils';

/** Geist "Switch" (https://vercel.com/geist/switch) — a segmented selector for a small set (2–3, at most a handful) of mutually exclusive options. Radio semantics: one segment is checked, Arrow keys cycle the selection. The active segment inverts (solid foreground fill), the rest stay quiet. */

export type SwitchOption<K extends string> = {
	key: K;
	label: string;
	icon?: ComponentType<{className?: string}>;
	count?: number;
};

export function GeistSwitch<K extends string>({
	options,
	value,
	onChange,
	fill,
	className,
}: {
	options: SwitchOption<K>[];
	value: K;
	onChange: (key: K) => void;
	/** Stretch segments evenly across the full width (toolbars, dialogs). */
	fill?: boolean;
	className?: string;
}) {
	const refs = useRef<(HTMLButtonElement | null)[]>([]);

	const handleKeyDown = (event: KeyboardEvent, index: number) => {
		let next: number | null = null;
		if (event.key === 'ArrowRight' || event.key === 'ArrowDown') {
			next = (index + 1) % options.length;
		} else if (event.key === 'ArrowLeft' || event.key === 'ArrowUp') {
			next = (index - 1 + options.length) % options.length;
		}
		if (next === null) return;
		event.preventDefault();
		onChange(options[next].key);
		requestAnimationFrame(() => refs.current[next]?.focus());
	};

	return (
		<div
			role="radiogroup"
			aria-orientation="horizontal"
			className={cn(
				'inline-flex w-fit flex-wrap items-center gap-0.5 rounded-full border border-border bg-background p-0.5',
				fill && 'flex w-full',
				className,
			)}>
			{options.map((option, index) => {
				const active = option.key === value;
				const Icon = option.icon;
				return (
					<button
						key={option.key}
						ref={node => {
							refs.current[index] = node;
						}}
						type="button"
						role="radio"
						aria-checked={active}
						tabIndex={active ? 0 : -1}
						onClick={() => onChange(option.key)}
						onKeyDown={event => handleKeyDown(event, index)}
						className={cn(
							'inline-flex cursor-pointer items-center justify-center gap-1.5 rounded-full px-3.5 py-1 text-[13px] font-medium whitespace-nowrap transition-colors outline-none focus-visible:ring-2 focus-visible:ring-ring/40',
							fill && 'flex-1',
							active
								? 'bg-foreground text-background'
								: 'text-muted-foreground hover:text-foreground',
						)}>
						{Icon && <Icon className="size-3.5" />}
						{option.label}
						{option.count !== undefined && (
							<span
								className={cn(
									'rounded-full px-1.5 py-px text-[10px] font-medium tabular-nums',
									active
										? 'bg-background/20 text-background'
										: 'bg-muted text-muted-foreground',
								)}>
								{option.count}
							</span>
						)}
					</button>
				);
			})}
		</div>
	);
}
