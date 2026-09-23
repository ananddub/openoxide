import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

/** A select sized for a table cell (no label, no hint). */
export function InlineSelect({
	value,
	onChange,
	options,
}: {
	value: string;
	onChange: (value: string) => void;
	options: {value: string; label: string}[];
}) {
	return (
		<Select value={value} onValueChange={next => next && onChange(next)}>
			<SelectTrigger className="h-9 w-full min-w-[84px]">
				<SelectValue />
			</SelectTrigger>
			<SelectContent>
				{options.map(option => (
					<SelectItem key={option.value} value={option.value}>
						{option.label}
					</SelectItem>
				))}
			</SelectContent>
		</Select>
	);
}
