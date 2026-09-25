import {useState} from 'react';
import {
	ChevronRight,
	Cloud,
	Globe,
	Loader2,
	Pencil,
	Plus,
	Trash2,
	X,
} from 'lucide-react';
import {Button} from '#/components/ui/button';

export type DnsRecord = {
	id: string;
	record_type: string;
	name: string;
	content: string;
	ttl?: number;
	proxied?: boolean;
};

/** One zone in the domains sheet; expands to its record rows. */
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
	const [confirmId, setConfirmId] = useState<string | null>(null);
	return (
		<div>
			<button
				type="button"
				onClick={onToggle}
				className="flex w-full items-center gap-3 px-4 py-3 text-left transition-colors hover:bg-accent/30">
				<div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05]">
					<Globe className="size-4 text-muted-foreground" />
				</div>
				<div className="min-w-0 flex-1">
					<span className="block truncate text-sm font-medium text-foreground">
						{zone.name}
					</span>
					<p className="mt-0.5 text-[11px] text-muted-foreground">
						{expanded
							? isLoading
								? 'Loading records…'
								: `${records.length} ${records.length === 1 ? 'record' : 'records'}`
							: 'Click to manage records'}
					</p>
				</div>
				<ChevronRight
					className={`size-4 shrink-0 text-muted-foreground transition-transform ${expanded ? 'rotate-90' : ''}`}
				/>
			</button>
			{expanded && (
				<div className="border-t bg-muted/20 px-4 py-2">
					{isLoading && !records.length && (
						<div className="flex items-center gap-2 py-2 text-xs text-muted-foreground">
							<Loader2 className="size-3.5 animate-spin" />
							Loading records…
						</div>
					)}
					{!isLoading && !records.length && (
						<p className="py-2 text-xs text-muted-foreground">
							No records in this zone yet.
						</p>
					)}
					<div className="divide-y divide-border/60">
						{records.map(record => (
							<div
								key={record.id}
								className="flex items-center gap-3 py-2">
								<span className="shrink-0 rounded bg-muted/60 px-1.5 py-0.5 font-mono text-[10px] font-medium text-muted-foreground">
									{record.record_type}
								</span>
								<span className="min-w-0 flex-1 truncate font-mono text-xs text-foreground">
									{record.name}
								</span>
								<span className="max-w-[45%] shrink-0 truncate font-mono text-xs text-muted-foreground">
									{record.content}
								</span>
								{record.ttl ? (
									<span className="shrink-0 font-mono text-[10px] text-muted-foreground/70">
										{record.ttl}s
									</span>
								) : null}
								{record.proxied && (
									<span
										title="Proxied by Cloudflare"
										className="shrink-0 text-amber-500">
										<Cloud className="size-3.5" />
									</span>
								)}
								<div className="flex shrink-0 items-center gap-0.5">
									{confirmId === record.id ? (
										<>
											<Button
												variant="ghost"
												size="icon"
												className="size-7 rounded-md text-muted-foreground hover:text-foreground"
												onClick={() => setConfirmId(null)}
												title="Keep record">
												<X className="size-3.5" />
											</Button>
											<Button
												variant="ghost"
												size="sm"
												className="h-7 rounded-md px-2 text-xs font-semibold text-destructive hover:bg-destructive/10 hover:text-destructive"
												onClick={() => {
													setConfirmId(null);
													onDelete(record);
												}}>
												Delete
											</Button>
										</>
									) : (
										<>
											<Button
												variant="ghost"
												size="icon"
												className="size-7 rounded-md text-muted-foreground hover:text-foreground"
												onClick={() => onEdit(record)}
												title="Edit record">
												<Pencil className="size-3.5" />
											</Button>
											<Button
												variant="ghost"
												size="icon"
												className="size-7 rounded-md text-muted-foreground hover:text-destructive"
												onClick={() => setConfirmId(record.id)}
												title="Delete record">
												<Trash2 className="size-3.5" />
											</Button>
										</>
									)}
								</div>
							</div>
						))}
					</div>
					<Button
						variant="ghost"
						size="sm"
						onClick={onCreate}
						className="mt-1 -ml-2 h-7 text-xs font-semibold text-primary">
						<Plus className="mr-1 size-3.5" /> New Record
					</Button>
				</div>
			)}
		</div>
	);
}
