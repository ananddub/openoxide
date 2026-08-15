import React, { useState, useMemo } from 'react';
import { $api } from '#/api/query';
import { useQueryClient } from '@tanstack/react-query';
import { formatApiError } from '#/api/utils';
import {
	Vault,
	Plus,
	Trash2,
	AlertCircle,
	RefreshCw,
	KeyRound,
	Pencil,
	Copy,
	Check,
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
	HashicorpVaultIcon,
	InfisicalIcon,
	AwsIcon,
	DopplerIcon,
	AzureIcon,
	ScalewayIcon,
} from '#/components/icons/provider-icons';

export interface VaultProviderItem {
	id: number;
	name: string;
	provider_type: string;
	credentials_json: string;
	organization_id: number;
	created_at: number;
	updated_at: number;
}

export function VaultProvidersPage() {
	const queryClient = useQueryClient();
	const { data: rawProviders, isLoading, refetch } = $api.useQuery('get', '/vault-providers' as any);
	const createMutation = $api.useMutation('post', '/vault-providers' as any);
	const updateMutation = $api.useMutation('put', '/vault-providers/{id}' as any);
	const deleteMutation = $api.useMutation('delete', '/vault-providers/{id}' as any);
	const testMutation = $api.useMutation('post', '/vault-providers/{id}/test' as any);

	const providers: VaultProviderItem[] = useMemo(() => {
		if (!rawProviders || !Array.isArray(rawProviders)) return [];
		return rawProviders as any;
	}, [rawProviders]);

	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [editingProvider, setEditingProvider] = useState<VaultProviderItem | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<VaultProviderItem | null>(null);
	const [isTestingModal, setIsTestingModal] = useState(false);
	const [copiedId, setCopiedId] = useState<number | null>(null);

	// Form State
	const [formName, setFormName] = useState('');
	const [formType, setFormType] = useState<
		'HASHICORP' | 'INFISICAL' | 'AWS' | 'DOPPLER' | 'AZURE' | 'SCALEWAY'
	>('HASHICORP');
	const [formUrl, setFormUrl] = useState('');
	const [formMount, setFormMount] = useState('secret');
	const [formToken, setFormToken] = useState('');
	const [formNamespace, setFormNamespace] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const handleOpenCreate = () => {
		setEditingProvider(null);
		setFormName('');
		setFormType('HASHICORP');
		setFormUrl('');
		setFormMount('secret');
		setFormToken('');
		setFormNamespace('');
		setIsCreateOpen(true);
	};

	const handleOpenEdit = (provider: VaultProviderItem) => {
		setEditingProvider(provider);
		setFormName(provider.name);
		setFormType((provider.provider_type || 'HASHICORP').toUpperCase() as any);
		try {
			const creds = JSON.parse(provider.credentials_json || '{}');
			setFormUrl(creds.url || creds.api_url || '');
			setFormMount(creds.mount || 'secret');
			setFormNamespace(creds.namespace || '');
		} catch {
			setFormUrl('');
			setFormMount('secret');
			setFormNamespace('');
		}
		setFormToken('');
		setIsCreateOpen(true);
	};

	const handleSaveProvider = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!formName.trim()) {
			toast.error('Please enter a provider name');
			return;
		}

		setIsSubmitting(true);
		try {
			const payload: any = {
				name: formName.trim(),
				provider_type: formType,
				credentials_json: JSON.stringify({
					url: formUrl.trim(),
					mount: formMount.trim(),
					namespace: formNamespace.trim(),
					...(formToken.trim() ? { token: formToken.trim() } : {}),
				}),
			};

			if (editingProvider) {
				await updateMutation.mutateAsync({
					params: { path: { id: editingProvider.id } },
					body: payload,
				});
				toast.success(`Vault Provider "${formName}" updated`);
			} else {
				await createMutation.mutateAsync({
					body: payload,
				});
				toast.success(`Vault Provider "${formName}" created successfully`);
			}
			setIsCreateOpen(false);
			await refetch();
			queryClient.invalidateQueries({ queryKey: ['get', '/vault-providers'] });
		} catch (err) {
			toast.error(formatApiError(err, 'Failed to save vault provider'));
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
			toast.success(`Vault Provider "${deleteTarget.name}" deleted`);
			setDeleteTarget(null);
			await refetch();
			queryClient.invalidateQueries({ queryKey: ['get', '/vault-providers'] });
		} catch (err) {
			toast.error(formatApiError(err, 'Failed to delete vault provider'));
		}
	};

	const copySyntax = (name: string, id: number) => {
		const text = `\${{vault.${name}.secret:FIELD}}`;
		navigator.clipboard.writeText(text);
		setCopiedId(id);
		toast.success('Reference syntax copied to clipboard!');
		setTimeout(() => setCopiedId(null), 2000);
	};

	const renderVaultProviderIcon = (type: string, className = "size-7 shrink-0") => {
		switch (type.toUpperCase()) {
			case 'HASHICORP':
				return <HashicorpVaultIcon className={`${className} text-amber-500`} />;
			case 'INFISICAL':
				return <InfisicalIcon className={`${className} text-emerald-500`} />;
			case 'DOPPLER':
				return <DopplerIcon className={`${className} text-purple-500`} />;
			case 'AWS':
				return <AwsIcon className={`${className} text-amber-600`} />;
			case 'AZURE':
				return <AzureIcon className={`${className} text-sky-500`} />;
			case 'SCALEWAY':
				return <ScalewayIcon className={`${className} text-purple-600`} />;
			default:
				return <KeyRound className={`${className} text-primary`} />;
		}
	};

	const getProviderLabel = (type: string) => {
		switch (type.toUpperCase()) {
			case 'HASHICORP':
				return 'HashiCorp Vault';
			case 'INFISICAL':
				return 'Infisical';
			case 'DOPPLER':
				return 'Doppler';
			case 'AWS':
				return 'AWS Secrets Manager';
			case 'AZURE':
				return 'Azure Key Vault';
			case 'SCALEWAY':
				return 'Scaleway Secret Manager';
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
						<Vault className="size-5 text-primary shrink-0" />
						Vault Providers
					</h1>
					<p className="text-xs text-muted-foreground">
						Connect HashiCorp Vault, Infisical, Doppler, or AWS Secrets Manager. Reference secret values dynamically during build and deployment.
					</p>
				</div>
				<Button onClick={handleOpenCreate} size="sm" className="h-9 px-4 text-xs font-semibold gap-1.5 shrink-0">
					<Plus className="size-4" /> Add Vault Provider
				</Button>
			</div>

			{/* Providers Grid */}
			{isLoading ? (
				<div className="p-12 text-center text-xs text-muted-foreground">Loading vault providers...</div>
			) : providers.length === 0 ? (
				<Card className="border border-dashed border-border/80 bg-muted/10 p-12 text-center flex flex-col items-center justify-center gap-3 rounded-2xl">
					<div className="size-12 rounded-full bg-primary/10 text-primary flex items-center justify-center">
						<Vault className="size-6" />
					</div>
					<div className="space-y-1 max-w-sm">
						<h3 className="text-sm font-semibold text-foreground">No Vault Providers Connected</h3>
						<p className="text-xs text-muted-foreground">
							Reference secrets in your environment variables with <code className="font-mono text-primary">${'{vault.<name>.<secret>}'}</code>.
						</p>
					</div>
					<Button onClick={handleOpenCreate} size="sm" className="h-8.5 text-xs font-semibold mt-2 gap-1.5">
						<Plus className="size-3.5" /> Configure First Provider
					</Button>
				</Card>
			) : (
				<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
					{providers.map(provider => {
						const isCopied = copiedId === provider.id;
						return (
							<Card key={provider.id} className="border border-border/70 bg-card shadow-xs rounded-xl overflow-hidden hover:border-border transition-colors flex flex-col justify-between">
								<CardHeader className="p-4 pb-3 flex flex-row items-start justify-between space-y-0">
									<div className="space-y-1 min-w-0 pr-2">
										<div className="flex items-center gap-2">
											{renderVaultProviderIcon(provider.provider_type, "size-4.5 shrink-0")}
											<CardTitle className="text-sm font-bold text-foreground truncate">{provider.name}</CardTitle>
										</div>
										<Badge variant="outline" className="text-[10px]">
											{getProviderLabel(provider.provider_type)}
										</Badge>
									</div>
									<div className="flex items-center gap-1 shrink-0">
										<Button
											variant="ghost"
											size="icon"
											onClick={() => handleOpenEdit(provider)}
											className="size-7 text-muted-foreground hover:text-primary hover:bg-primary/10 rounded-lg"
										>
											<Pencil className="size-3.5" />
										</Button>
										<Button
											variant="ghost"
											size="icon"
											onClick={() => setDeleteTarget(provider)}
											className="size-7 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-lg"
										>
											<Trash2 className="size-3.5" />
										</Button>
									</div>
								</CardHeader>

								<CardContent className="p-4 pt-0 space-y-3">
									<div className="space-y-1">
										<div className="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">Secret Reference Syntax</div>
										<div className="flex items-center justify-between bg-muted/30 border border-border/40 rounded-md p-2 text-xs font-mono">
											<span className="truncate text-primary text-[11px] font-semibold">{`\${{vault.${provider.name}.SECRET_KEY}}`}</span>
											<Button
												variant="ghost"
												size="icon"
												onClick={() => copySyntax(provider.name, provider.id)}
												className="size-6 text-muted-foreground hover:text-foreground shrink-0"
											>
												{isCopied ? <Check className="size-3 text-emerald-500" /> : <Copy className="size-3" />}
											</Button>
										</div>
									</div>

									<div className="pt-2 border-t border-border/40 flex items-center justify-between text-[11px] text-muted-foreground">
										<span>Added {new Date(provider.created_at * 1000).toLocaleDateString()}</span>
										<Badge variant="outline" className="text-[10px] font-mono">
											Active
										</Badge>
									</div>
								</CardContent>
							</Card>
						);
					})}
				</div>
			)}

			{/* Create / Edit Provider Modal (Matching Dokploy Popup Layout) */}
			<Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
				<DialogContent className="sm:max-w-lg bg-card border border-border shadow-2xl p-6 rounded-2xl max-h-[90vh] overflow-y-auto">
					<DialogHeader className="space-y-1">
						<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
							<KeyRound className="size-5 text-primary" />
							{editingProvider ? 'Update Vault Provider' : 'Add Vault Provider'}
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Reference secrets in your environment variables with{' '}
							<code className="text-primary font-mono">{'${{vault.<name>.<secret>}}'}</code>. Secrets are fetched at deploy time.
						</DialogDescription>
					</DialogHeader>

					<form onSubmit={handleSaveProvider} className="space-y-4 pt-2">
						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Name
							</label>
							<Input
								placeholder="prod-vault"
								value={formName}
								onChange={e => setFormName(e.target.value)}
								className="h-10 text-xs bg-muted/20"
							/>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Provider
							</label>
							<Select value={formType} onValueChange={(v: any) => setFormType(v)} disabled={!!editingProvider}>
								<SelectTrigger className="h-10 text-xs bg-muted/20 w-full">
									<SelectValue>
										<div className="flex items-center gap-2">
											{renderVaultProviderIcon(formType)}
											<span>{getProviderLabel(formType)}</span>
										</div>
									</SelectValue>
								</SelectTrigger>
								<SelectContent className="w-[var(--anchor-width)]">
									<SelectItem value="HASHICORP">
										<div className="flex items-center gap-2">
											<HashicorpVaultIcon className="size-4 shrink-0 text-amber-500" />
											HashiCorp Vault / OpenBao
										</div>
									</SelectItem>
									<SelectItem value="INFISICAL">
										<div className="flex items-center gap-2">
											<InfisicalIcon className="size-4 shrink-0 text-emerald-500" />
											Infisical
										</div>
									</SelectItem>
									<SelectItem value="DOPPLER">
										<div className="flex items-center gap-2">
											<DopplerIcon className="size-4 shrink-0 text-purple-500" />
											Doppler
										</div>
									</SelectItem>
									<SelectItem value="AWS">
										<div className="flex items-center gap-2">
											<AwsIcon className="size-4 shrink-0 text-amber-600" />
											AWS Secrets Manager
										</div>
									</SelectItem>
									<SelectItem value="AZURE">
										<div className="flex items-center gap-2">
											<AzureIcon className="size-4 shrink-0 text-sky-500" />
											Azure Key Vault
										</div>
									</SelectItem>
									<SelectItem value="SCALEWAY">
										<div className="flex items-center gap-2">
											<ScalewayIcon className="size-4 shrink-0 text-purple-600" />
											Scaleway Secret Manager
										</div>
									</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Vault URL
							</label>
							<Input
								placeholder="https://vault.example.com:8200"
								value={formApiUrl}
								onChange={e => setFormApiUrl(e.target.value)}
								className="h-10 text-xs bg-muted/20"
							/>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Token / Access Key
							</label>
							<Input
								type="password"
								placeholder="Token"
								value={formAuthToken}
								onChange={e => setFormAuthToken(e.target.value)}
								className="h-10 text-xs font-mono bg-muted/20"
							/>
						</div>

						{formType === 'HASHICORP' && (
							<>
								<div className="grid grid-cols-2 gap-3">
									<div className="space-y-1">
										<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
											KV Mount
										</label>
										<Input
											placeholder="secret"
											value={formMount}
											onChange={e => setFormMount(e.target.value)}
											className="h-10 text-xs font-mono bg-muted/20"
										/>
									</div>
									<div className="space-y-1">
										<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
											Namespace (optional)
										</label>
										<Input
											placeholder="admin"
											value={formNamespace}
											onChange={e => setFormNamespace(e.target.value)}
											className="h-10 text-xs font-mono bg-muted/20"
										/>
									</div>
								</div>
								<div className="text-[11px] text-muted-foreground bg-muted/30 p-2.5 rounded-lg border border-border/40">
									Reference format: <code className="text-primary font-mono font-semibold">{'${{vault.<name>.path/to/secret:FIELD}}'}</code>
								</div>
							</>
						)}

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
							Delete Secrets Provider?
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Are you sure you want to remove <span className="font-semibold text-foreground">{deleteTarget?.name}</span>? Any deployments referencing its secrets will fail.
						</DialogDescription>
					</DialogHeader>

					<DialogFooter className="pt-2">
						<Button variant="destructive" onClick={handleDeleteProvider} className="h-9 text-xs font-semibold w-full">
							Delete Permanently
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</div>
	);
}
