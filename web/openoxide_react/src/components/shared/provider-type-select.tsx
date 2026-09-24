import React from 'react';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {FieldLabel} from './field-label';

export interface ProviderOption {
	value: string;
	label: string;
	icon?: React.ReactNode;
}

/** Labelled provider-type dropdown whose trigger previews the selected option's icon and label. Used by DNS / Vault provider forms. */
export function ProviderTypeSelect({
	label = 'Provider Type',
	value,
	onChange,
	disabled,
	options,
}: {
	label?: string;
	value: string;
	onChange: (value: string) => void;
	disabled?: boolean;
	options: ProviderOption[];
}) {
	const current = options.find(o => o.value === value);
	return (
		<div className="space-y-1">
			<FieldLabel>{label}</FieldLabel>
			<Select
				value={value}
				onValueChange={(v: any) => onChange(v)}
				disabled={disabled}>
				<SelectTrigger className="h-10 w-full bg-muted/20 text-xs">
					<SelectValue>
						<div className="flex items-center gap-2">
							{current?.icon}
							<span>{current?.label ?? value}</span>
						</div>
					</SelectValue>
				</SelectTrigger>
				<SelectContent className="w-[var(--anchor-width)]">
					{options.map(option => (
						<SelectItem key={option.value} value={option.value}>
							<div className="flex items-center gap-2">
								{option.icon}
								{option.label}
							</div>
						</SelectItem>
					))}
				</SelectContent>
			</Select>
		</div>
	);
}
