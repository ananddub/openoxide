import {useState} from 'react';
import {ChevronLeft} from 'lucide-react';
import {
	useDnsRecordCreate,
	useDnsRecordDelete,
	useDnsRecordList,
	useDnsZoneList,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Switch} from '#/components/ui/switch';
import {toast} from 'sonner';
import {DnsZoneRow, type DnsRecord} from './dns-zone-row';
import {RecordTypeSelect} from './record-type-select';

type Props = {
	provider: {id: number; name: string; provider_type?: string};
	onClose: () => void;
};

const contentPlaceholders: Record<string, string> = {
	A: '203.0.113.10',
	AAAA: '2001:db8::1',
	CNAME: 'target.example.com',
	TXT: 'v=spf1 -all',
	MX: '10 mail.example.com',
	NS: 'ns1.example.com',
	SRV: '10 60 5060 sip.example.com',
	CAA: '0 issue "letsencrypt.org"',
};

const contentHints: Record<string, string> = {
	TXT: 'Text is stored exactly as you type it — quote parts you want preserved.',
	MX: 'Put the mail priority first, e.g. “10 mail.example.com”.',
	SRV: 'Four space-separated parts: priority weight port target, e.g. “10 60 5060 sip.example.com”.',
	CAA: 'Flags, tag and quoted value, e.g. ‘0 issue "letsencrypt.org"’ or ‘0 issuewild ";"’.',
	NS: 'A name-server hostname such as “ns1.example.com”. Careful — wrong NS data can delegate your zone away.',
};

/** Cloudflare-only: these record types can route through its proxy. */
const proxyableTypes = new Set(['A', 'AAAA', 'CNAME']);

type RecordDraft = {
	zoneId: string;
	zoneName: string;
	recordId?: string;
	recordType: string;
	name: string;
	content: string;
	ttl: string;
	proxied: boolean;
};

