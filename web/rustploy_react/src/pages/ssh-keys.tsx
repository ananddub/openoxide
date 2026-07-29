import {useState, useCallback} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {SshKeysHeader} from '#/components/ssh-keys/ssh-keys-header';
import {SshKeysList} from '#/components/ssh-keys/ssh-keys-list';
import {CreateKeyModal} from '#/components/ssh-keys/create-key-modal';
import {ViewKeyModal} from '#/components/ssh-keys/view-key-modal';
import {EditKeyModal} from '#/components/ssh-keys/edit-key-modal';
import {DeleteKeyModal} from '#/components/ssh-keys/delete-key-modal';

import type {SshKeyResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/ssh-keys')({
	component: SshKeysPage,
});

function SshKeysPage() {
	const [isAddOpen, setIsAddOpen] = useState(false);
	const [selectedKeyForView, setSelectedKeyForView] = useState<SshKeyResponse | null>(null);
	const [selectedKeyForEdit, setSelectedKeyForEdit] = useState<SshKeyResponse | null>(null);
	const [selectedKeyForDelete, setSelectedKeyForDelete] = useState<SshKeyResponse | null>(null);

	const {
		data: rawSshKeys = [],
		isLoading,
		refetch,
		isRefetching,
	} = $api.useQuery('get', '/ssh-keys');

	const sshKeys = Array.isArray(rawSshKeys) ? (rawSshKeys as SshKeyResponse[]) : [];

	const handleOpenAdd = useCallback(() => setIsAddOpen(true), []);
	const handleCloseAdd = useCallback(() => setIsAddOpen(false), []);

	const handleViewKey = useCallback((key: SshKeyResponse) => setSelectedKeyForView(key), []);
	const handleCloseView = useCallback(() => setSelectedKeyForView(null), []);

	const handleEditKey = useCallback((key: SshKeyResponse) => setSelectedKeyForEdit(key), []);
	const handleCloseEdit = useCallback(() => setSelectedKeyForEdit(null), []);

	const handleDeleteKey = useCallback((key: SshKeyResponse) => setSelectedKeyForDelete(key), []);
	const handleCloseDelete = useCallback(() => setSelectedKeyForDelete(null), []);

	return (
		<div className="flex flex-col gap-6 p-6 max-w-7xl mx-auto w-full">
			<SshKeysHeader
				onOpenAdd={handleOpenAdd}
				onRefresh={refetch}
				isRefetching={isRefetching}
				keys={sshKeys}
			/>

			<SshKeysList
				keys={sshKeys}
				isLoading={isLoading}
				onViewKey={handleViewKey}
				onEditKey={handleEditKey}
				onDeleteKey={handleDeleteKey}
			/>

			{isAddOpen && (
				<CreateKeyModal
					isOpen={isAddOpen}
					onClose={handleCloseAdd}
					onSuccess={refetch}
				/>
			)}

			{selectedKeyForView && (
				<ViewKeyModal
					isOpen={!!selectedKeyForView}
					sshKey={selectedKeyForView}
					onClose={handleCloseView}
				/>
			)}

			{selectedKeyForEdit && (
				<EditKeyModal
					isOpen={!!selectedKeyForEdit}
					sshKey={selectedKeyForEdit}
					onClose={handleCloseEdit}
					onSuccess={refetch}
				/>
			)}

			{selectedKeyForDelete && (
				<DeleteKeyModal
					isOpen={!!selectedKeyForDelete}
					sshKey={selectedKeyForDelete}
					onClose={handleCloseDelete}
					onSuccess={refetch}
				/>
			)}
		</div>
	);
}
