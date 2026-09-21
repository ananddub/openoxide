import {useState, useEffect} from 'react';
import {Save, Settings2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Switch} from '#/components/ui/switch';
import {useApplicationUpdateBuildConfig} from '#/api/live';
import {AdvancedSection} from '#/components/projects/detail/advanced/shared';

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
		<AdvancedSection
			icon={<Settings2 className="size-4 text-primary" />}
			title="General"
			description="Container start command, arguments, and git watch behaviour.">
			<div className="grid gap-4 sm:grid-cols-2">
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Start Command
					</Label>
					<Input
						placeholder="node server.js"
						value={command}
						onChange={e => setCommand(e.target.value)}
						className="font-mono text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Command Arguments
					</Label>
					<Input
						placeholder="--port 3000"
						value={args}
						onChange={e => setArgs(e.target.value)}
						className="font-mono text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Watch Paths
					</Label>
					<Input
						placeholder="src, package.json"
						value={watchPaths}
						onChange={e => setWatchPaths(e.target.value)}
						className="font-mono text-xs"
					/>
					<p className="text-[11px] text-muted-foreground">
						Comma-separated paths that trigger a rebuild on push. Empty
						watches the whole repository.
					</p>
				</div>
				<div className="flex items-center justify-between rounded-lg border border-border/40 px-3 py-2.5">
					<div>
						<Label className="text-xs font-bold text-muted-foreground">
							Git Submodules
						</Label>
						<p className="text-[11px] text-muted-foreground">
							Clone submodules during build
						</p>
					</div>
					<Switch checked={submodules} onCheckedChange={setSubmodules} />
				</div>
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

// ── Ports ─────────────────────────────────────────────────────────────
