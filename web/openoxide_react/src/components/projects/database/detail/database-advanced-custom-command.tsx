import {useState, useEffect} from 'react';
import {Terminal, Save, Plus, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {$api} from '#/api/query';
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

	let endpoint:
		| '/postgres/{id}'
		| '/mysql/{id}'
		| '/mariadb/{id}'
		| '/mongo/{id}'
		| '/redis/{id}'
		| '/libsql/{id}' = '/postgres/{id}';
	if (kind.includes('mysql')) endpoint = '/mysql/{id}';
	else if (kind.includes('mariadb')) endpoint = '/mariadb/{id}';
	else if (kind.includes('mongo')) endpoint = '/mongo/{id}';
	else if (kind.includes('redis')) endpoint = '/redis/{id}';
	else if (kind.includes('libsql')) endpoint = '/libsql/{id}';

	const patchDatabase = $api.useMutation('patch', endpoint as any);

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

	const handleAddArg = () => {
		setArgsList(prev => [...prev, '']);
	};

	const handleUpdateArg = (index: number, val: string) => {
		setArgsList(prev => {
			const next = [...prev];
			next[index] = val;
			return next;
		});
	};

	const handleRemoveArg = (index: number) => {
		setArgsList(prev => prev.filter((_, i) => i !== index));
	};

	const handleSave = async () => {
		setSaving(true);
		try {
			const filteredArgs = argsList.filter(a => a.trim().length > 0);
			await patchDatabase.mutateAsync({
				params: {path: {id: database?.id}},
				body: {
					docker_image: dockerImage.trim() || undefined,
					command: command.trim() || undefined,
					args: filteredArgs.length > 0 ? filteredArgs : undefined,
					env_var: envVar.trim() || undefined,
				},
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
		<section className="flex flex-col gap-5 rounded-2xl border border-border/80 bg-card p-6 shadow-sm">
			<div>
				<h3 className="flex items-center gap-2 text-base font-bold tracking-tight text-foreground">
					<Terminal className="size-4 text-primary" /> Docker Image,
					Command & Arguments
				</h3>
				<p className="mt-0.5 text-xs text-muted-foreground">
					Configure custom Docker image tags, startup command overrides,
					and extra engine arguments (e.g.{' '}
					<code className="rounded bg-muted/40 px-1 py-0.5 font-mono text-[11px]">
						-c max_connections=200
					</code>
					).
				</p>
			</div>

			<div className="grid grid-cols-1 gap-4 border-t border-border/40 pt-4 md:grid-cols-2">
				<div className="flex flex-col gap-1.5">
					<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
						Docker Image Tag
					</label>
					<Input
						placeholder="e.g. postgres:16-alpine"
						value={dockerImage}
						onChange={e => setDockerImage(e.target.value)}
						className="h-9 font-mono text-xs"
					/>
				</div>

				<div className="flex flex-col gap-1.5">
					<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
						Entrypoint Command
					</label>
					<Input
						placeholder="e.g. docker-entrypoint.sh"
						value={command}
						onChange={e => setCommand(e.target.value)}
						className="h-9 font-mono text-xs"
					/>
				</div>
			</div>

			{/* Extra Command Arguments List */}
			<div className="flex flex-col gap-2.5">
				<div className="flex items-center justify-between">
					<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
						Command Arguments (Args)
					</label>
					<Button
						type="button"
						variant="outline"
						size="sm"
						onClick={handleAddArg}
						className="flex h-7 items-center gap-1 px-2.5 text-[11px]">
						<Plus className="size-3" /> Add Argument
					</Button>
				</div>

				{argsList.length === 0 ? (
					<p className="rounded-lg border border-border/40 bg-muted/20 p-3 text-xs text-muted-foreground italic">
						No custom arguments configured. Default container parameters
						will be used.
					</p>
				) : (
					<div className="flex flex-col gap-2">
						{argsList.map((arg, idx) => (
							<div key={idx} className="flex items-center gap-2">
								<Input
									placeholder="e.g. -c shared_buffers=256MB"
									value={arg}
									onChange={e => handleUpdateArg(idx, e.target.value)}
									className="h-8 font-mono text-xs"
								/>
								<Button
									type="button"
									variant="ghost"
									size="icon"
									onClick={() => handleRemoveArg(idx)}
									className="size-8 text-destructive hover:bg-destructive/10">
									<Trash2 className="size-3.5" />
								</Button>
							</div>
						))}
					</div>
				)}
			</div>

			<div className="flex justify-end border-t border-border/40 pt-4">
				<Button
					onClick={handleSave}
					disabled={saving}
					className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
					<Save className="size-3.5" />{' '}
					{saving ? 'Saving...' : 'Save Custom Command'}
				</Button>
			</div>
		</section>
	);
}
