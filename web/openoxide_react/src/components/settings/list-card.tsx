import type {ReactNode} from 'react';
import {cn} from '#/api/utils';

/**
 * OpenShip's list-card pattern (their Settings → Team page): a quiet
 * `rounded-xl bg-card` panel with a slim header bar — title, count/
 * description, the section's action on the right — and a `divide-y`
 * body of full-bleed rows. No frames, no nested cards.
 */
export function ListCard({
	title,
	description,
	action,
	children,
}: {
	title: ReactNode;
	description?: ReactNode;
	action?: ReactNode;
	children: ReactNode;
}) {
	return (
		<div className="overflow-hidden rounded-xl bg-card">
			<div className="flex items-center justify-between gap-4 border-b border-border/40 px-5 py-3.5">
				<div className="min-w-0">
					<p className="text-sm font-semibold text-foreground">{title}</p>
					{description && (
						<p className="truncate text-xs text-muted-foreground">
							{description}
						</p>
					)}
				</div>
				{action && <div className="shrink-0">{action}</div>}
			</div>
			{children}
		</div>
	);
}

/** A row inside a ListCard: icon tile, primary + secondary text, controls right. */
export function ListRow({
	icon,
	title,
	subtitle,
	badges,
	children,
	onClick,
	className,
}: {
	icon?: ReactNode;
	title: ReactNode;
	subtitle?: ReactNode;
	badges?: ReactNode;
	children?: ReactNode;
	onClick?: () => void;
	className?: string;
}) {
	const inner = (
		<>
			{icon}
			<div className="min-w-0 flex-1">
				<div className="flex min-w-0 flex-wrap items-center gap-2">
					<span className="truncate text-sm font-medium text-foreground">
						{title}
					</span>
					{badges}
				</div>
				{subtitle && (
					<p className="mt-0.5 truncate text-xs text-muted-foreground">
						{subtitle}
					</p>
				)}
			</div>
			{children && (
				<div className="flex shrink-0 items-center gap-1.5">
					{children}
				</div>
			)}
		</>
	);
	const cls = cn(
		'flex w-full items-center gap-4 px-5 py-4 text-start transition-colors hover:bg-muted/30',
		onClick && 'cursor-pointer',
		className,
	);
	return onClick ? (
		<button type="button" onClick={onClick} className={cls}>
			{inner}
		</button>
	) : (
		<div className={cls}>{inner}</div>
	);
}

/** Quiet icon tile for a ListRow — fill, not frame. */
export function RowTile({
	children,
	className,
}: {
	children: ReactNode;
	className?: string;
}) {
	return (
		<span
			className={cn(
				'flex size-9 shrink-0 items-center justify-center rounded-full bg-muted text-sm font-medium text-foreground',
				className,
			)}>
			{children}
		</span>
	);
}

/** Centered quiet empty state inside a ListCard body. */
export function ListEmpty({
	icon: Icon,
	title,
	description,
	action,
}: {
	icon: React.ComponentType<{className?: string}>;
	title: string;
	description: string;
	action?: ReactNode;
}) {
	return (
		<div className="flex flex-col items-center justify-center px-6 py-16 text-center">
			<Icon className="mb-3 size-8 text-muted-foreground/40" />
			<h3 className="text-sm font-medium text-foreground">{title}</h3>
			<p className="mt-1 max-w-sm text-xs leading-relaxed text-muted-foreground">
				{description}
			</p>
			{action && <div className="mt-4">{action}</div>}
		</div>
	);
}
