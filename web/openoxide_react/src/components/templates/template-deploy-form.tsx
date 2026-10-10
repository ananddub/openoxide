import {useEffect, useMemo, useRef, useState} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {toast} from 'sonner';
import {
	Check,
	CheckCircle2,
	ChevronDown,
	ChevronLeft,
	Folder,
	Layers,
	Loader2,
	Rocket,
	Search,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Input} from '#/components/ui/input';
import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from '#/components/ui/popover';
import {Field} from '#/components/shared/field';
import {
	OptionRow,
	StepTimeline,
	type WizardStep,
} from '#/components/shared/sheet-wizard';
import {TemplateLogo} from './template-logo';
import {
	templateFileUrl,
	useActiveOrgId,
	useComposeInstallTemplate,
	useEnvironmentListByProject,
	useProjectListByOrganization,
	useRemoteServerList,
	type TemplateMeta,
} from '#/api/live';
import {cn, formatApiError, isLocalServerRow} from '#/api/utils';

const STEPS: WizardStep[] = [
	{id: 'destination', label: 'Destination'},
	{id: 'server', label: 'Server'},
	{id: 'name', label: 'Service name'},
];

/** Full journey — Install sits mid-flow and Ready ends it, like new-server. */
const TIMELINE: WizardStep[] = [
	...STEPS,
	{id: 'install', label: 'Install'},
	{id: 'ready', label: 'Ready'},
];

const STEP_DESCRIPTIONS = [
	'Where this stack lives — project and environment.',
	'Which host runs it — the panel host or a connected remote.',
	'Name the compose project, then install.',
];

type Phase = 'idle' | 'installing' | 'done';

/** Where the finished stack ended up, for the Ready step's View stack button. */
type InstalledTarget = {
	projectId: string;
	composeId?: string;
};

/** One row in the server picker. */
type ServerOption = {
	id: string;
	name: string;
	subtitle: string;
};

/** Searchable server picker — popover + filter box, same grammar as the DNS record-type select. */
function ServerSelect({
	value,
	options,
	onChange,
}: {
	value: string;
	options: ServerOption[];
	onChange: (id: string) => void;
}) {
	const [open, setOpen] = useState(false);
	const [query, setQuery] = useState('');
	const inputRef = useRef<HTMLInputElement>(null);
	const current = options.find(option => option.id === value);

	const filtered = useMemo(() => {
		const q = query.trim().toLowerCase();
		if (!q) return options;
		return options.filter(
			option =>
				option.name.toLowerCase().includes(q) ||
				option.subtitle.toLowerCase().includes(q),
		);
	}, [options, query]);

	useEffect(() => {
		if (open) inputRef.current?.focus({preventScroll: true});
	}, [open]);

	const pick = (id: string) => {
		onChange(id);
		setOpen(false);
	};

	return (
		<Popover
			open={open}
			onOpenChange={next => {
				setOpen(next);
				if (!next) setQuery('');
			}}>
			<PopoverTrigger
				type="button"
				className="flex h-9 w-full cursor-pointer items-center gap-2 rounded-md border border-input bg-card px-3 text-xs shadow-[0_1px_1px_rgba(0,0,0,0.02)] transition-colors duration-150 outline-none focus-visible:border-foreground/40 focus-visible:ring-2 focus-visible:ring-ring/30">
				<span className="shrink-0 font-medium">
					{current?.name ?? 'Pick a server'}
				</span>
				<span className="min-w-0 truncate text-muted-foreground">
					{current?.subtitle}
				</span>
				<ChevronDown className="ml-auto size-4 shrink-0 text-muted-foreground opacity-70" />
			</PopoverTrigger>
			<PopoverContent className="p-0" align="start">
				<div className="flex flex-col">
					<div className="flex items-center gap-2 border-b border-border p-2">
						<Search className="size-3.5 shrink-0 text-muted-foreground" />
						<Input
							ref={inputRef}
							placeholder="Search servers…"
							value={query}
							onChange={event => setQuery(event.target.value)}
							onKeyDown={event => {
								const [first] = filtered;
								if (event.key === 'Enter' && first) {
									event.preventDefault();
									pick(first.id);
								}
							}}
							className="h-7 flex-1 border-0 bg-transparent px-0 text-[13px] shadow-none focus-visible:ring-0"
						/>
					</div>
					<div className="flex max-h-60 flex-col gap-px overflow-y-auto p-1.5">
						{filtered.map(option => (
							<button
								key={option.id}
								type="button"
								onClick={() => pick(option.id)}
								className={cn(
									'flex w-full cursor-pointer items-center gap-2.5 rounded-md px-2 py-1.5 text-left text-[13px] transition-colors outline-none hover:bg-accent focus-visible:bg-accent',
									option.id === current?.id && 'bg-accent',
								)}>
								<span className="flex min-w-0 flex-1 flex-col">
									<span className="font-medium text-foreground">
										{option.name}
									</span>
									<span className="truncate text-[11px] text-muted-foreground">
										{option.subtitle}
									</span>
								</span>
								{option.id === current?.id && (
									<Check className="size-3.5 shrink-0 text-foreground" />
								)}
							</button>
						))}
						{!filtered.length && (
							<p className="px-2 py-5 text-center text-[13px] text-muted-foreground">
								No server matches &ldquo;{query}&rdquo;.
							</p>
						)}
					</div>
				</div>
			</PopoverContent>
		</Popover>
	);
}

