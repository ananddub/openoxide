import {useState, useEffect} from 'react';
import {Cpu, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface DatabaseAdvancedResourcesProps {
	database: any;
	onUpdated: () => void;
}

export function DatabaseAdvancedResources({
	database,
	onUpdated,
}: DatabaseAdvancedResourcesProps) {
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

	const [replicas, setReplicas] = useState(
		String(database?.replicas || '1'),
	);
	const [memRes, setMemRes] = useState(database?.memory_reservation || '');
	const [memLimit, setMemLimit] = useState(database?.memory_limit || '');
	const [cpuRes, setCpuRes] = useState(database?.cpu_reservation || '');
	const [cpuLimit, setCpuLimit] = useState(database?.cpu_limit || '');
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		setReplicas(String(database?.replicas || '1'));
		setMemRes(database?.memory_reservation || '');
		setMemLimit(database?.memory_limit || '');
		setCpuRes(database?.cpu_reservation || '');
		setCpuLimit(database?.cpu_limit || '');
	}, [database]);

	const handleSave = async () => {
		setSaving(true);
		try {
			await patchDatabase.mutateAsync({
				params: {path: {id: database?.id}},
				body: {
					memory_reservation: memRes.trim() || undefined,
					memory_limit: memLimit.trim() || undefined,
					cpu_reservation: cpuRes.trim() || undefined,
					cpu_limit: cpuLimit.trim() || undefined,
					replicas: replicas ? parseInt(replicas) : 1,
				},
			});
			toast.success('Resource limits updated');
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
					<Cpu className="size-4 text-primary" /> Container Replicas &
					Resource Limits
				</h3>
				<p className="mt-0.5 text-xs text-muted-foreground">
					Configure instance replicas, memory reservation/limit (e.g.{' '}
					<code className="rounded bg-muted/40 px-1 py-0.5 font-mono text-[11px]">
						512m
					</code>
					,{' '}
					<code className="rounded bg-muted/40 px-1 py-0.5 font-mono text-[11px]">
						2g
					</code>
					), and CPU allocation.
				</p>
			</div>

			<div className="grid grid-cols-1 gap-4 border-t border-border/40 pt-4 md:grid-cols-2">
				<div className="flex flex-col gap-1.5">
					<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
						Replicas
					</label>
					<Input
						type="number"
						min="1"
						max="10"
						value={replicas}
						onChange={e => setReplicas(e.target.value)}
						className="h-9 text-xs"
					/>
				</div>

				<div className="flex flex-col gap-1.5">
					<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
						Memory Reservation
					</label>
					<Input
						placeholder="e.g. 256m"
						value={memRes}
						onChange={e => setMemRes(e.target.value)}
						className="h-9 text-xs"
					/>
				</div>

				<div className="flex flex-col gap-1.5">
					<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
						Memory Limit
					</label>
					<Input
						placeholder="e.g. 1g"
						value={memLimit}
						onChange={e => setMemLimit(e.target.value)}
						className="h-9 text-xs"
					/>
				</div>

				<div className="flex flex-col gap-1.5">
					<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
						CPU Reservation
					</label>
					<Input
						placeholder="e.g. 0.2"
						value={cpuRes}
						onChange={e => setCpuRes(e.target.value)}
						className="h-9 text-xs"
					/>
				</div>

				<div className="flex flex-col gap-1.5">
					<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
						CPU Limit
					</label>
					<Input
						placeholder="e.g. 1.0"
						value={cpuLimit}
						onChange={e => setCpuLimit(e.target.value)}
						className="h-9 text-xs"
					/>
				</div>
			</div>

			<div className="flex justify-end border-t border-border/40 pt-4">
				<Button
					onClick={handleSave}
					disabled={saving}
					className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
					<Save className="size-3.5" />{' '}
					{saving ? 'Saving...' : 'Save Resources'}
				</Button>
			</div>
		</section>
	);
}
