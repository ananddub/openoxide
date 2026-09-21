import {Check} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {cn} from '#/api/utils';
import {OPTIONS, type ServiceKind} from './constants';

/** 2×2 grid of service-kind cards with active / SOON affordances. */
export function TypeCards({
	chosen,
	onPick,
}: {
	chosen: ServiceKind;
	onPick: (kind: ServiceKind) => void;
}) {
	return (
		<div className="grid grid-cols-2 gap-3 xl:grid-cols-4">
			{OPTIONS.map(o => {
				const isActive = o.id === chosen;
				return (
					<button
						key={o.id}
						type="button"
						onClick={() => onPick(o.id)}
						className={cn(
							'group relative flex flex-col items-start gap-2 rounded-xl border bg-card p-4 text-start transition-colors',
							isActive
								? 'border-primary/50 bg-primary/[0.04]'
								: 'border-border/50 hover:bg-muted/25',
						)}>
						<span className="flex size-9 items-center justify-center rounded-xl bg-muted/45 transition-colors group-hover:bg-muted/65">
							<o.icon
								className={cn(
									'size-5',
									isActive ? 'text-primary' : 'text-muted-foreground',
								)}
							/>
						</span>
						<span>
							<span className="block text-sm font-semibold text-foreground">
								{o.title}
							</span>
							<span className="mt-0.5 block text-[11px] leading-snug text-muted-foreground">
								{o.description}
							</span>
						</span>
						{o.soon && (
							<Badge
								variant="secondary"
								className="absolute end-3 top-3 px-1.5 py-0 text-[9px] font-semibold">
								SOON
							</Badge>
						)}
						{isActive && !o.soon && (
							<span className="absolute end-3 top-3 flex size-5 items-center justify-center rounded-full bg-primary text-primary-foreground">
								<Check className="size-3" />
							</span>
						)}
					</button>
				);
			})}
		</div>
	);
}
