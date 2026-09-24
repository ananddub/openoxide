import type {ReactNode} from 'react';
import {cn} from '#/api/utils';

/** Sheet-native form building blocks. The database Advanced panels reuse these so every side-panel form shares one airy, legible rhythm instead of the cramped micro-label look the inline accordions used. */

/** Label + control + optional helper line, stacked. */
export function Field({
	label,
	hint,
	htmlFor,
	className,
	children,
}: {
	label: ReactNode;
	hint?: ReactNode;
	htmlFor?: string;
	className?: string;
	children: ReactNode;
}) {
	return (
		<div className={cn('flex flex-col gap-2', className)}>
			<label
				htmlFor={htmlFor}
				className="text-sm leading-none font-medium text-foreground">
				{label}
			</label>
			{children}
			{hint && (
				<p className="text-[13px] leading-snug text-muted-foreground">
					{hint}
				</p>
			)}
		</div>
	);
}

/** Responsive two-column field grid used inside a sheet body. */
export function FieldGrid({
	className,
	children,
}: {
	className?: string;
	children: ReactNode;
}) {
	return (
		<div
			className={cn('grid grid-cols-1 gap-5 sm:grid-cols-2', className)}>
			{children}
		</div>
	);
}

/** A titled sub-section within a sheet, separated by a hairline rule. */
export function SheetGroup({
	title,
	description,
	children,
}: {
	title: ReactNode;
	description?: ReactNode;
	children: ReactNode;
}) {
	return (
		<section className="flex flex-col gap-5">
			<div className="flex flex-col gap-1 border-b border-border pb-3">
				<h4 className="text-xs font-semibold tracking-wider text-muted-foreground uppercase">
					{title}
				</h4>
				{description && (
					<p className="text-[13px] leading-snug text-muted-foreground">
						{description}
					</p>
				)}
			</div>
			{children}
		</section>
	);
}

/** Inline info / warning callout. */
export function SheetNote({
	tone = 'info',
	icon,
	children,
}: {
	tone?: 'info' | 'warn';
	icon?: ReactNode;
	children: ReactNode;
}) {
	return (
		<div
			className={cn(
				'flex items-start gap-2.5 rounded-lg border px-3.5 py-3 text-[13px] leading-snug',
				tone === 'warn'
					? 'border-amber-500/25 bg-amber-500/[0.07] text-amber-700 dark:text-amber-300'
					: 'border-blue-500/20 bg-blue-500/[0.06] text-blue-700 dark:text-blue-300',
			)}>
			{icon && <span className="mt-px shrink-0 opacity-80">{icon}</span>}
			<span>{children}</span>
		</div>
	);
}
