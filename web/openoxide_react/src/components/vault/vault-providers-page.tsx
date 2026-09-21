import {useState} from 'react';
import {useVaultList, useVaultProviderDelete} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {Vault, Copy, Check} from 'lucide-react';
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
	IconActionButton,
	EditIconButton,
	DeleteIconButton,
} from '#/components/shared/icon-action-button';
import {DeleteConfirmDialog} from '#/components/shared/delete-confirm-dialog';
import {
	VaultProviderIcon,
	getVaultProviderLabel,
	type VaultProviderItem,
} from './vault-provider-catalog';
import {VaultProviderFormDialog} from './vault-provider-form-dialog';

export type {VaultProviderItem};

export function VaultProvidersPage() {
	// Live vault providers query; the table-watch stream (vault_providers
	// table) invalidates it while mounted, so no zustand slice involved.
	const providersQuery = useVaultList();
	const providers = (providersQuery.data ?? []) as VaultProviderItem[];
	const deleteMutation = useVaultProviderDelete();

	const [isFormOpen, setIsFormOpen] = useState(false);
	const [editingProvider, setEditingProvider] =
		useState<VaultProviderItem | null>(null);
	const [deleteTarget, setDeleteTarget] =
		useState<VaultProviderItem | null>(null);
	const [copiedId, setCopiedId] = useState<number | null>(null);

	const handleOpenCreate = () => {
		setEditingProvider(null);
		setIsFormOpen(true);
	};

	const handleOpenEdit = (provider: VaultProviderItem) => {
		setEditingProvider(provider);
		setIsFormOpen(true);
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

	const copySyntax = (name: string, id: number) => {
		navigator.clipboard.writeText(`\${{vault.${name}.secret:FIELD}}`);
		setCopiedId(id);
		toast.success('Reference syntax copied to clipboard!');
		setTimeout(() => setCopiedId(null), 2000);
	};

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			<PageHeader
				variant="plain"
				title="Vault Providers"
				description="Connect HashiCorp Vault or Doppler and resolve secret references during build and deployment."
				actionLabel="Add Vault Provider"
				onAction={handleOpenCreate}
			/>

			{providersQuery.isLoading ? (
				<LoadingText>Loading vault providers...</LoadingText>
			) : providers.length === 0 ? (
				<EmptyStateCard
					icon={<Vault className="size-6" />}
					title="No Vault Providers Connected"
					description={
						<>
							Reference secrets in your environment variables with{' '}
							<code className="font-mono text-primary">
								${'{vault.<name>.<secret>}'}
							</code>
							.
						</>
					}
					onAction={handleOpenCreate}
				/>
			) : (
				<div className="flex flex-col gap-3">
					{providers.map(provider => (
						<ResourceRowCard
							key={provider.id}
							icon={
								<VaultProviderIcon
									type={provider.provider_type}
									className="size-7 shrink-0"
								/>
							}
							name={provider.name}
							badge={getVaultProviderLabel(provider.provider_type)}
							actions={
								<>
									<IconActionButton
										title="Copy reference syntax"
										onClick={() => copySyntax(provider.name, provider.id)}
										icon={
											copiedId === provider.id ? (
												<Check className="size-4 text-emerald-500" />
											) : (
												<Copy className="size-4" />
											)
										}
									/>
									<EditIconButton
										onClick={() => handleOpenEdit(provider)}
									/>
									<DeleteIconButton
										onClick={() => setDeleteTarget(provider)}
									/>
								</>
							}>
							<div className="flex items-center gap-2 text-[11px] text-muted-foreground">
								<Badge
									variant={
										provider.assignments?.length
											? 'secondary'
											: 'destructive'
									}>
									{provider.assignments?.length || 0} project assignments
								</Badge>
								<span>
									{provider.assignments?.length
										? 'Scoped access'
										: 'Available to all projects'}
								</span>
							</div>
							<div className="flex flex-wrap items-center gap-2 text-xs">
								<span className="rounded bg-muted/30 px-2 py-0.5 font-mono text-[11px] text-muted-foreground">
									${`{vault.${provider.name}.SECRET_KEY}`}
								</span>
								<AddedDate created_at={provider.created_at} />
							</div>
						</ResourceRowCard>
					))}
				</div>
			)}

			<VaultProviderFormDialog
				open={isFormOpen}
				onOpenChange={setIsFormOpen}
				provider={editingProvider}
			/>

			<DeleteConfirmDialog
				open={!!deleteTarget}
				onOpenChange={open => !open && setDeleteTarget(null)}
				title="Delete Secrets Provider?"
				description={
					<>
						Are you sure you want to remove{' '}
						<span className="font-semibold text-foreground">
							{deleteTarget?.name}
						</span>
						? Any deployments referencing its secrets will fail.
					</>
				}
				onConfirm={handleDeleteProvider}
			/>
		</div>
	);
}
