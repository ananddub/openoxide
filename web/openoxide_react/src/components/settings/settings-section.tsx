import {useState, type ComponentType, type ReactNode} from 'react';
import {ChevronDown} from 'lucide-react';
import {cn} from '#/api/utils';

/**
 * OpenShip's settings section card. In dark mode the panel IS the
 * separation: `bg-card` lifts off the page, no resting border (the
 * `--th-card-bd` ladder). Only the header row carries a hairline
 * divider, and that's `` against the card fill, not an
 * outer frame.
 */
export function SettingsSection({
	icon: Icon,
	title,
	description,
	children,
	action,
	iconBg = 'bg-foreground/[0.05]',
	iconColor = 'text-foreground',
	collapsible = false,
	defaultOpen = false,
}: {
	icon: ComponentType<{className?: string}>;
	title: string;
	description: string;
	children: ReactNode;
	action?: ReactNode;
	iconBg?: string;
	iconColor?: string;
	collapsible?: boolean;
	defaultOpen?: boolean;
}) {
	const [open, setOpen] = useState(defaultOpen);
	const expanded = collapsible ? open : true;

	const header = (
		<>
			<div
				className={cn(
					'flex size-9 shrink-0 items-center justify-center rounded-xl',
					iconBg,
				)}>
				<Icon className={cn('size-[18px]', iconColor)} />
			</div>
			<div className="min-w-0 flex-1">
				<h2 className="text-[15px] font-semibold text-foreground">
					{title}
				</h2>
				<p className="text-xs text-muted-foreground">{description}</p>
			</div>
			{collapsible && (
				<ChevronDown
					className={cn(
						'size-4 shrink-0 text-muted-foreground transition-transform',
						open && 'rotate-180',
					)}
				/>
			)}
		</>
	);

	return (
		<div className="rounded-xl border bg-card shadow-[0_1px_1px_rgba(0,0,0,0.02),0_2px_2px_rgba(0,0,0,0.04)]">
			<div className={cn('flex items-center', expanded && 'border-b ')}>
				{collapsible ? (
					<button
						type="button"
						onClick={() => setOpen(v => !v)}
						aria-expanded={open}
						className="flex min-w-0 flex-1 items-center gap-3 px-5 py-4 text-start">
						{header}
					</button>
				) : (
					<div className="flex min-w-0 flex-1 items-center gap-3 px-5 py-4">
						{header}
					</div>
				)}
				{action && <div className="shrink-0 ps-3 pe-5">{action}</div>}
			</div>
			{expanded && <div className="p-5">{children}</div>}
		</div>
	);
}

/**
 * OpenShip's preference row: a quiet filled row inside a section — title +
 * hint on the left, the control on the right. Fill, not border: the row
 * separates from the card by tone alone.
 */
export function SectionRow({
	title,
	hint,
	children,
}: {
	title: ReactNode;
	hint?: ReactNode;
	children?: ReactNode;
}) {
	return (
		<div className="flex items-center justify-between gap-4 rounded-lg border bg-card p-4">
			<div className="min-w-0">
				<p className="text-sm font-medium text-foreground">{title}</p>
				{hint && <p className="text-xs text-muted-foreground">{hint}</p>}
			</div>
			{children && <div className="shrink-0">{children}</div>}
		</div>
	);
}
