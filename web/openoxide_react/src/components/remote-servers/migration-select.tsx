import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

/** Labelled small Select used by the stateful migration picker. */
export function MigrationSelect({
	label,
	value,
	placeholder,
	options,
	emptyHint,
	onChange,
}: {
	label: string;
	value: string;
	placeholder: string;
	options: Array<{value: string; label: string}>;
	emptyHint?: string;
	onChange: (v: string) => void;
}) {
	return (
		<div className="flex flex-col gap-2">
			<label className="text-[11px] font-semibold text-foreground">
				{label}
			</label>
			<Select value={value} onValueChange={v => onChange(v ?? '')}>
				<SelectTrigger size="sm" className="h-9 w-full text-xs">
					<SelectValue placeholder={placeholder} />
				</SelectTrigger>
				<SelectContent>
					{options.map(o => (
						<SelectItem key={o.value} value={o.value}>
							{o.label}
						</SelectItem>
					))}
				</SelectContent>
			</Select>
			{emptyHint && options.length === 0 && (
				<p className="text-[11px] text-amber-500">{emptyHint}</p>
			)}
		</div>
	);
}
