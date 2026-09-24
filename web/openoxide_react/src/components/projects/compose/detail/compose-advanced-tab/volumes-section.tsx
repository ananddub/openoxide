import {useState} from 'react';
import {HardDrive, Package, Pencil, Plus, Trash2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {SheetGroup, SheetNote} from '#/components/shared/field';
import {formatApiError} from '#/api/utils';
import {
	useComposeMountDelete,
	useComposeMountList,
	useComposeMountUpsert,
} from '#/api/live';
import {
	MountFields,
	buildMountPayload,
	emptyMountForm,
	mountFormFrom,
	type MountFormState,
} from '#/components/projects/detail/advanced/mount-fields';

export function VolumesSection({compose}: {compose: any}) {
	const {data: mounts = [], refetch} = useComposeMountList(compose.id);
	const upsert = useComposeMountUpsert();
	const remove = useComposeMountDelete();

	const [view, setView] = useState<'list' | 'form'>('list');
	const [editingId, setEditingId] = useState<number | undefined>(
		undefined,
	);
	const [state, setState] = useState<MountFormState>(emptyMountForm);
	const [saving, setSaving] = useState(false);

	const set = (patch: Partial<MountFormState>) =>
		setState(prev => ({...prev, ...patch}));

	const openForm = (mount: any | null) => {
		setEditingId(mount?.id);
		setState(mountFormFrom(mount));
		setView('form');
	};
	const backToList = () => {
		setView('list');
		setEditingId(undefined);
		setState(emptyMountForm);
	};

	const save = async () => {
		if (!state.source.trim() || !state.target.trim()) return;
		setSaving(true);
		try {
			await upsert.mutateAsync({
				compose_id: compose.id,
				id: editingId,
				...buildMountPayload(state),
			});
			toast.success(editingId ? 'Volume updated' : 'Volume added');
			backToList();
			await refetch();
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	const del = async (mount: any) => {
		try {
			await remove.mutateAsync({compose_id: compose.id, id: mount.id});
			toast.success('Volume removed');
			await refetch();
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<SettingsSheet
			title={
				view === 'form'
					? editingId
						? 'Edit volume'
						: 'Add volume'
					: 'Volumes'
			}
			description={
				view === 'form'
					? 'Persistent storage or an injected file for the stack.'
					: 'Persistent volumes, host binds and file mounts'
			}
			icon={<HardDrive className="size-4" />}
			onOpenChange={next => {
				if (!next) backToList();
			}}
			meta={
				view === 'list' && mounts.length > 0 ? (
					<span className="rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">
						{mounts.length}
					</span>
				) : undefined
			}
			footerLeft={
				view === 'form' ? (
					<Button
						type="button"
						variant="ghost"
						onClick={backToList}
						className="h-9 px-4 text-sm">
						Back
					</Button>
				) : undefined
			}
			footer={
				view === 'form' ? (
					<Button
						onClick={save}
						disabled={
							saving || !state.source.trim() || !state.target.trim()
						}
						className="h-9 px-4 text-sm">
						{saving
							? 'Saving...'
							: editingId
								? 'Update volume'
								: 'Add volume'}
					</Button>
				) : undefined
			}>
			{view === 'list' ? (
				<SheetGroup
					title="Mounted volumes"
					description="Storage and files applied to every service in the stack.">
					{mounts.length === 0 ? (
						<div className="flex flex-col items-center gap-3 rounded-xl border border-dashed border-border bg-muted/20 px-4 py-10 text-center">
							<Package className="size-8 text-muted-foreground/60" />
							<p className="text-sm text-muted-foreground">
								No volumes or mounts configured yet.
							</p>
							<Button
								onClick={() => openForm(null)}
								className="h-9 gap-1.5 px-4 text-sm">
								<Plus className="size-4" /> Add volume
							</Button>
						</div>
					) : (
						<>
							<SheetNote tone="warn">
								Redeploy the stack after adding, editing, or deleting a
								mount.
							</SheetNote>
							<div className="flex flex-col gap-3">
								{mounts.map(mount => (
									<div
										key={mount.id}
										className="flex items-start justify-between gap-3 rounded-xl border border-border bg-card p-4">
										<div className="flex min-w-0 flex-col gap-2">
											<div className="flex items-center gap-2">
												<span className="rounded-md bg-primary/10 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-primary uppercase">
													{mount.mount_type}
												</span>
												<span className="truncate font-mono text-sm text-foreground">
													{mount.mount_path}
												</span>
											</div>
											<p className="truncate text-[13px] text-muted-foreground">
												<span className="font-mono">
													{mount.volume_name ||
														mount.host_path ||
														mount.file_path ||
														'-'}
												</span>
											</p>
										</div>
										<div className="flex shrink-0 gap-1">
											<Button
												variant="ghost"
												size="icon"
												onClick={() => openForm(mount)}
												aria-label="Edit mount">
												<Pencil className="size-4" />
											</Button>
											<Button
												variant="ghost"
												size="icon"
												onClick={() => del(mount)}
												aria-label="Delete mount"
												className="text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
												<Trash2 className="size-4" />
											</Button>
										</div>
									</div>
								))}
							</div>
							<Button
								onClick={() => openForm(null)}
								variant="outline"
								className="h-9 w-fit gap-1.5 text-sm">
								<Plus className="size-4" /> Add volume
							</Button>
						</>
					)}
				</SheetGroup>
			) : (
				<MountFields state={state} set={set} />
			)}
		</SettingsSheet>
	);
}
