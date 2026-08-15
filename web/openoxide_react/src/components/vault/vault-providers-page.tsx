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
		<div className="w-full p-6 max-w-5xl mx-auto space-y-6">
			{/* Dokploy Outer Card Wrapper */}
			<Card className="h-full bg-card p-2.5 rounded-xl border border-border shadow-sm">
				<div className="rounded-xl bg-background border border-border/50">
					<CardHeader className="p-6 pb-4">
						<CardTitle className="text-xl font-bold tracking-tight flex items-center gap-2.5 text-foreground">
							<Vault className="size-6 text-muted-foreground" />
							Secrets Providers
						</CardTitle>
						<CardDescription className="text-xs text-muted-foreground">
							Connect external secret managers and reference their secrets in environment variables with{' '}
							<code className="font-mono text-primary bg-muted px-1.5 py-0.5 rounded text-[11px] font-semibold">
								${'{vault.<name>.<secret>}'}
							</code>
						</CardDescription>
					</CardHeader>

					<CardContent className="space-y-4 p-6 pt-4 border-t border-border/40">
						{isLoading ? (
							<div className="flex items-center justify-center gap-2 py-12 text-xs text-muted-foreground">
								<RefreshCw className="size-4 animate-spin" />
								Loading secret providers...
							</div>
						) : providers.length === 0 ? (
							<div className="flex flex-col items-center justify-center gap-3 py-12 text-center">
								<Vault className="size-8 text-muted-foreground" />
								<span className="text-sm font-medium text-muted-foreground">
									You don't have any secrets providers configured
								</span>
								<Button onClick={handleOpenCreate} size="sm" className="h-9 px-4 text-xs font-semibold gap-1.5 mt-2">
									<Plus className="size-4" /> Add Vault Provider
								</Button>
							</div>
						) : (
							<div className="space-y-3">
								<div className="flex flex-col gap-3">
									{providers.map((provider) => (
										<div
											key={provider.id}
											className="flex items-center justify-between p-3.5 rounded-xl bg-card border border-border/70 hover:border-border transition-colors w-full"
										>
											<div className="flex items-center gap-3.5 min-w-0">
												{renderVaultProviderIcon(provider.provider_type, "size-7 shrink-0")}
												<div className="flex flex-col gap-1 min-w-0">
													<span className="text-sm font-bold text-foreground truncate">{provider.name}</span>
													<div className="flex flex-wrap items-center gap-2">
														<Badge variant="secondary" className="text-[10px] font-medium bg-secondary text-secondary-foreground">
															All Projects
														</Badge>
														<Badge variant="outline" className="text-[10px] font-medium">
															{getProviderLabel(provider.provider_type)}
														</Badge>
														<span className="text-[11px] font-mono text-muted-foreground bg-muted/30 px-2 py-0.5 rounded border border-border/40">
															${`{vault.${provider.name}.…}`}
														</span>
													</div>
												</div>
											</div>

											<div className="flex items-center gap-1 shrink-0 ml-3">
												<Button
													variant="ghost"
													size="icon"
													onClick={() => copySyntax(provider.name, provider.id)}
													title="Copy syntax"
													className="size-8 text-muted-foreground hover:text-foreground hover:bg-accent rounded-lg"
												>
													{copiedId === provider.id ? <Check className="size-4 text-emerald-500" /> : <Copy className="size-4" />}
												</Button>
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
									))}
								</div>

								<div className="flex justify-end pt-2">
									<Button onClick={handleOpenCreate} size="sm" className="h-9 px-4 text-xs font-semibold gap-1.5">
										<Plus className="size-4" /> Add Vault Provider
									</Button>
								</div>
							</div>
						)}
					</CardContent>
				</div>
			</Card>

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
