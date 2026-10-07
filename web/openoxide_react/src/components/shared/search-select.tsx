import {useEffect, useMemo, useRef, useState} from 'react';
import type {ReactNode} from 'react';
import {Check, ChevronDown, Search} from 'lucide-react';
import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from '#/components/ui/popover';
import {Input} from '#/components/ui/input';
import {cn} from '#/api/utils';

/** One row of the searchable select. */
export type SearchOption = {
	id: string;
	title: string;
	subtitle?: string;
	icon?: ReactNode;
};

/** Searchable dropdown — compact trigger showing the picked option, popover
 *  with a fixed search box and a scrolling list; same grammar as the AI
 *  provider/model selects. Used wherever an edit form needs a choice that a
 *  plain <Select> cannot label well. */
export function SearchSelect({
	value,
	options,
	onChange,
	placeholder = 'Select…',
	searchPlaceholder = 'Search…',
}: {
	value: string | null;
	options: SearchOption[];
	onChange: (id: string) => void;
	placeholder?: string;
	searchPlaceholder?: string;
}) {
	const [open, setOpen] = useState(false);
	const [query, setQuery] = useState('');
	const inputRef = useRef<HTMLInputElement>(null);

	const current = options.find(option => option.id === value);

	const filtered = useMemo(() => {
		const q = query.trim().toLowerCase();
		if (!q) return options;
		return options.filter(
			option =>
				option.title.toLowerCase().includes(q) ||
				option.subtitle?.toLowerCase().includes(q),
		);
	}, [options, query]);

	useEffect(() => {
		if (open) inputRef.current?.focus({preventScroll: true});
	}, [open]);

	const pick = (id: string) => {
		onChange(id);
		setOpen(false);
	};

	return (
		<Popover
			open={open}
			onOpenChange={next => {
				setOpen(next);
				if (!next) setQuery('');
			}}>
			<PopoverTrigger
				type="button"
				className="flex h-9 w-full cursor-pointer items-center gap-2 rounded-md border border-input bg-card px-3 text-xs shadow-[0_1px_1px_rgba(0,0,0,0.02)] transition-colors duration-150 outline-none focus-visible:border-foreground/40 focus-visible:ring-2 focus-visible:ring-ring/30">
				{current ? (
					<>
						{current.icon && (
							<span className="flex shrink-0 text-muted-foreground">
								{current.icon}
							</span>
						)}
						<span className="min-w-0 flex-1 truncate text-left font-medium">
							{current.title}
						</span>
						{current.subtitle && (
							<span className="hidden min-w-0 truncate text-muted-foreground sm:block">
								{current.subtitle}
							</span>
						)}
					</>
				) : (
					<span className="min-w-0 flex-1 truncate text-left text-muted-foreground">
						{placeholder}
					</span>
				)}
				<ChevronDown className="size-4 shrink-0 text-muted-foreground opacity-70" />
			</PopoverTrigger>
			<PopoverContent
				className="max-h-72 overflow-hidden p-0"
				align="start">
				<div className="flex max-h-72 flex-col">
					<div className="flex shrink-0 items-center gap-2 border-b border-border p-2">
						<Search className="size-3.5 shrink-0 text-muted-foreground" />
						<Input
							ref={inputRef}
							placeholder={searchPlaceholder}
							value={query}
							onChange={event => setQuery(event.target.value)}
							onKeyDown={event => {
								const [first] = filtered;
								if (event.key === 'Enter' && first) {
									event.preventDefault();
									pick(first.id);
								}
							}}
							className="h-7 flex-1 border-0 bg-transparent px-0 text-[13px] shadow-none focus-visible:ring-0"
						/>
					</div>
					<div className="flex max-h-56 flex-col gap-px overflow-y-auto p-1.5">
						{filtered.map(option => (
							<button
								key={option.id}
								type="button"
								onClick={() => pick(option.id)}
								className={cn(
									'flex w-full cursor-pointer items-center gap-2.5 rounded-md px-2 py-1.5 text-left text-[13px] transition-colors outline-none hover:bg-accent focus-visible:bg-accent',
									option.id === current?.id && 'bg-accent',
								)}>
								{option.icon && (
									<span className="flex size-4 shrink-0 items-center justify-center text-muted-foreground [&_svg]:size-4">
										{option.icon}
									</span>
								)}
								<span className="flex min-w-0 flex-1 flex-col">
									<span className="truncate font-medium text-foreground">
										{option.title}
									</span>
									{option.subtitle && (
										<span className="truncate text-[11px] text-muted-foreground">
											{option.subtitle}
										</span>
									)}
								</span>
								{option.id === current?.id && (
									<Check className="size-3.5 shrink-0 text-foreground" />
								)}
							</button>
						))}
						{filtered.length === 0 && (
							<p className="px-2 py-5 text-center text-[13px] text-muted-foreground">
								Nothing matches &ldquo;{query}&rdquo;.
							</p>
						)}
					</div>
				</div>
			</PopoverContent>
		</Popover>
	);
}
