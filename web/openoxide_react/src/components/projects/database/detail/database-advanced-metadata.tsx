import {useEffect, useState} from 'react';
import {FilePenLine, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
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
		<section className="flex flex-col gap-5 rounded-xl bg-card p-6 shadow-sm">
			<div>
				<h3 className="flex items-center gap-2 text-base font-bold">
					<FilePenLine className="size-4 text-primary" /> Database Details
				</h3>
				<p className="mt-0.5 text-xs text-muted-foreground">
					Update the display name and description shown across the project.
				</p>
			</div>
			<div className="flex flex-col gap-4 border-t  pt-4">
				<label className="flex flex-col gap-1.5 text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
					Name
					<Input
						value={name}
						onChange={event => setName(event.target.value)}
						className="h-9 text-xs normal-case"
					/>
				</label>
				<label className="flex flex-col gap-1.5 text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
					Description
					<Textarea
						value={description}
						onChange={event => setDescription(event.target.value)}
						className="min-h-24 resize-y text-xs normal-case"
					/>
				</label>
			</div>
			<div className="flex justify-end border-t  pt-4">
				<Button
					onClick={save}
					disabled={saving}
					className="h-9 gap-1.5 text-xs">
					<Save className="size-3.5" />{' '}
					{saving ? 'Saving...' : 'Save Details'}
				</Button>
			</div>
		</section>
	);
}
