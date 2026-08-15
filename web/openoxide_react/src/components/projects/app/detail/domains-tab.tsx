import React, { useState } from 'react';
import { Globe, Trash2, ExternalLink, RefreshCw, Lock, LockOpen, X, Plus } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Input } from '#/components/ui/input';
import { Badge } from '#/components/ui/badge';
import {
	Table,
	TableHeader,
	TableBody,
	TableHead,
	TableRow,
	TableCell,
} from '#/components/ui/table';
import {
	Select,
	SelectTrigger,
	SelectValue,
	SelectContent,
	SelectItem,
} from '#/components/ui/select';
import { toast } from 'sonner';
import { $api } from '#/api/query';
import { formatApiError } from '#/api/utils';

interface DomainsTabProps {
	app: any;
	domains?: any[];
	onRefresh?: () => void;
}

export function DomainsTab({ app, domains: passedDomains, onRefresh }: DomainsTabProps) {
	const [showAdd, setShowAdd] = useState(false);
	const [host, setHost] = useState('');
	const [port, setPort] = useState('3000');
	const [path, setPath] = useState('/');
	const [https, setHttps] = useState(false);
	const [certType, setCertType] = useState<'letsencrypt' | 'none'>('none');
	const [adding, setAdding] = useState(false);

	const [deleteId, setDeleteId] = useState<number | null>(null);
	const [deleting, setDeleting] = useState(false);

	const domains = Array.isArray(passedDomains) ? passedDomains : [];

	// Mutations
	const createDomain = $api.useMutation('post', '/domains');
	const deleteDomain = $api.useMutation('delete', '/domains/{id}');

	const handleAdd = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!host.trim()) return;

		setAdding(true);
		try {
			await createDomain.mutateAsync({
				body: {
					application_id: app.id,
					host: host.trim(),
					port: port ? parseInt(port) : undefined,
					path: path || '/',
					internal_path: '/',
					https,
					domain_type: 'HTTP',
					certificate_type: https ? certType : 'none',
					strip_path: false,
					middlewares: '',
				} as any,
			});
			toast.success('Domain added successfully');
			setShowAdd(false);
			setHost('');
			setPort('3000');
			setPath('/');
			setHttps(false);
			setCertType('none');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setAdding(false);
		}
	};

	const confirmDelete = async () => {
		if (!deleteId) return;
		setDeleting(true);
		try {
			await deleteDomain.mutateAsync({ params: { path: { id: deleteId } } });
			toast.success('Domain deleted successfully');
			setDeleteId(null);
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setDeleting(false);
		}
	};

	return (
		<div className="flex flex-col gap-6 w-full animate-in fade-in duration-200">
			{/* Header Toolbar */}
			<div className="flex items-center justify-between flex-wrap gap-4 border-b border-border/40 pb-4">
				<div className="flex flex-col gap-1">
					<h3 className="text-lg font-bold text-foreground tracking-tight">Domains</h3>
					<p className="text-xs text-muted-foreground">
						Configure custom domains and SSL certificates for your application
					</p>
				</div>

				<div className="flex items-center gap-2">
					<Button variant="outline" size="sm" onClick={() => onRefresh?.()} className="h-8 text-xs font-semibold">
						<RefreshCw className="size-3.5 mr-1.5" /> Refresh
					</Button>
					<Button size="sm" onClick={() => setShowAdd(true)} className="h-8 text-xs font-bold flex items-center gap-1.5">
						<Plus className="size-3.5" /> Add Domain
					</Button>
				</div>
			</div>

			{/* Domains Table List */}
			{domains.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 py-16 border border-dashed border-border/60 rounded-2xl bg-card/10 text-center">
					<Globe className="size-10 text-muted-foreground/35" />
					<div className="flex flex-col gap-1">
						<p className="text-sm font-bold text-foreground">No custom domains added</p>
						<p className="text-xs text-muted-foreground">
							Add a domain to route web traffic directly to your application
						</p>
					</div>
					<Button size="sm" onClick={() => setShowAdd(true)} className="mt-2 text-xs font-bold">
						<Plus className="size-3.5 mr-1.5" /> Add First Domain
					</Button>
				</div>
			) : (
				<div className="rounded-xl border border-border/60 bg-card overflow-hidden shadow-xs">
					<Table>
						<TableHeader>
							<TableRow className="border-b border-border/60 bg-muted/30 hover:bg-muted/30">
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">Domain / Host</TableHead>
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">Path & Port</TableHead>
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">SSL Status</TableHead>
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">Certificate</TableHead>
								<TableHead className="py-3.5 px-4 text-right font-bold text-foreground text-xs uppercase tracking-wider">Actions</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{domains.map((d: any) => {
								const url = `${d.https ? 'https' : 'http'}://${d.host}${d.path && d.path !== '/' ? d.path : ''}`;
								return (
									<TableRow key={d.id} className="border-b border-border/40 hover:bg-muted/40 transition-colors">
										<TableCell className="py-3.5 px-4 font-bold text-xs text-foreground font-mono">
											<div className="flex items-center gap-2.5">
												<Globe className="size-4 text-primary shrink-0" />
												<a
													href={url}
													target="_blank"
													rel="noreferrer"
													className="hover:underline hover:text-primary transition-colors flex items-center gap-1.5"
												>
													<span>{d.host}</span>
													<ExternalLink className="size-3.5 text-muted-foreground shrink-0" />
												</a>
											</div>
										</TableCell>
										<TableCell className="py-3.5 px-4 text-xs text-muted-foreground font-mono">
											Path: {d.path || '/'} · Port: {d.port || 80}
										</TableCell>
										<TableCell className="py-3.5 px-4">
											{d.https ? (
												<Badge variant="outline" className="text-[10px] font-semibold text-emerald-500 border-emerald-500/30 bg-emerald-500/10 flex items-center gap-1 w-fit">
													<Lock className="size-3" /> HTTPS (SSL)
												</Badge>
											) : (
												<Badge variant="secondary" className="text-[10px] font-semibold flex items-center gap-1 w-fit">
													<LockOpen className="size-3" /> HTTP
												</Badge>
											)}
										</TableCell>
										<TableCell className="py-3.5 px-4 text-xs text-muted-foreground font-mono">
											{d.certificate_type && d.certificate_type !== 'none' ? (
												<Badge variant="outline" className="text-[10px] font-mono text-primary border-primary/30">
													{d.certificate_type}
												</Badge>
											) : (
												'—'
											)}
										</TableCell>
										<TableCell className="py-3.5 px-4 text-right">
											<Button
												variant="ghost"
												size="icon"
												onClick={() => setDeleteId(d.id)}
												className="size-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-lg"
											>
												<Trash2 className="size-4" />
											</Button>
										</TableCell>
									</TableRow>
								);
							})}
						</TableBody>
					</Table>
				</div>
			)}

			{/* Add Domain Modal */}
			{showAdd && (
				<div className="fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4 backdrop-blur-xs">
					<div className="bg-card border border-border rounded-xl w-full max-w-md shadow-2xl p-6 flex flex-col gap-5 animate-in fade-in duration-150">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<div>
								<h3 className="text-base font-bold text-foreground">Add Domain</h3>
								<p className="text-xs text-muted-foreground mt-0.5">Configure domain proxy routing for your application</p>
							</div>
							<Button variant="ghost" size="icon" onClick={() => setShowAdd(false)} className="size-7 p-0 text-muted-foreground">
								<X className="size-4" />
							</Button>
						</div>

						<form onSubmit={handleAdd} className="flex flex-col gap-4">
							{/* Host */}
							<div className="flex flex-col gap-1.5">
								<label className="text-xs font-semibold text-foreground">Host</label>
								<Input placeholder="app.example.com" value={host} onChange={e => setHost(e.target.value)} required className="text-xs h-9" />
							</div>

							{/* Path */}
							<div className="flex flex-col gap-1.5">
								<label className="text-xs font-semibold text-foreground">Path</label>
								<Input placeholder="/" value={path} onChange={e => setPath(e.target.value)} className="text-xs h-9" />
							</div>

							{/* Container Port */}
							<div className="flex flex-col gap-1.5">
								<label className="text-xs font-semibold text-foreground">Container Port</label>
								<p className="text-[11px] text-muted-foreground leading-tight">
									The port where your application is running inside the container (e.g. 3000)
								</p>
								<Input type="number" placeholder="3000" value={port} onChange={e => setPort(e.target.value)} className="text-xs h-9" />
							</div>

							{/* HTTPS Toggle Card */}
							<div className="flex items-center justify-between p-3 border border-border/60 rounded-lg bg-muted/20">
								<div className="flex flex-col gap-0.5">
									<label className="text-xs font-semibold text-foreground">HTTPS (SSL)</label>
									<span className="text-[11px] text-muted-foreground">Enable SSL / HTTPS secure connection for this domain</span>
								</div>
								<button
									type="button"
									onClick={() => {
										const next = !https;
										setHttps(next);
										if (next && certType === 'none') setCertType('letsencrypt');
									}}
									className={`relative inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full transition-colors ${https ? 'bg-primary' : 'bg-muted'}`}
								>
									<span className={`pointer-events-none block size-3.5 rounded-full bg-background shadow-md transition-transform ${https ? 'translate-x-4.5' : 'translate-x-0.5'}`} />
								</button>
							</div>

							{/* Certificate Provider */}
							{https && (
								<div className="flex flex-col gap-1.5">
									<label className="text-xs font-semibold text-foreground">Certificate Provider</label>
									<Select value={certType} onValueChange={(val: any) => val && setCertType(val)}>
										<SelectTrigger size="sm" className="w-full text-xs font-semibold h-9">
											<SelectValue placeholder="Select provider" />
										</SelectTrigger>
										<SelectContent>
											<SelectItem value="letsencrypt">Let's Encrypt (Automated SSL)</SelectItem>
											<SelectItem value="none">None</SelectItem>
										</SelectContent>
									</Select>
								</div>
							)}

							<div className="flex justify-end border-t border-border/60 pt-4 mt-2">
								<Button type="submit" disabled={adding} className="w-full sm:w-auto h-9 px-6 font-bold text-xs bg-primary hover:bg-primary/90 text-primary-foreground shadow-md">
									{adding ? 'Adding...' : 'Add Domain'}
								</Button>
							</div>
						</form>
					</div>
				</div>
			)}

			{/* Delete Domain Confirmation Modal */}
			{deleteId !== null && (
				<div className="fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4 backdrop-blur-xs">
					<div className="bg-card border border-border rounded-xl w-full max-w-sm shadow-2xl p-5 flex flex-col gap-4 animate-in fade-in duration-150">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<h3 className="text-sm font-bold text-foreground">Delete Domain</h3>
							<Button variant="ghost" size="icon" onClick={() => setDeleteId(null)} className="size-7 p-0 text-muted-foreground">
								<X className="size-4" />
							</Button>
						</div>
						<p className="text-xs text-muted-foreground leading-relaxed">
							Are you sure you want to delete this domain?
						</p>
						<div className="flex justify-end border-t border-border/60 pt-3">
							<Button
								size="sm"
								variant="destructive"
								disabled={deleting}
								onClick={confirmDelete}
								className="w-full sm:w-auto h-9 px-6 font-bold text-xs shadow-md"
							>
								{deleting ? <RefreshCw className="size-3.5 animate-spin mr-1" /> : null}
								{deleting ? 'Deleting...' : 'Delete Domain'}
							</Button>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
