import {useState, useEffect} from 'react';
import {Save, Zap} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {useApplicationUpdateResourceConfig} from '#/api/live';
import {AdvancedSection} from '#/components/projects/detail/advanced/shared';

import type {AdvancedTabProps} from './types';

import {formatApiError} from '#/api/utils';

export function ResourceSection({app, onUpdated}: AdvancedTabProps) {
	const [memRes, setMemRes] = useState(app.memory_reservation || '');
	const [memLimit, setMemLimit] = useState(app.memory_limit || '');
	const [cpuRes, setCpuRes] = useState(app.cpu_reservation || '');
	const [cpuLimit, setCpuLimit] = useState(app.cpu_limit || '');
	const [replicas, setReplicas] = useState(String(app.replicas || '1'));
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		setMemRes(app.memory_reservation || '');
		setMemLimit(app.memory_limit || '');
		setCpuRes(app.cpu_reservation || '');
		setCpuLimit(app.cpu_limit || '');
		setReplicas(String(app.replicas || '1'));
	}, [
		app.memory_reservation,
		app.memory_limit,
		app.cpu_reservation,
		app.cpu_limit,
		app.replicas,
	]);

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
		<AdvancedSection
			icon={<Zap className="size-4 text-primary" />}
			title="Container Replicas & Resource Limits"
			description="Memory reservation/limit (e.g. 512m, 2g) and CPU allocation.">
			<div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Memory Reservation
					</Label>
					<Input
						placeholder="128m"
						value={memRes}
						onChange={e => setMemRes(e.target.value)}
						className="text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Memory Limit
					</Label>
					<Input
						placeholder="512m"
						value={memLimit}
						onChange={e => setMemLimit(e.target.value)}
						className="text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						CPU Reservation
					</Label>
					<Input
						placeholder="0.1"
						value={cpuRes}
						onChange={e => setCpuRes(e.target.value)}
						className="text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						CPU Limit
					</Label>
					<Input
						placeholder="0.5"
						value={cpuLimit}
						onChange={e => setCpuLimit(e.target.value)}
						className="text-xs"
					/>
				</div>
			</div>
			<div className="flex w-32 flex-col gap-1.5">
				<Label className="text-xs font-bold text-muted-foreground">
					Replicas
				</Label>
				<Input
					type="number"
					min="1"
					max="10"
					value={replicas}
					onChange={e => setReplicas(e.target.value)}
					className="text-xs"
				/>
			</div>
			<div className="flex justify-end">
				<Button
					onClick={handleSaveResources}
					disabled={saving}
					className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
					<Save className="h-3.5 w-3.5" />{' '}
					{saving ? 'Saving…' : 'Save Resource Limits'}
				</Button>
			</div>
		</AdvancedSection>
	);
}

// ── Danger zone ────────────────────────────────────────────────────────
