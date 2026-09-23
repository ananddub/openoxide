import type {ReactNode} from 'react';
import {Check} from 'lucide-react';
import {Label} from '#/components/ui/label';
import {cn} from '#/api/utils';

/**
 * Building blocks for the "Add Service" sheet grammar: a checkpoint row that
 * tells you what is being asked, radio-style choice rows, and a field that
 * gives its control room to breathe.
 */

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
		<div className="animate-in fade-in-0 slide-in-from-right-4 duration-200 ease-out">
			{children}
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
				<span className="flex size-5 shrink-0 animate-in zoom-in-75 duration-150 items-center justify-center rounded-full bg-primary text-primary-foreground">
					<Check className="size-3" />
				</span>
			)}
		</button>
	);
}

export function WizardField({
	label,
	hint,
	required,
	optional,
	action,
	children,
}: {
	label: string;
	hint?: ReactNode;
	required?: boolean;
	optional?: boolean;
	action?: ReactNode;
	children: ReactNode;
}) {
	return (
		<div className="flex flex-col gap-1.5">
			<div className="flex items-center justify-between gap-3">
				<Label className="text-[13px] font-medium text-foreground">
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
