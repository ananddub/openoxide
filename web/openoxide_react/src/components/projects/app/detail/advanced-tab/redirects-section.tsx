import {useState} from 'react';
import {Link2, Package, Pencil, Plus, Trash2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Switch} from '#/components/ui/switch';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field, SheetGroup} from '#/components/shared/field';
import {
	useApplicationRedirectDelete,
	useApplicationRedirectList,
	useApplicationRedirectUpsert,
	type ApplicationRedirectPayload,
} from '#/api/live';
import {formatApiError} from '#/api/utils';

export function RedirectsSection({app}: {app: any}) {
	const {data: redirects = [], refetch} = useApplicationRedirectList(app.id);
	const upsert = useApplicationRedirectUpsert();
	const remove = useApplicationRedirectDelete();

	const [view, setView] = useState<'list' | 'form'>('list');
	const [editingId, setEditingId] = useState<number | undefined>(undefined);
	const [regex, setRegex] = useState('');
	const [replacement, setReplacement] = useState('');
	const [permanent, setPermanent] = useState(true);
	const [saving, setSaving] = useState(false);

	const openForm = (redirect: ApplicationRedirectPayload | null) => {
		setEditingId(redirect?.id);
		setRegex(redirect?.regex || '');
		setReplacement(redirect?.replacement || '');
		setPermanent((redirect?.permanent ?? 1) !== 0);
		setView('form');
	};
	const backToList = () => {
		setView('list');
		setEditingId(undefined);
		setRegex('');
		setReplacement('');
		setPermanent(true);
	};

	const save = async () => {
		if (!regex.trim() || !replacement.trim()) return;
		setSaving(true);
		try {
			await upsert.mutateAsync({
				application_id: app.id,
				id: editingId,
				regex,
				replacement,
				permanent,
			});
			toast.success(editingId ? 'Redirect updated' : 'Redirect added');
			backToList();
			await refetch();
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	const del = async (redirect: ApplicationRedirectPayload) => {
		try {
			await remove.mutateAsync({application_id: app.id, id: redirect.id});
			toast.success('Redirect removed');
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
						? 'Edit redirect'
						: 'Add redirect'
					: 'Redirects'
			}
			description={
				view === 'form'
					? 'Match a request pattern and rewrite it at the edge.'
					: 'Regex rewrites handled at the edge router'
			}
			icon={<Link2 className="size-4" />}
			onOpenChange={next => {
				if (!next) backToList();
			}}
			meta={
				view === 'list' && redirects.length > 0 ? (
					<span className="rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">
						{redirects.length}
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
							saving || !regex.trim() || !replacement.trim()
						}
						className="h-9 px-4 text-sm">
						{saving
							? 'Saving...'
							: editingId
								? 'Update redirect'
								: 'Add redirect'}
					</Button>
				) : undefined
			}>
			{view === 'list' ? (
				<SheetGroup
					title="Traefik redirects"
					description="Regex-based request rewrites handled at the edge router.">
					{redirects.length === 0 ? (
						<div className="flex flex-col items-center gap-3 rounded-xl border border-dashed border-border bg-muted/20 px-4 py-10 text-center">
							<Package className="size-8 text-muted-foreground/60" />
							<p className="text-sm text-muted-foreground">
								No redirects configured yet.
							</p>
							<Button
								onClick={() => openForm(null)}
								className="h-9 gap-1.5 px-4 text-sm">
								<Plus className="size-4" /> Add redirect
							</Button>
						</div>
					) : (
						<>
							<div className="flex flex-col gap-3">
								{redirects.map(redirect => (
									<div
										key={redirect.id}
										className="flex items-start justify-between gap-3 rounded-xl border border-border bg-card p-4">
										<div className="flex min-w-0 flex-col gap-2">
											<div className="flex items-center gap-2">
												<span className="rounded-md bg-primary/10 px-2 py-0.5 text-[11px] font-semibold uppercase tracking-wide text-primary">
													{redirect.permanent ? '301' : '302'}
												</span>
												<span className="truncate font-mono text-sm text-foreground">
													{redirect.regex}
												</span>
											</div>
											<p className="truncate text-[13px] text-muted-foreground">
												<span className="font-mono">
													→ {redirect.replacement}
												</span>
											</p>
										</div>
										<div className="flex shrink-0 gap-1">
											<Button
												variant="ghost"
												size="icon"
												onClick={() => openForm(redirect)}
												aria-label="Edit redirect">
												<Pencil className="size-4" />
											</Button>
											<Button
												variant="ghost"
												size="icon"
												onClick={() => del(redirect)}
												aria-label="Delete redirect"
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
								<Plus className="size-4" /> Add redirect
							</Button>
						</>
					)}
				</SheetGroup>
			) : (
				<div className="flex flex-col gap-5">
					<Field
						label="Regex"
						hint="The request pattern to match.">
						<Input
							placeholder="https://old.example.com/(.*)"
							value={regex}
							onChange={e => setRegex(e.target.value)}
							className="font-mono"
						/>
					</Field>
					<Field
						label="Replacement"
						hint="Where matched requests are rewritten to.">
						<Input
							placeholder="https://example.com/$1"
							value={replacement}
							onChange={e => setReplacement(e.target.value)}
							className="font-mono"
						/>
					</Field>
					<div className="flex items-center justify-between gap-4 rounded-xl border border-border/50 px-4 py-3.5">
						<div className="min-w-0">
							<div className="text-sm font-medium text-foreground">
								Permanent (301)
							</div>
							<div className="text-[13px] text-muted-foreground">
								Toggle off to send a temporary 302 instead.
							</div>
						</div>
						<Switch
							checked={permanent}
							onCheckedChange={setPermanent}
							className="shrink-0"
						/>
					</div>
				</div>
			)}
		</SettingsSheet>
	);
}
