import {useState} from 'react';
import {FileCode2} from 'lucide-react';
import {toast} from 'sonner';
import {Badge} from '#/components/ui/badge';
import {
	useComposePatchDelete,
	useComposePatchList,
	type ComposePatchPayload,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {
	AddResourceButton,
	AdvancedSection,
	EmptyRow,
	ResourceRow,
} from '#/components/projects/detail/advanced/shared';
import {PatchFormDialog} from './patch-form-dialog';

export function PatchesSection({compose}: {compose: any}) {
	const {data: patches = []} = useComposePatchList(compose.id);
	const remove = useComposePatchDelete();
	const [open, setOpen] = useState(false);
	const [editing, setEditing] = useState<ComposePatchPayload | null>(null);
	const [deletingId, setDeletingId] = useState<number | null>(null);

	const show = (patch: ComposePatchPayload | null) => {
		setEditing(patch);
		setOpen(true);
	};

	const del = async (id: number) => {
		setDeletingId(id);
		try {
			await remove.mutateAsync({compose_id: compose.id, id});
			toast.success('Patch removed');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<AdvancedSection
			icon={<FileCode2 className="size-4 text-primary" />}
			title="Compose Patches"
			description="Overlay files merged into the compose definition at deploy time."
			action={
				<AddResourceButton onClick={() => show(null)} label="Add Patch" />
			}>
			<div className="flex flex-col gap-2">
				{patches.length === 0 ? (
					<EmptyRow label="No patches." />
				) : (
					patches.map(patch => (
						<ResourceRow
							key={patch.id}
							title={patch.file_path}
							subtitle={patch.content?.slice(0, 80)}
							badges={
								<Badge
									variant="secondary"
									className="text-[10px] font-semibold">
									{patch.patch_type}
								</Badge>
							}
							onEdit={() => show(patch)}
							onDelete={() => del(patch.id)}
							deleting={deletingId === patch.id}
						/>
					))
				)}
			</div>
			{open ? (
				<PatchFormDialog
					composeId={compose.id}
					patch={editing}
					open={open}
					onOpenChange={setOpen}
				/>
			) : null}
		</AdvancedSection>
	);
}
