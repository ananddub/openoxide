import {useState, useEffect} from 'react';
import {Cpu, Minus, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field, FieldGrid, SheetGroup} from '#/components/shared/field';
import {cn} from '#/api/utils';
import {toast} from 'sonner';
import {useDatabaseUpdate, parseDatabaseKind} from '#/api/live';
import {formatApiError} from '#/api/utils';

interface DatabaseAdvancedResourcesProps {
	database: any;
	onUpdated: () => void;
}

// Mirrors the plan presets used when creating a service, so the same
// Micro/Small/Medium/Large language carries into the database editor.
const PRESETS = [
	{id: 'micro', label: 'Micro', cpu: '0.25', mem: '256m', desc: 'Light workloads and small datasets.'},
	{id: 'small', label: 'Small', cpu: '0.5', mem: '512m', desc: 'Most databases. Recommended.'},
	{id: 'medium', label: 'Medium', cpu: '1', mem: '1024m', desc: 'Busy APIs and analytics.'},
	{id: 'large', label: 'Large', cpu: '2', mem: '2048m', desc: 'Heavy queries and large datasets.'},
];

function derivePlan(mem: string, cpu: string): string {
	if (!mem.trim() && !cpu.trim()) return 'default';
	const match = PRESETS.find(
		p => p.mem === mem.trim() && p.cpu === cpu.trim(),
	);
	return match ? match.id : 'custom';
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
	const dbKind = parseDatabaseKind(kind);
	const patchDatabase = useDatabaseUpdate();

	const [replicas, setReplicas] = useState(
		String(database?.replicas || '1'),
	);
	const [memLimit, setMemLimit] = useState(database?.memory_limit || '');
	const [cpuLimit, setCpuLimit] = useState(database?.cpu_limit || '');
	const [plan, setPlan] = useState('default');
	const [saving, setSaving] = useState(false);

	const numReplicas = Math.min(10, Math.max(1, parseInt(replicas) || 1));

	useEffect(() => {
		const nextMem = database?.memory_limit || '';
		const nextCpu = database?.cpu_limit || '';
		setReplicas(String(database?.replicas || '1'));
		setMemLimit(nextMem);
		setCpuLimit(nextCpu);
		setPlan(derivePlan(nextMem, nextCpu));
	}, [database]);

	const choosePlan = (id: string) => {
		setPlan(id);
		if (id === 'default') {
			setMemLimit('');
			setCpuLimit('');
		} else if (id !== 'custom') {
			const preset = PRESETS.find(p => p.id === id);
			if (preset) {
				setMemLimit(preset.mem);
				setCpuLimit(preset.cpu);
			}
		}
	};

	const handleSave = async () => {
		setSaving(true);
		try {
			await patchDatabase.mutateAsync({
				kind: dbKind,
				id: database?.id,
				memory_limit: memLimit.trim() || undefined,
				cpu_limit: cpuLimit.trim() || undefined,
				replicas: replicas ? parseInt(replicas) : 1,
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
		<SettingsSheet
			title="Replicas & Resource Limits"
			description="Instance count, memory and CPU"
			icon={<Cpu className="size-4" />}
			footer={
				<Button
					onClick={handleSave}
					disabled={saving}
					className="h-9 gap-1.5 px-4 text-sm">
					{saving ? 'Saving...' : 'Save changes'}
				</Button>
			}>
			<SheetGroup
				title="Resource plan"
				description="Pick a preset or set custom CPU and memory limits.">
				<div className="flex flex-col gap-2">
					{PRESETS.map(p => (
						<PlanCard
							key={p.id}
							selected={plan === p.id}
							onClick={() => choosePlan(p.id)}
							title={p.label}
							meta={`${p.cpu} vCPU · ${p.mem}`}
							desc={p.desc}
						/>
					))}
					<PlanCard
						selected={plan === 'custom'}
						onClick={() => setPlan('custom')}
						title="Custom"
						desc="Set your own CPU and memory limits."
					/>
					{plan === 'custom' && (
						<div className="my-1 animate-in fade-in-0 slide-in-from-top-2 duration-200">
							<FieldGrid>
								<Field label="Memory limit" hint="e.g. 512m">
									<Input
										placeholder="512m"
										value={memLimit}
										onChange={e => setMemLimit(e.target.value)}
									/>
								</Field>
								<Field label="CPU limit" hint="e.g. 1.0">
									<Input
										placeholder="1.0"
										value={cpuLimit}
										onChange={e => setCpuLimit(e.target.value)}
									/>
								</Field>
							</FieldGrid>
						</div>
					)}
					<PlanCard
						selected={plan === 'default'}
						onClick={() => choosePlan('default')}
						title="No limits"
						desc="Use the server defaults. Change anytime."
					/>
				</div>
			</SheetGroup>

			<SheetGroup
				title="Replicas"
				description="Number of database instances to run.">
				<div className="flex items-center justify-between gap-4 rounded-xl border border-border/50 px-4 py-3.5">
					<div className="min-w-0">
						<div className="text-sm font-medium text-foreground">
							Instance count
						</div>
						<div className="text-[13px] text-muted-foreground">
							Set above 1 to run multiple database tasks.
						</div>
					</div>
					<div className="flex shrink-0 items-center gap-1 rounded-lg border border-border/60 p-1">
						<button
							type="button"
							onClick={() =>
								setReplicas(String(Math.max(1, numReplicas - 1)))
							}
							disabled={numReplicas <= 1}
							className="flex size-8 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground disabled:pointer-events-none disabled:opacity-40">
							<Minus className="size-4" />
						</button>
						<Input
							type="number"
							min="1"
							max="10"
							value={replicas}
							onChange={e => setReplicas(e.target.value)}
							className="h-8 w-12 border-0 bg-transparent p-0 text-center text-sm font-medium tabular-nums focus-visible:ring-0 [appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
						/>
						<button
							type="button"
							onClick={() =>
								setReplicas(String(Math.min(10, numReplicas + 1)))
							}
							disabled={numReplicas >= 10}
							className="flex size-8 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground disabled:pointer-events-none disabled:opacity-40">
							<Plus className="size-4" />
						</button>
					</div>
				</div>
			</SheetGroup>
		</SettingsSheet>
	);
}

function PlanCard({
	selected,
	onClick,
	title,
	meta,
	desc,
}: {
	selected: boolean;
	onClick: () => void;
	title: string;
	meta?: string;
	desc: string;
}) {
	return (
		<button
			type="button"
			onClick={onClick}
			className={cn(
				'flex w-full items-center gap-3 rounded-xl border px-4 py-3.5 text-start transition-all duration-150',
				selected
					? 'border-primary/40 bg-primary/10'
					: 'border-border/50 hover:border-border hover:bg-muted/30',
			)}>
			<span className="min-w-0 flex-1">
				<span className="block text-sm text-foreground">
					<span className="font-medium">{title}</span>
					{meta && (
						<span className="text-[13px] text-muted-foreground">
							{' '}
							— {meta}
						</span>
					)}
				</span>
				<span className="block text-[13px] text-muted-foreground">
					{desc}
				</span>
			</span>
			<span
				className={cn(
					'flex size-5 shrink-0 items-center justify-center rounded-full border-2 transition-colors',
					selected ? 'border-primary' : 'border-muted-foreground/40',
				)}>
				{selected && (
					<span className="size-2.5 rounded-full bg-primary" />
				)}
			</span>
		</button>
	);
}
