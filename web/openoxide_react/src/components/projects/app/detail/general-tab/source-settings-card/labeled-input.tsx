import {Input} from '#/components/ui/input';

export function LabeledInput({
	label,
	value,
	onValueChange,
	placeholder,
	type = 'text',
	mono,
	className,
	labelClassName = 'text-foreground',
	inputClassName,
}: {
	label: string;
	value: string;
	onValueChange: (value: string) => void;
	placeholder?: string;
	type?: string;
	mono?: boolean;
	className?: string;
	labelClassName?: string;
	inputClassName?: string;
}) {
	return (
		<div className={`flex flex-col gap-1.5 ${className ?? ''}`}>
			<span className={`text-xs font-bold ${labelClassName}`}>
				{label}
			</span>
			<Input
				type={type}
				placeholder={placeholder}
				value={value}
				onChange={e => onValueChange(e.target.value)}
				className={`border- bg-card text-xs ${mono ? 'font-mono' : ''} ${inputClassName ?? 'h-9'}`}
			/>
		</div>
	);
}
