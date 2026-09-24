import {useState} from 'react';
import type {ReactNode} from 'react';
import {
	Save,
	Check,
	Server as ServerIcon,
	List as ListIcon,
	LayoutGrid,
	Boxes,
	Network,
	Search as SearchIcon,
} from 'lucide-react';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	useApplicationUpdateResourceConfig,
	useDatabaseUpdate,
	parseDatabaseKind,
} from '#/api/live';
import {cn} from '#/api/utils';
import {ServiceForm} from './service-form';
import {DbEngineList, DbEngineCard} from './database-fields';
import {useCreateService} from './use-create-service';
import {DB_KINDS, OPTIONS, type ServiceKind} from './constants';

const FORM_ID = 'create-service-sheet-form';

// ─── Checkpoint wizard: engine → server → resources → details ────────────────

const PRESETS = [
	{
		id: 'micro',
		label: 'Micro',
		cpu: '0.25',
		mem: '256m',
		desc: 'Background workers and tiny services.',
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
		desc: 'Busy APIs and databases.',
	},
	{
		id: 'large',
		label: 'Large',
		cpu: '2',
		mem: '2048m',
		desc: 'Heavy builds, analytics, large datasets.',
	},
];

type Step = 'stack' | 'engine' | 'server' | 'resources' | 'details';

const STEPS_FOR: Record<ServiceKind, Step[]> = {
	database: ['engine', 'server', 'resources', 'details'],
	application: ['server', 'resources', 'details'],
	compose: ['stack', 'server', 'details'],
	template: ['details'],
};

const STEP_LABEL: Record<Step, string> = {
	stack: 'Stack',
	engine: 'Engine',
	server: 'Server',
	resources: 'Resources',
	details: 'Details',
};

function Checkpoints({steps, idx}: {steps: Step[]; idx: number}) {
	return (
		<div className="flex items-center gap-x-3 pt-1">
			{steps.map((s, i) => (
				<span key={s} className="flex items-center gap-x-3">
					{i > 0 && (
						<span
							className={cn(
								'h-px w-4 shrink-0',
								i <= idx ? 'bg-primary/50' : 'bg-border',
							)}
						/>
					)}
					<span
						className={cn(
							'flex items-center gap-1.5 text-[11px] whitespace-nowrap',
							i === idx
								? 'font-medium text-foreground'
								: 'text-muted-foreground',
						)}>
						<span
							className={cn(
								'size-1.5 rounded-full',
								i < idx
									? 'bg-primary'
									: i === idx
										? 'bg-primary ring-2 ring-primary/25'
										: 'bg-muted-foreground/40',
							)}
						/>
						{STEP_LABEL[s]}
					</span>
				</span>
			))}
		</div>
	);
}

