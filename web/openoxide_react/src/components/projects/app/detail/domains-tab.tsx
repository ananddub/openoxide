import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {Edit2, ExternalLink, Globe, Lock, LockOpen, Plus, RefreshCw, Trash2} from 'lucide-react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {DomainDialog, type DomainFormData} from '#/components/projects/common/domain-dialog';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from '#/components/ui/table';
import {useDomainCreate, useDomainDelete, useDomainUpdate} from '#/api/live';

interface DomainsTabProps { app: any; domains?: any[]; onRefresh?: () => void; }

export function DomainsTab({app, domains: passedDomains, onRefresh}: DomainsTabProps) {
	const [dialogOpen, setDialogOpen] = useState(false);
	const [editingDomain, setEditingDomain] = useState<any | null>(null);
	const [deletingId, setDeletingId] = useState<number | null>(null);
	const queryClient = useQueryClient();
	const domains = Array.isArray(passedDomains) ? passedDomains : [];
	const createDomain = useDomainCreate();
	const updateDomain = useDomainUpdate();
	const deleteDomain = useDomainDelete();

	const openCreate = () => { setEditingDomain(null); setDialogOpen(true); };
	const saveDomain = async (data: DomainFormData) => {
		try {
			if (editingDomain?.id) await updateDomain.mutateAsync({id: editingDomain.id, ...data} as any);
			else await createDomain.mutateAsync({...data, application_id: app.id} as any);
			toast.success(editingDomain ? 'Domain updated successfully' : 'Domain added successfully');
			await queryClient.invalidateQueries();
			onRefresh?.();
		} catch (error) { throw new Error(formatApiError(error)); }
	};

	const removeDomain = async (id: number) => {
		setDeletingId(id);
		try { await deleteDomain.mutateAsync(id); toast.success('Domain deleted successfully'); await queryClient.invalidateQueries(); onRefresh?.(); }
		catch (error) { toast.error(formatApiError(error)); }
		finally { setDeletingId(null); }
	};

	return <div className="flex w-full flex-col gap-6">
		<div className="flex flex-wrap items-center justify-between gap-4 border-b pb-4">
			<div><h3 className="text-lg font-bold">Domains</h3><p className="text-xs text-muted-foreground">Configure routes, paths and TLS certificates for this application</p></div>
			<div className="flex gap-2"><Button variant="outline" size="sm" onClick={onRefresh}><RefreshCw className="mr-1.5 size-3.5" />Refresh</Button><Button size="sm" onClick={openCreate}><Plus className="mr-1.5 size-3.5" />Add Domain</Button></div>
		</div>
		{domains.length === 0 ? <div className="flex flex-col items-center gap-3 rounded-lg border border-dashed py-16 text-center"><Globe className="size-9 text-muted-foreground/40" /><div><p className="text-sm font-semibold">No custom domains</p><p className="text-xs text-muted-foreground">Add a route to expose this application</p></div><Button size="sm" onClick={openCreate}><Plus className="mr-1.5 size-3.5" />Add Domain</Button></div> :
		<div className="overflow-hidden rounded-lg border"><Table><TableHeader><TableRow><TableHead>Domain</TableHead><TableHead>Route</TableHead><TableHead>TLS</TableHead><TableHead>Certificate</TableHead><TableHead className="text-right">Actions</TableHead></TableRow></TableHeader><TableBody>{domains.map((domain: any) => {
			const url = `${domain.https ? 'https' : 'http'}://${domain.host}${domain.path && domain.path !== '/' ? domain.path : ''}`;
			const certificate = String(domain.certificate_type || 'NONE').toUpperCase();
			const certificateLabel = certificate === 'LETSENCRYPT' ? "Let's Encrypt" : certificate === 'CUSTOM' ? 'Custom resolver' : 'Manual/default TLS';
			return <TableRow key={domain.id}><TableCell><a href={url} target="_blank" rel="noreferrer" className="flex items-center gap-2 font-mono text-xs font-semibold hover:text-primary"><Globe className="size-4 text-primary" />{domain.host}<ExternalLink className="size-3" /></a></TableCell><TableCell className="font-mono text-xs text-muted-foreground">{domain.path || '/'} → {domain.internal_path || '/'} · {domain.port || 80}</TableCell><TableCell>{domain.https ? <Badge variant="outline" className="text-emerald-500"><Lock className="mr-1 size-3" />HTTPS</Badge> : <Badge variant="secondary"><LockOpen className="mr-1 size-3" />HTTP</Badge>}</TableCell><TableCell><Badge variant="outline" className="text-[10px]">{certificateLabel}</Badge></TableCell><TableCell className="text-right"><Button variant="ghost" size="icon" onClick={() => { setEditingDomain(domain); setDialogOpen(true); }}><Edit2 className="size-4" /></Button><Button variant="ghost" size="icon" disabled={deletingId === domain.id} onClick={() => removeDomain(domain.id)} className="text-muted-foreground hover:text-destructive">{deletingId === domain.id ? <RefreshCw className="size-4 animate-spin" /> : <Trash2 className="size-4" />}</Button></TableCell></TableRow>;
		})}</TableBody></Table></div>}
		<DomainDialog isOpen={dialogOpen} onClose={() => { setDialogOpen(false); setEditingDomain(null); }} editingDomain={editingDomain} onSave={saveDomain} />
	</div>;
}