/**
 * One-click deploy as a checkpoint wizard: Destination → Server → Service
 * name walk the same vertical StepTimeline grammar as the new-server sheet,
 * with Install live in the middle and Ready closing the flow.
 * Rendered inline as
 * the third view of the panel that showed the template — no second sheet
 * stacks on top. When the panel was opened from inside a project (create
 * page / dashboard), `projectId`/`environmentId` are pinned and those
 * selectors are not shown again.
 */
export function TemplateDeployForm({
	template,
	onCancel,
	projectId: pinnedProjectId,
	environmentId: pinnedEnvironmentId,
}: {
	template: TemplateMeta;
	onCancel: () => void;
	projectId?: number | string;
	environmentId?: number | string;
}) {
	const navigate = useNavigate();
	const orgId = useActiveOrgId();

	const [projectId, setProjectId] = useState(() =>
		pinnedProjectId != null ? String(pinnedProjectId) : '',
	);
	const [environmentId, setEnvironmentId] = useState(() =>
		pinnedEnvironmentId != null ? String(pinnedEnvironmentId) : '',
	);
	const [serverId, setServerId] = useState<string>('local');
	const [name, setName] = useState(() =>
		template.name.toLowerCase().replace(/\s+/g, '-'),
	);
	const [stepIdx, setStepIdx] = useState(0);
	const [phase, setPhase] = useState<Phase>('idle');
	const [installed, setInstalled] = useState<InstalledTarget | null>(null);

	const {data: rawProjects} = useProjectListByOrganization(orgId);
	const projects = rawProjects ?? [];
	const {data: rawEnvs} = useEnvironmentListByProject(
		BigInt(projectId || 0),
	);
	const envs = rawEnvs ?? [];
	const {data: rawServers} = useRemoteServerList();
	// The panel host has its own row in `servers` too; the hardcoded "Local"
	// option already covers it, so drop it here to avoid listing it twice.
	const servers = (rawServers ?? []).filter(
		server => !isLocalServerRow(server.ip_address),
	);
	const install = useComposeInstallTemplate();

	const pinnedProject =
		pinnedProjectId != null
			? projects.find(p => String(p.id) === String(pinnedProjectId))
			: undefined;
	const pinnedEnv =
		pinnedEnvironmentId != null
			? envs.find(e => String(e.id) === String(pinnedEnvironmentId))
			: undefined;

	useEffect(() => {
		if (pinnedProjectId) return;
		if (!projectId && projects[0]) setProjectId(String(projects[0].id));
	}, [projects, projectId, pinnedProjectId]);

	useEffect(() => {
		if (pinnedEnvironmentId != null) return;
		if (envs.length && !envs.some(e => String(e.id) === environmentId)) {
			setEnvironmentId(String(envs[0].id));
		}
	}, [envs, environmentId, pinnedEnvironmentId]);

	// A different template re-opens the wizard from step one.
	useEffect(() => {
		setStepIdx(0);
		setPhase('idle');
		setInstalled(null);
	}, [template.id]);

	const last = stepIdx === STEPS.length - 1;
	const installing = phase === 'installing';
	const destinationReady = Boolean(projectId && environmentId);
	// The server step always has a choice (Local is preselected); only the
	// destination can genuinely block progress.
	const canContinue = stepIdx === 0 ? destinationReady : true;
	const activeIdx = installing
		? STEPS.length
		: phase === 'done'
			? TIMELINE.length - 1
			: stepIdx;
	const nodeState = installing
		? ('running' as const)
		: phase === 'done'
			? ('success' as const)
			: undefined;

	const viewStack = () => {
		if (!installed) return;
		if (installed.composeId) {
			navigate({
				to: `/projects/${installed.projectId}/compose/${installed.composeId}` as never,
			});
		} else {
			navigate({
				to: '/projects/$id',
				params: {id: installed.projectId},
			});
		}
	};

	const handleInstall = async () => {
		if (!projectId || !environmentId) return;
		setPhase('installing');
		try {
			const [composeResponse, tomlResponse] = await Promise.all([
				fetch(templateFileUrl(template.id, 'docker-compose.yml')),
				fetch(templateFileUrl(template.id, 'template.toml')),
			]);
			if (!composeResponse.ok) {
				throw new Error(
					`Could not download compose file (${composeResponse.status})`,
				);
			}
			const composeFile = await composeResponse.text();
			const toml = tomlResponse.ok ? await tomlResponse.text() : undefined;
			const response = await install.mutateAsync({
				template: composeFile,
				environment_id: BigInt(environmentId),
				name: name.trim() || undefined,
				server_id: serverId === 'local' ? undefined : Number(serverId),
				// Send the raw template.toml: the backend resolves both the
				// [variables] generators (${password:N}, ${domain}) and the
				// [config].env entries that reference them (BB_MINIO_ACCESS_KEY
				// etc.), so `${...}` interpolation in the compose file gets real
				// values instead of blanks.
				env_var: toml,
			});
			toast.success(`${template.name} installed`);
			const composeId = (response as {id?: bigint})?.id;
			setInstalled({
				projectId,
				composeId: composeId ? String(composeId) : undefined,
			});
			setPhase('done');
		} catch (error) {
			toast.error(formatApiError(error));
			setPhase('idle');
		}
	};

	const back = () => {
		if (phase !== 'idle') return;
		if (stepIdx > 0) setStepIdx(i => i - 1);
		else onCancel();
	};

	return (
		<div
			key={template.id}
			className="flex min-h-0 w-full flex-1 animate-in flex-col gap-5 duration-200 ease-out fade-in-0 slide-in-from-right-4">
			<div className="flex shrink-0 items-center gap-3">
				<TemplateLogo template={template} className="size-9 shrink-0" />
				<div className="min-w-0">
					<div className="flex items-center gap-2">
						<p className="truncate text-sm font-semibold text-foreground">
							{template.name}
						</p>
						<Badge
							variant="outline"
							className="shrink-0 font-mono text-[10px] font-normal">
							{template.version}
						</Badge>
					</div>
					<p className="mt-0.5 truncate text-xs text-muted-foreground">
						{installing
							? 'Fetching the compose file and generating template secrets…'
							: phase === 'done'
								? 'The stack is installed and ready to open.'
								: STEP_DESCRIPTIONS[stepIdx]}
					</p>
				</div>
			</div>

			<div className="flex min-h-0 flex-1 flex-col overflow-y-auto pr-0.5">
				<StepTimeline
					steps={TIMELINE}
					idx={activeIdx}
					nodeState={nodeState}>
					<div key={phase === 'idle' ? `step-${stepIdx}` : phase}>
						{installing ? (
							<p className="flex items-center gap-2 text-xs text-muted-foreground">
								<Loader2 className="size-3.5 animate-spin text-primary" />
								Installing{' '}
								<span className="font-medium text-foreground">
									{template.name}
								</span>{' '}
								on{' '}
								<span className="font-medium text-foreground">
									{serverId === 'local'
										? 'the local host'
										: (servers.find(s => String(s.id) === serverId)
												?.name ?? 'the selected server')}
								</span>
								…
							</p>
						) : phase === 'done' ? (
							<p className="flex items-center gap-2 text-xs text-muted-foreground">
								<CheckCircle2 className="size-3.5 text-emerald-500" />
								Installed — the stack is ready to open.
							</p>
						) : stepIdx === 0 ? (
							<div className="flex flex-col gap-4">
								{!pinnedProjectId ? (
									<Field
										label="Project"
										hint="The project that will own this stack.">
										{projects.length === 0 ? (
											<p className="py-2 text-xs text-muted-foreground">
												No projects yet — create a project first.
											</p>
										) : (
											<div className="flex max-h-52 flex-col gap-2 overflow-y-auto pr-0.5">
												{projects.map(project => (
													<OptionRow
														key={String(project.id)}
														selected={String(project.id) === projectId}
														onClick={() => {
															setProjectId(String(project.id));
															setEnvironmentId('');
														}}
														icon={<Folder className="size-4.5" />}
														title={project.name}
													/>
												))}
											</div>
										)}
									</Field>
								) : (
									<Field
										label="Project"
										hint="Pinned — the panel was opened from inside this project.">
										<OptionRow
											selected
											onClick={() => {}}
											icon={<Folder className="size-4.5" />}
											title={pinnedProject?.name ?? 'This project'}
											disabled
										/>
									</Field>
								)}

								{!pinnedEnvironmentId ? (
									<Field
										label="Environment"
										hint="Deploy settings and secrets live per environment.">
										{envs.length === 0 ? (
											<p className="py-2 text-xs text-muted-foreground">
												{projectId
													? 'This project has no environments yet.'
													: 'Pick a project first.'}
											</p>
										) : (
											<div className="flex max-h-52 flex-col gap-2 overflow-y-auto pr-0.5">
												{envs.map(environment => (
													<OptionRow
														key={String(environment.id)}
														selected={
															String(environment.id) === environmentId
														}
														onClick={() =>
															setEnvironmentId(String(environment.id))
														}
														icon={<Layers className="size-4.5" />}
														title={environment.name}
													/>
												))}
											</div>
										)}
									</Field>
								) : (
									<Field label="Environment" hint="Pinned environment.">
										<OptionRow
											selected
											onClick={() => {}}
											icon={<Layers className="size-4.5" />}
											title={pinnedEnv?.name ?? 'Current environment'}
											disabled
										/>
									</Field>
								)}
							</div>
						) : stepIdx === 1 ? (
							<Field
								label="Server"
								hint="The panel's own host or a connected remote — search by name or IP.">
								<ServerSelect
									value={serverId}
									onChange={setServerId}
									options={[
										{
											id: 'local',
											name: 'Local (this server)',
											subtitle: "The panel's own host",
										},
										...servers.map(server => ({
											id: String(server.id),
											name: server.name,
											subtitle: server.ip_address,
										})),
									]}
								/>
							</Field>
						) : (
							<Field
								label="Service name"
								hint="Compose project name. Keep it lowercase.">
								<Input
									value={name}
									onChange={e => setName(e.target.value)}
									placeholder={template.id}
									className="font-mono"
								/>
							</Field>
						)}
					</div>
				</StepTimeline>
			</div>

			<div className="flex shrink-0 flex-col gap-2 pt-2">
				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={back}
						disabled={installing}
						className="gap-1.5">
						<ChevronLeft className="size-3.5" />
						Back
					</Button>
					{phase === 'done' ? (
						<Button
							onClick={viewStack}
							className="h-8 flex-1 gap-2 text-sm">
							<CheckCircle2 className="size-4" />
							View stack
						</Button>
					) : phase === 'idle' && !last ? (
						<Button
							onClick={() => setStepIdx(i => i + 1)}
							disabled={!canContinue}
							className="h-8 flex-1 gap-2 text-sm">
							Continue
						</Button>
					) : (
						<Button
							onClick={handleInstall}
							disabled={installing || !destinationReady}
							className="h-8 flex-1 gap-2 text-sm">
							{installing ? (
								<Loader2 className="size-4 animate-spin" />
							) : (
								<Rocket className="size-4" />
							)}
							{installing ? 'Installing…' : `Install ${template.name}`}
						</Button>
					)}
				</div>
			</div>
		</div>
	);
}
