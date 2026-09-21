import {Input} from '#/components/ui/input';
import {Button} from '#/components/ui/button';
import {Plus} from 'lucide-react';
import {Field} from './field';

/** Editable key/value label list for the swarm labels form. */
export function LabelsList({
	value,
	set,
}: {
	value: any;
	set: (key: string, value: any) => void;
}) {
	const labels = Array.isArray(value)
		? value
		: value?.key
			? [{key: value.key, value: value.value || ''}]
			: [];
	const update = (index: number, patch: any) =>
		set(
			'labels',
			labels.map((item: any, i: number) =>
				i === index ? {...item, ...patch} : item,
			),
		);
	return (
		<div className="flex max-w-2xl flex-col gap-3">
			<div>
				<h4 className="text-sm font-medium">Labels</h4>
				<p className="text-xs text-muted-foreground">
					Add key-value labels to your service.
				</p>
			</div>
			{labels.map((label: any, index: number) => (
				<div key={index} className="flex flex-col gap-3 rounded-md  p-3">
					<Field label="Label key">
						<Input
							value={label.key || ''}
							placeholder="com.example.app.name"
							onChange={e => update(index, {key: e.target.value})}
						/>
					</Field>
					<Field label="Label value">
						<Input
							value={label.value || ''}
							placeholder="my-app"
							onChange={e => update(index, {value: e.target.value})}
						/>
					</Field>
					<Button
						type="button"
						variant="destructive"
						size="sm"
						className="w-fit"
						onClick={() =>
							set(
								'labels',
								labels.filter((_: any, i: number) => i !== index),
							)
						}>
						Remove
					</Button>
				</div>
			))}
			<Button
				type="button"
				variant="outline"
				size="sm"
				className="w-fit"
				onClick={() => set('labels', [...labels, {key: '', value: ''}])}>
				<Plus className="mr-1 size-3" />
				Add label
			</Button>
		</div>
	);
}
