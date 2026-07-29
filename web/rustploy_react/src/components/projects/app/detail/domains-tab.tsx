import {useState} from 'react';
import {Globe, Trash2, ExternalLink, RefreshCw, Info, Lock, LockOpen, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Badge} from '#/components/ui/badge';
import {Card, CardContent, CardDescription, CardHeader, CardTitle} from '#/components/ui/card';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface DomainsTabProps {
	app: any;
}

export function DomainsTab({app}: DomainsTabProps) {
	const [showAdd, setShowAdd] = useState(false);
	const [host, setHost] = useState('');
	const [port, setPort] = useState('3000');
	const [path, setPath] = useState('/');
	const [https, setHttps] = useState(false);
	const [certType, setCertType] = useState<'letsencrypt' | 'none'>('none');
	const [adding, setAdding] = useState(false);

	const [deleteId, setDeleteId] = useState<number | null>(null);
	const [deleting, setDeleting] = useState(false);

	// Fetch Domains
	const {data: domains = [], isLoading: isLoadingDomains, refetch} = $api.useQuery(
		'get',
		'/domains/application/{application_id}',
		{
			params: {path: {application_id: app.id}},
		}
	);

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
		<div className="flex flex-col gap-5 w-full">
			<Card className="bg-background border border-border">
				<CardHeader className="flex flex-row items-center justify-between flex-wrap gap-4 pb-4">
					<div className="flex flex-col gap-1">
						<CardTitle className="text-xl font-bold">Domains</CardTitle>
						<CardDescription className="text-xs text-muted-foreground">
							Domains are used to access the application
						</CardDescription>
					</div>

					<div className="flex items-center gap-2">
						<Button variant="outline" size="sm" onClick={() => refetch()} className="h-8 text-xs font-semibold">
							<RefreshCw className="w-3.5 h-3.5 mr-1.5" /> Refresh
						</Button>
						<Button size="sm" onClick={() => setShowAdd(true)} className="h-8 text-xs font-semibold">
							<Globe className="w-3.5 h-3.5 mr-1.5" /> Add Domain
						</Button>
					</div>
				</CardHeader>

				<CardContent>
					{isLoadingDomains ? (
						<div className="flex w-full flex-row gap-2 min-h-[30vh] justify-center items-center">
							<RefreshCw className="w-5 h-5 animate-spin text-muted-foreground" />
							<span className="text-sm text-muted-foreground font-medium">Loading domains...</span>
						</div>
					) : domains.length === 0 ? (
						<div className="flex w-full flex-col items-center justify-center gap-3 min-h-[35vh]">
							<Globe className="w-8 h-8 text-muted-foreground/40" />
							<span className="text-sm font-medium text-muted-foreground">
								To access the application it is required to set at least 1 domain
							</span>
						</div>
					) : (
						<div className="grid grid-cols-1 gap-4 xl:grid-cols-2 w-full">
							{domains.map((d: any) => {
								const url = `${d.https ? 'https' : 'http'}://${d.host}${d.path && d.path !== '/' ? d.path : ''}`;
								return (
									<Card key={d.id} className="relative overflow-hidden w-full border border-border transition-all hover:shadow-sm bg-card">
										<CardContent className="p-5 flex flex-col gap-3">
											{/* Top Row: URL & Action */}
											<div className="flex items-center justify-between gap-2">
												<a
													href={url}
													target="_blank"
													rel="noreferrer"
													className="text-base font-semibold text-foreground hover:underline flex items-center gap-1.5 break-all"
												>
													{d.host}
													<ExternalLink className="w-3.5 h-3.5 text-muted-foreground" />
												</a>
												<Button
													variant="ghost"
													size="icon"
													onClick={() => setDeleteId(d.id)}
													className="h-8 w-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10 shrink-0"
												>
													<Trash2 className="w-4 h-4" />
												</Button>
											</div>

											{/* Badges Row */}
											<div className="flex flex-wrap items-center gap-2 pt-1">
												<Badge variant="secondary" className="text-xs font-normal">
													<Info className="w-3 h-3 mr-1" /> Path: {d.path || '/'}
												</Badge>
												<Badge variant="secondary" className="text-xs font-normal">
													<Info className="w-3 h-3 mr-1" /> Port: {d.port || 80}
												</Badge>
												<Badge variant={d.https ? 'default' : 'secondary'} className="text-xs font-normal">
													{d.https ? (
														<span className="flex items-center gap-1"><Lock className="w-3 h-3" /> HTTPS</span>
													) : (
														<span className="flex items-center gap-1"><LockOpen className="w-3 h-3" /> HTTP</span>
													)}
												</Badge>
												{d.certificate_type && d.certificate_type !== 'none' && (
													<Badge variant="outline" className="text-xs font-normal">
														Cert: {d.certificate_type}
													</Badge>
												)}
											</div>
										</CardContent>
									</Card>
								);
							})}
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
								<p className="text-xs text-muted-foreground mt-0.5">Configure domain proxy routing for your application</p>
							</div>
							<Button variant="ghost" size="icon" onClick={() => setShowAdd(false)} className="h-7 w-7 p-0 text-muted-foreground">
								<X className="w-4 h-4" />
							</Button>
						</div>

						<form onSubmit={handleAdd} className="flex flex-col gap-4">
							{/* Host */}
							<div className="flex flex-col gap-1.5">
								<label className="text-xs font-semibold text-foreground">Host</label>
								<Input placeholder="api.dokploy.com" value={host} onChange={e => setHost(e.target.value)} required className="text-xs h-9" />
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
									The port where your application is running inside the container (e.g., 3000 for Node.js, 80 for Nginx, 8080 for Java)
								</p>
								<Input type="number" placeholder="3000" value={port} onChange={e => setPort(e.target.value)} className="text-xs h-9" />
							</div>

							{/* HTTPS Toggle Card */}
							<div className="flex items-center justify-between p-3 border border-border rounded-lg bg-card/50">
								<div className="flex flex-col gap-0.5">
									<label className="text-xs font-semibold text-foreground">HTTPS</label>
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
				<div className="fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4 backdrop-blur-sm">
					<div className="bg-card border border-border rounded-xl w-full max-w-sm shadow-2xl p-5 flex flex-col gap-4 animate-in fade-in duration-150">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<h3 className="text-sm font-bold text-foreground">Delete Domain</h3>
							<Button variant="ghost" size="icon" onClick={() => setDeleteId(null)} className="h-7 w-7 p-0 text-muted-foreground">
								<X className="w-4 h-4" />
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
								{deleting ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1" /> : null}
								{deleting ? 'Deleting...' : 'Delete Domain'}
							</Button>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
