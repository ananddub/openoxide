import React, { useState, useMemo } from 'react';
import { useVaultProviders } from 'virtual:openoxide-live';
import { $api, formatApiError } from '@/api/client';
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
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card';
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
	DialogDescription,
	DialogFooter,
} from '@/components/ui/dialog';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '@/components/ui/select';
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
	const { data: rawProviders, loading: isLoading } = useVaultProviders();
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
	const [formType, setFormType] = useState<'HASHICORP' | 'INFISICAL' | 'DOPPLER' | 'AWS'>('HASHICORP');
	const [formApiUrl, setFormApiUrl] = useState('');
	const [formAuthToken, setFormAuthToken] = useState('');
	const [formNamespace, setFormNamespace] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const handleOpenCreate = () => {
		setFormName('');
		setFormType('HASHICORP');
		setFormApiUrl('https://vault.example.com:8200');
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
			} as any);

			toast.success('Vault Provider added successfully');
			setIsCreateOpen(false);
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleDeleteProvider = async () => {
		if (!deleteTarget) return;
		try {
			await deleteMutation.mutateAsync({
				params: { path: { id: deleteTarget.id } },
			} as any);
			toast.success('Vault Provider deleted');
			setDeleteTarget(null);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const handleTestConnection = async (provider: VaultProviderItem) => {
		setTestingId(provider.id);
		try {
			const res: any = await testMutation.mutateAsync({
				params: { path: { id: provider.id } },
			} as any);
			if (res?.success) {
				toast.success(res.message || 'Vault connection verified!');
			} else {
				toast.error(res?.message || 'Connection test failed');
			}
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setTestingId(null);
		}
	};

	const handleCopySyntax = (provider: VaultProviderItem) => {
		const syntax = `vault://${provider.name.toLowerCase().replace(/\s+/g, '-')}/path#KEY`;
		navigator.clipboard.writeText(syntax);
		setCopiedId(provider.id);
		toast.success('Copied Vault Secret reference syntax to clipboard');
		setTimeout(() => setCopiedId(null), 2000);
	};

	return (
		<div className="p-6 max-w-7xl mx-auto space-y-6">
			<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-border/40 pb-5">
				<div>
					<h1 className="text-2xl font-bold tracking-tight text-foreground flex items-center gap-2.5">
						<KeyRound className="size-6 text-primary" />
						External Vault Providers
					</h1>
					<p className="text-xs text-muted-foreground mt-1">
						Connect HashiCorp Vault, Infisical, Doppler, or AWS Secrets Manager to inject secrets into deployments.
					</p>
				</div>

				<Button onClick={handleOpenCreate} className="h-9 text-xs font-semibold gap-2 shadow-sm">
					<Plus className="size-4" />
					Add Vault Provider
				</Button>
			</div>

			{isLoading ? (
				<div className="flex items-center justify-center p-12">
					<RefreshCw className="size-6 text-primary animate-spin" />
				</div>
			) : providers.length === 0 ? (
				<Card className="border border-dashed border-border/80 bg-muted/10 p-10 text-center rounded-2xl">
					<ShieldCheck className="size-12 text-muted-foreground mx-auto mb-3 opacity-50" />
					<h3 className="text-base font-bold text-foreground">No Vault Providers Configured</h3>
					<p className="text-xs text-muted-foreground max-w-md mx-auto mt-1 mb-5">
						Securely sync environment variables directly from HashiCorp Vault, Infisical, or Doppler.
					</p>
					<Button onClick={handleOpenCreate} variant="outline" className="h-9 text-xs font-semibold gap-2">
						<Plus className="size-4" />
						Add First Vault Provider
					</Button>
				</Card>
			) : (
				<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
					{providers.map(p => (
						<Card key={p.id} className="bg-card border border-border/60 shadow-sm hover:shadow-md transition-all rounded-xl flex flex-col justify-between">
							<CardHeader className="p-5 pb-3 space-y-2">
								<div className="flex items-center justify-between">
									<Badge variant="secondary" className="text-[10px] font-mono uppercase tracking-wider font-bold bg-primary/10 text-primary border-primary/20">
										{p.provider_type}
									</Badge>
									<Button
										variant="ghost"
										size="icon"
										onClick={() => setDeleteTarget(p)}
										className="size-7 text-muted-foreground hover:text-rose-500 hover:bg-rose-500/10 rounded-lg"
									>
										<Trash2 className="size-3.5" />
									</Button>
								</div>
								<div>
									<CardTitle className="text-base font-bold text-foreground tracking-tight">{p.name}</CardTitle>
									<CardDescription className="text-xs text-muted-foreground font-mono truncate mt-0.5" title={p.api_url}>
										{p.api_url}
									</CardDescription>
								</div>
							</CardHeader>

							<CardContent className="p-5 pt-0 space-y-3">
								{p.namespace && (
									<div className="text-[11px] text-muted-foreground font-mono flex items-center justify-between bg-muted/30 px-2.5 py-1.5 rounded-md">
										<span>Namespace / Slug:</span>
										<span className="font-semibold text-foreground">{p.namespace}</span>
									</div>
								)}

								<div className="flex items-center gap-2 pt-2 border-t border-border/40">
									<Button
										variant="outline"
										size="sm"
										onClick={() => handleTestConnection(p)}
										disabled={testingId === p.id}
										className="h-8 text-xs font-semibold flex-1 gap-1.5"
									>
										{testingId === p.id ? (
											<RefreshCw className="size-3 animate-spin text-primary" />
										) : (
											<ExternalLink className="size-3 text-muted-foreground" />
										)}
										Test Connection
									</Button>

									<Button
										variant="secondary"
										size="sm"
										onClick={() => handleCopySyntax(p)}
										className="h-8 text-xs font-semibold gap-1.5"
										title="Copy Secret Syntax"
									>
										{copiedId === p.id ? <Check className="size-3 text-emerald-500" /> : <Copy className="size-3 text-muted-foreground" />}
										Copy Ref
									</Button>
								</div>
							</CardContent>
						</Card>
					))}
				</div>
			)}

			{/* Create Vault Provider Modal */}
			<Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
				<DialogContent className="sm:max-w-md bg-card border border-border shadow-2xl p-6 rounded-2xl">
					<DialogHeader className="space-y-1">
						<DialogTitle className="text-lg font-bold tracking-tight flex items-center gap-2">
							<KeyRound className="size-5 text-primary" />
							Add Vault Provider
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Connect an external Secret Manager API to fetch production variables.
						</DialogDescription>
					</DialogHeader>

					<form onSubmit={handleSaveProvider} className="space-y-4 pt-2">
						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Provider Name <span className="text-destructive">*</span>
							</label>
							<Input
								placeholder="e.g. Production Vault"
								value={formName}
								onChange={e => setFormName(e.target.value)}
								className="h-10 text-xs bg-muted/20"
								autoFocus
							/>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Provider Engine
							</label>
							<Select value={formType} onValueChange={(v: any) => setFormType(v)}>
								<SelectTrigger className="h-10 text-xs bg-muted/20">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="HASHICORP">HashiCorp Vault</SelectItem>
									<SelectItem value="INFISICAL">Infisical Vault</SelectItem>
									<SelectItem value="DOPPLER">Doppler Secrets Manager</SelectItem>
									<SelectItem value="AWS">AWS Secrets Manager</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								API Base URL <span className="text-destructive">*</span>
							</label>
							<Input
								placeholder="https://vault.example.com:8200"
								value={formApiUrl}
								onChange={e => setFormApiUrl(e.target.value)}
								className="h-10 text-xs font-mono bg-muted/20"
							/>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								API Token / Secret Key <span className="text-destructive">*</span>
							</label>
							<Input
								type="password"
								placeholder="hvs.xxxxxxxxxxxxxxxxxxxx"
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

						<DialogFooter className="pt-3 gap-2">
							<Button type="button" variant="outline" onClick={() => setIsCreateOpen(false)} className="h-9 text-xs">
								Cancel
							</Button>
							<Button type="submit" disabled={isSubmitting} className="h-9 text-xs font-semibold gap-1.5">
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
							Delete Vault Provider?
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Are you sure you want to remove <span className="font-semibold text-foreground">{deleteTarget?.name}</span>? Any deployments relying on its secrets will fail.
						</DialogDescription>
					</DialogHeader>

					<DialogFooter className="pt-2 gap-2">
						<Button variant="outline" onClick={() => setDeleteTarget(null)} className="h-9 text-xs">
							Cancel
						</Button>
						<Button variant="destructive" onClick={handleDeleteProvider} className="h-9 text-xs font-semibold">
							Delete Permanently
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</div>
	);
}
