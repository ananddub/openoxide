import {useState} from 'react';
import {Globe, Trash2, ExternalLink, RefreshCw, Info, Lock, LockOpen, X, Box} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Badge} from '#/components/ui/badge';
import {Card, CardContent, CardDescription, CardHeader, CardTitle} from '#/components/ui/card';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface DomainsTabProps {
	app?: any;
	targetId?: number;
	targetType?: 'application' | 'compose';
}

export function DomainsTab({app, targetId, targetType}: DomainsTabProps) {
	const entityId = app?.id || targetId || 0;
	const isCompose = targetType === 'compose' || app?.compose_type !== undefined || app?.compose_file !== undefined;

	const [showAdd, setShowAdd] = useState(false);
	const [host, setHost] = useState('');
	const [serviceName, setServiceName] = useState('');
	const [port, setPort] = useState('3000');
	const [path, setPath] = useState('/');
	const [https, setHttps] = useState(false);
	const [certType, setCertType] = useState<'letsencrypt' | 'none'>('none');
	const [adding, setAdding] = useState(false);

	const [deleteId, setDeleteId] = useState<number | null>(null);
	const [deleting, setDeleting] = useState(false);

	// Fetch Application Domains
	const {data: appDomains = [], isLoading: isLoadingApp, refetch: refetchApp} = $api.useQuery(
		'get',
		'/domains/application/{application_id}',
		{
			params: {path: {application_id: entityId}},
		},
		{
			enabled: !isCompose && entityId > 0,
		}
	);

	// Fetch Compose Domains
	const {data: composeDomains = [], isLoading: isLoadingCompose, refetch: refetchCompose} = $api.useQuery(
		'get',
		'/domains/compose/{compose_id}',
		{
			params: {path: {compose_id: entityId}},
		},
		{
			enabled: isCompose && entityId > 0,
		}
	);

	const domains = isCompose ? composeDomains : appDomains;
	const isLoadingDomains = isCompose ? isLoadingCompose : isLoadingApp;
	const refetch = isCompose ? refetchCompose : refetchApp;

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
					application_id: isCompose ? undefined : entityId,
					compose_id: isCompose ? entityId : undefined,
					service_name: isCompose ? (serviceName.trim() || undefined) : undefined,
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
			setServiceName('');
			setPort('3000');
			setPath('/');
			setHttps(false);
			setCertType('none');
			refetch();
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
			await deleteDomain.mutateAsync({params: {path: {id: deleteId}}});
			toast.success('Domain deleted successfully');
			setDeleteId(null);
			refetch();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setDeleting(false);
		}
	};

	return (
		<div className="flex flex-col gap-6">
			<Card className="bg-card border-border shadow-sm">
				<CardHeader className="flex flex-row items-center justify-between space-y-0 pb-4">
					<div>
						<CardTitle className="text-base font-bold flex items-center gap-2">
							<Globe className="w-4 h-4 text-primary" /> Domains
						</CardTitle>
						<CardDescription className="text-xs mt-1">
							Configure HTTP/HTTPS proxy domains, ports, and SSL certificates for your service.
						</CardDescription>
					</div>
					<div className="flex items-center gap-2">
						<Button
							variant="outline"
							size="sm"
							onClick={() => refetch()}
							disabled={isLoadingDomains}
							className="h-8 text-xs font-semibold"
						>
							<RefreshCw className={`w-3.5 h-3.5 ${isLoadingDomains ? 'animate-spin' : ''}`} />
						</Button>
						<Button size="sm" onClick={() => setShowAdd(true)} className="h-8 text-xs font-semibold">
							+ Add Domain
						</Button>
					</div>
				</CardHeader>

				<CardContent>
					{isLoadingDomains ? (
						<div className="flex justify-center py-10">
							<RefreshCw className="w-5 h-5 animate-spin text-muted-foreground" />
						</div>
					) : domains.length === 0 ? (
						<div className="flex flex-col items-center justify-center py-12 text-center border border-dashed border-border/80 rounded-xl bg-muted/10">
							<Globe className="w-8 h-8 text-muted-foreground/50 mb-2" />
							<p className="text-sm font-semibold text-foreground">No custom domains configured</p>
							<p className="text-xs text-muted-foreground max-w-sm mt-1 mb-4">
								Add a domain name to expose your service to the internet via Traefik reverse proxy.
							</p>
							<Button size="sm" onClick={() => setShowAdd(true)} className="h-8 text-xs font-semibold">
								Add your first domain
							</Button>
						</div>
					) : (
						<div className="flex flex-col gap-3">
							{domains.map((d: any) => (
								<div
									key={d.id}
									className="flex flex-col sm:flex-row sm:items-center justify-between p-4 border border-border rounded-xl bg-card hover:bg-muted/20 transition-colors gap-3"
								>
									<div className="flex items-start gap-3">
										<div className="p-2 rounded-lg bg-primary/10 border border-primary/20 shrink-0 mt-0.5">
											{d.https ? <Lock className="w-4 h-4 text-emerald-500" /> : <LockOpen className="w-4 h-4 text-amber-500" />}
										</div>
										<div className="flex flex-col gap-1">
											<div className="flex items-center gap-2 flex-wrap">
												<a
													href={`${d.https ? 'https' : 'http'}://${d.host}`}
													target="_blank"
													rel="noreferrer"
													className="text-sm font-bold text-foreground hover:text-primary flex items-center gap-1"
												>
													{d.host}
													<ExternalLink className="w-3 h-3 text-muted-foreground" />
												</a>

												{d.https ? (
													<Badge variant="outline" className="border-emerald-500/30 text-emerald-500 bg-emerald-500/10 text-[10px] h-5 font-bold">
														HTTPS ({d.certificate_type || 'SSL'})
													</Badge>
												) : (
													<Badge variant="outline" className="border-amber-500/30 text-amber-500 bg-amber-500/10 text-[10px] h-5 font-bold">
														HTTP
													</Badge>
												)}

												{d.service_name && (
													<Badge variant="outline" className="border-primary/30 text-primary bg-primary/10 text-[10px] h-5 font-bold flex items-center gap-1">
														<Box className="w-3 h-3" /> {d.service_name}
													</Badge>
												)}
											</div>

											<div className="flex items-center gap-3 text-xs text-muted-foreground">
												<span>Port: <strong className="text-foreground font-mono">{d.port || 3000}</strong></span>
												<span>Path: <strong className="text-foreground font-mono">{d.path || '/'}</strong></span>
											</div>
										</div>
									</div>

									<div className="flex items-center justify-end gap-2 shrink-0">
										<Button
											variant="ghost"
											size="sm"
											onClick={() => setDeleteId(d.id)}
											className="h-8 text-xs text-destructive hover:bg-destructive/10 hover:text-destructive font-semibold"
										>
											<Trash2 className="w-3.5 h-3.5 mr-1" /> Delete
										</Button>
									</div>
								</div>
							))}
						</div>
					)}
				</CardContent>
			</Card>

			{/* Add Domain Modal */}
			{showAdd && (
				<div className="fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4 backdrop-blur-sm">
					<div className="bg-card border border-border rounded-xl w-full max-w-md shadow-2xl p-6 flex flex-col gap-5 animate-in fade-in duration-150">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<div>
								<h3 className="text-base font-bold text-foreground">Add Domain</h3>
								<p className="text-xs text-muted-foreground mt-0.5">Configure domain proxy routing for your service</p>
							</div>
							<Button variant="ghost" size="icon" onClick={() => setShowAdd(false)} className="h-7 w-7 p-0 text-muted-foreground">
								<X className="w-4 h-4" />
							</Button>
						</div>

						<form onSubmit={handleAdd} className="flex flex-col gap-4">
							{/* Service Name (for Compose Stacks) */}
							{isCompose && (
								<div className="flex flex-col gap-1.5">
									<label className="text-xs font-semibold text-foreground flex items-center gap-1.5">
										<Box className="w-3.5 h-3.5 text-primary" /> Service Name inside Compose Stack
									</label>
									<p className="text-[11px] text-muted-foreground leading-tight">
										Target compose service (e.g. <code className="text-primary font-mono">web</code>, <code className="text-primary font-mono">api</code>, <code className="text-primary font-mono">frontend</code>)
									</p>
									<Input placeholder="e.g. web" value={serviceName} onChange={e => setServiceName(e.target.value)} required className="text-xs h-9 font-mono" />
								</div>
							)}

							{/* Host */}
							<div className="flex flex-col gap-1.5">
								<label className="text-xs font-semibold text-foreground">Host</label>
								<Input placeholder="api.mydomain.com" value={host} onChange={e => setHost(e.target.value)} required className="text-xs h-9" />
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
									Port inside the container (e.g., 3000 for Node.js, 80 for Nginx, 8080 for Java)
								</p>
								<Input type="number" placeholder="3000" value={port} onChange={e => setPort(e.target.value)} className="text-xs h-9" />
							</div>

							{/* HTTPS Toggle Card */}
							<div className="flex items-center justify-between p-3 border border-border rounded-lg bg-card/50">
								<div className="flex flex-col gap-0.5">
									<label className="text-xs font-semibold text-foreground">HTTPS</label>
									<span className="text-[11px] text-muted-foreground">Enable SSL / HTTPS secure connection</span>
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
									<span className={`pointer-events-none block w-3.5 h-3.5 rounded-full bg-background shadow-lg transition-transform ${https ? 'translate-x-4.5' : 'translate-x-0.5'}`} />
								</button>
							</div>

							{/* Certificate Provider */}
							{https && (
								<div className="flex flex-col gap-1.5">
									<label className="text-xs font-semibold text-foreground">Certificate Provider</label>
									<select
										value={certType}
										onChange={e => setCertType(e.target.value as any)}
										className="bg-card border border-border rounded-md h-9 px-3 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary"
									>
										<option value="letsencrypt">Let's Encrypt (Automated SSL)</option>
										<option value="none">None</option>
									</select>
								</div>
							)}

							<div className="flex justify-end gap-2 border-t border-border/60 pt-4 mt-2">
								<Button type="button" variant="outline" onClick={() => setShowAdd(false)} className="h-8 text-xs font-semibold">
									Cancel
								</Button>
								<Button type="submit" disabled={adding} className="h-8 text-xs font-semibold">
									{adding ? 'Adding...' : 'Add Domain'}
								</Button>
							</div>
						</form>
					</div>
				</div>
			)}

			{/* Delete Confirmation Modal */}
			{deleteId && (
				<div className="fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4 backdrop-blur-sm">
					<div className="bg-card border border-border rounded-xl w-full max-w-sm shadow-2xl p-5 flex flex-col gap-4">
						<h3 className="text-sm font-bold text-foreground">Delete Domain</h3>
						<p className="text-xs text-muted-foreground leading-relaxed">
							Are you sure you want to remove this domain routing configuration? Traefik proxy rules will be removed.
						</p>
						<div className="flex justify-end gap-2 border-t border-border/60 pt-3">
							<Button variant="outline" size="sm" onClick={() => setDeleteId(null)} className="h-8 text-xs font-semibold">
								Cancel
							</Button>
							<Button variant="destructive" size="sm" onClick={confirmDelete} disabled={deleting} className="h-8 text-xs font-semibold">
								{deleting ? 'Deleting...' : 'Delete Domain'}
							</Button>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
