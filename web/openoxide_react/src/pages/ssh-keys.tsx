import {useCallback, useMemo, useState} from 'react';
import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {KeyRound, Shield, Sparkles} from 'lucide-react';
import {SshKeysHeader} from '#/components/ssh-keys/ssh-keys-header';
import {SshKeysList} from '#/components/ssh-keys/ssh-keys-list';
import {ViewKeyModal} from '#/components/ssh-keys/view-key-modal';
import {DeleteKeyModal} from '#/components/ssh-keys/delete-key-modal';
import {
	ResourceGuideCard,
	ResourceInfoCard,
} from '#/components/shared/resource-sidebar';

import {useSshKeyItems} from '#/api/live';

import type {SshKeyResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/ssh-keys')({
	component: SshKeysPage,
});

function SshKeysPage() {
	const navigate = useNavigate();
	const [selectedKeyForView, setSelectedKeyForView] =
		useState<SshKeyResponse | null>(null);
	const [selectedKeyForDelete, setSelectedKeyForDelete] =
		useState<SshKeyResponse | null>(null);

	const sshKeysQuery = useSshKeyItems();
	const sshKeys = (sshKeysQuery.data ?? []) as unknown as SshKeyResponse[];

	const handleOpenAdd = useCallback(
		() => void navigate({to: '/ssh-keys/new'}),
		[navigate],
	);

	const handleViewKey = useCallback(
		(key: SshKeyResponse) => setSelectedKeyForView(key),
		[],
	);
	const handleCloseView = useCallback(
		() => setSelectedKeyForView(null),
		[],
	);

	const handleDeleteKey = useCallback(
		(key: SshKeyResponse) => setSelectedKeyForDelete(key),
		[],
	);
	const handleCloseDelete = useCallback(
		() => setSelectedKeyForDelete(null),
		[],
	);

	const stats = useMemo(() => {
		const used = sshKeys.filter(k => k.last_used_at).length;
		return {total: sshKeys.length, used};
	}, [sshKeys]);

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			<SshKeysHeader
				onOpenAdd={handleOpenAdd}
				onRefresh={() => {}}
				isRefetching={false}
				keys={sshKeys}
			/>

			<div className="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_340px]">
				<div className="flex min-w-0 flex-col gap-6">
					<SshKeysList
						keys={sshKeys}
						isLoading={sshKeysQuery.isLoading}
						onViewKey={handleViewKey}
						onDeleteKey={handleDeleteKey}
					/>
				</div>

				{/* ── Right rail ── */}
				<div className="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start">
					<ResourceInfoCard
						title="Overview"
						icon={KeyRound}
						rows={[
							{icon: KeyRound, label: 'Stored keys', value: stats.total},
							{icon: Shield, label: 'Used by servers', value: stats.used},
						]}
					/>
					<ResourceGuideCard
						title="Working with keys"
						icon={Sparkles}
						steps={[
							{
								title: 'Generate safely',
								text: 'Create ED25519 or RSA pairs in the panel — the private key stays in the vault, never on disk here.',
							},
							{
								title: 'Authorize once',
								text: 'Run the shown command on the target host so it trusts the public key.',
							},
							{
								title: 'Attach & deploy',
								text: 'Pick the key when adding a remote server; setup and deployments reuse it.',
							},
						]}
					/>
				</div>
			</div>

			{selectedKeyForView && (
				<ViewKeyModal
					isOpen={!!selectedKeyForView}
					sshKey={selectedKeyForView}
					onClose={handleCloseView}
				/>
			)}

			{selectedKeyForDelete && (
				<DeleteKeyModal
					isOpen={!!selectedKeyForDelete}
					sshKey={selectedKeyForDelete}
					onClose={handleCloseDelete}
					onSuccess={() => {}}
				/>
			)}
		</div>
	);
}
