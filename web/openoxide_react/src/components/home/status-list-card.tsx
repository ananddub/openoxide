interface StatusItem {
	dotClass: string;
	label: string;
	count: number;
}

interface StatusListCardProps {
	label: string;
	items: StatusItem[];
}

export function StatusListCard({label, items}: StatusListCardProps) {
	return (
		<div className="flex min-h-[140px] flex-col gap-3 rounded-xl border border-border bg-card p-5 shadow-2xs transition-all duration-200 hover:border-border/80">
			<span className="text-xs font-medium tracking-wider text-muted-foreground uppercase">
				{label}
			</span>
			<ul className="flex flex-col gap-1.5">
				{items.map(item => (
					<li
						key={item.label}
						className="flex items-center gap-2.5 text-sm">
						<span
							className={`size-2 shrink-0 rounded-full ${item.dotClass}`}
							aria-hidden
						/>
						<span className="w-8 font-semibold text-foreground tabular-nums">
							{item.count}
						</span>
						<span className="font-normal text-muted-foreground">
							{item.label}
						</span>
					</li>
				))}
			</ul>
		</div>
	);
}
