import React, { useState, useMemo } from 'react';
import { useVaultList } from 'virtual:openoxide-live';
import { $api } from '#/api/query';
import { formatApiError } from '#/api/utils';
import {
	KeyRound,
	Plus,
	Trash2,
	CheckCircle2,
	AlertCircle,
	ExternalLink,
	ShieldCheck,
	RefreshCw,
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
	DialogFooter,
} from '#/components/ui/dialog';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import { toast } from 'sonner';

export interface VaultProviderItem {
	id: number;
	name: String;
	provider_type: string;
	api_url: string;
	auth_token: string;
	namespace?: string;
	config_json?: string;
	organization_id: number;
	created_at: number;
	updated_at: number;
}

export function VaultProvidersPage() {
	const { data: rawProviders, loading: isLoading } = useVaultList();
	const createMutation = $api.useMutation('post', '/vault-providers');
	const deleteMutation = $api.useMutation('delete', '/vault-providers/{id}');
	const testMutation = $api.useMutation('post', '/vault-providers/{id}/test');

	const providers: VaultProviderItem[] = useMemo(() => {
		if (!rawProviders || !Array.isArray(rawProviders)) return [];
		return rawProviders as any;
	}, [rawProviders]);

	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [deleteTarget, setDeleteTarget] = useState<VaultProviderItem | null>(null);
	const [testingId, setTestingId] = useState<number | null>(null);
	const [copiedId, setCopiedId] = useState<number | null>(null);

	// Form State
	const [formName, setFormName] = useState('');
	const [formType, setFormType] = useState<'HASHICORP' | 'INFISICAL' | 'DOPPLER' | 'AWS' | 'AZURE' | 'SCALEWAY'>('HASHICORP');
	const [formApiUrl, setFormApiUrl] = useState('');
	const [formAuthToken, setFormAuthToken] = useState('');
	const [formNamespace, setFormNamespace] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const handleOpenCreate = () => {
		setFormName('');
		setFormType('HASHICORP');
		setFormApiUrl('');
		setFormAuthToken('');
		setFormNamespace('');
		setIsCreateOpen(true);
	};

	const handleSaveProvider = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!formName.trim() || !formApiUrl.trim() || !formAuthToken.trim()) {
			toast.error('Please fill in all required fields');
			return;
		}

		setIsSubmitting(true);
		try {
			await createMutation.mutateAsync({
				body: {
					name: formName.trim(),
					provider_type: formType,
					api_url: formApiUrl.trim(),
					auth_token: formAuthToken.trim(),
					namespace: formNamespace.trim() || undefined,
				},
			});
			toast.success(`Secrets Provider "${formName}" created successfully`);
			setIsCreateOpen(false);
		} catch (err) {
			toast.error(formatApiError(err, 'Failed to create secrets provider'));
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleDeleteProvider = async () => {
		if (!deleteTarget) return;
		try {
			await deleteMutation.mutateAsync({
				params: { path: { id: deleteTarget.id } },
			});
			toast.success(`Secrets Provider "${deleteTarget.name}" deleted`);
			setDeleteTarget(null);
		} catch (err) {
			toast.error(formatApiError(err, 'Failed to delete secrets provider'));
		}
	};

	const handleTestConnection = async (provider: VaultProviderItem) => {
		setTestingId(provider.id);
		try {
			const res: any = await testMutation.mutateAsync({
				params: { path: { id: provider.id } },
			});
			if (res.data?.success || res?.success) {
				toast.success(res.data?.message || res?.message || 'Connection verified!');
			} else {
				toast.error(res.data?.message || res?.message || 'Connection test failed');
			}
		} catch (err) {
			toast.error(formatApiError(err, 'Connection test failed'));
		} finally {
			setTestingId(null);
		}
	};

	const copyReferenceSyntax = (provider: VaultProviderItem) => {
		const syntax = `\${{vault.${provider.name}.SECRET_KEY}}`;
		navigator.clipboard.writeText(syntax);
		setCopiedId(provider.id);
		toast.success(`Copied syntax: ${syntax}`);
		setTimeout(() => setCopiedId(null), 2000);
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
		<div className="w-full p-6">
			<Card className="h-full bg-sidebar p-2.5 rounded-xl max-w-5xl mx-auto border border-border/60">
				<div className="rounded-xl bg-background shadow-md">
					<CardHeader className="flex flex-row items-center justify-between gap-4">
						<div className="space-y-1">
							<CardTitle className="text-xl flex flex-row items-center gap-2 text-foreground font-bold">
								<KeyRound className="size-6 text-primary shrink-0" />
								Secrets Providers
							</CardTitle>
							<CardDescription className="text-xs text-muted-foreground">
								Connect external secret managers and reference their secrets in environment variables with{' '}
								<code className="text-primary font-mono font-semibold bg-muted/40 px-1.5 py-0.5 rounded text-[11px]">
									{'${{vault.<name>.<secret>}}'}
								</code>
							</CardDescription>
						</div>
						<Button onClick={handleOpenCreate} size="sm" className="h-9 px-4 text-xs font-semibold gap-1.5 shrink-0">
							<Plus className="size-4" /> Add Provider
						</Button>
					</CardHeader>

					<CardContent className="space-y-4 py-6 border-t border-border/40">
						{isLoading ? (
							<div className="p-12 text-center text-xs text-muted-foreground">Loading secrets providers...</div>
						) : providers.length === 0 ? (
							<div className="flex flex-col items-center gap-3 min-h-[25vh] justify-center text-center p-8">
								<KeyRound className="size-8 text-muted-foreground/60" />
								<span className="text-sm font-medium text-muted-foreground">
									You don't have any secrets providers configured
								</span>
								<Button onClick={handleOpenCreate} size="sm" className="h-8.5 text-xs font-semibold gap-1.5 mt-1">
									<Plus className="size-3.5" /> Add Provider
								</Button>
							</div>
						) : (
							<div className="flex flex-col gap-3 min-h-[20vh]">
								{providers.map(provider => {
									const isTesting = testingId === provider.id;
									const isCopied = copiedId === provider.id;
									return (
										<div key={provider.id} className="flex items-center justify-between bg-sidebar p-1 w-full rounded-xl border border-border/60">
											<div className="flex items-center justify-between p-4 rounded-lg bg-background border border-border/60 w-full gap-4">
												<div className="flex flex-row items-center gap-3.5 min-w-0">
													<div className="size-10 rounded-lg bg-primary/10 text-primary flex items-center justify-center shrink-0 border border-primary/20">
														<KeyRound className="size-5" />
													</div>
													<div className="flex flex-col gap-1 min-w-0">
														<span className="text-sm font-bold text-foreground truncate">{provider.name}</span>
														<div className="flex flex-wrap items-center gap-2">
															<Badge variant="secondary" className="text-[11px] font-semibold">
																{getProviderLabel(provider.provider_type)}
															</Badge>
															<Badge variant="outline" className="text-[11px] font-mono font-medium text-muted-foreground">
																{`\${{vault.${provider.name}.…}}`}
															</Badge>
														</div>
													</div>
												</div>

												<div className="flex items-center gap-2 shrink-0">
													<Button
														variant="outline"
														size="sm"
														onClick={() => copyReferenceSyntax(provider)}
														className="h-8 text-xs font-medium gap-1 px-2.5"
													>
														{isCopied ? <Check className="size-3.5 text-emerald-500" /> : <Copy className="size-3.5" />}
														Copy Syntax
													</Button>
													<Button
														variant="outline"
														size="sm"
														onClick={() => handleTestConnection(provider)}
														disabled={isTesting}
														className="h-8 text-xs font-semibold gap-1 px-2.5"
													>
														{isTesting ? (
															<RefreshCw className="size-3.5 animate-spin" />
														) : (
															<ShieldCheck className="size-3.5 text-emerald-500" />
														)}
														Test API
													</Button>
													<Button
														variant="ghost"
														size="icon"
														onClick={() => setDeleteTarget(provider)}
														className="size-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-lg"
													>
														<Trash2 className="size-4" />
													</Button>
												</div>
											</div>
										</div>
									);
								})}
							</div>
						)}
					</CardContent>
				</div>
			</Card>

			{/* Create Provider Modal */}
			<Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
				<DialogContent className="sm:max-w-md bg-card border border-border shadow-2xl p-6 rounded-2xl">
					<DialogHeader className="space-y-1">
						<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
							<KeyRound className="size-5 text-primary" />
							Add Secrets Provider
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Connect HashiCorp Vault, Infisical, Doppler, AWS Secrets Manager, Azure Key Vault or Scaleway.
						</DialogDescription>
					</DialogHeader>

					<form onSubmit={handleSaveProvider} className="space-y-4 pt-2">
						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Provider Name
							</label>
							<Input
								placeholder="e.g. prod-vault"
								value={formName}
								onChange={e => setFormName(e.target.value)}
								className="h-10 text-xs bg-muted/20"
							/>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Provider Type
							</label>
							<Select value={formType} onValueChange={(v: any) => setFormType(v)}>
								<SelectTrigger className="h-10 text-xs bg-muted/20 w-full">
									<SelectValue />
								</SelectTrigger>
								<SelectContent className="w-[var(--anchor-width)]">
									<SelectItem value="HASHICORP">HashiCorp Vault</SelectItem>
									<SelectItem value="INFISICAL">Infisical Vault</SelectItem>
									<SelectItem value="DOPPLER">Doppler Secrets Manager</SelectItem>
									<SelectItem value="AWS">AWS Secrets Manager</SelectItem>
									<SelectItem value="AZURE">Azure Key Vault</SelectItem>
									<SelectItem value="SCALEWAY">Scaleway Secret Manager</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								API Base URL / Endpoint
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
								Authentication Token / Secret Key
							</label>
							<Input
								type="password"
								placeholder="Token or Secret Key"
								value={formAuthToken}
								onChange={e => setFormAuthToken(e.target.value)}
								className="h-10 text-xs font-mono bg-muted/20"
							/>
						</div>

						{formType === 'HASHICORP' && (
							<div className="space-y-1">
								<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
									Vault Namespace (Optional)
								</label>
								<Input
									placeholder="admin/production"
									value={formNamespace}
									onChange={e => setFormNamespace(e.target.value)}
									className="h-10 text-xs font-mono bg-muted/20"
								/>
							</div>
						)}

						<DialogFooter className="pt-3">
							<Button type="submit" disabled={isSubmitting} className="h-9 text-xs font-semibold gap-1.5 w-full">
								{isSubmitting && <RefreshCw className="size-3 animate-spin" />}
								Save Provider
							</Button>
						</DialogFooter>
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
