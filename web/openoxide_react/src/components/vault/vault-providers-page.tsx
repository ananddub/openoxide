import {useMemo, useState} from 'react';
import {KeyRound, LayoutGrid, List, Plus} from 'lucide-react';
import {toast} from 'sonner';
import {formatApiError, cn} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ListEmpty,
} from '#/components/settings/list-card';
import {ConfirmDeleteDialog} from '#/components/settings/confirm-delete-dialog';
import {
	useVaultList,
	useVaultProviderDelete,
	useVaultProviderTest,
} from '#/api/live';
import {
	getVaultProviderLabel,
	vaultProviderOptions,
	vaultReferenceSyntax,
	type VaultProviderItem,
} from './vault-provider-catalog';
import {VaultProviderFormDialog} from './vault-provider-form-dialog';
import {
	VaultProvidersList,
	type VaultProvidersView,
} from './vault-providers-list';
import {VaultSecretsSheet} from './vault-secrets-sheet';

export type {VaultProviderItem};

const VIEW_STORAGE_KEY = 'vault-providers-view';

/**
 * Vault providers — the SSH Keys / Projects grammar: toolbar (search +
 * provider filter + view switch + New) above a bordered list card or grid
 * cards, create/edit in a right sheet, delete a quiet AlertDialog.
 */
