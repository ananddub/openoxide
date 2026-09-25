import {useState} from 'react';
import {Shuffle} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Switch} from '#/components/ui/switch';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field, SheetGroup, SheetNote} from '#/components/shared/field';
import {formatApiError} from '#/api/utils';
import {useComposeUpdate} from '#/api/live';
import type {ComposeAdvancedTabProps} from './types';

export function RandomizeSection({
	compose,
	onUpdated,
}: ComposeAdvancedTabProps) {
	const update = useComposeUpdate();
	const [randomize, setRandomize] = useState<boolean>(
		!!compose?.randomize,
	);
	const [saving, setSaving] = useState(false);

	const dirty = randomize !== !!compose?.randomize;

	const save = async () => {
		setSaving(true);
		try {
			await update.mutateAsync({id: compose.id, randomize});
			toast.success(
				randomize
					? 'Service-name randomization enabled'
					: 'Service-name randomization disabled',
			);
			onUpdated?.();
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<SettingsSheet
			title="Service Names"
			description="Whether deploys rename services to avoid collisions"
			icon={<Shuffle className="size-4" />}>
			<SheetGroup
				title="Randomize service names"
				description="When enabled, every deploy renames each service, volume, network, config and secret to `{name}-<suffix>` so multiple copies of this stack can share one Docker host.">
				{compose?.randomize && compose?.suffix && (
					<SheetNote tone="info">
						Current suffix:
						<code className="mx-1 rounded bg-muted/60 px-1 py-0.5 font-mono text-[12px]">
							{compose.suffix}
						</code>
					</SheetNote>
				)}
				<Field label="Randomize on deploy">
					<div className="flex items-center gap-3">
						<Switch checked={randomize} onCheckedChange={setRandomize} />
						<span className="text-xs text-muted-foreground">
							{randomize
								? `Services deploy as {name}-${compose?.suffix || '<suffix>'}`
								: 'Services keep their literal compose names'}
						</span>
					</div>
				</Field>
				{dirty && (
					<Button
						onClick={save}
						disabled={saving}
						className="h-9 gap-1.5 self-start px-4 text-sm">
						{saving ? 'Saving...' : 'Save changes'}
					</Button>
				)}
			</SheetGroup>
		</SettingsSheet>
	);
}
