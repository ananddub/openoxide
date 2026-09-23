import {useState} from 'react';
import {Globe, Loader2} from 'lucide-react';
import {
	useDnsRecordCreate,
	useDnsRecordDelete,
	useDnsRecordList,
	useDnsZoneList,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {toast} from 'sonner';
import {DnsZoneRow, type DnsRecord} from './dns-zone-row';
import {
	DnsRecordDraftDialog,
	type RecordDraft,
} from './dns-record-draft-dialog';

type Props = {
	provider: {id: number; name: string};
	onClose: () => void;
};

/** Zones + records management sheet for one DNS provider. */
export function DnsZonesDialog({provider, onClose}: Props) {
	const [expanded, setExpanded] = useState<string | null>(null);
	const [draft, setDraft] = useState<RecordDraft | null>(null);
	const zonesQuery = useDnsZoneList(BigInt(provider.id));
	const recordsQuery = useDnsRecordList(
		BigInt(provider.id),
		expanded || '',
	);
	const upsert = useDnsRecordCreate();
	const remove = useDnsRecordDelete();
	const zones = (zonesQuery.data || []) as Array<{
		id: string;
		name: string;
	}>;
	const records = (recordsQuery.data || []) as DnsRecord[];

	const startCreate = (zoneId: string, zoneName: string) =>
		setDraft({
			zoneId,
			recordType: 'A',
			name: zoneName,
			content: '',
			ttl: '',
		});
	const startEdit = (record: DnsRecord, zoneId: string) =>
		setDraft({
			zoneId,
			recordId: record.id,
			recordType: record.record_type === 'CNAME' ? 'CNAME' : 'A',
			name: record.name,
			content: record.content,
			ttl: record.ttl ? String(record.ttl) : '',
		});
	const save = async () => {
		if (!draft?.name.trim() || !draft.content.trim())
			return toast.error('Record name and content are required');
		try {
			await upsert.mutateAsync({
				params: {path: {id: provider.id}},
				body: {
					zone_id: draft.zoneId,
					record_type: draft.recordType,
					name: draft.name.trim(),
					content: draft.content.trim(),
					ttl: draft.ttl ? Number(draft.ttl) : undefined,
				},
			} as any);
			toast.success(draft.recordId ? 'Record updated' : 'Record created');
			setDraft(null);
			await recordsQuery.refetch();
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};
	const deleteRecord = async (recordId: string) => {
		if (!expanded || !confirm('Delete this DNS record?')) return;
		try {
			await remove.mutateAsync({
				params: {
					path: {id: provider.id, zone_id: expanded, record_id: recordId},
				},
			} as any);
			toast.success('Record deleted');
			await recordsQuery.refetch();
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	return (
		<Sheet
			open
			onOpenChange={value => {
				if (value) return;
				setExpanded(null);
				setDraft(null);
				onClose();
			}}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2 pr-8">
						<Globe className="size-4 text-primary" />
						Domains for {provider.name}
					</SheetTitle>
					<SheetDescription>
						Zones available to this provider. Expand a zone to inspect and
						manage A/CNAME records.
					</SheetDescription>
				</SheetHeader>
				<div className="flex flex-1 flex-col gap-2 overflow-y-auto px-6 py-5">
					{zonesQuery.isLoading && (
						<div className="flex items-center justify-center gap-2 py-8 text-sm text-muted-foreground">
							<Loader2 className="size-4 animate-spin" /> Loading zones...
						</div>
					)}
					{zonesQuery.isError && (
						<p className="text-sm text-destructive">
							{formatApiError(zonesQuery.error)}
						</p>
					)}
					{!zonesQuery.isLoading && !zones.length && (
						<p className="py-8 text-center text-sm text-muted-foreground">
							No zones found for this provider.
						</p>
					)}
					{zones.map(zone => (
						<DnsZoneRow
							key={zone.id}
							zone={zone}
							expanded={expanded === zone.id}
							isLoading={recordsQuery.isLoading}
							records={expanded === zone.id ? records : []}
							onToggle={() =>
								setExpanded(expanded === zone.id ? null : zone.id)
							}
							onEdit={record => startEdit(record, zone.id)}
							onDelete={record => deleteRecord(record.id)}
							onCreate={() => startCreate(zone.id, zone.name)}
						/>
					))}
				</div>
				<DnsRecordDraftDialog
					draft={draft}
					saving={upsert.isPending}
					onChange={setDraft}
					onClose={() => setDraft(null)}
					onSave={save}
				/>
			</SheetContent>
		</Sheet>
	);
}
