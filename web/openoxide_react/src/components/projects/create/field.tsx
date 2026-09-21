export function Field({
	label,
	children,
}: {
	label: string;
	children: React.ReactNode;
}) {
	return (
		<div className="flex flex-col gap-1.5">
			<span className="text-xs font-bold text-muted-foreground">
				{label}
			</span>
			{children}
		</div>
	);
}
