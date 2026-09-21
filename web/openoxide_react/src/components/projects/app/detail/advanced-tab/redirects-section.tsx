import {useState} from 'react';
import {Link2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Switch} from '#/components/ui/switch';
import {Badge} from '#/components/ui/badge';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	useApplicationRedirectDelete,
	useApplicationRedirectList,
	useApplicationRedirectUpsert,
	type ApplicationRedirectPayload,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {
	AddResourceButton,
	AdvancedSection,
	EmptyRow,
	ResourceRow,
} from '#/components/projects/detail/advanced/shared';

export function RedirectsSection({app}: {app: any}) {
	const {data: redirects = []} = useApplicationRedirectList(app.id);
	const upsert = useApplicationRedirectUpsert();
	const remove = useApplicationRedirectDelete();
	const [open, setOpen] = useState(false);
	const [editing, setEditing] =
		useState<ApplicationRedirectPayload | null>(null);
	const [saving, setSaving] = useState(false);
	const [deletingId, setDeletingId] = useState<number | null>(null);
	const [regex, setRegex] = useState('');
	const [replacement, setReplacement] = useState('');
	const [permanent, setPermanent] = useState(true);

	const show = (redirect: ApplicationRedirectPayload | null) => {
		setEditing(redirect);
		setRegex(redirect?.regex || '');
		setReplacement(redirect?.replacement || '');
		setPermanent((redirect?.permanent ?? 1) !== 0);
		setOpen(true);
	};

	const save = async () => {
		setSaving(true);
		try {
			await upsert.mutateAsync({
				application_id: app.id,
				id: editing?.id,
				regex,
				replacement,
				permanent,
			});
			toast.success(editing ? 'Redirect updated' : 'Redirect added');
			setOpen(false);
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
			toast.success('Redirect removed');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<AdvancedSection
			icon={<Link2 className="size-4 text-primary" />}
			title="Traefik Redirects"
			description="Regex-based request rewrites handled at the edge router."
			action={
				<AddResourceButton
					onClick={() => show(null)}
					label="Add Redirect"
				/>
			}>
			<div className="flex flex-col gap-2">
				{redirects.length === 0 ? (
					<EmptyRow label="No redirects." />
				) : (
					redirects.map(redirect => (
						<ResourceRow
							key={redirect.id}
							title={redirect.regex}
							subtitle={`→ ${redirect.replacement}`}
							badges={
								<Badge
									variant="secondary"
									className="text-[10px] font-semibold">
									{redirect.permanent ? '301' : '302'}
								</Badge>
							}
							onEdit={() => show(redirect)}
							onDelete={() => del(redirect.id)}
							deleting={deletingId === redirect.id}
						/>
					))
				)}
			</div>
			<Dialog open={open} onOpenChange={setOpen}>
				<DialogContent className="sm:max-w-md">
					<DialogHeader>
						<DialogTitle>Redirect</DialogTitle>
						<DialogDescription>
							Match a request pattern and rewrite it to a target.
						</DialogDescription>
					</DialogHeader>
					<div className="flex flex-col gap-3">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Regex
							</Label>
							<Input
								placeholder="https://old.example.com/(.*)"
								value={regex}
								onChange={e => setRegex(e.target.value)}
								className="font-mono text-xs"
							/>
						</div>
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Replacement
							</Label>
							<Input
								placeholder="https://example.com/$1"
								value={replacement}
								onChange={e => setReplacement(e.target.value)}
								className="font-mono text-xs"
							/>
						</div>
						<div className="flex items-center justify-between rounded-lg border border-border/40 px-3 py-2.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Permanent (301)
							</Label>
							<Switch checked={permanent} onCheckedChange={setPermanent} />
						</div>
					</div>
					<DialogFooter>
						<Button
							variant="outline"
							onClick={() => setOpen(false)}
							className="text-xs">
							Cancel
						</Button>
						<Button
							disabled={saving || !regex.trim() || !replacement.trim()}
							onClick={save}
							className="text-xs">
							{saving ? 'Saving…' : editing ? 'Update' : 'Add'}
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</AdvancedSection>
	);
}

// ── Basic Auth ─────────────────────────────────────────────────────────
