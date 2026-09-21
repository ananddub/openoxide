import {useState} from 'react';
import {Save, Zap} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {formatApiError} from '#/api/utils';
import {useComposeUpdate} from '#/api/live';
import {AdvancedSection} from '#/components/projects/detail/advanced/shared';
import type {ComposeAdvancedTabProps} from './types';

export function CommandSection({
	compose,
	onUpdated,
}: ComposeAdvancedTabProps) {
	const update = useComposeUpdate();
	const [command, setCommand] = useState(compose.command || '');
	const [saving, setSaving] = useState(false);

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
		<AdvancedSection
			icon={<Zap className="size-4 text-primary" />}
			title="Deployment Command"
			description="Shell command executed instead of the default `docker compose up`. Leave empty for the default flow.">
			<div className="flex flex-col gap-1.5">
				<Label className="text-xs font-bold text-muted-foreground">
					Command
				</Label>
				<Input
					placeholder="docker compose -f docker-compose.yml up -d"
					value={command}
					onChange={e => setCommand(e.target.value)}
					className="font-mono text-xs"
				/>
			</div>
			<div className="flex justify-end">
				<Button
					onClick={save}
					disabled={saving}
					className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
					<Save className="h-3.5 w-3.5" /> {saving ? 'Saving…' : 'Save'}
				</Button>
			</div>
		</AdvancedSection>
	);
}

// ── Volumes / Mounts ───────────────────────────────────────────────────
