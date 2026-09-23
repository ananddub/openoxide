import {useState} from 'react';
import {
	AlertTriangle,
	FileText,
	FolderInput,
	HardDrive,
	Package,
	Pencil,
	Plus,
	Trash2,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field, SheetGroup, SheetNote} from '#/components/shared/field';
import {cn} from '#/api/utils';
import {toast} from 'sonner';
import {
	useDatabaseMountCreate,
	useDatabaseMountDelete,
	useDatabaseMountList,
	useDatabaseMountUpdate,
	parseDatabaseKind,
} from '#/api/live';
import {formatApiError} from '#/api/utils';

type DatabaseMount = {
	id: number;
	mount_type: string;
	volume_name?: string | null;
	host_path?: string | null;
	file_path?: string | null;
	content?: string | null;
	mount_path: string;
};

const MOUNT_TYPES = [
	{value: 'BIND', label: 'Bind Mount', icon: FolderInput},
	{value: 'VOLUME', label: 'Volume Mount', icon: HardDrive},
	{value: 'FILE', label: 'File Mount', icon: FileText},
] as const;

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

	const [view, setView] = useState<'list' | 'form'>('list');
	const [editing, setEditing] = useState<DatabaseMount | null>(null);
	const [type, setType] = useState('VOLUME');
	const [source, setSource] = useState('');
	const [target, setTarget] = useState('');
	const [content, setContent] = useState('');
	const [saving, setSaving] = useState(false);

	const openForm = (mount: DatabaseMount | null) => {
		setEditing(mount);
		setType(mount?.mount_type || 'VOLUME');
		setSource(
			mount?.volume_name || mount?.host_path || mount?.file_path || '',
		);
		setTarget(mount?.mount_path || '');
		setContent(mount?.content || '');
		setView('form');
	};
	const backToList = () => {
		setView('list');
		setEditing(null);
	};

	const saveMount = async () => {
		setSaving(true);
		try {
			const body = {
				kind,
				database_id: databaseId,
				mount_type: type,
				volume_name: type === 'VOLUME' ? source.trim() : undefined,
				host_path: type === 'BIND' ? source.trim() : undefined,
				file_path: type === 'FILE' ? source.trim() : undefined,
				content: type === 'FILE' ? content : undefined,
				mount_path: target.trim(),
			};
			if (editing) await update.mutateAsync({...body, id: editing.id});
			else await create.mutateAsync(body);
			toast.success(editing ? 'Volume updated' : 'Volume added');
			backToList();
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
		<SettingsSheet
			title={
				view === 'form'
					? editing
						? 'Edit volume'
						: 'Add volume'
					: 'Volumes'
			}
			description={
				view === 'form'
					? 'Persistent storage or a file for this database.'
					: 'Persistent volumes, host binds and file mounts'
			}
			icon={<HardDrive className="size-4" />}
			onOpenChange={next => {
				if (!next) backToList();
			}}
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
			meta={
				view === 'list' && mounts.length > 0 ? (
					<span className="rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">
						{mounts.length}
					</span>
				) : undefined
			}
			footer={
				view === 'form' ? (
					<Button
						onClick={saveMount}
						disabled={saving || !source.trim() || !target.trim()}
						className="h-9 px-4 text-sm">
						{saving
							? 'Saving...'
							: editing
								? 'Update volume'
								: 'Add volume'}
					</Button>
				) : undefined
			}>
			{view === 'list' ? (
				<SheetGroup
					title="Mounted volumes"
					description="Storage attached to this database container.">
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
								Redeploy the database after adding, editing, or deleting
								a mount.
							</SheetNote>
							<div className="flex flex-col gap-3">
								{mounts.map(mount => (
									<div
										key={mount.id}
										className="flex items-start justify-between gap-3 rounded-xl border border-border bg-card p-4">
										<div className="flex min-w-0 flex-col gap-2">
											<div className="flex items-center gap-2">
												<span className="rounded-md bg-primary/10 px-2 py-0.5 text-[11px] font-semibold uppercase tracking-wide text-primary">
													{mount.mount_type}
												</span>
												<span className="truncate font-mono text-sm text-foreground">
													{mount.mount_path}
												</span>
											</div>
											<p className="truncate text-[13px] text-muted-foreground">
												<span className="text-muted-foreground/70">
													{mount.mount_type === 'VOLUME'
														? 'Volume'
														: mount.mount_type === 'BIND'
															? 'Host path'
															: 'File'}
												</span>{' '}
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
												onClick={() => deleteMount(mount)}
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
				<div className="flex flex-col gap-5">
					{type === 'BIND' && (
						<div className="flex items-start gap-2.5 rounded-lg border border-amber-500/30 bg-amber-500/5 px-3.5 py-3 text-[13px] leading-snug text-amber-700 dark:text-amber-300">
							<AlertTriangle className="mt-0.5 size-4 shrink-0" />
							<span>
								The host path must exist on every Swarm node where this
								database can run. Named volumes are safer for clustered
								deployments.
							</span>
						</div>
					)}

					<div className="flex flex-col gap-2.5">
						<p className="text-sm font-medium leading-none text-foreground">
							Mount type
						</p>
						<div className="grid grid-cols-1 gap-3 sm:grid-cols-3">
							{MOUNT_TYPES.map(option => {
								const selected = type === option.value;
								return (
									<button
										key={option.value}
										type="button"
										onClick={() => setType(option.value)}
										className={cn(
											'flex flex-col items-start gap-2 rounded-xl border px-4 py-3.5 text-start transition-all duration-150',
											selected
												? 'border-primary/40 bg-primary/10 text-primary'
												: 'border-border/50 text-foreground hover:border-border hover:bg-muted/30',
										)}>
										<option.icon className="size-5" />
										<span className="text-sm font-medium">
											{option.label}
										</span>
									</button>
								);
							})}
						</div>
					</div>

					<div className="grid grid-cols-1 gap-5 sm:grid-cols-2">
						<Field
							label={
								type === 'VOLUME'
									? 'Volume name'
									: type === 'BIND'
										? 'Host path'
										: 'File path'
							}
							hint={
								type === 'VOLUME'
									? 'A named Docker volume to attach.'
									: 'Absolute path on the host or container.'
							}>
							<Input
								value={source}
								onChange={event => setSource(event.target.value)}
								placeholder={
									type === 'VOLUME' ? 'database-data' : '/absolute/path'
								}
							/>
						</Field>
						<Field
							label="Mount path"
							hint="Where it appears inside the database container.">
							<Input
								value={target}
								onChange={event => setTarget(event.target.value)}
								placeholder="/var/lib/database/data"
							/>
						</Field>
					</div>

					{type === 'FILE' && (
						<Field
							label="Content"
							hint="The file is written with exactly this content.">
							<Textarea
								value={content}
								onChange={event => setContent(event.target.value)}
								className="min-h-48 font-mono text-xs"
							/>
						</Field>
					)}
				</div>
			)}
		</SettingsSheet>
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
