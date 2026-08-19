import { useState } from 'react';
import { ChevronDown, ChevronRight, Globe, Loader2, Pencil, Plus, Trash2 } from 'lucide-react';
import { $api } from '#/api/query';
import { formatApiError } from '#/api/utils';
import { Badge } from '#/components/ui/badge';
import { Button } from '#/components/ui/button';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '#/components/ui/dialog';
import { Input } from '#/components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '#/components/ui/select';
import { toast } from 'sonner';

type Props = { provider: { id: number; name: string } };
type RecordDraft = { zoneId: string; recordId?: string; recordType: 'A' | 'CNAME'; name: string; content: string; ttl: string };

export function DnsZonesDialog({ provider }: Props) {
  const [open, setOpen] = useState(false);
  const [expanded, setExpanded] = useState<string | null>(null);
  const [draft, setDraft] = useState<RecordDraft | null>(null);
  const zonesQuery = $api.useQuery('get', '/dns-providers/{id}/zones' as any, {
    params: { path: { id: provider.id } },
    enabled: open,
  } as any);
  const recordsQuery = $api.useQuery('get', '/dns-providers/{id}/zones/{zone_id}/records' as any, {
    params: { path: { id: provider.id, zone_id: expanded || '' } },
    enabled: open && !!expanded,
  } as any);
  const upsert = $api.useMutation('post', '/dns-providers/{id}/records' as any);
  const remove = $api.useMutation('delete', '/dns-providers/{id}/zones/{zone_id}/records/{record_id}' as any);
  const zones = (zonesQuery.data?.data || zonesQuery.data || []) as Array<{ id: string; name: string }>;
  const records = (recordsQuery.data?.data || recordsQuery.data || []) as Array<{ id: string; record_type: string; name: string; content: string; ttl?: number }>;

  const startCreate = (zoneId: string, zoneName: string) => setDraft({ zoneId, recordType: 'A', name: zoneName, content: '', ttl: '' });
  const startEdit = (record: typeof records[number], zoneId: string) => setDraft({ zoneId, recordId: record.id, recordType: record.record_type === 'CNAME' ? 'CNAME' : 'A', name: record.name, content: record.content, ttl: record.ttl ? String(record.ttl) : '' });
  const save = async () => {
    if (!draft?.name.trim() || !draft.content.trim()) return toast.error('Record name and content are required');
    try {
      await upsert.mutateAsync({ params: { path: { id: provider.id } }, body: { zone_id: draft.zoneId, record_type: draft.recordType, name: draft.name.trim(), content: draft.content.trim(), ttl: draft.ttl ? Number(draft.ttl) : undefined } } as any);
      toast.success(draft.recordId ? 'Record updated' : 'Record created');
      setDraft(null);
      await recordsQuery.refetch();
    } catch (error) { toast.error(formatApiError(error, 'Failed to save DNS record')); }
  };
  const deleteRecord = async (recordId: string) => {
    if (!expanded || !confirm('Delete this DNS record?')) return;
    try {
      await remove.mutateAsync({ params: { path: { id: provider.id, zone_id: expanded, record_id: recordId } } } as any);
      toast.success('Record deleted');
      await recordsQuery.refetch();
    } catch (error) { toast.error(formatApiError(error, 'Failed to delete DNS record')); }
  };

  return <>
    <Button variant="outline" size="sm" onClick={() => setOpen(true)}><Globe className="size-3.5 mr-1.5" /> View Domains</Button>
    <Dialog open={open} onOpenChange={(value) => { setOpen(value); if (!value) { setExpanded(null); setDraft(null); } }}>
      <DialogContent className="sm:max-w-2xl max-h-[90vh] overflow-y-auto">
        <DialogHeader><DialogTitle>Domains for {provider.name}</DialogTitle><DialogDescription>Zones available to this provider. Expand a zone to inspect and manage A/CNAME records.</DialogDescription></DialogHeader>
        {zonesQuery.isLoading && <div className="flex items-center justify-center gap-2 py-8 text-sm text-muted-foreground"><Loader2 className="size-4 animate-spin" /> Loading zones...</div>}
        {zonesQuery.isError && <p className="text-sm text-destructive">{formatApiError(zonesQuery.error, 'Failed to load zones')}</p>}
        {!zonesQuery.isLoading && !zones.length && <p className="py-8 text-center text-sm text-muted-foreground">No zones found for this provider.</p>}
        <div className="space-y-2">
          {zones.map((zone) => <div key={zone.id} className="rounded-md border">
            <button type="button" className="flex w-full items-center gap-2 px-3 py-2 text-left text-sm" onClick={() => setExpanded(expanded === zone.id ? null : zone.id)}>
              {expanded === zone.id ? <ChevronDown className="size-4" /> : <ChevronRight className="size-4" />}<Globe className="size-4 text-muted-foreground" />{zone.name}
            </button>
            {expanded === zone.id && <div className="space-y-2 border-t px-3 py-3">
              {recordsQuery.isLoading && <div className="flex gap-2 text-sm text-muted-foreground"><Loader2 className="size-4 animate-spin" /> Loading records...</div>}
              {records.map((record) => <div key={record.id} className="flex items-center gap-2 rounded border px-2 py-1.5 text-xs">
                <Badge variant="outline">{record.record_type}</Badge><span className="truncate font-medium">{record.name}</span><span className="flex-1 truncate text-muted-foreground">{record.content}</span>
                <Button variant="ghost" size="icon" className="size-7" onClick={() => startEdit(record, zone.id)} title="Edit record"><Pencil className="size-3.5" /></Button>
                <Button variant="ghost" size="icon" className="size-7 text-destructive" onClick={() => deleteRecord(record.id)} title="Delete record"><Trash2 className="size-3.5" /></Button>
              </div>)}
              {!recordsQuery.isLoading && !records.length && <p className="text-xs text-muted-foreground">No records found.</p>}
              <Button variant="outline" size="sm" onClick={() => startCreate(zone.id, zone.name)}><Plus className="mr-1.5 size-3.5" /> Add Record</Button>
            </div>}
          </div>)}
        </div>
        <Dialog open={!!draft} onOpenChange={(value) => !value && setDraft(null)}>
          <DialogContent className="sm:max-w-md"><DialogHeader><DialogTitle>{draft?.recordId ? 'Edit Record' : 'Add Record'}</DialogTitle><DialogDescription>Only A and CNAME records are supported.</DialogDescription></DialogHeader>
            {draft && <div className="grid gap-3">
              <Select value={draft.recordType} onValueChange={(value: 'A' | 'CNAME') => setDraft({ ...draft, recordType: value })}><SelectTrigger><SelectValue /></SelectTrigger><SelectContent><SelectItem value="A">A</SelectItem><SelectItem value="CNAME">CNAME</SelectItem></SelectContent></Select>
              <Input placeholder="app.example.com" value={draft.name} onChange={(event) => setDraft({ ...draft, name: event.target.value })} />
              <Input placeholder={draft.recordType === 'A' ? '203.0.113.10' : 'target.example.com'} value={draft.content} onChange={(event) => setDraft({ ...draft, content: event.target.value })} />
              <Input type="number" placeholder="TTL (optional)" value={draft.ttl} onChange={(event) => setDraft({ ...draft, ttl: event.target.value })} />
              <Button onClick={save} disabled={upsert.isPending}>{upsert.isPending ? 'Saving...' : draft.recordId ? 'Update Record' : 'Create Record'}</Button>
            </div>}
          </DialogContent>
        </Dialog>
      </DialogContent>
    </Dialog>
  </>;
}
