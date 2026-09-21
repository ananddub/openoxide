import {useState} from 'react';
import {useDnsList, useDnsProviderDelete} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {Globe} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';
import {PageHeader} from '#/components/shared/page-header';
import {
	EmptyStateCard,
	LoadingText,
} from '#/components/shared/empty-state-card';
import {
	ResourceRowCard,
	AddedDate,
} from '#/components/shared/resource-row-card';
import {
	EditIconButton,
	DeleteIconButton,
} from '#/components/shared/icon-action-button';
import {DeleteConfirmDialog} from '#/components/shared/delete-confirm-dialog';
import {
	DnsProviderIcon,
	getDnsProviderLabel,
} from './dns-provider-catalog';
import type {DnsProviderItem} from './dns-provider-catalog';
import {DnsProviderFormDialog} from './dns-provider-form-dialog';
import {DnsZonesDialog} from './dns-zones-dialog';

export type {DnsProviderItem};

export function DnsProvidersPage() {
	// Live DNS providers query; the table-watch stream (dns_providers
	// table) invalidates it while mounted, so no zustand slice involved.
	const providersQuery = useDnsList();
	const providers = (providersQuery.data ?? []) as DnsProviderItem[];
	const deleteMutation = useDnsProviderDelete();

	const [isFormOpen, setIsFormOpen] = useState(false);
	const [editingProvider, setEditingProvider] =
		useState<DnsProviderItem | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<DnsProviderItem | null>(
		null,
	);

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

	return (
		<div className="mx-auto max-w-6xl space-y-6 p-6">
			<PageHeader
				icon={<Globe className="size-5 shrink-0 text-primary" />}
				title="DNS Providers & Wildcard SSL"
				description="Connect Cloudflare DNS for automated Let's Encrypt DNS-01 SSL challenges and wildcard domains."
				actionLabel="Add DNS Provider"
				onAction={handleOpenCreate}
			/>

			{providersQuery.isLoading ? (
				<LoadingText>Loading DNS providers...</LoadingText>
			) : providers.length === 0 ? (
				<EmptyStateCard
					icon={<Globe className="size-6" />}
					title="No DNS Providers Connected"
					description="Add a DNS Provider to issue automatic Let's Encrypt Wildcard SSL certificates for all your applications."
					onAction={handleOpenCreate}
				/>
			) : (
				<div className="flex flex-col gap-3">
					{providers.map(provider => (
						<ResourceRowCard
							key={provider.id}
							icon={
								<DnsProviderIcon
									type={provider.provider_type}
									className="size-7 shrink-0"
								/>
							}
							name={provider.name}
							badge={getDnsProviderLabel(provider.provider_type)}
							actions={
								<>
									<DnsZonesDialog provider={provider} />
									<EditIconButton
										onClick={() => handleOpenEdit(provider)}
									/>
									<DeleteIconButton
										onClick={() => setDeleteTarget(provider)}
									/>
								</>
							}>
							<div className="flex flex-wrap items-center gap-2 text-xs">
								<span className="rounded bg-emerald-500/10 px-2 py-0.5 font-mono text-[11px] font-semibold text-emerald-500">
									Wildcard Ready
								</span>
								<Badge
									variant="secondary"
									className="bg-secondary text-[10px] font-medium text-secondary-foreground">
									Zones and records available via View Domains
								</Badge>
								<AddedDate created_at={provider.created_at} />
							</div>
						</ResourceRowCard>
					))}
				</div>
			)}

			<DnsProviderFormDialog
				open={isFormOpen}
				onOpenChange={setIsFormOpen}
				provider={editingProvider}
			/>

			<DeleteConfirmDialog
				open={!!deleteTarget}
				onOpenChange={open => !open && setDeleteTarget(null)}
				title="Delete DNS Provider?"
				description={
					<>
						Are you sure you want to remove{' '}
						<span className="font-semibold text-foreground">
							{deleteTarget?.name}
						</span>
						? Wildcard SSL renewals for linked domains will stop working.
					</>
				}
				onConfirm={handleDeleteProvider}
			/>
		</div>
	);
}
