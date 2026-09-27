import React, {useEffect, useMemo, useRef, useState} from 'react';
import {Check, ChevronDown, Search} from 'lucide-react';
import {cn} from '#/api/utils';
import {Input} from '#/components/ui/input';
import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from '#/components/ui/popover';
import {WizardField} from '#/components/shared/sheet-wizard';

export interface ProviderOption {
	value: string;
	label: string;
	icon?: React.ReactNode;
}

/**
 * Provider-type picker. A plain Select can't hold a filter box, and this list
 * is ~70 brands deep, so it's the searchable-popover idiom instead.
 */
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
	const [open, setOpen] = useState(false);
	const [query, setQuery] = useState('');
	const inputRef = useRef<HTMLInputElement>(null);
	const current = options.find(option => option.value === value);

	const filtered = useMemo(() => {
		const q = query.trim().toLowerCase();
		if (!q) return options;
		return options.filter(
			option =>
				option.label.toLowerCase().includes(q) ||
				option.value.toLowerCase().includes(q),
		);
	}, [options, query]);

	useEffect(() => {
		if (open) inputRef.current?.focus({preventScroll: true});
	}, [open]);

	const pick = (next: string) => {
		onChange(next);
		setOpen(false);
	};

	return (
		<WizardField label={label}>
			<Popover
				open={open && !disabled}
				onOpenChange={next => {
					setOpen(next);
					if (!next) setQuery('');
				}}>
				<PopoverTrigger
					type="button"
					disabled={disabled}
					className="flex h-9 w-full cursor-pointer items-center gap-2 overflow-hidden rounded-md border border-input bg-card px-3 text-xs shadow-[0_1px_1px_rgba(0,0,0,0.02)] transition-colors duration-150 outline-none focus-visible:border-foreground/40 focus-visible:ring-2 focus-visible:ring-ring/30 disabled:cursor-not-allowed disabled:opacity-50">
					{current?.icon}
					<span className="min-w-0 flex-1 truncate text-left">
						{current?.label ?? value}
					</span>
					<ChevronDown className="size-4 shrink-0 text-muted-foreground opacity-70" />
				</PopoverTrigger>
				<PopoverContent className="p-0" align="start">
					<div className="flex items-center gap-2 border-b border-border p-2">
						<Search className="size-3.5 shrink-0 text-muted-foreground" />
						<Input
							ref={inputRef}
							placeholder="Search providers…"
							value={query}
							onChange={event => setQuery(event.target.value)}
							onKeyDown={event => {
								const [first] = filtered;
								if (event.key === 'Enter' && first) {
									event.preventDefault();
									pick(first.value);
								}
							}}
							className="h-7 flex-1 border-0 bg-transparent px-0 text-[13px] shadow-none focus-visible:ring-0"
						/>
					</div>
					<div className="flex max-h-72 flex-col gap-px overflow-y-auto p-1.5">
						{filtered.map(option => (
							<button
								key={option.value}
								type="button"
								onClick={() => pick(option.value)}
								className={cn(
									'flex w-full cursor-pointer items-center gap-2.5 rounded-md px-2 py-1.5 text-left text-[13px] transition-colors outline-none hover:bg-accent focus-visible:bg-accent',
									option.value === value &&
										'bg-accent',
								)}>
								<span className="flex size-6 shrink-0 items-center justify-center">
									{option.icon}
								</span>
								<span className="min-w-0 flex-1 truncate text-foreground">
									{option.label}
								</span>
								{option.value === value && (
									<Check className="size-3.5 shrink-0 text-foreground" />
								)}
							</button>
						))}
						{!filtered.length && (
							<p className="px-2 py-5 text-center text-[13px] text-muted-foreground">
								No provider matches “{query}”.
							</p>
						)}
					</div>
				</PopoverContent>
			</Popover>
		</WizardField>
	);
}
