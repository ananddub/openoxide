import React, { useState, useMemo } from 'react';
import { $api } from '#/api/query';
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
	const { data: rawProviders, isLoading } = $api.useQuery('get', '/dns-providers' as any);
	const createMutation = $api.useMutation('post', '/dns-providers' as any);
	const deleteMutation = $api.useMutation('delete', '/dns-providers/{id}' as any);
	const testMutation = $api.useMutation('post', '/dns-providers/{id}/test' as any);

	const providers: DnsProviderItem[] = useMemo(() => {
		if (!rawProviders || !Array.isArray(rawProviders)) return [];
		return rawProviders as any;
	}, [rawProviders]);

	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [deleteTarget, setDeleteTarget] = useState<DnsProviderItem | null>(null);
	const [testingId, setTestingId] = useState<number | null>(null);

	// Form State
	const [formName, setFormName] = useState('');
	const [formType, setFormType] = useState<'CLOUDFLARE' | 'ROUTE53' | 'DIGITALOCEAN' | 'HETZNER'>('CLOUDFLARE');
	const [formToken, setFormToken] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const handleOpenCreate = () => {
		setFormName('');
		setFormType('CLOUDFLARE');
		setFormToken('');
		setIsCreateOpen(true);
	};

	const handleSaveProvider = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!formName.trim() || !formToken.trim()) {
			toast.error('Please fill in all required fields');
			return;
		}

		setIsSubmitting(true);
		try {
			await createMutation.mutateAsync({
				body: {
					name: formName.trim(),
					provider_type: formType,
					credentials_json: JSON.stringify(formToken.trim()),
				},
			});
			toast.success(`DNS Provider "${formName}" created successfully`);
			setIsCreateOpen(false);
		} catch (err) {
			toast.error(formatApiError(err, 'Failed to create DNS Provider'));
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
			toast.success(`DNS Provider "${deleteTarget.name}" deleted`);
			setDeleteTarget(null);
		} catch (err) {
			toast.error(formatApiError(err, 'Failed to delete DNS provider'));
		}
	};

	const handleTestConnection = async (provider: DnsProviderItem) => {
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

	const getProviderBadge = (type: string) => {
		switch (type.toUpperCase()) {
			case 'CLOUDFLARE':
				return <Badge className="bg-amber-500/10 text-amber-500 border-amber-500/30">Cloudflare</Badge>;
			case 'ROUTE53':
				return <Badge className="bg-orange-500/10 text-orange-500 border-orange-500/30">AWS Route53</Badge>;
			case 'DIGITALOCEAN':
				return <Badge className="bg-blue-500/10 text-blue-500 border-blue-500/30">DigitalOcean</Badge>;
			case 'HETZNER':
				return <Badge className="bg-rose-500/10 text-rose-500 border-rose-500/30">Hetzner DNS</Badge>;
			default:
				return <Badge variant="outline">{type}</Badge>;
		}
	};

	return (
		<div className="p-6 space-y-6 max-w-6xl mx-auto">
			{/* Page Header */}
			<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-border/40 pb-5">
				<div className="space-y-1">
					<h1 className="text-xl font-bold tracking-tight flex items-center gap-2 text-foreground">
						<Globe className="size-5 text-primary" />
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

			{/* Providers Grid */}
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
				<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
					{providers.map(provider => {
						const isTesting = testingId === provider.id;
						return (
							<Card key={provider.id} className="border border-border/70 bg-card shadow-xs rounded-xl overflow-hidden hover:border-border transition-colors">
								<CardHeader className="p-4 pb-3 flex flex-row items-start justify-between space-y-0">
									<div className="space-y-1">
										<div className="flex items-center gap-2">
											<CardTitle className="text-sm font-bold text-foreground">{provider.name}</CardTitle>
											{getProviderBadge(provider.provider_type)}
										</div>
									</div>
									<Button
										variant="ghost"
										size="icon"
										onClick={() => setDeleteTarget(provider)}
										className="size-7 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-lg"
									>
										<Trash2 className="size-3.5" />
									</Button>
								</CardHeader>

								<CardContent className="p-4 pt-0 space-y-4">
									<div className="space-y-1 text-xs">
										<div className="text-[11px] font-semibold text-muted-foreground uppercase tracking-wider">Authentication Token</div>
										<div className="font-mono text-muted-foreground bg-muted/30 px-2.5 py-1 rounded-md flex items-center gap-1.5 border border-border/40">
											<ShieldCheck className="size-3.5 text-emerald-500 shrink-0" />
											<span>••••••••••••••••</span>
										</div>
									</div>

									<div className="pt-2 border-t border-border/40 flex items-center justify-between text-[11px] text-muted-foreground">
										<span>Added {new Date(provider.created_at * 1000).toLocaleDateString()}</span>
										<Button
											size="sm"
											onClick={() => handleTestConnection(provider)}
											disabled={isTesting}
											className="h-7 text-[11px] font-semibold gap-1.5 px-3 bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 transition-all shadow-2xs"
										>
											{isTesting ? (
												<RefreshCw className="size-3 animate-spin text-emerald-400" />
											) : (
												<CheckCircle2 className="size-3 text-emerald-400" />
											)}
											Test Connection
										</Button>
									</div>
								</CardContent>
							</Card>
						);
					})}
				</div>
			)}

			{/* Create Provider Modal */}
			<Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
				<DialogContent className="sm:max-w-md bg-card border border-border shadow-2xl p-6 rounded-2xl">
					<DialogHeader className="space-y-1">
						<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
							<Globe className="size-5 text-primary" />
							Add DNS Provider
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
							<Select value={formType} onValueChange={(v: any) => setFormType(v)}>
								<SelectTrigger className="h-10 text-xs bg-muted/20 w-full">
									<SelectValue />
								</SelectTrigger>
								<SelectContent className="w-[var(--anchor-width)]">
									<SelectItem value="CLOUDFLARE">Cloudflare DNS</SelectItem>
									<SelectItem value="ROUTE53">AWS Route53</SelectItem>
									<SelectItem value="DIGITALOCEAN">DigitalOcean DNS</SelectItem>
									<SelectItem value="HETZNER">Hetzner DNS</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								API Token / Secret Key
							</label>
							<Input
								type="password"
								placeholder="API Token with Zone:DNS Edit permissions"
								value={formToken}
								onChange={e => setFormToken(e.target.value)}
								className="h-10 text-xs font-mono bg-muted/20"
							/>
						</div>

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
							Delete DNS Provider?
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Are you sure you want to remove <span className="font-semibold text-foreground">{deleteTarget?.name}</span>? Wildcard SSL renewals for linked domains will stop working.
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
