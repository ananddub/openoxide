import {useState} from 'react';
import {Shield} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	useApplicationSecurityDelete,
	useApplicationSecurityList,
	useApplicationSecurityUpsert,
	type ApplicationSecurityPayload,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {
	AddResourceButton,
	AdvancedSection,
	EmptyRow,
	ResourceRow,
} from '#/components/projects/detail/advanced/shared';

export function SecuritySection({app}: {app: any}) {
	const {data: securities = []} = useApplicationSecurityList(app.id);
	const upsert = useApplicationSecurityUpsert();
	const remove = useApplicationSecurityDelete();
	const [open, setOpen] = useState(false);
	const [editing, setEditing] =
		useState<ApplicationSecurityPayload | null>(null);
	const [saving, setSaving] = useState(false);
	const [deletingId, setDeletingId] = useState<number | null>(null);
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');

	const show = (security: ApplicationSecurityPayload | null) => {
		setEditing(security);
		setUsername(security?.username || '');
		setPassword('');
		setOpen(true);
	};

	const save = async () => {
		setSaving(true);
		try {
			await upsert.mutateAsync({
				application_id: app.id,
				id: editing?.id,
				username,
				password,
			});
			toast.success(
				editing ? 'Credentials updated' : 'Basic auth enabled',
			);
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
			toast.success('Basic auth removed');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<AdvancedSection
			icon={<Shield className="size-4 text-primary" />}
			title="Basic Auth"
			description="Gate the whole service behind HTTP basic authentication."
			action={
				<AddResourceButton
					onClick={() => show(null)}
					label={
						securities.length ? 'Add Credential' : 'Enable Basic Auth'
					}
				/>
			}>
			<div className="flex flex-col gap-2">
				{securities.length === 0 ? (
					<EmptyRow label="Basic auth is disabled." />
				) : (
					securities.map(security => (
						<ResourceRow
							key={security.id}
							title={security.username}
							subtitle={
								security.has_password ? 'Password set' : 'No password'
							}
							onEdit={() => show(security)}
							onDelete={() => del(security.id)}
							deleting={deletingId === security.id}
						/>
					))
				)}
			</div>
			<Dialog open={open} onOpenChange={setOpen}>
				<DialogContent className="sm:max-w-md">
					<DialogHeader>
						<DialogTitle>Basic Auth Credential</DialogTitle>
						<DialogDescription>
							Visitors must authenticate with these credentials.
						</DialogDescription>
					</DialogHeader>
					<div className="flex flex-col gap-3">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Username
							</Label>
							<Input
								placeholder="admin"
								value={username}
								onChange={e => setUsername(e.target.value)}
								className="text-xs"
							/>
						</div>
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Password
							</Label>
							<Input
								type="password"
								placeholder={editing ? '••••••••' : 'password'}
								value={password}
								onChange={e => setPassword(e.target.value)}
								className="text-xs"
							/>
							{editing && (
								<p className="text-[11px] text-muted-foreground">
									Leave unchanged to keep the current password.
								</p>
							)}
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
							disabled={saving || !username.trim() || !password.trim()}
							onClick={save}
							className="text-xs">
							{saving ? 'Saving…' : editing ? 'Update' : 'Enable'}
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</AdvancedSection>
	);
}

// ── Webhook ────────────────────────────────────────────────────────────
