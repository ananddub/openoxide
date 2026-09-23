import {useState, useEffect} from 'react';
import {Settings2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Switch} from '#/components/ui/switch';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field, SheetGroup} from '#/components/shared/field';
import {useApplicationUpdateBuildConfig} from '#/api/live';

import type {AdvancedTabProps} from './types';

import {formatApiError} from '#/api/utils';

export function GeneralSection({app, onUpdated}: AdvancedTabProps) {
	const update = useApplicationUpdateBuildConfig();
	const [command, setCommand] = useState(app.command || '');
	const [args, setArgs] = useState(app.args || '');
	const [watchPaths, setWatchPaths] = useState(app.watch_paths || '');
	const [submodules, setSubmodules] = useState(
		Boolean(app.enable_submodules),
	);
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		setCommand(app.command || '');
		setArgs(app.args || '');
		setWatchPaths(app.watch_paths || '');
		setSubmodules(Boolean(app.enable_submodules));
	}, [app.command, app.args, app.watch_paths, app.enable_submodules]);

	const save = async () => {
		setSaving(true);
		try {
			await update.mutateAsync({
				id: app.id,
				command,
				args,
				watch_paths: watchPaths,
				enable_submodules: submodules,
			});
			toast.success('General settings saved');
			onUpdated();
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<SettingsSheet
			title="General"
			description="Container command, arguments and git watch behaviour"
			icon={<Settings2 className="size-4" />}
			footer={
				<Button
					onClick={save}
					disabled={saving}
					className="h-9 gap-1.5 px-4 text-sm">
					{saving ? 'Saving...' : 'Save changes'}
				</Button>
			}>
			<SheetGroup
				title="Container command"
				description="Override how the container starts and what it watches.">
				<div className="flex flex-col gap-5">
					<Field
						label="Start command"
						hint="Leave blank to use the image's default entrypoint.">
						<Input
							placeholder="node server.js"
							value={command}
							onChange={e => setCommand(e.target.value)}
							className="font-mono"
						/>
					</Field>
					<Field label="Command arguments">
						<Input
							placeholder="--port 3000"
							value={args}
							onChange={e => setArgs(e.target.value)}
							className="font-mono"
						/>
					</Field>
					<Field
						label="Watch paths"
						hint="Comma-separated paths that trigger a rebuild on push. Empty watches the whole repository.">
						<Input
							placeholder="src, package.json"
							value={watchPaths}
							onChange={e => setWatchPaths(e.target.value)}
							className="font-mono"
						/>
					</Field>
				</div>
			</SheetGroup>

			<SheetGroup
				title="Git submodules"
				description="Clone submodules alongside the repository during build.">
				<div className="flex items-center justify-between gap-4 rounded-xl border border-border/50 px-4 py-3.5">
					<div className="min-w-0">
						<div className="text-sm font-medium text-foreground">
							Enable submodules
						</div>
						<div className="text-[13px] text-muted-foreground">
							Pull submodule repositories during the build.
						</div>
					</div>
					<Switch
						checked={submodules}
						onCheckedChange={setSubmodules}
						className="shrink-0"
					/>
				</div>
			</SheetGroup>
		</SettingsSheet>
	);
}
