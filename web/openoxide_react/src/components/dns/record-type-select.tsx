import {useEffect, useMemo, useRef, useState} from 'react';
import {
	Check,
	ChevronDown,
	FileText,
	Link2,
	Mail,
	Network,
	Search,
	Server,
} from 'lucide-react';
import {cn} from '#/api/utils';
import {Input} from '#/components/ui/input';
import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from '#/components/ui/popover';

/**
 * The record types the panel can write. `hint` doubles as the search key —
 * the bare codes (A, AAAA, CNAME) are unfindable by anything but memory.
 */
export const recordTypes = [
	{id: 'A', hint: 'IPv4 address', icon: Server},
	{id: 'AAAA', hint: 'IPv6 address', icon: Network},
	{id: 'CNAME', hint: 'Alias to another name', icon: Link2},
	{
		id: 'TXT',
		hint: 'Free-form text — SPF, DKIM, verification',
		icon: FileText,
	},
	{id: 'MX', hint: 'Mail server for this zone', icon: Mail},
] as const;

export type SupportedRecordType = (typeof recordTypes)[number]['id'];

type RecordType = (typeof recordTypes)[number];

export function isSupportedRecordType(
	value: string,
): value is SupportedRecordType {
	return recordTypes.some(type => type.id === value);
}

/** Record type picker — glyph on every option, filter box on top of the list. */
export function RecordTypeSelect({
	value,
	onChange,
}: {
	value: SupportedRecordType;
	onChange: (type: SupportedRecordType) => void;
}) {
	const [open, setOpen] = useState(false);
	const [query, setQuery] = useState('');
	const inputRef = useRef<HTMLInputElement>(null);
	const current =
		recordTypes.find(type => type.id === value) ?? recordTypes[0];
	const CurrentIcon = current.icon;

	const filtered: readonly RecordType[] = useMemo(() => {
		const q = query.trim().toLowerCase();
		if (!q) return recordTypes;
		return recordTypes.filter(
			type =>
				type.id.toLowerCase().includes(q) ||
				type.hint.toLowerCase().includes(q),
		);
	}, [query]);

	useEffect(() => {
		if (open) inputRef.current?.focus({preventScroll: true});
	}, [open]);

	const pick = (type: SupportedRecordType) => {
		onChange(type);
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
				<CurrentIcon
					className="size-4 shrink-0 text-muted-foreground"
					strokeWidth={1.7}
				/>
				<span className="shrink-0 font-medium">
					{current.id}
				</span>
				<span className="min-w-0 truncate text-muted-foreground">
					{current.hint}
				</span>
				<ChevronDown className="ml-auto size-4 shrink-0 text-muted-foreground opacity-70" />
			</PopoverTrigger>
			<PopoverContent className="p-0" align="start">
				<div className="flex flex-col">
					<div className="flex items-center gap-2 border-b border-border p-2">
						<Search className="size-3.5 shrink-0 text-muted-foreground" />
						<Input
							ref={inputRef}
							placeholder="Search record types…"
							value={query}
							onChange={event => setQuery(event.target.value)}
							onKeyDown={event => {
							const [first] = filtered;
								if (event.key === 'Enter' && filtered[0])&& filtered[0]) first) {
									event.preventDefault();
									pick(first.id);
								}
							}}
							className="h-7 flex-1 border-0 bg-transparent px-0 text-[13px] shadow-none focus-visible:ring-0"
						/>
					</div>
					<div className="flex max-h-60 flex-col gap-px overflow-y-auto p-1.5">
						{filtered.map(type => (
							<button
								key={type.id}
								type="button"
								onClick={() => pick(type.id)}
								className={cn(
									'flex w-full cursor-pointer items-center gap-2.5 rounded-md px-2 py-1.5 text-left text-[13px] transition-colors outline-none hover:bg-accent focus-visible:bg-accent',
									type.id === current.id &&
										'bg-accent',
								)}>
								<span className="flex size-7 shrink-0 items-center justify-center rounded-md border border-border/60 bg-card text-muted-foreground">
									<type.icon
										className="size-4"
										strokeWidth={1.7}
									/>
								</span>
								<span className="flex min-w-0 flex-1 flex-col">
									<span className="font-medium text-foreground">
										{type.id}
									</span>
									<span className="truncate text-[11px] text-muted-foreground">
										{type.hint}
									</span>
								</span>
								{type.id === current.id && (
									<Check className="size-3.5 shrink-0 text-foreground" />
								)}
							</button>
						))}
						{!filtered.length && (
							<p className="px-2 py-5 text-center text-[13px] text-muted-foreground">
								No record type matches “{query}”.
							</p>
						)}
					</div>
				</div>
			</PopoverContent>
		</Popover>
	);
}
