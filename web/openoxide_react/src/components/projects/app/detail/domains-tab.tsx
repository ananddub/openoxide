import React, {useState} from 'react';
import {
	Globe,
	Trash2,
	ExternalLink,
	RefreshCw,
	Lock,
	LockOpen,
	X,
	Plus,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Badge} from '#/components/ui/badge';
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
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface DomainsTabProps {
	app: any;
	domains?: any[];
	onRefresh?: () => void;
}

export function DomainsTab({
	app,
	domains: passedDomains,
	onRefresh,
}: DomainsTabProps) {
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
			await deleteDomain.mutateAsync({params: {path: {id: deleteId}}});
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
		<div className="flex w-full animate-in flex-col gap-6 duration-200 fade-in">
			{/* Header Toolbar */}
			<div className="flex flex-wrap items-center justify-between gap-4 border-b border-border/40 pb-4">
				<div className="flex flex-col gap-1">
					<h3 className="text-lg font-bold tracking-tight text-foreground">
						Domains
					</h3>
					<p className="text-xs text-muted-foreground">
						Configure custom domains and SSL certificates for your
						application
					</p>
				</div>

				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={() => onRefresh?.()}
						className="h-8 text-xs font-semibold">
						<RefreshCw className="mr-1.5 size-3.5" /> Refresh
					</Button>
					<Button
						size="sm"
						onClick={() => setShowAdd(true)}
						className="flex h-8 items-center gap-1.5 text-xs font-bold">
						<Plus className="size-3.5" /> Add Domain
					</Button>
				</div>
			</div>

			{/* Domains Table List */}
			{domains.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 rounded-2xl border border-dashed border-border/60 bg-card/10 py-16 text-center">
					<Globe className="size-10 text-muted-foreground/35" />
					<div className="flex flex-col gap-1">
						<p className="text-sm font-bold text-foreground">
							No custom domains added
						</p>
						<p className="text-xs text-muted-foreground">
							Add a domain to route web traffic directly to your
							application
						</p>
					</div>
					<Button
						size="sm"
						onClick={() => setShowAdd(true)}
						className="mt-2 text-xs font-bold">
						<Plus className="mr-1.5 size-3.5" /> Add First Domain
					</Button>
				</div>
			) : (
				<div className="overflow-hidden rounded-xl border border-border/60 bg-card shadow-xs">
					<Table>
						<TableHeader>
							<TableRow className="border-b border-border/60 bg-muted/30 hover:bg-muted/30">
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									Domain / Host
								</TableHead>
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									Path & Port
								</TableHead>
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									SSL Status
								</TableHead>
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									Certificate
								</TableHead>
								<TableHead className="px-4 py-3.5 text-right text-xs font-bold tracking-wider text-foreground uppercase">
									Actions
								</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{domains.map((d: any) => {
								const url = `${d.https ? 'https' : 'http'}://${d.host}${d.path && d.path !== '/' ? d.path : ''}`;
								return (
									<TableRow
										key={d.id}
										className="border-b border-border/40 transition-colors hover:bg-muted/40">
										<TableCell className="px-4 py-3.5 font-mono text-xs font-bold text-foreground">
											<div className="flex items-center gap-2.5">
												<Globe className="size-4 shrink-0 text-primary" />
												<a
													href={url}
													target="_blank"
													rel="noreferrer"
													className="flex items-center gap-1.5 transition-colors hover:text-primary hover:underline">
													<span>{d.host}</span>
													<ExternalLink className="size-3.5 shrink-0 text-muted-foreground" />
												</a>
											</div>
										</TableCell>
										<TableCell className="px-4 py-3.5 font-mono text-xs text-muted-foreground">
											Path: {d.path || '/'} · Port: {d.port || 80}
										</TableCell>
										<TableCell className="px-4 py-3.5">
											{d.https ? (
												<Badge
													variant="outline"
													className="flex w-fit items-center gap-1 border-emerald-500/30 bg-emerald-500/10 text-[10px] font-semibold text-emerald-500">
													<Lock className="size-3" /> HTTPS (SSL)
												</Badge>
											) : (
												<Badge
													variant="secondary"
													className="flex w-fit items-center gap-1 text-[10px] font-semibold">
													<LockOpen className="size-3" /> HTTP
												</Badge>
											)}
										</TableCell>
										<TableCell className="px-4 py-3.5 font-mono text-xs text-muted-foreground">
											{d.certificate_type &&
											d.certificate_type !== 'none' ? (
												<Badge
													variant="outline"
													className="border-primary/30 font-mono text-[10px] text-primary">
													{d.certificate_type}
												</Badge>
											) : (
												'—'
											)}
										</TableCell>
										<TableCell className="px-4 py-3.5 text-right">
											<Button
												variant="ghost"
												size="icon"
												onClick={() => setDeleteId(d.id)}
												className="size-8 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
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
				<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-xs">
					<div className="flex w-full max-w-md animate-in flex-col gap-5 rounded-xl border border-border bg-card p-6 shadow-2xl duration-150 fade-in">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<div>
								<h3 className="text-base font-bold text-foreground">
									Add Domain
								</h3>
								<p className="mt-0.5 text-xs text-muted-foreground">
									Configure domain proxy routing for your application
								</p>
							</div>
							<Button
								variant="ghost"
								size="icon"
								onClick={() => setShowAdd(false)}
								className="size-7 p-0 text-muted-foreground">
								<X className="size-4" />
							</Button>
						</div>

						<form onSubmit={handleAdd} className="flex flex-col gap-4">
							{/* Host */}
							<div className="flex flex-col gap-1.5">
								<label className="text-xs font-semibold text-foreground">
									Host
								</label>
								<Input
									placeholder="app.example.com"
									value={host}
									onChange={e => setHost(e.target.value)}
									required
									className="h-9 text-xs"
								/>
							</div>

							{/* Path */}
							<div className="flex flex-col gap-1.5">
								<label className="text-xs font-semibold text-foreground">
									Path
								</label>
								<Input
									placeholder="/"
									value={path}
									onChange={e => setPath(e.target.value)}
									className="h-9 text-xs"
								/>
							</div>

							{/* Container Port */}
							<div className="flex flex-col gap-1.5">
								<label className="text-xs font-semibold text-foreground">
									Container Port
								</label>
								<p className="text-[11px] leading-tight text-muted-foreground">
									The port where your application is running inside the
									container (e.g. 3000)
								</p>
								<Input
									type="number"
									placeholder="3000"
									value={port}
									onChange={e => setPort(e.target.value)}
									className="h-9 text-xs"
								/>
							</div>

							{/* HTTPS Toggle Card */}
							<div className="flex items-center justify-between rounded-lg border border-border/60 bg-muted/20 p-3">
								<div className="flex flex-col gap-0.5">
									<label className="text-xs font-semibold text-foreground">
										HTTPS (SSL)
									</label>
									<span className="text-[11px] text-muted-foreground">
										Enable SSL / HTTPS secure connection for this domain
									</span>
								</div>
								<button
									type="button"
									onClick={() => {
										const next = !https;
										setHttps(next);
										if (next && certType === 'none')
											setCertType('letsencrypt');
									}}
									className={`relative inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full transition-colors ${https ? 'bg-primary' : 'bg-muted'}`}>
									<span
										className={`pointer-events-none block size-3.5 rounded-full bg-background shadow-md transition-transform ${https ? 'translate-x-4.5' : 'translate-x-0.5'}`}
									/>
								</button>
							</div>

							{/* Certificate Provider */}
							{https && (
								<div className="flex flex-col gap-1.5">
									<label className="text-xs font-semibold text-foreground">
										Certificate Provider
									</label>
									<Select
										value={certType}
										onValueChange={(val: any) => val && setCertType(val)}>
										<SelectTrigger
											size="sm"
											className="h-9 w-full text-xs font-semibold">
											<SelectValue placeholder="Select provider" />
										</SelectTrigger>
										<SelectContent>
											<SelectItem value="letsencrypt">
												Let's Encrypt (Automated SSL)
											</SelectItem>
											<SelectItem value="none">None</SelectItem>
										</SelectContent>
									</Select>
								</div>
							)}

							<div className="mt-2 flex justify-end border-t border-border/60 pt-4">
								<Button
									type="submit"
									disabled={adding}
									className="h-9 w-full bg-primary px-6 text-xs font-bold text-primary-foreground shadow-md hover:bg-primary/90 sm:w-auto">
									{adding ? 'Adding...' : 'Add Domain'}
								</Button>
							</div>
						</form>
					</div>
				</div>
			)}

			{/* Delete Domain Confirmation Modal */}
			{deleteId !== null && (
				<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-xs">
					<div className="flex w-full max-w-sm animate-in flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-2xl duration-150 fade-in">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<h3 className="text-sm font-bold text-foreground">
								Delete Domain
							</h3>
							<Button
								variant="ghost"
								size="icon"
								onClick={() => setDeleteId(null)}
								className="size-7 p-0 text-muted-foreground">
								<X className="size-4" />
							</Button>
						</div>
						<p className="text-xs leading-relaxed text-muted-foreground">
							Are you sure you want to delete this domain?
						</p>
						<div className="flex justify-end border-t border-border/60 pt-3">
							<Button
								size="sm"
								variant="destructive"
								disabled={deleting}
								onClick={confirmDelete}
								className="h-9 w-full px-6 text-xs font-bold shadow-md sm:w-auto">
								{deleting ? (
									<RefreshCw className="mr-1 size-3.5 animate-spin" />
								) : null}
								{deleting ? 'Deleting...' : 'Delete Domain'}
							</Button>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
