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
		<div className="rounded-xl border border-border bg-card p-5 min-h-[140px] flex flex-col gap-3 shadow-2xs hover:border-border/80 transition-all duration-200">
			<span className="text-xs uppercase tracking-wider text-muted-foreground font-medium">
				{label}
			</span>
			<ul className="flex flex-col gap-1.5">
				{items.map((item) => (
					<li key={item.label} className="flex items-center gap-2.5 text-sm">
						<span
							className={`size-2 rounded-full shrink-0 ${item.dotClass}`}
							aria-hidden
						/>
						<span className="font-semibold tabular-nums w-8 text-foreground">{item.count}</span>
						<span className="text-muted-foreground font-normal">{item.label}</span>
					</li>
				))}
			</ul>
		</div>
	);
}
