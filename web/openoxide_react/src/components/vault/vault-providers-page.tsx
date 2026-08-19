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
import { Textarea } from '#/components/ui/textarea';
import { useProjectsList } from '#/hooks/projects/use-projects-list';
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
import {
	HashicorpVaultIcon,
	DopplerIcon,
} from '#/components/icons/provider-icons';
import { useAppStore } from '#/stores/app-store';

export interface VaultProviderItem {
	id: number;
	name: string;
	provider_type: string;
	credentials_json: string;
	organization_id: number;
	created_at: number;
	updated_at: number;
	assignments?: Array<{ project_id: number; environment_ids: number[] }>;
}

export function VaultProvidersPage() {
	const queryClient = useQueryClient();
	const storeProviders = useAppStore((state) => state.vaultProviders);
	const isVaultLoading = useAppStore((state) => state.isVaultLoading);
	const addVaultStore = useAppStore((state) => state.addVaultProvider);
	const updateVaultStore = useAppStore((state) => state.updateVaultProvider);
	const deleteVaultStore = useAppStore((state) => state.deleteVaultProvider);

	const createMutation = $api.useMutation('post', '/vault-providers' as any);
	const updateMutation = $api.useMutation('put', '/vault-providers/{id}' as any);
	const deleteMutation = $api.useMutation('delete', '/vault-providers/{id}' as any);
	const testMutation = $api.useMutation('post', '/vault-providers/{id}/test' as any);

	const providers = storeProviders;
	const isLoading = isVaultLoading && providers.length === 0;

	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [editingProvider, setEditingProvider] = useState<any | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<VaultProviderItem | null>(null);
	const [isTestingModal, setIsTestingModal] = useState(false);
	const [copiedId, setCopiedId] = useState<number | null>(null);

	// Form State
	const [formName, setFormName] = useState('');
	const [formType, setFormType] = useState<
		'HASHICORP' | 'INFISICAL' | 'DOPPLER' | 'AWS' | 'SCALEWAY' | 'AZURE'
	>('HASHICORP');
	const [formUrl, setFormUrl] = useState('');
	const [formMount, setFormMount] = useState('secret');
	const [formToken, setFormToken] = useState('');
	const [formNamespace, setFormNamespace] = useState('');
	const [formConfigJson, setFormConfigJson] = useState('');
	const [formFields, setFormFields] = useState<Record<string, string>>({});
	const [formAssignments, setFormAssignments] = useState('[]');
	const [assignmentProject, setAssignmentProject] = useState('');
	const { projects } = useProjectsList();
	const environmentsQuery = $api.useQuery('get', '/environments/project/{project_id}' as any, {
		params: { path: { project_id: Number(assignmentProject) || 0 } },
		enabled: !!assignmentProject,
	} as any);
	const [isSubmitting, setIsSubmitting] = useState(false);

	const handleOpenCreate = () => {
		setEditingProvider(null);
		setFormName('');
		setFormType('HASHICORP');
		setFormUrl('');
		setFormMount('secret');
		setFormToken('');
		setFormNamespace('');
		setFormConfigJson('');
		setFormFields({});
		setFormAssignments('[]');
		setIsCreateOpen(true);
	};

	const handleOpenEdit = (provider: any) => {
		setEditingProvider(provider);
		setFormName(provider.name);
		setFormType((provider.provider_type || 'HASHICORP').toUpperCase() as any);
		setFormUrl(provider.api_url || '');
		setFormToken('');
		setFormNamespace(provider.namespace || '');
		setFormMount('secret');
		setFormConfigJson(provider.config_json || '');
		try { setFormFields(JSON.parse(provider.config_json || '{}')); } catch { setFormFields({}); }
		setFormAssignments(JSON.stringify(provider.assignments || [], null, 2));
		setIsCreateOpen(true);
	};

	const field = (name: string) => formFields[name] || '';
	const setField = (name: string, value: string) => setFormFields(prev => ({ ...prev, [name]: value }));
	const providerConfig = () => {
		if (formType === 'HASHICORP') return { mount: formMount.trim() || 'secret' };
		if (formType === 'DOPPLER') return { project: field('project') || undefined, config: field('config') || undefined };
		if (formType === 'INFISICAL') return { siteUrl: field('siteUrl') || 'https://app.infisical.com', clientId: field('clientId'), clientSecret: field('clientSecret'), projectId: field('projectId'), environmentSlug: field('environmentSlug'), secretPath: field('secretPath') || '/' };
		if (formType === 'AWS') return { region: field('region'), accessKeyId: field('accessKeyId'), secretAccessKey: field('secretAccessKey'), endpoint: field('endpoint') || undefined };
		if (formType === 'AZURE') return { vaultUri: field('vaultUri'), tenantId: field('tenantId'), clientId: field('clientId'), clientSecret: field('clientSecret') };
		return { region: field('region') || 'fr-par', projectId: field('projectId'), secretKey: field('secretKey'), apiUrl: field('apiUrl') || 'https://api.scaleway.com' };
	};
	const parsedAssignments = () => {
		try {
			const value = JSON.parse(formAssignments);
			if (!Array.isArray(value)) throw new Error();
			return value;
		} catch {
			throw new Error('Assignments must be a JSON array, for example [{"project_id":1,"environment_ids":[2]}]');
		}
	};
	const assignmentRows = (() => { try { const value = JSON.parse(formAssignments); return Array.isArray(value) ? value : []; } catch { return []; } })();
	const toggleAssignment = (projectId: number, environmentId?: number) => {
		const rows = assignmentRows.map((row: any) => ({ project_id: Number(row.project_id), environment_ids: (row.environment_ids || []).map(Number) }));
		let row = rows.find((item: any) => item.project_id === projectId);
		if (!row) { row = { project_id: projectId, environment_ids: [] }; rows.push(row); }
		if (environmentId) {
			row.environment_ids = row.environment_ids.includes(environmentId) ? row.environment_ids.filter((id: number) => id !== environmentId) : [...row.environment_ids, environmentId];
		}
		setFormAssignments(JSON.stringify(rows));
	};
	const renderProviderFields = () => {
		const definitions: Record<string, Array<[string, string, string]>> = {
			INFISICAL: [['siteUrl', 'Site URL', 'https://app.infisical.com'], ['clientId', 'Client ID', 'Universal Auth client ID'], ['clientSecret', 'Client Secret', 'Universal Auth client secret'], ['projectId', 'Project ID', 'Infisical project ID'], ['environmentSlug', 'Environment', 'prod'], ['secretPath', 'Secret Path', '/']],
			AWS: [['region', 'Region', 'us-east-1'], ['accessKeyId', 'Access Key ID', 'AKIA...'], ['secretAccessKey', 'Secret Access Key', 'Secret access key'], ['endpoint', 'Endpoint (optional)', 'https://...']],
			DOPPLER: [['project', 'Project (optional)', 'Doppler project'], ['config', 'Config (optional)', 'Doppler config']],
			AZURE: [['vaultUri', 'Vault URI', 'https://my-vault.vault.azure.net'], ['tenantId', 'Tenant ID', 'Azure tenant ID'], ['clientId', 'Client ID', 'Application client ID'], ['clientSecret', 'Client Secret', 'Application client secret']],
			SCALEWAY: [['region', 'Region', 'fr-par'], ['projectId', 'Project ID', 'Scaleway project ID'], ['secretKey', 'Secret Key', 'Scaleway secret key'], ['apiUrl', 'API URL (optional)', 'https://api.scaleway.com']],
		};
		return (definitions[formType] || []).map(([name, label, placeholder]) => <div key={name} className="space-y-1"><label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">{label}</label><Input type={name.toLowerCase().includes('secret') || name === 'secretKey' ? 'password' : 'text'} placeholder={placeholder} value={field(name)} onChange={e => setField(name, e.target.value)} className="h-10 text-xs font-mono bg-muted/20" /></div>);
	};

	const handleSaveProvider = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!formName.trim() || (!editingProvider && ['HASHICORP', 'DOPPLER'].includes(formType) && !formToken.trim())) {
			toast.error('Please fill in all required fields');
			return;
		}

		setIsSubmitting(true);
		try {
			const assignments = parsedAssignments();
			if (editingProvider) {
				const updatePayload: any = {
					name: formName.trim(),
					api_url: formUrl.trim() || undefined,
					...(formToken.trim() ? { auth_token: formToken.trim() } : {}),
					namespace: formNamespace.trim() || undefined,
					config_json: JSON.stringify(providerConfig()),
					assignments,
				};

				// Optimistic Zustand update
				updateVaultStore(editingProvider.id, updatePayload);

				await updateMutation.mutateAsync({
					params: { path: { id: editingProvider.id } },
					body: updatePayload,
				});
				toast.success(`Vault Provider "${formName}" updated`);
			} else {
				const defaultApiUrl =
					formUrl.trim() ||
						(formType === 'DOPPLER'
							? 'https://api.doppler.com'
							: 'http://localhost:8200');

				const createPayload: any = {
					name: formName.trim(),
					provider_type: formType,
					api_url: defaultApiUrl,
					auth_token: formToken.trim(),
					namespace: formNamespace.trim() || undefined,
					config_json: JSON.stringify(providerConfig()),
					assignments,
				};

				const res: any = await createMutation.mutateAsync({
					body: createPayload,
				});

				// Optimistic Zustand add
				if (res.data || res) {
					addVaultStore(res.data || res);
				}

				toast.success(`Vault Provider "${formName}" created successfully`);
			}
			setIsCreateOpen(false);
			queryClient.invalidateQueries({ queryKey: ['get', '/vault-providers'] });
		} catch (err) {
			toast.error(formatApiError(err, 'Failed to save vault provider'));
		} finally {
			setIsSubmitting(false);
		}
	};

	const testConnectionMutation = $api.useMutation('post', '/vault-providers/test-connection' as any);

	const handleTestModalConnection = async () => {
		if (!editingProvider && ['HASHICORP', 'DOPPLER'].includes(formType) && !formToken.trim()) {
			toast.error('Please enter a Token / Access Key to test connection');
			return;
		}

		setIsTestingModal(true);
		try {
			if (editingProvider) {
				const res: any = await testMutation.mutateAsync({
					params: { path: { id: editingProvider.id } },
				});
				const result = res.data || res;
				if (result?.success) {
					toast.success(result.message || 'Connection successful!');
				} else {
					toast.error(result?.message || 'Connection failed!');
				}
			} else {
				// Call Rust backend proxy to bypass CORS and run live HTTP ping
				const res: any = await testConnectionMutation.mutateAsync({
					body: {
						name: formName.trim() || 'Test',
						provider_type: formType,
						api_url: formUrl.trim(),
											auth_token: formToken.trim(),
										namespace: formNamespace.trim() || undefined,
						config_json: JSON.stringify(providerConfig()),
					},
				});
				const result = res.data || res;
				if (result?.success) {
					toast.success(result.message || 'Vault connection verified successfully!');
				} else {
					toast.error(result?.message || 'Vault connection test failed!');
				}
			}
		} catch (err: any) {
			toast.error(err?.message || formatApiError(err, 'Connection test failed'));
		} finally {
			setIsTestingModal(false);
		}
	};

	const handleDeleteProvider = async () => {
		if (!deleteTarget) return;
		const targetId = deleteTarget.id;
		deleteVaultStore(targetId);
		try {
			await deleteMutation.mutateAsync({
				params: { path: { id: targetId } },
			});
			toast.success(`Vault Provider "${deleteTarget.name}" deleted`);
			setDeleteTarget(null);
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
			case 'DOPPLER':
				return <DopplerIcon className={`${className} text-purple-500`} />;
			default:
				return <KeyRound className={`${className} text-primary`} />;
		}
	};

	const getProviderLabel = (type: string) => {
			switch (type.toUpperCase()) {
			case 'HASHICORP':
				return 'HashiCorp Vault';
			case 'DOPPLER':
				return 'Doppler';
			case 'INFISICAL':
				return 'Infisical';
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
						Connect HashiCorp Vault or Doppler and resolve secret references during build and deployment.
					</p>
				</div>
				<Button onClick={handleOpenCreate} size="sm" className="h-9 px-4 text-xs font-semibold gap-1.5 shrink-0">
					<Plus className="size-4" /> Add Vault Provider
				</Button>
			</div>

			{/* Providers List (Horizontal Card Rows) */}
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
				<div className="flex flex-col gap-3">
					{providers.map(provider => {
						const isCopied = copiedId === provider.id;
						return (
							<Card
								key={provider.id}
								className="border border-border/70 bg-card p-4 rounded-xl shadow-2xs hover:border-border transition-all flex flex-col sm:flex-row sm:items-center justify-between gap-4"
							>
								<div className="flex items-center gap-3.5 min-w-0">
									{renderVaultProviderIcon(provider.provider_type, "size-7 shrink-0")}
									<div className="flex flex-col gap-1 min-w-0">
										<div className="flex items-center gap-2.5">
											<span className="text-sm font-bold text-foreground truncate">{provider.name}</span>
											<Badge variant="outline" className="text-[10px] font-mono shrink-0">
												{getProviderLabel(provider.provider_type)}
											</Badge>
										</div>
										<div className="flex items-center gap-2 text-[11px] text-muted-foreground">
											<Badge variant={provider.assignments?.length ? 'secondary' : 'destructive'}>{provider.assignments?.length || 0} project assignments</Badge>
											<span>{provider.assignments?.length ? 'Scoped access' : 'Available to all projects'}</span>
										</div>
										<div className="flex flex-wrap items-center gap-2 text-xs">
											<span className="font-mono text-muted-foreground bg-muted/30 px-2 py-0.5 rounded border border-border/40 text-[11px]">
												${`{vault.${provider.name}.SECRET_KEY}`}
											</span>
											<span className="text-[11px] text-muted-foreground">
												• Added {new Date(provider.created_at * 1000).toLocaleDateString()}
											</span>
										</div>
									</div>
								</div>

								<div className="flex items-center gap-1 shrink-0 self-end sm:self-center">
									<Button
										variant="ghost"
										size="icon"
										onClick={() => copySyntax(provider.name, provider.id)}
										title="Copy reference syntax"
										className="size-8 text-muted-foreground hover:text-foreground hover:bg-accent rounded-lg"
									>
										{isCopied ? <Check className="size-4 text-emerald-500" /> : <Copy className="size-4" />}
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
									<SelectItem value="DOPPLER">
										<div className="flex items-center gap-2">
											<DopplerIcon className="size-4 shrink-0 text-purple-500" />
											Doppler
										</div>
									</SelectItem>
					{(['INFISICAL','AWS','AZURE','SCALEWAY'] as const).map(type => <SelectItem key={type} value={type}>{getProviderLabel(type)}</SelectItem>)}
								</SelectContent>
							</Select>
						</div>

						{formType === 'HASHICORP' && <div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Vault URL
							</label>
							<Input
								placeholder="https://vault.example.com:8200"
								value={formUrl}
								onChange={e => setFormUrl(e.target.value)}
								className="h-10 text-xs bg-muted/20"
							/>
						</div>}

						<div className="space-y-1">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								{formType === 'DOPPLER' ? 'Service Token' : formType === 'HASHICORP' ? 'Vault Token' : 'Authentication Secret'}
							</label>
							<Input
								type="password"
								placeholder={editingProvider ? 'Leave blank to keep existing token' : 'Token'}
								value={formToken}
								onChange={e => setFormToken(e.target.value)}
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
						{formType !== 'HASHICORP' && <div className="grid gap-3">{renderProviderFields()}</div>}

						<div className="space-y-2 rounded-lg border border-border/50 bg-muted/10 p-3">
							<div className="flex items-center justify-between"><label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">Project Assignments</label><Badge variant="outline">{assignmentRows.length} selected</Badge></div>
							<Select value={assignmentProject} onValueChange={value => { setAssignmentProject(value); toggleAssignment(Number(value)); }}>
								<SelectTrigger className="h-9 text-xs"><SelectValue placeholder="Assign to a project" /></SelectTrigger>
								<SelectContent>{(projects as any[]).map(project => <SelectItem key={project.id} value={String(project.id)}>{project.name}</SelectItem>)}</SelectContent>
							</Select>
							{assignmentProject && <div className="space-y-1"><p className="text-[11px] text-muted-foreground">Select environments (empty means every environment in this project).</p>{environmentsQuery.isLoading ? <p className="text-xs text-muted-foreground">Loading environments...</p> : ((environmentsQuery.data?.data || environmentsQuery.data || []) as any[]).map(environment => { const row = assignmentRows.find((item: any) => Number(item.project_id) === Number(assignmentProject)); const checked = !!row?.environment_ids?.map(Number).includes(Number(environment.id)); return <label key={environment.id} className="flex items-center gap-2 text-xs"><input type="checkbox" checked={checked} onChange={() => toggleAssignment(Number(assignmentProject), Number(environment.id))} />{environment.name}</label>; })}</div>}
							{assignmentRows.length > 0 && <div className="flex flex-wrap gap-1">{assignmentRows.map((row: any) => <Badge key={row.project_id} variant="secondary">Project {row.project_id}{row.environment_ids?.length ? ` (${row.environment_ids.length} env)` : ' (all envs)'}</Badge>)}</div>}
							<p className="text-[11px] text-muted-foreground">No assignments means this provider is available to all projects.</p>
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
