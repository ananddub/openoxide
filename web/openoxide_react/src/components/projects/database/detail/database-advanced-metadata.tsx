import {useEffect, useState} from 'react';
import {FilePenLine} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field} from '#/components/shared/field';
import {toast} from 'sonner';
import {useDatabaseUpdate, parseDatabaseKind} from '#/api/live';
import {formatApiError} from '#/api/utils';

export function DatabaseAdvancedMetadata({database, onUpdated}: any) {
	const patchDatabase = useDatabaseUpdate();
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		setName(database?.name || '');
		setDescription(database?.description || '');
	}, [database]);

	const save = async () => {
		if (!name.trim()) return toast.error('Database name is required');
		setSaving(true);
		try {
			await patchDatabase.mutateAsync({
				kind: parseDatabaseKind(database?.kind || 'postgres'),
				id: Number(database.id),
				name: name.trim(),
				description: description.trim(),
			});
			toast.success('Database details updated');
			onUpdated();
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setSaving(false);
		}
	};

	return (
		<SettingsSheet
			title="Database Details"
			description="Display name and description"
			icon={<FilePenLine className="size-4" />}
			footer={
				<Button
					onClick={save}
					disabled={saving}
					className="h-9 gap-1.5 px-4 text-sm">
					{saving ? 'Saving...' : 'Save changes'}
				</Button>
			}>
			<Field
				label="Display name"
				hint="How this database is listed across the dashboard.">
				<Input
					value={name}
					onChange={event => setName(event.target.value)}
					placeholder="my-primary-db"
				/>
			</Field>
			<Field label="Description" hint="Optional notes about what this database holds.">
				<Textarea
					value={description}
					onChange={event => setDescription(event.target.value)}
					placeholder="Postgres cluster serving the payments service"
					className="min-h-28 resize-y"
				/>
			</Field>
		</SettingsSheet>
	);
}
