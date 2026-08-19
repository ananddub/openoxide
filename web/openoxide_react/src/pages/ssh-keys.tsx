import {useState, useCallback} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {SshKeysHeader} from '#/components/ssh-keys/ssh-keys-header';
import {SshKeysList} from '#/components/ssh-keys/ssh-keys-list';
import {CreateKeyModal} from '#/components/ssh-keys/create-key-modal';
import {ViewKeyModal} from '#/components/ssh-keys/view-key-modal';
import {DeleteKeyModal} from '#/components/ssh-keys/delete-key-modal';

import {useAppStore} from '#/stores/app-store';

import type {SshKeyResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/ssh-keys')({
	component: SshKeysPage,
});

function SshKeysPage() {
	const [isAddOpen, setIsAddOpen] = useState(false);
	const [selectedKeyForView, setSelectedKeyForView] =
		useState<SshKeyResponse | null>(null);
	const [selectedKeyForDelete, setSelectedKeyForDelete] =
		useState<SshKeyResponse | null>(null);

	const storeSshKeys = useAppStore(state => state.sshKeys);

	const sshKeys = (storeSshKeys ?? []) as unknown as SshKeyResponse[];
	const isLoading = false;

	const handleOpenAdd = useCallback(() => setIsAddOpen(true), []);
	const handleCloseAdd = useCallback(() => setIsAddOpen(false), []);

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

	return (
		<div className="mx-auto flex w-full max-w-5xl flex-col gap-6 pb-12">
			<SshKeysHeader
				onOpenAdd={handleOpenAdd}
				onRefresh={() => {}}
				isRefetching={false}
				keys={sshKeys}
			/>

			<SshKeysList
				keys={sshKeys}
				isLoading={isLoading}
				onViewKey={handleViewKey}
				onDeleteKey={handleDeleteKey}
			/>

			{isAddOpen && (
				<CreateKeyModal
					isOpen={isAddOpen}
					onClose={handleCloseAdd}
					onSuccess={() => {}}
				/>
			)}

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
