import {useState} from 'react';
import {ChevronLeft} from 'lucide-react';
import {
	useDnsRecordCreate,
	useDnsRecordDelete,
	useDnsRecordList,
	useDnsRecordUpdate,
	useDnsZoneList,
} from '#/api/live';
import {cn, formatApiError} from '#/api/utils';
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
	NS: 'A name-server hostname such as “ns1.example.com”. Careful — wrong NS data can delegate your zone away.',
};

/** Types whose value is a tuple of several parts — edited as separate fields. */
type StructuredField = {
	key: string;
	label: string;
	placeholder: string;
	numeric?: boolean;
};
const structuredFields: Record<string, StructuredField[]> = {
	MX: [
		{key: 'priority', label: 'Priority', placeholder: '10', numeric: true},
		{
			key: 'exchange',
			label: 'Mail server',
			placeholder: 'mail.example.com',
		},
	],
	SRV: [
		{key: 'priority', label: 'Priority', placeholder: '10', numeric: true},
		{key: 'weight', label: 'Weight', placeholder: '60', numeric: true},
		{key: 'port', label: 'Port', placeholder: '5060', numeric: true},
		{key: 'target', label: 'Target', placeholder: 'sip.example.com'},
	],
	CAA: [
		{key: 'flag', label: 'Flag', placeholder: '0', numeric: true},
		{key: 'tag', label: 'Tag', placeholder: 'issue'},
		{key: 'value', label: 'Value', placeholder: '"letsencrypt.org"'},
	],
};

type Parts = Record<string, string>;

/** Split a stored value back into its named parts; null when it doesn't fit.
 * Partial values are padded so the form stays structured while typing. */
const parseParts = (type: string, content: string): Parts | null => {
	const fields = structuredFields[type];
	if (!fields) return null;
	const raw = content.trim();
	if (!raw)
		return Object.fromEntries(fields.map(f => [f.key, ''])) as Parts;
	let parts: (string | undefined)[] | null;
	if (type === 'CAA') {
		// The value may contain spaces inside quotes — split on the first two only.
		const match = raw.match(/^(\d+)(?:\s+(\S+)(?:\s+([\s\S]+))?)?$/);
		parts = match ? match.slice(1) : null;
	} else {
		const pieces = raw.split(/\s+/);
		parts = pieces.length <= fields.length ? pieces : null;
		if (parts) while (parts.length < fields.length) parts.push('');
	}
	if (!parts) return null;
	return Object.fromEntries(
		fields.map((f, i) => [f.key, parts[i] ?? '']),
	) as Parts;
};

const buildContent = (type: string, parts: Parts): string => {
	const values = structuredFields[type].map(f =>
		(parts[f.key] ?? '').trim(),
	);
	while (values.length && !values[values.length - 1]) values.pop();
	return values.join(' ');
};

/** Cloudflare-only: these record types can route through its proxy. */
const proxyableTypes = new Set(['A', 'AAAA', 'CNAME']);

type RecordDraft = {
	zoneId: string;
	zoneName: string;
	/** The row this draft was opened from, kept aside from the edited fields so the
	 * server can still find it after a rename or a type change. */
	editing?: {id: string; name: string; recordType: string; content: string};
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
	const edit = useDnsRecordUpdate();
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
			editing: {
				id: record.id,
				name: record.name,
				recordType: record.record_type,
				content: record.content,
			},
			recordType: record.record_type,
			name: record.name,
			content: record.content,
			ttl: record.ttl ? String(record.ttl) : '',
			proxied: !!record.proxied,
		});
	const save = async () => {
		if (!draft?.name.trim() || !draft.content.trim())
			return toast.error('Record name and content are required');
		const fields = structuredFields[draft.recordType];
		if (fields) {
			const parts = parseParts(draft.recordType, draft.content);
			if (!parts || fields.some(f => !(parts[f.key] ?? '').trim()))
				return toast.error(`Fill in every ${draft.recordType} field`);
		}
		const payload = {
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
		};
		try {
			if (draft.editing) {
				await edit.mutateAsync({
					...payload,
					target: {
						id: draft.editing.id || undefined,
						name: draft.editing.name,
						record_type: draft.editing.recordType,
						content: draft.editing.content,
					},
				});
			} else {
				await upsert.mutateAsync(payload);
			}
			toast.success(draft.editing ? 'Record updated' : 'Record created');
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
				id: record.id || undefined,
				record_type: record.record_type,
				content: record.content,
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
	const fields = structuredFields[draft.recordType];
	const parts = fields
		? parseParts(draft.recordType, draft.content)
		: null;
	return (
		<div
			key="form"
			className="flex min-h-0 flex-1 animate-in flex-col duration-200 ease-out fade-in-0 slide-in-from-right-4">
			<SheetHeader>
				<SheetTitle className="truncate pr-8">
					{draft.editing ? 'Edit Record' : 'New Record'}
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
						onChange={type =>
							onChange({...draft, recordType: type, content: ''})
						}
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
				{fields && parts ? (
					<div className="grid gap-1.5">
						<span className={fieldLabel}>Value</span>
						<div className="flex flex-wrap gap-2">
							{fields.map(field => (
								<div
									key={field.key}
									className={cn(
										'grid gap-1',
										field.numeric
											? 'w-[104px] shrink-0'
											: 'min-w-[190px] flex-1',
									)}>
									<Input
										type={field.numeric ? 'number' : 'text'}
										placeholder={field.placeholder}
										value={parts[field.key] ?? ''}
										onChange={event =>
											onChange({
												...draft,
												content: buildContent(draft.recordType, {
													...parts,
													[field.key]: event.target.value,
												}),
											})
										}
									/>
									<span className={fieldLabel}>{field.label}</span>
								</div>
							))}
						</div>
					</div>
				) : (
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
							{fields
								? `Stored value isn’t in the usual ${draft.recordType} format — edit it as raw text.`
								: hint}
						</p>
					</div>
				)}
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
						: draft.editing
							? 'Update Record'
							: 'Save Record'}
				</Button>
			</SheetFooter>
		</div>
	);
}
