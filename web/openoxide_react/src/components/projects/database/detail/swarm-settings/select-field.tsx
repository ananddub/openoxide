import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

type Option = {value: string; label: string};
type Props = {
	label: string;
	value: string;
	options: Option[];
	onChange: (value: string) => void;
};

export function SwarmSelectField({
	label,
	value,
	options,
	onChange,
}: Props) {
	return (
		<label className="flex flex-col gap-1.5 text-sm font-medium">
			{label}
			{selectDescription(label) && <span className="text-xs font-normal text-muted-foreground">{selectDescription(label)}</span>}
			<Select value={value} onValueChange={next => next && onChange(next)}>
				<SelectTrigger className="w-full">
					<SelectValue placeholder={`Select ${label.toLowerCase()}`} />
				</SelectTrigger>
				<SelectContent>
					{options.map(option => (
						<SelectItem key={option.value} value={option.value}>
							{option.label}
						</SelectItem>
					))}
				</SelectContent>
			</Select>
		</label>
	);
}

function selectDescription(label: string) {
	const values: Record<string, string> = {
		'Service Mode': 'Replicated runs a fixed number of tasks; Global runs one task per node.',
		Condition: 'Choose when a stopped task should be restarted.',
		'Failure Action': 'Action to take when an update fails.',
		Order: 'Whether old or new tasks start first during updates.',
		'Endpoint Mode': 'VIP uses a virtual IP; DNSRR resolves each task directly.',
		Protocol: 'Transport protocol used by the published port.',
		'Publish mode': 'Ingress uses the routing mesh; host binds on the node.',
	};
	return values[label] || '';
}
