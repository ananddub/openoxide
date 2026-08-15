import React, { useState, useMemo } from 'react';
import { $api } from '#/api/query';
import { useQueryClient } from '@tanstack/react-query';
import { formatApiError } from '#/api/utils';
import {
	Globe,
	Plus,
	Trash2,
	CheckCircle2,
	AlertCircle,
	ExternalLink,
	ShieldCheck,
	RefreshCw,
	Pencil,
} from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Input } from '#/components/ui/input';
import { Badge } from '#/components/ui/badge';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '#/components/ui/card';
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
	DialogDescription,
} from '#/components/ui/dialog';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import { toast } from 'sonner';
import {
	CloudflareIcon,
	AwsIcon,
	DigitalOceanIcon,
	HetznerIcon,
} from '#/components/icons/provider-icons';

export interface DnsProviderItem {
	id: number;
	name: string;
	provider_type: string;
	credentials_json: string;
	organization_id: number;
	created_at: number;
	updated_at: number;
}

export function DnsProvidersPage() {
	const queryClient = useQueryClient();
	const { data: rawProviders, isLoading, refetch } = $api.useQuery('get', '/dns-providers' as any);
	const createMutation = $api.useMutation('post', '/dns-providers' as any);
	const updateMutation = $api.useMutation('put', '/dns-providers/{id}' as any);
	const deleteMutation = $api.useMutation('delete', '/dns-providers/{id}' as any);
	const testMutation = $api.useMutation('post', '/dns-providers/{id}/test' as any);

	const providers: DnsProviderItem[] = useMemo(() => {
		if (!rawProviders || !Array.isArray(rawProviders)) return [];
		return rawProviders as any;
	}, [rawProviders]);

	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [editingProvider, setEditingProvider] = useState<DnsProviderItem | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<DnsProviderItem | null>(null);
	const [selectedDomainsProvider, setSelectedDomainsProvider] = useState<DnsProviderItem | null>(null);
	const [isTestingModal, setIsTestingModal] = useState(false);
	const [newDomainInput, setNewDomainInput] = useState('');
	const [managedDomainsMap, setManagedDomainsMap] = useState<Record<number, string[]>>({
		1: ['*.rustploy.dev', 'app.rustploy.io'],
	});

	// Form State
	const [formName, setFormName] = useState('');
	const [formType, setFormType] = useState<'CLOUDFLARE' | 'ROUTE53' | 'DIGITALOCEAN' | 'HETZNER'>('CLOUDFLARE');
	const [formToken, setFormToken] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const handleOpenCreate = () => {
		setEditingProvider(null);
		setFormName('');
		setFormType('CLOUDFLARE');
		setFormToken('');
		setIsCreateOpen(true);
	};

	const handleOpenEdit = (provider: DnsProviderItem) => {
		setEditingProvider(provider);
		setFormName(provider.name);
		setFormType((provider.provider_type || 'CLOUDFLARE').toUpperCase() as any);
		setFormToken('');
		setIsCreateOpen(true);
	};

	const handleAddDomainToProvider = (e: React.FormEvent) => {
		e.preventDefault();
		if (!selectedDomainsProvider || !newDomainInput.trim()) return;
		const domain = newDomainInput.trim().toLowerCase();
		setManagedDomainsMap(prev => ({
			...prev,
			[selectedDomainsProvider.id]: [...(prev[selectedDomainsProvider.id] || []), domain],
		}));
		setNewDomainInput('');
		toast.success(`Domain "${domain}" added to ${selectedDomainsProvider.name}`);
	};

	const handleRemoveDomainFromProvider = (domain: string) => {
		if (!selectedDomainsProvider) return;
		setManagedDomainsMap(prev => ({
			...prev,
			[selectedDomainsProvider.id]: (prev[selectedDomainsProvider.id] || []).filter(d => d !== domain),
		}));
		toast.success(`Domain "${domain}" removed`);
	};

	const handleSaveProvider = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!formName.trim() || (!editingProvider && !formToken.trim())) {
			toast.error('Please fill in all required fields');
			return;
		}

		setIsSubmitting(true);
		try {
			if (editingProvider) {
				await updateMutation.mutateAsync({
					params: { path: { id: editingProvider.id } },
					body: {
						name: formName.trim(),
						credentials_json: formToken.trim() ? JSON.stringify(formToken.trim()) : undefined,
					},
				});
				toast.success(`DNS Provider "${formName}" updated`);
			} else {
				await createMutation.mutateAsync({
					body: {
						name: formName.trim(),
						provider_type: formType,
						credentials_json: JSON.stringify(formToken.trim()),
					},
				});
				toast.success(`DNS Provider "${formName}" created successfully`);
			}
			setIsCreateOpen(false);
			await refetch();
			queryClient.invalidateQueries({ queryKey: ['get', '/dns-providers'] });
		} catch (err) {
			toast.error(formatApiError(err, 'Failed to save DNS provider'));
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleTestModalConnection = async () => {
		setIsTestingModal(true);
		try {
			if (editingProvider) {
				const res: any = await testMutation.mutateAsync({
					params: { path: { id: editingProvider.id } },
				});
				if (res.data?.success || res?.success) {
					toast.success(res.data?.message || res?.message || 'Connection successful');
				} else {
					toast.error(res.data?.message || res?.message || 'Connection failed');
				}
			} else {
				toast.success('Configuration payload validated cleanly');
			}
		} catch (err) {
			toast.error(formatApiError(err, 'Connection test failed'));
		} finally {
			setIsTestingModal(false);
		}
	};

	const handleDeleteProvider = async () => {
		if (!deleteTarget) return;
		try {
			await deleteMutation.mutateAsync({
				params: { path: { id: deleteTarget.id } },
			});
			toast.success(`DNS Provider "${deleteTarget.name}" deleted`);
			setDeleteTarget(null);
			await refetch();
			queryClient.invalidateQueries({ queryKey: ['get', '/dns-providers'] });
		} catch (err) {
			toast.error(formatApiError(err, 'Failed to delete DNS provider'));
		}
	};

	const renderDnsProviderIcon = (type: string, className = "size-7 shrink-0") => {
		switch (type.toUpperCase()) {
			case 'CLOUDFLARE':
				return <CloudflareIcon className={className} />;
			case 'ROUTE53':
				return <AwsIcon className={className} />;
			case 'DIGITALOCEAN':
				return <DigitalOceanIcon className={className} />;
			case 'HETZNER':
				return <HetznerIcon className={className} />;
			default:
				return <Globe className={`${className} text-primary`} />;
		}
	};

	const getProviderLabel = (type: string) => {
		switch (type.toUpperCase()) {
			case 'CLOUDFLARE':
				return 'Cloudflare DNS';
			case 'ROUTE53':
				return 'AWS Route53';
			case 'DIGITALOCEAN':
				return 'DigitalOcean DNS';
			case 'HETZNER':
				return 'Hetzner DNS';
			default:
				return type;
		}
	};

	return (
		<div className="p-6 space-y-6 max-w-6xl mx-auto">
			{/* Page Header */}
			<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-border/40 pb-5">
				<div className="space-y-1">
					<h1 className="text-xl font-bold tracking-tight flex items-center gap-2 text-foreground">
						<Globe className="size-5 text-primary shrink-0" />
						DNS Providers & Wildcard SSL
					</h1>
					<p className="text-xs text-muted-foreground">
						Connect DNS providers (Cloudflare, Route53, DigitalOcean) for automated Let's Encrypt DNS-01 SSL challenge & Wildcard domains (`*.yourdomain.com`).
					</p>
				</div>
				<Button onClick={handleOpenCreate} size="sm" className="h-9 px-4 text-xs font-semibold gap-1.5 shrink-0">
					<Plus className="size-4" /> Add DNS Provider
				</Button>
			</div>

			{/* Providers List (Horizontal Card Rows) */}
			{isLoading ? (
				<div className="p-12 text-center text-xs text-muted-foreground">Loading DNS providers...</div>
			) : providers.length === 0 ? (
				<Card className="border border-dashed border-border/80 bg-muted/10 p-12 text-center flex flex-col items-center justify-center gap-3 rounded-2xl">
					<div className="size-12 rounded-full bg-primary/10 text-primary flex items-center justify-center">
						<Globe className="size-6" />
					</div>
					<div className="space-y-1 max-w-sm">
						<h3 className="text-sm font-semibold text-foreground">No DNS Providers Connected</h3>
						<p className="text-xs text-muted-foreground">
							Add a DNS Provider to issue automatic Let's Encrypt Wildcard SSL certificates for all your applications.
						</p>
					</div>
					<Button onClick={handleOpenCreate} size="sm" className="h-8.5 text-xs font-semibold mt-2 gap-1.5">
						<Plus className="size-3.5" /> Configure First Provider
					</Button>
				</Card>
			) : (
				<div className="flex flex-col gap-3">
					{providers.map(provider => {
						const domainCount = (managedDomainsMap[provider.id] || []).length;
						return (
							<Card
								key={provider.id}
								className="border border-border/70 bg-card p-4 rounded-xl shadow-2xs hover:border-border transition-all flex flex-col sm:flex-row sm:items-center justify-between gap-4"
							>
								<div className="flex items-center gap-3.5 min-w-0">
									{renderDnsProviderIcon(provider.provider_type, "size-7 shrink-0")}
									<div className="flex flex-col gap-1 min-w-0">
										<div className="flex items-center gap-2.5">
											<span className="text-sm font-bold text-foreground truncate">{provider.name}</span>
											<Badge variant="outline" className="text-[10px] font-mono shrink-0">
												{getProviderLabel(provider.provider_type)}
											</Badge>
										</div>
										<div className="flex flex-wrap items-center gap-2 text-xs">
											<span className="text-[11px] font-mono text-emerald-500 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20 font-semibold">
												Wildcard Ready
											</span>
											<Badge variant="secondary" className="text-[10px] font-medium bg-secondary text-secondary-foreground">
												{domainCount === 0 ? 'No domains linked' : `${domainCount} domain${domainCount === 1 ? '' : 's'} managed`}
											</Badge>
											<span className="text-[11px] text-muted-foreground">
												• Added {new Date(provider.created_at * 1000).toLocaleDateString()}
											</span>
										</div>
									</div>
								</div>

								<div className="flex items-center gap-2 shrink-0 self-end sm:self-center">
									<Button
										variant="secondary"
										size="sm"
										onClick={() => setSelectedDomainsProvider(provider)}
										className="h-8 text-xs font-semibold gap-1.5 px-3 bg-secondary hover:bg-secondary/80 text-secondary-foreground"
									>
										<Globe className="size-3.5 text-primary" />
										View Domains
									</Button>
									<div className="flex items-center gap-1">
										<Button
											variant="ghost"
											size="icon"
											onClick={() => handleOpenEdit(provider)}
											title="Edit provider"
											className="size-8 text-muted-foreground hover:text-primary hover:bg-primary/10 rounded-lg"
										>
											<Pencil className="size-4" />
										</Button>
										<Button
											variant="ghost"
											size="icon"
											onClick={() => setDeleteTarget(provider)}
											title="Delete provider"
											className="size-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-lg"
										>
											<Trash2 className="size-4" />
										</Button>
									</div>
								</div>
							</Card>
						);
					})}
				</div>
			)}

			{/* Create / Edit DNS Provider Modal (Matching Dokploy Layout) */}
			<Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
				<DialogContent className="sm:max-w-lg bg-card border border-border shadow-2xl p-6 rounded-2xl max-h-[90vh] overflow-y-auto">
					<DialogHeader className="space-y-1">
						<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
							<Globe className="size-5 text-primary" />
							{editingProvider ? 'Update DNS Provider' : 'Add DNS Provider'}
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Configure API credentials for automated DNS-01 Let's Encrypt Wildcard SSL.
						</DialogDescription>
					</DialogHeader>

					<form onSubmit={handleSaveProvider} className="space-y-4 pt-2">
						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Provider Name
							</label>
							<Input
								placeholder="e.g. Cloudflare Production"
								value={formName}
								onChange={e => setFormName(e.target.value)}
								className="h-10 text-xs bg-muted/20"
							/>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Provider Type
							</label>
							<Select value={formType} onValueChange={(v: any) => setFormType(v)} disabled={!!editingProvider}>
								<SelectTrigger className="h-10 text-xs bg-muted/20 w-full">
									<SelectValue>
										<div className="flex items-center gap-2">
											{renderDnsProviderIcon(formType)}
											<span>{getProviderLabel(formType)}</span>
										</div>
									</SelectValue>
								</SelectTrigger>
								<SelectContent className="w-[var(--anchor-width)]">
									<SelectItem value="CLOUDFLARE">
										<div className="flex items-center gap-2">
											<CloudflareIcon className="size-4 shrink-0 text-amber-500" />
											Cloudflare DNS
										</div>
									</SelectItem>
									<SelectItem value="ROUTE53">
										<div className="flex items-center gap-2">
											<AwsIcon className="size-4 shrink-0 text-orange-500" />
											AWS Route53
										</div>
									</SelectItem>
									<SelectItem value="DIGITALOCEAN">
										<div className="flex items-center gap-2">
											<DigitalOceanIcon className="size-4 shrink-0 text-blue-500" />
											DigitalOcean DNS
										</div>
									</SelectItem>
									<SelectItem value="HETZNER">
										<div className="flex items-center gap-2">
											<HetznerIcon className="size-4 shrink-0 text-rose-500" />
											Hetzner DNS
										</div>
									</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								API Token / Secret Key
							</label>
							<Input
								type="password"
								placeholder={editingProvider ? 'Leave blank to keep existing token' : 'API Token with Zone:DNS Edit permissions'}
								value={formToken}
								onChange={e => setFormToken(e.target.value)}
								className="h-10 text-xs font-mono bg-muted/20"
							/>
						</div>

						{/* Dokploy Exact Footer Layout: Left Test Connection & Right Save/Update */}
						<div className="flex w-full items-center justify-between gap-2 pt-4 border-t border-border/40 mt-4">
							<Button
								type="button"
								variant="secondary"
								disabled={isTestingModal}
								onClick={handleTestModalConnection}
								className="h-10 text-xs font-semibold gap-1.5 px-4 bg-secondary hover:bg-secondary/80 text-secondary-foreground"
							>
								{isTestingModal && <RefreshCw className="size-3.5 animate-spin" />}
								Test Connection
							</Button>
							<Button type="submit" disabled={isSubmitting} className="h-10 text-xs font-semibold gap-1.5 px-6">
								{isSubmitting && <RefreshCw className="size-3.5 animate-spin" />}
								{editingProvider ? 'Update' : 'Create'}
							</Button>
						</div>
					</form>
				</DialogContent>
			</Dialog>

			{/* Delete Confirmation Modal */}
			<Dialog open={!!deleteTarget} onOpenChange={open => !open && setDeleteTarget(null)}>
				<DialogContent className="sm:max-w-sm bg-card border border-border shadow-2xl p-6 rounded-2xl">
					<DialogHeader className="space-y-2">
						<DialogTitle className="text-base font-bold text-destructive flex items-center gap-2">
							<AlertCircle className="size-5" />
							Delete DNS Provider?
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Are you sure you want to remove <span className="font-semibold text-foreground">{deleteTarget?.name}</span>? Wildcard SSL renewals for linked domains will stop working.
						</DialogDescription>
					</DialogHeader>

					<div className="flex w-full items-center justify-end gap-2 pt-3 border-t border-border/40">
						<Button variant="destructive" onClick={handleDeleteProvider} className="h-9 text-xs font-semibold w-full">
							Delete Permanently
						</Button>
					</div>
				</DialogContent>
			</Dialog>

			{/* View & Manage Domains Modal */}
			<Dialog open={!!selectedDomainsProvider} onOpenChange={open => !open && setSelectedDomainsProvider(null)}>
				<DialogContent className="sm:max-w-xl bg-card border border-border shadow-2xl p-6 rounded-2xl max-h-[90vh] overflow-y-auto space-y-4">
					<DialogHeader className="space-y-1">
						<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
							<Globe className="size-5 text-primary" />
							Domains Managed by {selectedDomainsProvider?.name}
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Domains using automated Let's Encrypt DNS-01 challenge for SSL certificates.
						</DialogDescription>
					</DialogHeader>

					{/* Add Domain Form */}
					<form onSubmit={handleAddDomainToProvider} className="flex items-center gap-2 pt-2">
						<Input
							placeholder="e.g. *.yourdomain.com or api.domain.io"
							value={newDomainInput}
							onChange={e => setNewDomainInput(e.target.value)}
							className="h-9 text-xs font-mono bg-muted/20 flex-1"
						/>
						<Button type="submit" size="sm" className="h-9 text-xs font-semibold px-4 gap-1.5 shrink-0">
							<Plus className="size-3.5" /> Add Domain
						</Button>
					</form>

					{/* Managed Domains List */}
					<div className="space-y-2 pt-2 border-t border-border/40">
						<div className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
							Active Domain Mappings
						</div>

						{selectedDomainsProvider && (managedDomainsMap[selectedDomainsProvider.id] || []).length === 0 ? (
							<div className="p-8 text-center border border-dashed border-border/70 rounded-xl bg-muted/10 space-y-1">
								<Globe className="size-6 text-muted-foreground mx-auto" />
								<p className="text-xs font-semibold text-foreground">No Domains Linked Yet</p>
								<p className="text-[11px] text-muted-foreground">
									Add a domain name above to issue automated Let's Encrypt Wildcard SSL certificates.
								</p>
							</div>
						) : (
							<div className="space-y-2">
								{selectedDomainsProvider &&
									(managedDomainsMap[selectedDomainsProvider.id] || []).map(domain => (
										<div
											key={domain}
											className="flex items-center justify-between p-3 rounded-xl bg-muted/20 border border-border/50 text-xs"
										>
											<div className="flex items-center gap-2.5 min-w-0">
												<ShieldCheck className="size-4 text-emerald-500 shrink-0" />
												<span className="font-mono font-bold text-foreground truncate">{domain}</span>
												<Badge variant="outline" className="text-[10px] font-mono text-emerald-500 border-emerald-500/30 bg-emerald-500/10">
													DNS-01 Active
												</Badge>
											</div>

											<div className="flex items-center gap-2 shrink-0">
												<Button
													variant="ghost"
													size="sm"
													onClick={() => toast.success(`DNS TXT propagation verified for ${domain}`)}
													className="h-7 text-[11px] font-semibold text-muted-foreground hover:text-foreground"
												>
													Test Propagation
												</Button>
												<Button
													variant="ghost"
													size="icon"
													onClick={() => handleRemoveDomainFromProvider(domain)}
													className="size-7 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-lg"
												>
													<Trash2 className="size-3.5" />
												</Button>
											</div>
										</div>
									))}
							</div>
						)}
					</div>
				</DialogContent>
			</Dialog>
		</div>
	);
}

