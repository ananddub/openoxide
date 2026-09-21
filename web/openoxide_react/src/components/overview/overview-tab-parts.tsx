import type {LucideIcon} from 'lucide-react';
import {Search} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectTrigger,
	SelectValue,
	SelectContent,
	SelectItem,
} from '#/components/ui/select';
import {
	Table,
	TableHeader,
	TableBody,
	TableHead,
	TableRow,
} from '#/components/ui/table';

export type FilterOption = {value: string; label: string};

/** Search box with trailing icon used by overview tab toolbars. */
export function TabSearchInput({
	placeholder,
	value,
	onChange,
}: {
	placeholder: string;
	value: string;
	onChange: (v: string) => void;
}) {
	return (
		<div className="relative">
			<Input
				placeholder={placeholder}
				value={value}
				onChange={e => onChange(e.target.value)}
				className="h-9 w-[190px] pr-9 text-xs"
			/>
			<Search className="absolute top-1/2 right-3 size-3.5 -translate-y-1/2 text-muted-foreground" />
		</div>
	);
}

/** Filter select whose trigger reads "<title>: <selected label>". */
export function TabFilterSelect({
	title,
	value,
	onChange,
	options,
	width = 'w-[145px]',
}: {
	title: string;
	value: string;
	onChange: (v: string) => void;
	options: FilterOption[];
	width?: string;
}) {
	const current = options.find(o => o.value === value);
	return (
		<Select value={value} onValueChange={val => val && onChange(val)}>
			<SelectTrigger
				size="sm"
				className={`h-9 ${width} text-xs font-semibold`}>
				<SelectValue>{`${title}: ${current?.label ?? ''}`}</SelectValue>
			</SelectTrigger>
			<SelectContent>
				{options.map(o => (
					<SelectItem key={o.value} value={o.value}>
						{o.label}
					</SelectItem>
				))}
			</SelectContent>
		</Select>
	);
}

/** Centered empty-state block for a tab with no rows. */
export function TabEmptyState({
	icon: Icon,
	title,
	description,
}: {
	icon: LucideIcon;
	title: string;
	description: string;
}) {
	return (
		<div className="flex flex-col items-center justify-center rounded-xl p-12 text-center">
			<Icon className="mb-2 size-8 text-muted-foreground/40" />
			<p className="text-sm font-semibold text-foreground">{title}</p>
			<p className="text-xs text-muted-foreground">{description}</p>
		</div>
	);
}

/** Scrollable card table with a sticky header built from column labels. */
export function TabTable({
	columns,
	children,
}: {
	columns: string[];
	children: React.ReactNode;
}) {
	return (
		<div className="max-h-[calc(100vh-340px)] min-h-[280px] overflow-y-auto rounded-xl border bg-card">
			<Table>
				<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
					<TableRow className="border-b  hover:bg-transparent">
						{columns.map((c, i) => (
							<TableHead
								key={c}
								className={
									'sticky top-0 z-20 bg-card/95 px-4 py-3.5 backdrop-blur-md' +
									(i === columns.length - 1 ? ' text-right' : '')
								}>
								{c}
							</TableHead>
						))}
					</TableRow>
				</TableHeader>
				<TableBody>{children}</TableBody>
			</Table>
		</div>
	);
}
