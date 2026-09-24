import {useState} from 'react';
import {Globe, Package, Plus, Trash2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field, FieldGrid, SheetGroup} from '#/components/shared/field';
import {Num} from '#/components/projects/database/detail/swarm-settings/field';
import {
	useApplicationPortCreate,
	useApplicationPortDelete,
	useApplicationPortList,
	type ApplicationPortPayload,
} from '#/api/live';
import {formatApiError} from '#/api/utils';

export function PortsSection({app}: {app: any}) {
	const {data: ports = [], refetch} = useApplicationPortList(app.id);
	const create = useApplicationPortCreate();
	const remove = useApplicationPortDelete();

	const [view, setView] = useState<'list' | 'form'>('list');
	const [published, setPublished] = useState<number | undefined>(
		undefined,
	);
	const [target, setTarget] = useState<number | undefined>(undefined);
	const [protocol, setProtocol] = useState('TCP');
	const [mode, setMode] = useState('HOST');
	const [saving, setSaving] = useState(false);

	const resetForm = () => {
		setPublished(undefined);
		setTarget(undefined);
		setProtocol('TCP');
		setMode('HOST');
	};
	const backToList = () => {
		setView('list');
		resetForm();
	};

	const save = async () => {
		if (published == null || target == null) return;
		setSaving(true);
		try {
			await create.mutateAsync({
				application_id: app.id,
				published_port: Number(published),
				target_port: Number(target),
				protocol,
				publish_mode: mode,
			});
			toast.success('Port added');
			backToList();
			await refetch();
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	const del = async (port: ApplicationPortPayload) => {
		try {
			await remove.mutateAsync({application_id: app.id, id: port.id});
			toast.success('Port removed');
			await refetch();
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<SettingsSheet
			title={view === 'form' ? 'Add port' : 'Ports'}
			description={
				view === 'form'
					? 'Map a host or ingress port to a container port.'
					: 'Published container ports'
			}
			icon={<Globe className="size-4" />}
			onOpenChange={next => {
				if (!next) backToList();
			}}
			meta={
				view === 'list' && ports.length > 0 ? (
					<span className="rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">
						{ports.length}
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
						disabled={saving || published == null || target == null}
						className="h-9 px-4 text-sm">
						{saving ? 'Saving...' : 'Add port'}
					</Button>
				) : undefined
			}>
			{view === 'list' ? (
				<SheetGroup
					title="Published ports"
					description="Ports exposed on the host or through the swarm ingress mesh.">
					{ports.length === 0 ? (
						<div className="flex flex-col items-center gap-3 rounded-xl border border-dashed border-border bg-muted/20 px-4 py-10 text-center">
							<Package className="size-8 text-muted-foreground/60" />
							<p className="text-sm text-muted-foreground">
								No published ports yet.
							</p>
							<Button
								onClick={() => setView('form')}
								className="h-9 gap-1.5 px-4 text-sm">
								<Plus className="size-4" /> Add port
							</Button>
						</div>
					) : (
						<>
							<div className="flex flex-col gap-3">
								{ports.map(port => (
									<div
										key={port.id}
										className="flex items-center justify-between gap-3 rounded-xl border border-border bg-card p-4">
										<div className="flex min-w-0 flex-col gap-1.5">
											<div className="flex items-center gap-2">
												<span className="rounded-md bg-primary/10 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-primary uppercase">
													{port.protocol}
												</span>
												<span className="truncate font-mono text-sm text-foreground">
													{port.published_port} → {port.target_port}
												</span>
											</div>
											<p className="text-[13px] text-muted-foreground">
												{port.publish_mode === 'INGRESS'
													? 'Swarm ingress mesh'
													: 'Host binding'}
											</p>
										</div>
										<Button
											variant="ghost"
											size="icon"
											onClick={() => del(port)}
											aria-label="Delete port"
											className="shrink-0 text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
											<Trash2 className="size-4" />
										</Button>
									</div>
								))}
							</div>
							<Button
								onClick={() => setView('form')}
								variant="outline"
								className="h-9 w-fit gap-1.5 text-sm">
								<Plus className="size-4" /> Add port
							</Button>
						</>
					)}
				</SheetGroup>
			) : (
				<div className="flex flex-col gap-5">
					<FieldGrid>
						<Num
							label="Published port"
							value={published}
							set={setPublished}
						/>
						<Num label="Target port" value={target} set={setTarget} />
					</FieldGrid>
					<FieldGrid>
						<Field
							label="Protocol"
							hint="Transport protocol for the port.">
							<Select
								value={protocol}
								onValueChange={next => next && setProtocol(next)}>
								<SelectTrigger>
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="TCP">TCP</SelectItem>
									<SelectItem value="UDP">UDP</SelectItem>
								</SelectContent>
							</Select>
						</Field>
						<Field
							label="Publish mode"
							hint="Host binds directly; ingress routes via the mesh.">
							<Select
								value={mode}
								onValueChange={next => next && setMode(next)}>
								<SelectTrigger>
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="HOST">Host</SelectItem>
									<SelectItem value="INGRESS">Ingress</SelectItem>
								</SelectContent>
							</Select>
						</Field>
					</FieldGrid>
				</div>
			)}
		</SettingsSheet>
	);
}
