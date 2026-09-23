import {Input} from '#/components/ui/input';
import {Button} from '#/components/ui/button';
import {Plus} from 'lucide-react';

/** Editable list of string rows (constraints / preferences / platforms). */
export function PlacementRows({
	label,
	values,
	placeholder,
	set,
}: {
	label: string;
	values: any;
	placeholder: string;
	set: (values: string[]) => void;
}) {
	const rows = Array.isArray(values)
		? values.map((item: any) =>
				typeof item === 'string'
					? item
					: item.SpreadDescriptor ||
						(item.Architecture || item.OS
							? `${item.Architecture || ''}:${item.OS || ''}`
							: ''),
			)
		: values
			? String(values).split(',')
			: [''];
	return (
		<div className="space-y-2">
			<div>
				<h4 className="text-sm font-medium">{label}</h4>
			</div>
			{rows.map((row: string, index: number) => (
				<div key={index} className="flex gap-2">
					<Input
						value={row}
						placeholder={placeholder}
						onChange={e =>
							set(
								rows.map((item, i) =>
									i === index ? e.target.value : item,
								),
							)
						}
					/>
					{rows.length > 1 && (
						<Button
							type="button"
							variant="destructive"
							size="sm"
							onClick={() => set(rows.filter((_, i) => i !== index))}>
							Remove
						</Button>
					)}
				</div>
			))}
			<Button
				type="button"
				variant="outline"
				size="sm"
				className="w-fit"
				onClick={() => set([...rows, ''])}>
				<Plus className="mr-1 size-3" />
				Add {label.toLowerCase()}
			</Button>
		</div>
	);
}
