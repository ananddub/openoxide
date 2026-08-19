import React, {useState, useMemo} from 'react';
import {$api} from '#/api/query';
import {useQueryClient} from '@tanstack/react-query';
import {formatApiError} from '#/api/utils';
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
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Badge} from '#/components/ui/badge';
import {
	Card,
	CardHeader,
	CardTitle,
	CardDescription,
	CardContent,
} from '#/components/ui/card';
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
import {toast} from 'sonner';
import {CloudflareIcon, AwsIcon} from '#/components/icons/provider-icons';
import {useAppStore} from '#/stores/app-store';
import {DnsZonesDialog} from './dns-zones-dialog';

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
	const storeProviders = useAppStore(state => state.dnsProviders);
	const isDnsLoading = useAppStore(state => state.isDnsLoading);
	const addDnsStore = useAppStore(state => state.addDnsProvider);
	const updateDnsStore = useAppStore(state => state.updateDnsProvider);
	const deleteDnsStore = useAppStore(state => state.deleteDnsProvider);

	const createMutation = $api.useMutation('post', '/dns-providers' as any);
	const updateMutation = $api.useMutation(
		'put',
		'/dns-providers/{id}' as any,
	);
	const deleteMutation = $api.useMutation(
		'delete',
		'/dns-providers/{id}' as any,
	);
	const testMutation = $api.useMutation(
		'post',
		'/dns-providers/{id}/test' as any,
	);

	const providers = storeProviders;
	const isLoading = isDnsLoading && providers.length === 0;

	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [editingProvider, setEditingProvider] = useState<any | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<any | null>(null);
	const [isTestingModal, setIsTestingModal] = useState(false);

	// Form State
	const [formName, setFormName] = useState('');
	const [formType, setFormType] = useState<'CLOUDFLARE' | 'ROUTE53'>(
		'CLOUDFLARE',
	);
	const [formToken, setFormToken] = useState('');
	const [formAccessKey, setFormAccessKey] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const handleOpenCreate = () => {
		setEditingProvider(null);
		setFormName('');
		setFormType('CLOUDFLARE');
		setFormToken('');
		setFormAccessKey('');
		setIsCreateOpen(true);
	};

	const handleOpenEdit = (provider: any) => {
		setEditingProvider(provider);
		setFormName(provider.name);
		setFormType(
			(provider.provider_type || 'CLOUDFLARE').toUpperCase() as any,
		);
		setFormToken('');
		try {
			setFormAccessKey(
				JSON.parse(provider.credentials_json || '{}').accessKeyId || '',
			);
		} catch {
			setFormAccessKey('');
		}
		setIsCreateOpen(true);
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
				const updatePayload: any = {
					name: formName.trim(),
					credentials_json: formToken.trim()
						? formType === 'ROUTE53'
							? JSON.stringify({
									accessKeyId: formAccessKey.trim(),
									secretAccessKey: formToken.trim(),
								})
							: JSON.stringify(formToken.trim())
						: undefined,
				};

				updateDnsStore(editingProvider.id, updatePayload);

				await updateMutation.mutateAsync({
					params: {path: {id: editingProvider.id}},
					body: updatePayload,
				});
				toast.success(`DNS Provider "${formName}" updated`);
			} else {
				const createPayload: any = {
					name: formName.trim(),
					provider_type: formType,
					credentials_json:
						formType === 'ROUTE53'
							? JSON.stringify({
									accessKeyId: formAccessKey.trim(),
									secretAccessKey: formToken.trim(),
								})
							: JSON.stringify(formToken.trim()),
				};

				const res: any = await createMutation.mutateAsync({
					body: createPayload,
				});

				if (res.data || res) {
					addDnsStore(res.data || res);
				}

				toast.success(`DNS Provider "${formName}" created successfully`);
			}
			setIsCreateOpen(false);
			queryClient.invalidateQueries({queryKey: ['get', '/dns-providers']});
		} catch (err) {
			toast.error(formatApiError(err, 'Failed to save DNS provider'));
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleDeleteProvider = async () => {
		if (!deleteTarget) return;
		const targetId = deleteTarget.id;
		deleteDnsStore(targetId);
		try {
			await deleteMutation.mutateAsync({
				params: {path: {id: targetId}},
			});
			toast.success(`DNS Provider "${deleteTarget.name}" deleted`);
			setDeleteTarget(null);
			queryClient.invalidateQueries({queryKey: ['get', '/dns-providers']});
		} catch (err) {
			toast.error(formatApiError(err, 'Failed to delete DNS provider'));
		}
	};

	const testConnectionMutation = $api.useMutation(
		'post',
		'/dns-providers/test-connection' as any,
	);

	const handleTestModalConnection = async () => {
		if (!editingProvider && !formToken.trim()) {
			toast.error('Please enter an API Token to test connection');
			return;
		}

		setIsTestingModal(true);
		try {
			if (editingProvider) {
				const res: any = await testMutation.mutateAsync({
					params: {path: {id: editingProvider.id}},
				});
				const result = res.data || res;
				if (result?.success) {
					toast.success(
						result.message || 'DNS API token verified successfully!',
					);
				} else {
					toast.error(
						result?.message || 'DNS API token verification failed',
					);
				}
			} else {
				// Call Rust backend proxy to bypass browser CORS & run official API test
				const res: any = await testConnectionMutation.mutateAsync({
					body: {
						name: formName.trim() || 'Test',
						provider_type: formType,
						credentials_json:
							formType === 'ROUTE53'
								? JSON.stringify({
										accessKeyId: formAccessKey.trim(),
										secretAccessKey: formToken.trim(),
									})
								: JSON.stringify(formToken.trim()),
					},
				});
				const result = res.data || res;
				if (result?.success) {
					toast.success(
						result.message || 'DNS API Token verified successfully!',
					);
				} else {
					toast.error(
						result?.message || 'DNS API Token verification failed',
					);
				}
			}
		} catch (err: any) {
			toast.error(
				err?.message || formatApiError(err, 'DNS Connection test failed'),
			);
		} finally {
			setIsTestingModal(false);
		}
	};

	const renderDnsProviderIcon = (
		type: string,
		className = 'size-7 shrink-0',
	) => {
		switch (type.toUpperCase()) {
			case 'CLOUDFLARE':
				return <CloudflareIcon className={className} />;
			default:
				return <Globe className={`${className} text-primary`} />;
		}
	};

	const getProviderLabel = (type: string) => {
		switch (type.toUpperCase()) {
			case 'CLOUDFLARE':
				return 'Cloudflare DNS';
			default:
				return type;
		}
	};

	return (
		<div className="mx-auto max-w-6xl space-y-6 p-6">
			{/* Page Header */}
			<div className="flex flex-col justify-between gap-4 border-b border-border/40 pb-5 sm:flex-row sm:items-center">
				<div className="space-y-1">
					<h1 className="flex items-center gap-2 text-xl font-bold tracking-tight text-foreground">
						<Globe className="size-5 shrink-0 text-primary" />
						DNS Providers & Wildcard SSL
					</h1>
					<p className="text-xs text-muted-foreground">
						Connect Cloudflare DNS for automated Let's Encrypt DNS-01 SSL
						challenges and wildcard domains.
					</p>
				</div>
				<Button
					onClick={handleOpenCreate}
					size="sm"
					className="h-9 shrink-0 gap-1.5 px-4 text-xs font-semibold">
					<Plus className="size-4" /> Add DNS Provider
				</Button>
			</div>

			{/* Providers List (Horizontal Card Rows) */}
			{isLoading ? (
				<div className="p-12 text-center text-xs text-muted-foreground">
					Loading DNS providers...
				</div>
			) : providers.length === 0 ? (
				<Card className="flex flex-col items-center justify-center gap-3 rounded-2xl border border-dashed border-border/80 bg-muted/10 p-12 text-center">
					<div className="flex size-12 items-center justify-center rounded-full bg-primary/10 text-primary">
						<Globe className="size-6" />
					</div>
					<div className="max-w-sm space-y-1">
						<h3 className="text-sm font-semibold text-foreground">
							No DNS Providers Connected
						</h3>
						<p className="text-xs text-muted-foreground">
							Add a DNS Provider to issue automatic Let's Encrypt Wildcard
							SSL certificates for all your applications.
						</p>
					</div>
					<Button
						onClick={handleOpenCreate}
						size="sm"
						className="mt-2 h-8.5 gap-1.5 text-xs font-semibold">
						<Plus className="size-3.5" /> Configure First Provider
					</Button>
				</Card>
			) : (
				<div className="flex flex-col gap-3">
					{providers.map(provider => {
						return (
							<Card
								key={provider.id}
								className="flex flex-col justify-between gap-4 rounded-xl border border-border/70 bg-card p-4 shadow-2xs transition-all hover:border-border sm:flex-row sm:items-center">
								<div className="flex min-w-0 items-center gap-3.5">
									{renderDnsProviderIcon(
										provider.provider_type,
										'size-7 shrink-0',
									)}
									<div className="flex min-w-0 flex-col gap-1">
										<div className="flex items-center gap-2.5">
											<span className="truncate text-sm font-bold text-foreground">
												{provider.name}
											</span>
											<Badge
												variant="outline"
												className="shrink-0 font-mono text-[10px]">
												{getProviderLabel(provider.provider_type)}
											</Badge>
										</div>
										<div className="flex flex-wrap items-center gap-2 text-xs">
											<span className="rounded border border-emerald-500/20 bg-emerald-500/10 px-2 py-0.5 font-mono text-[11px] font-semibold text-emerald-500">
												Wildcard Ready
											</span>
											<Badge
												variant="secondary"
												className="bg-secondary text-[10px] font-medium text-secondary-foreground">
												Zones and records available via View Domains
											</Badge>
											<span className="text-[11px] text-muted-foreground">
												• Added{' '}
												{new Date(
													provider.created_at * 1000,
												).toLocaleDateString()}
											</span>
										</div>
									</div>
								</div>

								<div className="flex shrink-0 items-center gap-2 self-end sm:self-center">
									<DnsZonesDialog provider={provider} />
									<div className="flex items-center gap-1">
										<Button
											variant="ghost"
											size="icon"
											onClick={() => handleOpenEdit(provider)}
											title="Edit provider"
											className="size-8 rounded-lg text-muted-foreground hover:bg-primary/10 hover:text-primary">
											<Pencil className="size-4" />
										</Button>
										<Button
											variant="ghost"
											size="icon"
											onClick={() => setDeleteTarget(provider)}
											title="Delete provider"
											className="size-8 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
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
				<DialogContent className="max-h-[90vh] overflow-y-auto rounded-2xl border border-border bg-card p-6 shadow-2xl sm:max-w-lg">
					<DialogHeader className="space-y-1">
						<DialogTitle className="flex items-center gap-2 text-base font-bold text-foreground">
							<Globe className="size-5 text-primary" />
							{editingProvider
								? 'Update DNS Provider'
								: 'Add DNS Provider'}
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Configure API credentials for automated DNS-01 Let's Encrypt
							Wildcard SSL.
						</DialogDescription>
					</DialogHeader>

					<form onSubmit={handleSaveProvider} className="space-y-4 pt-2">
						<div className="space-y-1">
							<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
								Provider Name
							</label>
							<Input
								placeholder="e.g. Cloudflare Production"
								value={formName}
								onChange={e => setFormName(e.target.value)}
								className="h-10 bg-muted/20 text-xs"
							/>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
								Provider Type
							</label>
							<Select
								value={formType}
								onValueChange={(v: any) => setFormType(v)}
								disabled={!!editingProvider}>
								<SelectTrigger className="h-10 w-full bg-muted/20 text-xs">
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
											<AwsIcon className="size-4 shrink-0" />
											Amazon Route53
										</div>
									</SelectItem>
								</SelectContent>
							</Select>
						</div>

						{formType === 'ROUTE53' && (
							<div className="space-y-1">
								<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
									AWS Access Key ID
								</label>
								<Input
									value={formAccessKey}
									onChange={e => setFormAccessKey(e.target.value)}
									className="h-10 bg-muted/20 font-mono text-xs"
								/>
							</div>
						)}
						<div className="space-y-1">
							<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
								API Token / Secret Key
							</label>
							<Input
								type="password"
								placeholder={
									editingProvider
										? 'Leave blank to keep existing token'
										: 'API Token with Zone:DNS Edit permissions'
								}
								value={formToken}
								onChange={e => setFormToken(e.target.value)}
								className="h-10 bg-muted/20 font-mono text-xs"
							/>
						</div>

						{/* Dokploy Exact Footer Layout: Left Test Connection & Right Save/Update */}
						<div className="mt-4 flex w-full items-center justify-between gap-2 border-t border-border/40 pt-4">
							<Button
								type="button"
								variant="secondary"
								disabled={isTestingModal}
								onClick={handleTestModalConnection}
								className="h-10 gap-1.5 bg-secondary px-4 text-xs font-semibold text-secondary-foreground hover:bg-secondary/80">
								{isTestingModal && (
									<RefreshCw className="size-3.5 animate-spin" />
								)}
								Test Connection
							</Button>
							<Button
								type="submit"
								disabled={isSubmitting}
								className="h-10 gap-1.5 px-6 text-xs font-semibold">
								{isSubmitting && (
									<RefreshCw className="size-3.5 animate-spin" />
								)}
								{editingProvider ? 'Update' : 'Create'}
							</Button>
						</div>
					</form>
				</DialogContent>
			</Dialog>

			{/* Delete Confirmation Modal */}
			<Dialog
				open={!!deleteTarget}
				onOpenChange={open => !open && setDeleteTarget(null)}>
				<DialogContent className="rounded-2xl border border-border bg-card p-6 shadow-2xl sm:max-w-sm">
					<DialogHeader className="space-y-2">
						<DialogTitle className="flex items-center gap-2 text-base font-bold text-destructive">
							<AlertCircle className="size-5" />
							Delete DNS Provider?
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Are you sure you want to remove{' '}
							<span className="font-semibold text-foreground">
								{deleteTarget?.name}
							</span>
							? Wildcard SSL renewals for linked domains will stop working.
						</DialogDescription>
					</DialogHeader>

					<div className="flex w-full items-center justify-end gap-2 border-t border-border/40 pt-3">
						<Button
							variant="destructive"
							onClick={handleDeleteProvider}
							className="h-9 w-full text-xs font-semibold">
							Delete Permanently
						</Button>
					</div>
				</DialogContent>
			</Dialog>
		</div>
	);
}