const fieldLabel = 'text-xs font-medium text-muted-foreground';

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
	const supportsProxy =
		provider.provider_type?.toUpperCase() === 'CLOUDFLARE';

	const startCreate = (zoneId: string, zoneName: string) =>
		setDraft({
			zoneId,
			zoneName,
			recordType: 'A',
			name: zoneName,
			content: '',
			ttl: '',
			proxied: false,
		});
	const startEdit = (
		record: DnsRecord,
		zoneId: string,
		zoneName: string,
	) =>
		setDraft({
			zoneId,
			zoneName,
			recordId: record.id,
			recordType: record.record_type,
			name: record.name,
			content: record.content,
			ttl: record.ttl ? String(record.ttl) : '',
			proxied: !!record.proxied,
		});
	const save = async () => {
		if (!draft?.name.trim() || !draft.content.trim())
			return toast.error('Record name and content are required');
		try {
			await upsert.mutateAsync({
				provider_id: provider.id,
				zone_id: draft.zoneId,
				record_type: draft.recordType,
				name: draft.name.trim(),
				content: draft.content.trim(),
				ttl: draft.ttl ? Number(draft.ttl) : undefined,
				proxied:
					supportsProxy && proxyableTypes.has(draft.recordType)
						? draft.proxied
						: undefined,
			});
			toast.success(draft.recordId ? 'Record updated' : 'Record created');
			setDraft(null);
			await recordsQuery.refetch();
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};
	const deleteRecord = async (record: DnsRecord) => {
		if (!expanded) return;
		try {
			await remove.mutateAsync({
				provider_id: provider.id,
				zone_id: expanded,
				name: record.name,
			});
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
			<SheetContent className="sm:data-[side=right]:w-[560px]">
				{draft ? (
					<RecordFormView
						draft={draft}
						saving={upsert.isPending}
						supportsProxy={supportsProxy}
						onChange={setDraft}
						onBack={() => setDraft(null)}
						onSave={save}
					/>
				) : (
					<ZonesListView
						providerName={provider.name}
						isLoading={zonesQuery.isLoading}
						isError={zonesQuery.isError}
						error={zonesQuery.error}
						zones={zones}
						records={records}
						expanded={expanded}
						recordsLoading={recordsQuery.isLoading}
						onToggle={zoneId =>
							setExpanded(expanded === zoneId ? null : zoneId)
						}
						onEdit={startEdit}
						onDelete={deleteRecord}
						onCreate={startCreate}
					/>
				)}
			</SheetContent>
		</Sheet>
	);
}

function ZonesListView({
	providerName,
	isLoading,
	isError,
	error,
	zones,
	records,
	expanded,
	recordsLoading,
	onToggle,
	onEdit,
	onDelete,
	onCreate,
}: {
	providerName: string;
	isLoading: boolean;
	isError: boolean;
	error: unknown;
	zones: Array<{id: string; name: string}>;
	records: DnsRecord[];
	expanded: string | null;
	recordsLoading: boolean;
	onToggle: (zoneId: string) => void;
	onEdit: (record: DnsRecord, zoneId: string, zoneName: string) => void;
	onDelete: (record: DnsRecord) => void;
	onCreate: (zoneId: string, zoneName: string) => void;
}) {
	return (
		<div
			key="list"
			className="flex min-h-0 flex-1 animate-in flex-col duration-200 ease-out fade-in-0 slide-in-from-right-4">
			<SheetHeader>
				<SheetTitle className="truncate pr-8">
					Domains — {providerName}
				</SheetTitle>
				<SheetDescription>
					Expand a zone to view and manage its DNS records.
				</SheetDescription>
			</SheetHeader>
			<div className="flex min-h-0 flex-1 flex-col overflow-y-auto px-5 py-4">
				{isLoading && (
					<div className="overflow-hidden rounded-xl border bg-card">
						{[1, 2, 3].map(i => (
							<div
								key={i}
								className="flex items-center gap-3 border-b border-border/60 px-4 py-3 last:border-b-0">
								<div className="size-8 shrink-0 animate-pulse rounded-lg bg-muted/60" />
								<div className="flex-1 space-y-2">
									<div className="h-3.5 w-40 animate-pulse rounded bg-muted/60" />
									<div className="h-3 w-24 animate-pulse rounded bg-muted/60" />
								</div>
							</div>
						))}
					</div>
				)}
				{isError && (
					<p className="text-sm text-destructive">
						{formatApiError(error)}
					</p>
				)}
				{!isLoading && !zones.length && (
					<p className="py-10 text-center text-sm text-muted-foreground">
						No zones found for this provider.
					</p>
				)}
				{!!zones.length && (
					<div className="divide-y overflow-hidden rounded-xl border bg-card">
						{zones.map(zone => (
							<DnsZoneRow
								key={zone.id}
								zone={zone}
								expanded={expanded === zone.id}
								isLoading={recordsLoading}
								records={expanded === zone.id ? records : []}
								onToggle={() => onToggle(zone.id)}
								onEdit={record => onEdit(record, zone.id, zone.name)}
								onDelete={onDelete}
								onCreate={() => onCreate(zone.id, zone.name)}
							/>
						))}
					</div>
				)}
			</div>
		</div>
	);
}

function RecordFormView({
	draft,
	saving,
	supportsProxy,
	onChange,
	onBack,
	onSave,
}: {
	draft: RecordDraft;
	saving: boolean;
	supportsProxy: boolean;
	onChange: (draft: RecordDraft) => void;
	onBack: () => void;
	onSave: () => void;
}) {
	const showProxy = supportsProxy && proxyableTypes.has(draft.recordType);
	const hint =
		contentHints[draft.recordType] ??
		'Values are sent to the provider exactly as typed.';
	return (
		<div
			key="form"
			className="flex min-h-0 flex-1 animate-in flex-col duration-200 ease-out fade-in-0 slide-in-from-right-4">
			<SheetHeader>
				<SheetTitle className="truncate pr-8">
					{draft.recordId ? 'Edit Record' : 'New Record'}
				</SheetTitle>
				<SheetDescription>
					DNS record for {draft.zoneName}.
				</SheetDescription>
			</SheetHeader>
			<div className="flex min-h-0 flex-1 flex-col gap-3.5 overflow-y-auto px-5 py-4">
				<div className="grid gap-1.5">
					<span className={fieldLabel}>Record type</span>
					<RecordTypeSelect
						value={draft.recordType}
						onChange={type => onChange({...draft, recordType: type})}
					/>
				</div>
				<div className="grid gap-1.5">
					<span className={fieldLabel}>Name</span>
					<Input
						placeholder="app.example.com"
						value={draft.name}
						onChange={event =>
							onChange({...draft, name: event.target.value})
						}
					/>
				</div>
				<div className="grid gap-1.5">
					<span className={fieldLabel}>Value</span>
					<Input
						placeholder={contentPlaceholders[draft.recordType]}
						value={draft.content}
						onChange={event =>
							onChange({...draft, content: event.target.value})
						}
					/>
					<p className="text-[11px] leading-relaxed text-muted-foreground">
						{hint}
					</p>
				</div>
				<div className="grid gap-1.5">
					<span className={fieldLabel}>TTL (optional)</span>
					<Input
						type="number"
						placeholder="3600"
						value={draft.ttl}
						onChange={event =>
							onChange({...draft, ttl: event.target.value})
						}
					/>
				</div>
				{showProxy && (
					<div className="flex items-center justify-between rounded-lg border bg-muted/20 px-3 py-2.5">
						<div className="flex flex-col gap-0.5 pr-3">
							<span className="text-xs font-medium text-foreground">
								Proxy through Cloudflare
							</span>
							<span className="text-[11px] leading-relaxed text-muted-foreground">
								Hides the origin IP and enables Cloudflare CDN, caching and
								DDoS protection for this record.
							</span>
						</div>
						<Switch
							checked={draft.proxied}
							onCheckedChange={proxied => onChange({...draft, proxied})}
						/>
					</div>
				)}
			</div>
			<SheetFooter>
				<Button
					variant="ghost"
					onClick={onBack}
					className="mr-auto gap-1 text-muted-foreground">
					<ChevronLeft className="size-4" />
					Back to zones
				</Button>
				<Button onClick={onSave} disabled={saving}>
					{saving
						? 'Saving…'
						: draft.recordId
							? 'Update Record'
							: 'Save Record'}
				</Button>
			</SheetFooter>
		</div>
	);
}
