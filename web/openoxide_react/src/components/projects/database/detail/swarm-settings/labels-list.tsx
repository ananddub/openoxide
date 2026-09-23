import {Input} from '#/components/ui/input';
import {DataTable} from './list-shell';

type Label = {key: string; value: string};

/** Editable key/value label list rendered as a two-column table. */
export function LabelsList({
	value,
	set,
}: {
	value: any;
	set: (key: string, value: any) => void;
}) {
	const labels: Label[] = Array.isArray(value)
		? value
		: value?.key
			? [{key: value.key, value: value.value || ''}]
			: [];
	return (
		<DataTable
			title="Labels"
			description="Add key-value labels to your service."
			addLabel="Add label"
			empty="No labels yet."
			rows={labels}
			columns={[
				{
					header: 'Key',
					cell: (row, update) => (
						<Input
							value={row.key || ''}
							placeholder="com.example.app.name"
							className="h-9"
							onChange={e => update({key: e.target.value})}
						/>
					),
				},
				{
					header: 'Value',
					cell: (row, update) => (
						<Input
							value={row.value || ''}
							placeholder="my-app"
							className="h-9"
							onChange={e => update({value: e.target.value})}
						/>
					),
				},
			]}
			onAdd={() => set('labels', [...labels, {key: '', value: ''}])}
			onUpdate={(index, patch) =>
				set(
					'labels',
					labels.map((item, i) =>
						i === index ? {...item, ...patch} : item,
					),
				)
			}
			onRemove={index =>
				set(
					'labels',
					labels.filter((_, i) => i !== index),
				)
			}
		/>
	);
}
