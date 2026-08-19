interface StatCardProps {
	label: string;
	value: string;
	delta?: string;
}

export function StatCard({label, value, delta}: StatCardProps) {
	return (
		<div className="flex min-h-[140px] flex-col justify-between rounded-xl border border-border bg-card p-5 shadow-2xs transition-all duration-200 hover:border-border/80">
			<span className="text-xs font-medium tracking-wider text-muted-foreground uppercase">
				{label}
			</span>
			<div className="flex flex-col gap-1">
				<span className="text-3xl font-semibold tracking-tight text-foreground">
					{value}
				</span>
				{delta && (
					<span className="text-xs font-normal text-muted-foreground">
						{delta}
					</span>
				)}
			</div>
		</div>
	);
}
