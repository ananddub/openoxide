interface StatCardProps {
	label: string;
	value: string;
	delta?: string;
}

export function StatCard({label, value, delta}: StatCardProps) {
	return (
		<div className="rounded-xl border border-border bg-card p-5 min-h-[140px] flex flex-col justify-between shadow-2xs hover:border-border/80 transition-all duration-200">
			<span className="text-xs uppercase tracking-wider text-muted-foreground font-medium">
				{label}
			</span>
			<div className="flex flex-col gap-1">
				<span className="text-3xl font-semibold tracking-tight text-foreground">{value}</span>
				{delta && (
					<span className="text-xs text-muted-foreground font-normal">{delta}</span>
				)}
			</div>
		</div>
	);
}
