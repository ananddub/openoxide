import {useMemo, useState} from 'react';
import {Globe, LayoutGrid, List, Plus} from 'lucide-react';
import {toast} from 'sonner';
import {cn, formatApiError} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	ListEmpty,
	SectionHeader,
	Toolbar,
	ToolbarSearch,
} from '#/components/settings/list-card';
import {ConfirmDeleteDialog} from '#/components/settings/confirm-delete-dialog';
import {useDnsList, useDnsProviderDelete} from '#/api/live';
import type {SshKeysView} from '#/components/ssh-keys/ssh-keys-list';
import {
	dnsProviderOptions,
	getDnsProviderLabel,
	type DnsProviderItem,
} from './dns-provider-catalog';
import {DnsProviderFormDialog} from './dns-provider-form-dialog';
import {DnsProvidersList} from './dns-providers-list';
import {DnsZonesDialog} from './dns-zones-dialog';

export type {DnsProviderItem};

const VIEW_STORAGE_KEY = 'dns-providers-view';

/**
 * DNS providers — the SSH Keys / Projects grammar: a section title with the
 * primary action, then a toolbar (search + provider filter + grid/list
 * toggle) and grid cards or one bordered list card below; create/edit opens
 * a Geist sheet, delete a quiet AlertDialog.
 */
export function DnsProvidersPage() {
	// Live DNS providers query; the table-watch stream (dns_providers
	// table) invalidates it while mounted, so no zustand slice involved.
	const providersQuery = useDnsList();
	const providers = (providersQuery.data ?? []) as DnsProviderItem[];
	const deleteMutation = useDnsProviderDelete();

	const [search, setSearch] = useState('');
	const [typeFilter, setTypeFilter] = useState('All');
	const [view, setView] = useState<SshKeysView>(() =>
		typeof window !== 'undefined' &&
		localStorage.getItem(VIEW_STORAGE_KEY) === 'list'
			? 'list'
			: 'grid',
	);
	const [isFormOpen, setIsFormOpen] = useState(false);
	const [editingProvider, setEditingProvider] =
		useState<DnsProviderItem | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<DnsProviderItem | null>(
		null,
	);
	const [zonesProvider, setZonesProvider] =
		useState<DnsProviderItem | null>(null);

	const q = search.trim().toLowerCase();
	const filtered = useMemo(
		() =>
			providers.filter(provider => {
				if (
					typeFilter !== 'All' &&
					provider.provider_type.toUpperCase() !== typeFilter
				)
					return false;
				if (!q) return true;
				return [provider.name, getDnsProviderLabel(provider.provider_type)]
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

	const handleViewChange = (next: SshKeysView) => {
		setView(next);
		localStorage.setItem(VIEW_STORAGE_KEY, next);
	};

	const handleOpenCreate = () => {
		setEditingProvider(null);
		setIsFormOpen(true);
	};

	const handleOpenEdit = (provider: DnsProviderItem) => {
		setEditingProvider(provider);
		setIsFormOpen(true);
	};

	const handleDeleteProvider = async () => {
		if (!deleteTarget) return;
		try {
			await deleteMutation.mutateAsync({id: deleteTarget.id});
			toast.success(`DNS Provider "${deleteTarget.name}" deleted`);
			setDeleteTarget(null);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const createButton = (
		<Button onClick={handleOpenCreate} className="h-9 gap-1.5">
			<Plus className="size-3.5" />
			Add DNS Provider
		</Button>
	);

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="DNS Providers"
				description="Provider credentials used to read zones and issue Let's Encrypt wildcard certificates."
				action={createButton}
			/>

			{/* Search + provider filter + grid/list toggle */}
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
										: getDnsProviderLabel(typeFilter)}
								</SelectValue>
							</SelectTrigger>
							<SelectContent className="bg-card">
								<SelectItem value="All">All Providers</SelectItem>
								{dnsProviderOptions.map(option => (
									<SelectItem key={option.value} value={option.value}>
										{option.label}
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
					placeholder="Search DNS providers…"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			{providersQuery.isLoading || providers.length > 0 ? (
				<DnsProvidersList
					providers={filtered}
					isLoading={providersQuery.isLoading}
					hasFilters={hasFilters}
					view={view}
					onClearFilters={clearFilters}
					onDomains={setZonesProvider}
					onEdit={handleOpenEdit}
					onDelete={setDeleteTarget}
				/>
			) : (
				<ListEmpty
					icon={Globe}
					title="No DNS Providers Connected"
					description="Add a DNS provider to read its zones and issue Let's Encrypt wildcard certificates for your applications."
					action={createButton}
				/>
			)}

			<DnsProviderFormDialog
				open={isFormOpen}
				onOpenChange={setIsFormOpen}
				provider={editingProvider}
			/>

			{zonesProvider && (
				<DnsZonesDialog
					provider={zonesProvider}
					onClose={() => setZonesProvider(null)}
				/>
			)}

			<ConfirmDeleteDialog
				open={!!deleteTarget}
				onOpenChange={open => !open && setDeleteTarget(null)}
				title="Delete DNS provider"
				description={
					<span>
						<span className="font-medium text-foreground">
							{deleteTarget?.name}
						</span>{' '}
						will stop serving zones — wildcard certificate renewals for
						domains that rely on it fail until it is repointed. This cannot
						be undone.
					</span>
				}
				confirmLabel="Delete provider"
				busy={deleteMutation.isPending}
				onConfirm={() => void handleDeleteProvider()}
			/>
		</div>
	);
}
