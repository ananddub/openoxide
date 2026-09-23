import {useState, useEffect} from 'react';
import {Terminal} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Textarea} from '#/components/ui/textarea';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field, SheetGroup, SheetNote} from '#/components/shared/field';
import {formatApiError} from '#/api/utils';
import {useComposeUpdate} from '#/api/live';
import type {ComposeAdvancedTabProps} from './types';

export function CommandSection({
	compose,
	onUpdated,
}: ComposeAdvancedTabProps) {
	const update = useComposeUpdate();
	const [command, setCommand] = useState(compose.command || '');
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		setCommand(compose.command || '');
	}, [compose.command]);

	const save = async () => {
		setSaving(true);
		try {
			await update.mutateAsync({id: compose.id, command: command || ''});
			toast.success('Deployment command saved');
			onUpdated?.();
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<SettingsSheet
			title="Command"
			description="Shell command run instead of the default compose up"
			icon={<Terminal className="size-4" />}
			footer={
				<Button
					onClick={save}
					disabled={saving}
					className="h-9 gap-1.5 px-4 text-sm">
					{saving ? 'Saving...' : 'Save changes'}
				</Button>
			}>
			<SheetGroup
				title="Deployment command"
				description="Overrides how the stack is brought up at deploy time.">
				<SheetNote tone="info">
					Leave empty to use the default
					<code className="mx-1 rounded bg-muted/60 px-1 py-0.5 font-mono text-[12px]">
						docker compose up -d
					</code>
					flow.
				</SheetNote>
				<Field label="Command">
					<Textarea
						rows={3}
						placeholder="docker compose -f docker-compose.yml up -d"
						value={command}
						onChange={e => setCommand(e.target.value)}
						className="font-mono text-xs"
					/>
				</Field>
			</SheetGroup>
		</SettingsSheet>
	);
}