function SelectRow({
	selected,
	onClick,
	icon,
	title,
	subtitle,
}: {
	selected: boolean;
	onClick: () => void;
	icon?: ReactNode;
	title: string;
	subtitle?: string;
}) {
	return (
		<button
			type="button"
			onClick={onClick}
			className={cn(
				'flex w-full items-center gap-3.5 rounded-xl border px-4 py-3.5 text-start transition-all duration-150',
				selected
					? 'border-primary/40 bg-primary/10'
					: 'border-border/50 hover:border-border hover:bg-muted/30',
			)}>
			{icon && (
				<span className="flex size-8 shrink-0 items-center justify-center text-muted-foreground">
					{icon}
				</span>
			)}
			<span className="min-w-0 flex-1">
				<span className="block text-sm font-medium text-foreground">
					{title}
				</span>
				{subtitle && (
					<span className="block truncate text-[13px] text-muted-foreground">
						{subtitle}
					</span>
				)}
			</span>
			{selected && (
				<span className="flex size-5 shrink-0 animate-in items-center justify-center rounded-full bg-primary text-primary-foreground duration-150 zoom-in-75">
					<Check className="size-3" />
				</span>
			)}
		</button>
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

export function CreateServiceSheet({
	open,
	projectId,
	envId,
	kind,
	onClose,
	onCreated,
}: {
	open: boolean;
	projectId: number;
	envId?: number;
	kind: ServiceKind;
	onClose: () => void;
	onCreated: () => void;
}) {
	const steps = STEPS_FOR[kind];
	const [stepIdx, setStepIdx] = useState(0);
	const step = steps[stepIdx];
	const last = stepIdx === steps.length - 1;

	// Nothing pre-selected: Continue stays disabled until a choice is made,
	// exactly like Vercel's Browse Storage modal.
	const [picked, setPicked] = useState<string | null>(null);
	// Compose orchestrator choice (docker-compose vs docker stack/swarm),
	// asked up-front like the database engine step.
	const [stackPicked, setStackPicked] = useState<string | null>(null);
	const [serverQuery, setServerQuery] = useState('');
	const [engineView, setEngineView] = useState<'list' | 'grid'>('list');
	const [plan, setPlan] = useState<string | null>('small');
	const [customCpu, setCustomCpu] = useState('');
	const [customMem, setCustomMem] = useState('');

	const form = useCreateService({
		projectId,
		envParam: envId,
		typeParam: kind,
		onCreated: newId => {
			void applyLimits(newId).finally(() => {
				onCreated();
				onClose();
			});
		},
	});

	const appResources = useApplicationUpdateResourceConfig();
	const dbUpdate = useDatabaseUpdate();

	const applyLimits = async (newId?: number | bigint) => {
		if (!newId || !plan || plan === 'default') return;
		const preset = PRESETS.find(p => p.id === plan);
		const cpu =
			plan === 'custom' ? customCpu.trim() || undefined : preset?.cpu;
		const mem =
			plan === 'custom' ? customMem.trim() || undefined : preset?.mem;
		if (!cpu && !mem) return;
		try {
			if (kind === 'application') {
				await appResources.mutateAsync({
					id: newId,
					cpu_limit: cpu,
					memory_limit: mem,
				});
			} else if (kind === 'database') {
				await dbUpdate.mutateAsync({
					kind: parseDatabaseKind(form.dbKind),
					id: newId,
					cpu_limit: cpu,
					memory_limit: mem,
				});
			}
		} catch {
			// Service is already created — limits can be set later in Advanced.
		}
	};

	const option = OPTIONS.find(o => o.id === kind);
	const {
		submitting,
		name,
		selectedEnv,
		selectedEnvId,
		dbKind,
		setDbKind,
		setDockerImage,
		serverId,
		setServerId,
		servers,
		setComposeType,
	} = form;

	const canContinue =
		step === 'stack'
			? !!stackPicked
			: step === 'engine'
				? !!picked
				: step === 'resources'
					? !!plan
					: true;

	const next = () => {
		if (step === 'stack' && stackPicked) {
			setComposeType(stackPicked);
		}
		if (step === 'engine' && picked) {
			const k = DB_KINDS.find(x => x.id === picked);
			if (k) {
				setDbKind(k.id);
				setDockerImage(k.defaultImage);
			}
		}
		setStepIdx(i => Math.min(i + 1, steps.length - 1));
	};

	const serverQ = serverQuery.trim().toLowerCase();
	const visibleServers = servers.filter((s: any) => {
		if (!serverQ) return true;
		return (
			String(s?.name ?? '')
				.toLowerCase()
				.includes(serverQ) ||
			String(s?.ip_address ?? '')
				.toLowerCase()
				.includes(serverQ)
		);
	});
	const showLocal = !serverQ || 'local this server'.includes(serverQ);

	const stepDesc: Record<Step, string> = {
		stack: 'Choose how your stack is orchestrated.',
		engine: 'Create databases that you can connect to your services.',
		server: 'Choose where this service will run.',
		resources: 'Pick a plan or set custom CPU and memory limits.',
		details: option?.description || 'Configure and deploy a new service.',
	};

	return (
		<Sheet open={open} onOpenChange={o => !o && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>New {option?.title || 'Service'}</SheetTitle>
					<SheetDescription>{stepDesc[step]}</SheetDescription>
					{steps.length > 1 && <Checkpoints steps={steps} idx={stepIdx} />}
				</SheetHeader>

				<div className="flex-1 overflow-y-auto px-6 py-6">
					<div
						key={step}
						className="animate-in duration-200 ease-out fade-in-0 slide-in-from-right-4">
						{step === 'stack' && (
							<div className="flex flex-col gap-2">
								<SelectRow
									selected={stackPicked === 'DOCKER-COMPOSE'}
									onClick={() => setStackPicked('DOCKER-COMPOSE')}
									icon={<Boxes className="size-5" />}
									title="Docker Compose"
									subtitle="Run the stack on a single server."
								/>
								<SelectRow
									selected={stackPicked === 'STACK'}
									onClick={() => setStackPicked('STACK')}
									icon={<Network className="size-5" />}
									title="Docker Stack (Swarm)"
									subtitle="Run the stack across your cluster nodes."
								/>
							</div>
						)}

						{step === 'engine' && (
							<div className="flex flex-col gap-3">
								<div className="flex justify-end gap-0.5">
									{(
										[
											['list', ListIcon],
											['grid', LayoutGrid],
										] as const
									).map(([v, VIcon]) => (
										<button
											key={v}
											type="button"
											aria-label={`${v} view`}
											onClick={() => setEngineView(v)}
											className={cn(
												'flex size-7 items-center justify-center rounded-md transition-colors',
												engineView === v
													? 'bg-muted/60 text-foreground'
													: 'text-muted-foreground hover:bg-muted/30 hover:text-foreground',
											)}>
											<VIcon className="size-4" />
										</button>
									))}
								</div>
								<DbEngineList
									selected={picked ?? ''}
									onSelect={k => setPicked(k.id)}
									view={engineView}
								/>
							</div>
						)}

						{step === 'server' && (
							<div className="flex flex-col gap-2">
								{/* Servers are dynamic — search keeps long fleets usable. */}
								<div className="relative mb-1">
									<SearchIcon className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
									<Input
										placeholder="Search servers…"
										value={serverQuery}
										onChange={e => setServerQuery(e.target.value)}
										className="pl-9"
									/>
								</div>
								<div className="flex max-h-[min(55vh,480px)] flex-col gap-2 overflow-y-auto pr-0.5">
									{showLocal && (
										<SelectRow
											selected={serverId === 'local'}
											onClick={() => setServerId('local')}
											icon={<ServerIcon className="size-5" />}
											title="Local"
											subtitle="This server"
										/>
									)}
									{visibleServers.map((s: any) => (
										<SelectRow
											key={String(s.id)}
											selected={serverId === String(s.id)}
											onClick={() => setServerId(String(s.id))}
											icon={<ServerIcon className="size-5" />}
											title={s.name}
											subtitle={s.ip_address}
										/>
									))}
									{!showLocal && visibleServers.length === 0 && (
										<p className="py-6 text-center text-[13px] text-muted-foreground">
											No servers match "{serverQuery.trim()}".
										</p>
									)}
								</div>
							</div>
						)}

						{step === 'resources' && (
							<div className="flex flex-col gap-2">
								{PRESETS.map(p => (
									<PlanCard
										key={p.id}
										selected={plan === p.id}
										onClick={() => setPlan(p.id)}
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
									<div className="mt-1 grid animate-in grid-cols-2 gap-3 duration-200 fade-in-0 slide-in-from-top-2">
										<div className="flex flex-col gap-1.5">
											<span className="text-xs font-semibold text-foreground">
												vCPU
											</span>
											<Input
												placeholder="0.5"
												value={customCpu}
												onChange={e => setCustomCpu(e.target.value)}
											/>
										</div>
										<div className="flex flex-col gap-1.5">
											<span className="text-xs font-semibold text-foreground">
												Memory
											</span>
											<Input
												placeholder="512m"
												value={customMem}
												onChange={e => setCustomMem(e.target.value)}
											/>
										</div>
									</div>
								)}
								<PlanCard
									selected={plan === 'default'}
									onClick={() => setPlan('default')}
									title="No limits"
									desc="Use the server defaults. Change anytime in Advanced."
								/>
							</div>
						)}

						{step === 'details' && (
							<div className="flex flex-col gap-5">
								{kind === 'database' && <DbEngineCard kind={dbKind} />}
								<ServiceForm
									form={form}
									sheet
									hideServer
									formId={FORM_ID}
								/>
							</div>
						)}
					</div>
				</div>

				{last ? (
					<SheetFooter className="justify-between">
						<p className="self-center text-xs text-muted-foreground">
							Deploying into{' '}
							<span className="font-semibold text-foreground">
								{selectedEnv?.name || 'default'}
							</span>
						</p>
						<div className="flex items-center gap-3">
							{stepIdx > 0 && (
								<Button
									variant="outline"
									onClick={() => setStepIdx(i => i - 1)}>
									Back
								</Button>
							)}
							<Button
								type="submit"
								form={FORM_ID}
								disabled={submitting || !name.trim() || !selectedEnvId}
								className="h-10 gap-1.5 px-5">
								<Save className="size-3.5" />
								{submitting
									? 'Creating…'
									: `Create ${option?.title.toLowerCase() || 'service'}`}
							</Button>
						</div>
					</SheetFooter>
				) : (
					<SheetFooter>
						{stepIdx > 0 && (
							<Button
								variant="outline"
								onClick={() => setStepIdx(i => i - 1)}>
								Back
							</Button>
						)}
						<Button disabled={!canContinue} onClick={next}>
							Continue
						</Button>
					</SheetFooter>
				)}
			</SheetContent>
		</Sheet>
	);
}
