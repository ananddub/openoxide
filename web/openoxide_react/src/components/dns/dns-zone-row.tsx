import {
	ChevronDown,
	ChevronRight,
	Globe,
	Loader2,
	Pencil,
	Plus,
	Trash2,
} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';

export type DnsRecord = {
	id: string;
	record_type: string;
	name: string;
	content: string;
	ttl?: number;
};

/** One zone line in the domains dialog; expands to its record rows. */
export function DnsZoneRow({
	zone,
	expanded,
	isLoading,
	records,
	onToggle,
	onEdit,
	onDelete,
	onCreate,
}: {
	zone: {id: string; name: string};
	expanded: boolean;
	isLoading: boolean;
	records: DnsRecord[];
	onToggle: () => void;
	onEdit: (record: DnsRecord) => void;
	onDelete: (record: DnsRecord) => void;
	onCreate: () => void;
}) {
	return (
		<div className="rounded-md ">
			<button
				type="button"
				className="flex w-full items-center gap-2 px-3 py-2 text-left text-sm"
				onClick={onToggle}>
				{expanded ? (
					<ChevronDown className="size-4" />
				) : (
					<ChevronRight className="size-4" />
				)}
				<Globe className="size-4 text-muted-foreground" />
				{zone.name}
			</button>
			{expanded && (
				<div className="space-y-2 border-t px-3 py-3">
					{isLoading && (
						<div className="flex gap-2 text-sm text-muted-foreground">
							<Loader2 className="size-4 animate-spin" /> Loading
							records...
						</div>
					)}
					{records.map(record => (
						<div
							key={record.id}
							className="flex items-center gap-2 rounded  px-2 py-1.5 text-xs">
							<Badge variant="outline">{record.record_type}</Badge>
							<span className="truncate font-medium">{record.name}</span>
							<span className="flex-1 truncate text-muted-foreground">
								{record.content}
							</span>
							<Button
								variant="ghost"
								size="icon"
								className="size-7"
								onClick={() => onEdit(record)}
								title="Edit record">
								<Pencil className="size-3.5" />
							</Button>
							<Button
								variant="ghost"
								size="icon"
								className="size-7 text-destructive"
								onClick={() => onDelete(record)}
								title="Delete record">
								<Trash2 className="size-3.5" />
							</Button>
						</div>
					))}
					{!isLoading && !records.length && (
						<p className="text-xs text-muted-foreground">
							No records found.
						</p>
					)}
					<Button variant="outline" size="sm" onClick={onCreate}>
						<Plus className="mr-1.5 size-3.5" /> Add Record
					</Button>
				</div>
			)}
		</div>
	);
}
