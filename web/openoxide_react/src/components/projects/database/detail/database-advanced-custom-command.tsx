import {useState, useEffect} from 'react';
import {Terminal, Plus, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field, SheetGroup} from '#/components/shared/field';
import {toast} from 'sonner';
import {useDatabaseUpdate, parseDatabaseKind} from '#/api/live';
import {formatApiError} from '#/api/utils';

interface DatabaseAdvancedCustomCommandProps {
	database: any;
	onUpdated: () => void;
}

export function DatabaseAdvancedCustomCommand({
	database,
	onUpdated,
}: DatabaseAdvancedCustomCommandProps) {
	const kind = (
		database?.kind ||
		database?.database_kind ||
		'postgres'
	).toLowerCase();
	const dbKind = parseDatabaseKind(kind);
	const patchDatabase = useDatabaseUpdate();

	const [dockerImage, setDockerImage] = useState(
		database?.docker_image || '',
	);
	const [command, setCommand] = useState(database?.command || '');
	const [argsList, setArgsList] = useState<string[]>(
		Array.isArray(database?.args) ? database.args : [],
	);
	const [envVar, setEnvVar] = useState(database?.env_var || '');
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		setDockerImage(database?.docker_image || '');
		setCommand(database?.command || '');
		setArgsList(Array.isArray(database?.args) ? database.args : []);
		setEnvVar(database?.env_var || '');
	}, [database]);

	const handleAddArg = () => setArgsList(prev => [...prev, '']);

	const handleUpdateArg = (index: number, val: string) => {
		setArgsList(prev => {
			const next = [...prev];
			next[index] = val;
			return next;
		});
	};

	const handleRemoveArg = (index: number) =>
		setArgsList(prev => prev.filter((_, i) => i !== index));

	const handleSave = async () => {
		setSaving(true);
		try {
			const filteredArgs = argsList.filter(a => a.trim().length > 0);
			await patchDatabase.mutateAsync({
				kind: dbKind,
				id: database?.id,
				docker_image: dockerImage.trim() || undefined,
				command: command.trim() || undefined,
				args: filteredArgs.length > 0 ? filteredArgs : undefined,
				env_var: envVar.trim() || undefined,
			});
			toast.success('Custom image & command updated');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<SettingsSheet
			title="Docker Image, Command & Arguments"
			description="Custom image tag and startup overrides"
			icon={<Terminal className="size-4" />}
			footer={
				<Button
					onClick={handleSave}
					disabled={saving}
					className="h-9 gap-1.5 px-4 text-sm">
					{saving ? 'Saving...' : 'Save changes'}
				</Button>
			}>
			<SheetGroup
				title="Image & entrypoint"
				description="Override the container image and startup command.">
				<div className="flex flex-col gap-5">
					<Field label="Docker image tag" hint="Leave blank to keep the default.">
						<Input
							placeholder="postgres:16-alpine"
							value={dockerImage}
							onChange={e => setDockerImage(e.target.value)}
							className="font-mono"
						/>
					</Field>
					<Field label="Entrypoint command">
						<Input
							placeholder="docker-entrypoint.sh"
							value={command}
							onChange={e => setCommand(e.target.value)}
							className="font-mono"
						/>
					</Field>
				</div>
			</SheetGroup>

			<SheetGroup
				title="Command arguments"
				description="Extra flags passed to the entrypoint, one per line.">
				{argsList.length === 0 ? (
					<div className="rounded-lg border border-dashed border-border bg-muted/20 px-4 py-6 text-center text-[13px] text-muted-foreground">
						No custom arguments — default container parameters are used.
					</div>
				) : (
					<div className="flex flex-col gap-2.5">
						{argsList.map((arg, idx) => (
							<div key={idx} className="flex items-center gap-2">
								<Input
									placeholder="-c shared_buffers=256MB"
									value={arg}
									onChange={e => handleUpdateArg(idx, e.target.value)}
									className="font-mono"
								/>
								<Button
									type="button"
									variant="ghost"
									size="icon"
									onClick={() => handleRemoveArg(idx)}
									className="size-10 shrink-0 text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
									<Trash2 className="size-4" />
								</Button>
							</div>
						))}
					</div>
				)}
				<Button
					type="button"
					variant="outline"
					onClick={handleAddArg}
					className="h-9 w-fit gap-1.5 text-sm">
					<Plus className="size-4" /> Add argument
				</Button>
			</SheetGroup>
		</SettingsSheet>
	);
}
