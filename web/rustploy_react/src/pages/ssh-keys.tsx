import {useState} from 'react';
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

	return (
		<div className="flex flex-col gap-6 p-6 max-w-7xl mx-auto w-full">
			<SshKeysHeader
				onOpenAdd={() => setIsAddOpen(true)}
				onRefresh={refetch}
				isRefetching={isRefetching}
			/>

			<SshKeysList
				keys={sshKeys}
				isLoading={isLoading}
				onViewKey={key => setSelectedKeyForView(key)}
				onEditKey={key => setSelectedKeyForEdit(key)}
				onDeleteKey={key => setSelectedKeyForDelete(key)}
			/>

			<CreateKeyModal
				isOpen={isAddOpen}
				onClose={() => setIsAddOpen(false)}
				onSuccess={refetch}
			/>

			<ViewKeyModal
				isOpen={!!selectedKeyForView}
				sshKey={selectedKeyForView}
				onClose={() => setSelectedKeyForView(null)}
			/>

			<EditKeyModal
				isOpen={!!selectedKeyForEdit}
				sshKey={selectedKeyForEdit}
				onClose={() => setSelectedKeyForEdit(null)}
				onSuccess={refetch}
			/>

			<DeleteKeyModal
				isOpen={!!selectedKeyForDelete}
				sshKey={selectedKeyForDelete}
				onClose={() => setSelectedKeyForDelete(null)}
				onSuccess={refetch}
			/>
		</div>
	);
}
