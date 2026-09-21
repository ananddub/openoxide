import {useState} from 'react';
import {HardDrive, Package, Pencil, Plus, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {
	useDatabaseMountCreate,
	useDatabaseMountDelete,
	useDatabaseMountList,
	useDatabaseMountUpdate,
	parseDatabaseKind,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {
	DatabaseVolumeDialog,
	type DatabaseMount,
} from './database-volume-dialog';

export function DatabaseAdvancedVolumes({database, onUpdated}: any) {
	const kind = parseDatabaseKind(database?.kind || 'postgres');
	const databaseId = Number(database?.id);
	const {data, refetch} = useDatabaseMountList(
		kind,
		BigInt(databaseId || 0),
	);
	const mounts = normalizeMounts(data);
	const create = useDatabaseMountCreate();
	const update = useDatabaseMountUpdate();
	const remove = useDatabaseMountDelete();
	const [open, setOpen] = useState(false);
	const [editing, setEditing] = useState<DatabaseMount | null>(null);
	const [saving, setSaving] = useState(false);
	const showDialog = (mount: DatabaseMount | null) => {
		setEditing(mount);
		setOpen(true);
	};
	const saveMount = async (value: Omit<DatabaseMount, 'id'>) => {
		setSaving(true);
		try {
			const body = {
				kind,
				database_id: databaseId,
				mount_type: value.mount_type,
				volume_name: value.volume_name || undefined,
				host_path: value.host_path || undefined,
				file_path: value.file_path || undefined,
				content: value.content ?? undefined,
				mount_path: value.mount_path,
			};
			if (editing) await update.mutateAsync({...body, id: editing.id});
			else await create.mutateAsync(body);
			toast.success(editing ? 'Volume updated' : 'Volume added');
			setOpen(false);
			await refetch();
			onUpdated();
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setSaving(false);
		}
	};
	const deleteMount = async (mount: DatabaseMount) => {
		try {
			await remove.mutateAsync({
				kind,
				database_id: databaseId,
				id: mount.id,
			});
			toast.success('Volume removed');
			await refetch();
			onUpdated();
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};
	return (
		<section className="flex flex-col gap-5 rounded-xl bg-card p-6 shadow-sm">
			<div className="flex flex-wrap items-start justify-between gap-4">
				<div>
					<h3 className="flex items-center gap-2 text-base font-bold">
						<HardDrive className="size-4 text-primary" /> Volumes
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Configure persistent volumes, host binds, and file mounts.
					</p>
				</div>
				{mounts.length > 0 && (
					<Button
						onClick={() => showDialog(null)}
						className="h-9 gap-1.5 text-xs">
						<Plus className="size-3.5" /> Add Volume
					</Button>
				)}
			</div>
			{mounts.length === 0 ? (
				<div className="flex flex-col items-center gap-3 border-t  py-10 text-muted-foreground">
					<Package className="size-8" />
					<p className="text-sm">No volumes/mounts configured</p>
					<Button
						onClick={() => showDialog(null)}
						className="h-9 gap-1.5 text-xs">
						<Plus className="size-3.5" /> Add Volume
					</Button>
				</div>
			) : (
				<div className="flex flex-col gap-4 border-t  pt-4">
					<div className="rounded-lg bg-amber-500/5 p-3 text-xs text-amber-700 dark:text-amber-300">
						Redeploy the database after adding, editing, or deleting a
						mount.
					</div>
					{mounts.map(mount => (
						<div
							key={mount.id}
							className="flex flex-col justify-between gap-4 rounded-lg  p-4 sm:flex-row sm:items-center">
							<div className="grid flex-1 grid-cols-1 gap-4 sm:grid-cols-3">
								<MountValue label="Mount Type" value={mount.mount_type} />
								<MountValue
									label={
										mount.mount_type === 'VOLUME'
											? 'Volume Name'
											: mount.mount_type === 'BIND'
												? 'Host Path'
												: 'File Path'
									}
									value={
										mount.volume_name ||
										mount.host_path ||
										mount.file_path ||
										'-'
									}
								/>
								<MountValue label="Mount Path" value={mount.mount_path} />
							</div>
							<div className="flex gap-1">
								<Button
									variant="ghost"
									size="icon"
									onClick={() => showDialog(mount)}>
									<Pencil className="size-4" />
								</Button>
								<Button
									variant="ghost"
									size="icon"
									className="text-destructive"
									onClick={() => deleteMount(mount)}>
									<Trash2 className="size-4" />
								</Button>
							</div>
						</div>
					))}
				</div>
			)}
			<DatabaseVolumeDialog
				open={open}
				mount={editing}
				saving={saving}
				onOpenChange={setOpen}
				onSave={saveMount}
			/>
		</section>
	);
}

function normalizeMounts(value: unknown): DatabaseMount[] {
	if (Array.isArray(value)) return value as DatabaseMount[];
	if (
		value &&
		typeof value === 'object' &&
		Array.isArray((value as {items?: unknown}).items)
	) {
		return (value as {items: DatabaseMount[]}).items;
	}
	return [];
}

function MountValue({label, value}: {label: string; value: string}) {
	return (
		<div className="min-w-0">
			<p className="text-xs font-medium">{label}</p>
			<p className="mt-1 truncate text-xs text-muted-foreground">
				{value}
			</p>
		</div>
	);
}
