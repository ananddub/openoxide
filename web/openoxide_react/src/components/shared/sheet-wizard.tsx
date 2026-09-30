import type {ReactNode} from 'react';
import {Check, Loader, X} from 'lucide-react';
import {Label} from '#/components/ui/label';
import {cn} from '#/api/utils';

/** Building blocks for the "Add Service" sheet grammar: a checkpoint row that tells you what is being asked, radio-style choice rows, and a field that gives its control room to breathe. */

export type WizardStep = {id: string; label: string};

export function StepCheckpoints({
	steps,
	idx,
}: {
	steps: WizardStep[];
	idx: number;
}) {
	return (
		<div className="flex items-center gap-1.5 pt-1">
			{steps.map((s, i) => (
				<span key={s.id} className="flex items-center gap-1.5">
					{i > 0 && (
						<span
							className={cn(
								'h-px w-3',
								i <= idx ? 'bg-primary/50' : 'bg-border',
							)}
						/>
					)}
					<span
						className={cn(
							'flex items-center gap-1.5 text-[11px]',
							i === idx
								? 'font-medium text-foreground'
								: 'text-muted-foreground',
						)}>
						<span
							className={cn(
								'size-1.5 rounded-full',
								i < idx
									? 'bg-primary'
									: i === idx
										? 'bg-primary ring-2 ring-primary/25'
										: 'bg-muted-foreground/40',
							)}
						/>
						{s.label}
					</span>
				</span>
			))}
		</div>
	);
}

/** Wrapper so each step slides in; the step itself owns its spacing. */
export function StepBody({children}: {children: ReactNode}) {
	return (
		<div className="animate-in duration-200 ease-out fade-in-0 slide-in-from-right-4">
			{children}
		</div>
	);
}

/** Vertical timeline of wizard steps — nodes with connector lines; the
 *  active node carries the step's content directly beneath its label.
 *  `nodeState` re-skins the active node for live work (spinner / verdict). */
export function StepTimeline({
	steps,
	idx,
	nodeState,
	children,
}: {
	steps: WizardStep[];
	idx: number;
	nodeState?: 'running' | 'success' | 'failed';
	children: ReactNode;
}) {
	return (
		<div className="flex flex-col">
			{steps.map((s, i) => {
				const done = i < idx;
				const active = i === idx;
				const lastRow = i === steps.length - 1;
				return (
					<div key={s.id} className="flex gap-3">
						<div className="flex w-5 shrink-0 flex-col items-center">
							<span
								className={cn(
									'flex size-5 shrink-0 items-center justify-center rounded-full text-[10px] font-semibold transition-colors duration-200',
									done &&
										'bg-emerald-500/15 text-emerald-600 dark:text-emerald-400',
									active &&
										nodeState === 'success' &&
										'bg-emerald-500/15 text-emerald-600 dark:text-emerald-400',
									active &&
										nodeState === 'failed' &&
										'bg-red-500/15 text-red-600 dark:text-red-400',
									active &&
										!nodeState &&
										'bg-primary text-primary-foreground ring-4 ring-primary/15',
									active &&
										nodeState === 'running' &&
										'bg-primary text-primary-foreground ring-4 ring-primary/15',
									!done && !active && 'border text-muted-foreground/50',
								)}>
								{done ? (
									<Check className="size-3" />
								) : active && nodeState === 'running' ? (
									<Loader className="size-3 animate-spin" />
								) : active && nodeState === 'success' ? (
									<Check className="size-3" />
								) : active && nodeState === 'failed' ? (
									<X className="size-3" />
								) : (
									i + 1
								)}
							</span>
							{!lastRow && (
								<span
									className={cn(
										'w-px grow',
										done ? 'bg-emerald-500/40' : 'bg-border',
									)}
								/>
							)}
						</div>
						<div className={cn('min-w-0 flex-1', !lastRow && 'pb-5')}>
							<p
								className={cn(
									'text-sm leading-5',
									active
										? 'font-medium text-foreground'
										: done
											? 'text-emerald-600 dark:text-emerald-400'
											: 'text-muted-foreground/70',
								)}>
								{s.label}
							</p>
							{active && <div className="pt-3">{children}</div>}
						</div>
					</div>
				);
			})}
		</div>
	);
}

export function OptionRow({
	selected,
	onClick,
	icon,
	title,
	subtitle,
	disabled,
}: {
	selected: boolean;
	onClick: () => void;
	icon?: ReactNode;
	title: string;
	subtitle?: string;
	disabled?: boolean;
}) {
	return (
		<button
			type="button"
			onClick={onClick}
			disabled={disabled}
			className={cn(
				'flex w-full items-center gap-3.5 rounded-xl border px-4 py-3.5 text-start transition-all duration-150',
				selected
					? 'border-primary/40 bg-primary/10'
					: 'border-border/50 hover:border-border hover:bg-muted/30',
				disabled && 'cursor-not-allowed opacity-60',
			)}>
			{icon && (
				<span className="flex size-8 shrink-0 items-center justify-center text-muted-foreground">
					{icon}
				</span>
			)}
			<span className="min-w-0 flex-1">
				<span className="block text-sm font-medium text-foreground">
					{title}
				</span>
				{subtitle && (
					<span className="block truncate text-[13px] text-muted-foreground">
						{subtitle}
					</span>
				)}
			</span>
			{selected && (
				<span className="flex size-5 shrink-0 animate-in items-center justify-center rounded-full bg-primary text-primary-foreground duration-150 zoom-in-75">
					<Check className="size-3" />
				</span>
			)}
		</button>
	);
}

export function WizardField({
	label,
	htmlFor,
	hint,
	required,
	optional,
	action,
	children,
}: {
	label: string;
	htmlFor?: string;
	hint?: ReactNode;
	required?: boolean;
	optional?: boolean;
	action?: ReactNode;
	children: ReactNode;
}) {
	return (
		<div className="flex flex-col gap-1.5">
			<div className="flex items-center justify-between gap-3">
				<Label
					htmlFor={htmlFor}
					className="text-xs font-semibold text-foreground">
					{label}
					{required && <span className="text-primary"> *</span>}
					{optional && (
						<span className="text-[12px] font-normal text-muted-foreground">
							{' '}
							(optional)
						</span>
					)}
				</Label>
				{action}
			</div>
			{children}
			{hint && (
				<p className="text-[13px] leading-relaxed text-muted-foreground">
					{hint}
				</p>
			)}
		</div>
	);
}
