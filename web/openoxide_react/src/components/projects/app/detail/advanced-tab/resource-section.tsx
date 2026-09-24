import {useState, useEffect} from 'react';
import {Cpu, Minus, Plus} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field, FieldGrid, SheetGroup} from '#/components/shared/field';
import {cn} from '#/api/utils';
import {useApplicationUpdateResourceConfig} from '#/api/live';

import type {AdvancedTabProps} from './types';

import {formatApiError} from '#/api/utils';

// Mirrors the plan presets used when creating a service, so the same Micro/Small/Medium/Large language carries into the app editor.
const PRESETS = [
	{
		id: 'micro',
		label: 'Micro',
		cpu: '0.25',
		mem: '256m',
		desc: 'Light workloads and background jobs.',
	},
	{
		id: 'small',
		label: 'Small',
		cpu: '0.5',
		mem: '512m',
		desc: 'Most applications. Recommended.',
	},
	{
		id: 'medium',
		label: 'Medium',
		cpu: '1',
		mem: '1024m',
		desc: 'Busy APIs and moderate traffic.',
	},
	{
		id: 'large',
		label: 'Large',
		cpu: '2',
		mem: '2048m',
		desc: 'Heavy compute and high traffic.',
	},
];

function derivePlan(memLimit: string, cpuLimit: string): string {
	if (!memLimit.trim() && !cpuLimit.trim()) return 'default';
	const match = PRESETS.find(
		p => p.mem === memLimit.trim() && p.cpu === cpuLimit.trim(),
	);
	return match ? match.id : 'custom';
}

export function ResourceSection({app, onUpdated}: AdvancedTabProps) {
	const [memRes, setMemRes] = useState(app.memory_reservation || '');
	const [memLimit, setMemLimit] = useState(app.memory_limit || '');
	const [cpuRes, setCpuRes] = useState(app.cpu_reservation || '');
	const [cpuLimit, setCpuLimit] = useState(app.cpu_limit || '');
	const [replicas, setReplicas] = useState(String(app.replicas || '1'));
	const [plan, setPlan] = useState('default');
	const [saving, setSaving] = useState(false);

	const numReplicas = Math.min(10, Math.max(1, parseInt(replicas) || 1));

	useEffect(() => {
		const nextMem = app.memory_limit || '';
		const nextCpu = app.cpu_limit || '';
		setMemRes(app.memory_reservation || '');
		setMemLimit(nextMem);
		setCpuRes(app.cpu_reservation || '');
		setCpuLimit(nextCpu);
		setReplicas(String(app.replicas || '1'));
		setPlan(derivePlan(nextMem, nextCpu));
	}, [
		app.memory_reservation,
		app.memory_limit,
		app.cpu_reservation,
		app.cpu_limit,
		app.replicas,
	]);

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

	const patchResources = useApplicationUpdateResourceConfig();

	const handleSaveResources = async () => {
		setSaving(true);
		try {
			await patchResources.mutateAsync({
				id: app.id,
				memory_reservation: memRes || undefined,
				memory_limit: memLimit || undefined,
				cpu_reservation: cpuRes || undefined,
				cpu_limit: cpuLimit || undefined,
				replicas: replicas ? parseInt(replicas) : 1,
			});
			toast.success('Resource limits saved successfully');
			onUpdated();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<SettingsSheet
			title="Resources & Replicas"
			description="Memory, CPU and instance count"
			icon={<Cpu className="size-4" />}
			meta={
				<span className="rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">
					{numReplicas}×
				</span>
			}
			footer={
				<Button
					onClick={handleSaveResources}
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
						desc="Set your own reservations and limits."
					/>
					{plan === 'custom' && (
						<div className="animate-in rounded-xl border border-border/50 bg-muted/20 p-4 duration-200 fade-in-0 slide-in-from-top-2">
							<FieldGrid>
								<Field label="Memory reservation">
									<Input
										placeholder="e.g. 128m"
										value={memRes}
										onChange={e => setMemRes(e.target.value)}
									/>
								</Field>
								<Field label="Memory limit">
									<Input
										placeholder="e.g. 512m"
										value={memLimit}
										onChange={e => setMemLimit(e.target.value)}
									/>
								</Field>
								<Field label="CPU reservation">
									<Input
										placeholder="e.g. 0.1"
										value={cpuRes}
										onChange={e => setCpuRes(e.target.value)}
									/>
								</Field>
								<Field label="CPU limit">
									<Input
										placeholder="e.g. 0.5"
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
				description="Number of container copies the service should run.">
				<div className="flex items-center justify-between gap-4 rounded-xl border border-border/50 px-4 py-3.5">
					<div className="min-w-0">
						<div className="text-sm font-medium text-foreground">
							Instance count
						</div>
						<div className="text-[13px] text-muted-foreground">
							Set above 1 to scale out additional tasks.
						</div>
					</div>
					<div className="flex shrink-0 items-center gap-1 rounded-lg border border-border/60 p-1">
						<button
							type="button"
							onClick={() =>
								setReplicas(String(Math.max(1, numReplicas - 1)))
							}
							disabled={numReplicas <= 1}
							aria-label="Decrease replicas"
							className="flex size-8 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground disabled:pointer-events-none disabled:opacity-40">
							<Minus className="size-4" />
						</button>
						<Input
							type="number"
							min="1"
							max="10"
							value={replicas}
							onChange={e => setReplicas(e.target.value)}
							className="h-8 w-12 [appearance:textfield] border-0 bg-transparent p-0 text-center text-sm font-medium tabular-nums focus-visible:ring-0 [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
						/>
						<button
							type="button"
							onClick={() =>
								setReplicas(String(Math.min(10, numReplicas + 1)))
							}
							disabled={numReplicas >= 10}
							aria-label="Increase replicas"
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
				{selected && <span className="size-2.5 rounded-full bg-primary" />}
			</span>
		</button>
	);
}
