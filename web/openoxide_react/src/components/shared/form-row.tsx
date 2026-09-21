/** Labeled field wrapper: a small heading above its control. Shared by the
 * sheet/dialog forms so label styling stays consistent. */
export function FormRow({
	label,
	children,
}: {
	label: string;
	children: React.ReactNode;
}) {
	return (
		<div className="flex flex-col gap-2">
			<label className="text-sm leading-none font-medium">{label}</label>
			{children}
		</div>
	);
}