export function VaultProvidersPage() {
	// Live vault providers query; the table-watch stream (vault_providers
	// table) invalidates it while mounted, so no zustand slice involved.
	const providersQuery = useVaultList();
	const providers = (providersQuery.data ?? []) as VaultProviderItem[];
	const deleteMutation = useVaultProviderDelete();
	const testMutation = useVaultProviderTest();

	const [search, setSearch] = useState('');
	const [typeFilter, setTypeFilter] = useState('All');
	const [view, setView] = useState<VaultProvidersView>(() =>
		typeof window !== 'undefined' &&
		localStorage.getItem(VIEW_STORAGE_KEY) === 'list'
			? 'list'
			: 'grid',
	);
	const [isFormOpen, setIsFormOpen] = useState(false);
	const [editingProvider, setEditingProvider] =
		useState<VaultProviderItem | null>(null);
	const [deleteTarget, setDeleteTarget] =
		useState<VaultProviderItem | null>(null);
	const [secretsProvider, setSecretsProvider] =
		useState<VaultProviderItem | null>(null);
	const [copiedId, setCopiedId] = useState<number | null>(null);
	const [testingId, setTestingId] = useState<number | null>(null);

	const q = search.trim().toLowerCase();
	const filtered = useMemo(
		() =>
			providers.filter(provider => {
				if (typeFilter !== 'All' && provider.provider_type !== typeFilter)
					return false;
				if (!q) return true;
				return [provider.name, provider.api_url, provider.namespace]
					.filter(Boolean)
					.join(' ')
					.toLowerCase()
					.includes(q);
			}),
		[providers, q, typeFilter],
	);

	const hasFilters = q !== '' || typeFilter !== 'All';
	const clearFilters = () => {
		setSearch('');
		setTypeFilter('All');
	};

	const handleViewChange = (next: VaultProvidersView) => {
		setView(next);
		localStorage.setItem(VIEW_STORAGE_KEY, next);
	};

	const handleOpenCreate = () => {
		setEditingProvider(null);
		setIsFormOpen(true);
	};

	const handleOpenEdit = (provider: VaultProviderItem) => {
		setEditingProvider(provider);
		setIsFormOpen(true);
	};

	const handleCopySyntax = (provider: VaultProviderItem) => {
		navigator.clipboard.writeText(vaultReferenceSyntax(provider.name));
		setCopiedId(provider.id);
		toast.success('Reference syntax copied to clipboard!');
		setTimeout(() => setCopiedId(null), 2000);
	};

	const handleTest = async (provider: VaultProviderItem) => {
		setTestingId(provider.id);
		try {
			const result = await testMutation.mutateAsync({id: provider.id});
			if (result?.success)
				toast.success(result.message || `${provider.name} reachable`);
			else toast.error(result?.message || 'Connection test failed');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setTestingId(null);
		}
	};

	const handleDeleteProvider = async () => {
		if (!deleteTarget) return;
		try {
			await deleteMutation.mutateAsync({id: deleteTarget.id});
			toast.success(`Vault Provider "${deleteTarget.name}" deleted`);
			setDeleteTarget(null);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const createButton = (
		<Button onClick={handleOpenCreate} className="h-9 gap-1.5">
			<Plus className="size-3.5" /> Add Vault Provider
		</Button>
	);

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="Vault Providers"
				description="Connect an external secret store and reference its values as environment variables."
				action={createButton}
			/>

			<Toolbar
				action={
					<>
						<Select
							value={typeFilter}
							onValueChange={value => value && setTypeFilter(value)}>
							<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
								<SelectValue>
									{typeFilter === 'All'
										? 'All Providers'
										: getVaultProviderLabel(typeFilter)}
								</SelectValue>
							</SelectTrigger>
							<SelectContent className="bg-card">
								<SelectItem value="All">All Providers</SelectItem>
								{vaultProviderOptions.map(option => (
									<SelectItem key={option.value} value={option.value}>
										{getVaultProviderLabel(option.value)}
									</SelectItem>
								))}
							</SelectContent>
						</Select>

						<div className="flex items-center rounded-lg border bg-card p-1">
							<button
								type="button"
								aria-label="Grid view"
								title="Grid view"
								onClick={() => handleViewChange('grid')}
								className={cn(
									'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground',
									view === 'grid' && 'bg-accent text-foreground',
								)}>
								<LayoutGrid className="size-3.5" />
							</button>
							<button
								type="button"
								aria-label="List view"
								title="List view"
								onClick={() => handleViewChange('list')}
								className={cn(
									'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground',
									view === 'list' && 'bg-accent text-foreground',
								)}>
								<List className="size-3.5" />
							</button>
						</div>
					</>
				}>
				<ToolbarSearch
					value={search}
					onChange={setSearch}
					placeholder="Search vault providers…"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			{providersQuery.isLoading || providers.length > 0 ? (
				<VaultProvidersList
					providers={filtered}
					isLoading={providersQuery.isLoading}
					hasFilters={hasFilters}
					view={view}
					copiedId={copiedId}
					testingId={testingId}
					onClearFilters={clearFilters}
					onSecrets={setSecretsProvider}
					onTest={provider => void handleTest(provider)}
					onCopy={handleCopySyntax}
					onEdit={handleOpenEdit}
					onDelete={setDeleteTarget}
				/>
			) : (
				<ListEmpty
					icon={KeyRound}
					title="No Vault Providers Connected"
					description="Connect HashiCorp Vault, Doppler, Infisical, AWS, Azure or Scaleway and reference secrets as environment variables."
					action={createButton}
				/>
			)}

			<VaultProviderFormDialog
				open={isFormOpen}
				onOpenChange={setIsFormOpen}
				provider={editingProvider}
			/>

			<VaultSecretsSheet
				provider={secretsProvider}
				onClose={() => setSecretsProvider(null)}
			/>

			<ConfirmDeleteDialog
				open={!!deleteTarget}
				onOpenChange={open => !open && setDeleteTarget(null)}
				title="Delete vault provider"
				description={
					<span>
						<span className="font-medium text-foreground">
							{deleteTarget?.name}
						</span>{' '}
						will stop resolving secrets — any deployment referencing{' '}
						<code className="font-mono">
							{'${{vault.<name>.<secret>}}'}
						</code>{' '}
						fails until it is repointed. This cannot be undone.
					</span>
				}
				confirmLabel="Delete provider"
				busy={deleteMutation.isPending}
				onConfirm={() => void handleDeleteProvider()}
			/>
		</div>
	);
}
