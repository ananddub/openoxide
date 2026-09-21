import {useState} from 'react';
import {HardDrive} from 'lucide-react';
import {toast} from 'sonner';
import {Badge} from '#/components/ui/badge';
import {
	useApplicationMountDelete,
	useApplicationMountList,
	useApplicationMountUpsert,
} from '#/api/live';
import {
	AddResourceButton,
	AdvancedSection,
	EmptyRow,
	ResourceRow,
} from '#/components/projects/detail/advanced/shared';
import {
	MountDialog,
	type MountValue,
} from '#/components/projects/detail/advanced/mount-dialog';

import {formatApiError} from '#/api/utils';

export function VolumesSection({app}: {app: any}) {
	const {data: mounts = []} = useApplicationMountList(app.id);
	const upsert = useApplicationMountUpsert();
	const remove = useApplicationMountDelete();
	const [open, setOpen] = useState(false);
	const [editing, setEditing] = useState<MountValue | null>(null);
	const [saving, setSaving] = useState(false);
	const [deletingId, setDeletingId] = useState<number | null>(null);

	const save = async (value: Omit<MountValue, 'id'>) => {
		setSaving(true);
		try {
			await upsert.mutateAsync({
				application_id: app.id,
				id: editing?.id,
				...value,
			});
			toast.success(editing ? 'Volume updated' : 'Volume added');
			setOpen(false);
			setEditing(null);
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	const del = async (id: number) => {
		setDeletingId(id);
		try {
			await remove.mutateAsync({application_id: app.id, id});
			toast.success('Volume removed');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<AdvancedSection
			icon={<HardDrive className="size-4 text-primary" />}
			title="Volumes / Mounts"
			description="Persistent storage and injected files for this container."
			action={
				<AddResourceButton
					onClick={() => {
						setEditing(null);
						setOpen(true);
					}}
					label="Add Volume"
				/>
			}>
			<div className="flex flex-col gap-2">
				{mounts.length === 0 ? (
					<EmptyRow label="No volumes or mounts." />
				) : (
					mounts.map(mount => (
						<ResourceRow
							key={mount.id}
							title={mount.mount_path}
							subtitle={
								mount.volume_name ||
								mount.host_path ||
								mount.file_path ||
								''
							}
							badges={
								<Badge
									variant="secondary"
									className="text-[10px] font-semibold">
									{mount.mount_type}
								</Badge>
							}
							onEdit={() => {
								setEditing({...mount});
								setOpen(true);
							}}
							onDelete={() => del(mount.id)}
							deleting={deletingId === mount.id}
						/>
					))
				)}
			</div>
			<MountDialog
				open={open}
				mount={editing}
				saving={saving}
				onOpenChange={setOpen}
				onSave={save}
			/>
		</AdvancedSection>
	);
}

// ── Redirects ──────────────────────────────────────────────────────────
