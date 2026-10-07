import type {ReactNode} from 'react';
import {Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';

/** Card frame shared by the server's management tabs: title, one caption line and an optional refresh affordance. */
export function Panel({
	title,
	caption,
	isRefreshing,
	onRefresh,
	children,
}: {
	title: string;
	caption?: string;
	isRefreshing?: boolean;
	onRefresh?: () => void;
	children: ReactNode;
}) {
	return (
		<section className="overflow-hidden rounded-xl border border-border bg-card">
			<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
				<div className="min-w-0 flex-1">
					<h3 className="text-[15px] font-semibold text-foreground">
						{title}
					</h3>
					{caption && (
						<p className="text-xs text-muted-foreground">{caption}</p>
					)}
				</div>
				{onRefresh && (
					<Button
						variant="outline"
						size="sm"
						onClick={onRefresh}
						disabled={isRefreshing}
						className="h-8 shrink-0 cursor-pointer gap-1.5 text-xs">
						<Loader2
							className={`size-3.5 ${isRefreshing ? 'animate-spin' : ''}`}
						/>
						Refresh
					</Button>
				)}
			</div>
			{children}
		</section>
	);
}

/** Status pill with the colours every server tab uses for ok / indeterminate / not-ok. */
export function PanelPill({
	ok,
	label,
	tone = 'default',
}: {
	ok: boolean;
	label: string;
	/** `warn` keeps an indeterminate state amber; `muted` marks a finished one as neither good nor failing. */
	tone?: 'default' | 'warn' | 'muted';
}) {
	return (
		<span
			className={`rounded-full px-2 py-0.5 text-[11px] font-medium ${
				ok
					? 'bg-emerald-500/10 text-emerald-500'
					: tone === 'warn'
						? 'bg-amber-500/10 text-amber-600 dark:text-amber-500'
						: tone === 'muted'
							? 'bg-muted text-muted-foreground'
							: 'bg-rose-500/10 text-rose-500'
			}`}>
			{label}
		</span>
	);
}
