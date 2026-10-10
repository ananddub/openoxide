import {useState, type ReactNode} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {
	Sparkles,
	Loader,
	Check,
	Plus,
	Trash2,
	Server as ServerIcon,
} from 'lucide-react';
import {toast} from 'sonner';
import {
	useAiCreateGeneration,
	useAiDeployGeneration,
	useAiEnabledSettings,
	useAiReviewGeneration,
	type AiGenerationPayload,
	type AiSuggestionPayload,
} from '#/api/live/ai';
import {useRemoteServerList} from '#/api/live/servers/queries';
import {formatApiError} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {Switch} from '#/components/ui/switch';
import {FieldLabel} from '#/components/ui/field';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {cn} from '#/api/utils';
import {StepTimeline} from '#/components/shared/sheet-wizard';

type Step = 'prompt' | 'variants' | 'deploy';

const STEPS: Step[] = ['prompt', 'variants', 'deploy'];
const STEP_LABEL: Record<Step, string> = {
	prompt: 'Describe',
	variants: 'Review',
	deploy: 'Deploy',
};

const EXAMPLE_PROMPTS = [
	'Deploy a Next.js app with a PostgreSQL database and Redis, exposed on its own domain.',
	'Set up self-hosted Uptime Kuma with a Postgres backend.',
	'Run n8n automation with a Postgres database and Redis queue.',
];

function cloneSuggestions(
	list: AiSuggestionPayload[],
): AiSuggestionPayload[] {
	return list.map(s => ({
		...s,
		env_variables: s.env_variables.map(e => ({...e})),
		domains: s.domains.map(d => ({...d})),
		config_files: s.config_files.map(f => ({...f})),
	}));
}

// ─── Checkpoints (vertical StepTimeline grammar) ─────────────────────────────

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

function RowShell({children}: {children: ReactNode}) {
	return <div className="flex items-center gap-2">{children}</div>;
}

function RemoveBtn({onClick}: {onClick: () => void}) {
	return (
		<button
			type="button"
			onClick={onClick}
			aria-label="Remove"
			className="flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-destructive/10 hover:text-destructive">
			<Trash2 className="size-3.5" />
		</button>
	);
}

function AddBtn({label, onClick}: {label: string; onClick: () => void}) {
	return (
		<button
			type="button"
			onClick={onClick}
			className="flex cursor-pointer items-center gap-1 self-start px-1 text-xs font-semibold text-primary hover:underline">
			<Plus className="size-3.5" />
			{label}
		</button>
	);
}

/** Editable list of key/value env variables for the selected suggestion. */
function EnvRows({
	suggestion,
	onChange,
}: {
	suggestion: AiSuggestionPayload;
	onChange: (next: AiSuggestionPayload) => void;
}) {
	return (
		<div className="flex flex-col gap-2">
			{suggestion.env_variables.map((env, i) => (
				<RowShell key={i}>
					<Input
						value={env.name}
						placeholder="KEY"
						onChange={e =>
							onChange({
								...suggestion,
								env_variables: suggestion.env_variables.map((x, j) =>
									j === i ? {...x, name: e.target.value} : x,
								),
							})
						}
						className="h-8 font-mono text-xs"
					/>
					<Input
						value={env.value}
						placeholder="value"
						onChange={e =>
							onChange({
								...suggestion,
								env_variables: suggestion.env_variables.map((x, j) =>
									j === i ? {...x, value: e.target.value} : x,
								),
							})
						}
						className="h-8 flex-1 font-mono text-xs"
					/>
					<RemoveBtn
						onClick={() =>
							onChange({
								...suggestion,
								env_variables: suggestion.env_variables.filter(
									(_, j) => j !== i,
								),
							})
						}
					/>
				</RowShell>
			))}
			<AddBtn
				label="Add variable"
				onClick={() =>
					onChange({
						...suggestion,
						env_variables: [
							...suggestion.env_variables,
							{name: '', value: ''},
						],
					})
				}
			/>
		</div>
	);
}

function DomainRows({
	suggestion,
	onChange,
}: {
	suggestion: AiSuggestionPayload;
	onChange: (next: AiSuggestionPayload) => void;
}) {
	const set = (
		i: number,
		patch: Partial<AiSuggestionPayload['domains'][number]>,
	) =>
		onChange({
			...suggestion,
			domains: suggestion.domains.map((d, j) =>
				j === i ? {...d, ...patch} : d,
			),
		});
	return (
		<div className="flex flex-col gap-2">
			{suggestion.domains.map((d, i) => (
				<RowShell key={i}>
					<Input
						value={d.host}
						placeholder="host"
						onChange={e => set(i, {host: e.target.value})}
						className="h-8 flex-1 font-mono text-xs"
					/>
					<Input
						value={String(d.port)}
						placeholder="port"
						onChange={e => set(i, {port: Number(e.target.value) || 0})}
						className="h-8 w-20 font-mono text-xs"
					/>
					<Input
						value={d.service_name}
						placeholder="service"
						onChange={e => set(i, {service_name: e.target.value})}
						className="h-8 w-32 font-mono text-xs"
					/>
					<RemoveBtn
						onClick={() =>
							onChange({
								...suggestion,
								domains: suggestion.domains.filter((_, j) => j !== i),
							})
						}
					/>
				</RowShell>
			))}
			<AddBtn
				label="Add domain"
				onClick={() =>
					onChange({
						...suggestion,
						domains: [
							...suggestion.domains,
							{host: '', port: 80, service_name: ''},
						],
					})
				}
			/>
		</div>
	);
}

function FileRows({
	suggestion,
	onChange,
}: {
	suggestion: AiSuggestionPayload;
	onChange: (next: AiSuggestionPayload) => void;
}) {
	return (
		<div className="flex flex-col gap-2">
			{suggestion.config_files.map((f, i) => (
				<div key={i} className="flex flex-col gap-1.5">
					<RowShell>
						<Input
							value={f.file_path}
							placeholder="path e.g. ../files/config.yml"
							onChange={e =>
								onChange({
									...suggestion,
									config_files: suggestion.config_files.map((x, j) =>
										j === i ? {...x, file_path: e.target.value} : x,
									),
								})
							}
							className="h-8 font-mono text-xs"
						/>
						<RemoveBtn
							onClick={() =>
								onChange({
									...suggestion,
									config_files: suggestion.config_files.filter(
										(_, j) => j !== i,
									),
								})
							}
						/>
					</RowShell>
					<Textarea
						value={f.content}
						rows={4}
						className="font-mono text-xs"
						placeholder="file content"
						onChange={e =>
							onChange({
								...suggestion,
								config_files: suggestion.config_files.map((x, j) =>
									j === i ? {...x, content: e.target.value} : x,
								),
							})
						}
					/>
				</div>
			))}
			<AddBtn
				label="Add config file"
				onClick={() =>
					onChange({
						...suggestion,
						config_files: [
							...suggestion.config_files,
							{file_path: '', content: ''},
						],
					})
				}
			/>
		</div>
	);
}

/** "AI Assistant": prompt → up to three deployable Compose stacks → human review/edit → one-click deploy, all inference server-side. */
export function AiAssistantSheet({
	open,
	projectId,
	envId,
	onClose,
}: {
	open: boolean;
	projectId: number;
	envId?: number;
	onClose: () => void;
}) {
	const navigate = useNavigate();
	const {settings, isPending} = useAiEnabledSettings();
	const createGeneration = useAiCreateGeneration();
	const reviewGeneration = useAiReviewGeneration();
	const deployGeneration = useAiDeployGeneration();
	const {data: rawServers} = useRemoteServerList();
	const servers = rawServers ?? [];

	const [stepIdx, setStepIdx] = useState(0);
	const step = STEPS[stepIdx];
	const [aiSettingId, setAiSettingId] = useState<string | null>(null);
	const [request, setRequest] = useState('');
	const [generation, setGeneration] = useState<AiGenerationPayload | null>(
		null,
	);
	const [drafts, setDrafts] = useState<AiSuggestionPayload[]>([]);
	const [selectedId, setSelectedId] = useState<string>('');
	const [serverId, setServerId] = useState('local');
	const [deployNow, setDeployNow] = useState(true);

	const busy =
		createGeneration.isPending ||
		reviewGeneration.isPending ||
		deployGeneration.isPending;
	const selected =
		drafts.find(d => d.id === selectedId) ?? drafts[0] ?? null;

	const reset = () => {
		setStepIdx(0);
		setRequest('');
		setGeneration(null);
		setDrafts([]);
		setSelectedId('');
		setServerId('local');
		setDeployNow(true);
	};

	const patchSelected = (next: AiSuggestionPayload) =>
		setDrafts(list => list.map(d => (d.id === next.id ? next : d)));

	const generate = async () => {
		if (!aiSettingId || !request.trim()) return;
		try {
			const gen = await createGeneration.mutateAsync({
				aiSettingId: Number(aiSettingId),
				request: request.trim(),
			});
			setGeneration(gen);
			const copy = cloneSuggestions(gen.suggestions);
			setDrafts(copy);
			setSelectedId(copy[0]?.id ?? '');
			setStepIdx(1);
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	const saveReview = async () => {
		if (!generation) return;
		try {
			await reviewGeneration.mutateAsync({
				id: generation.id,
				suggestions: drafts,
			});
			setStepIdx(2);
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	const deploy = async () => {
		if (!generation || !selected) return;
		if (!envId) {
			toast.error('Select an environment first.');
			return;
		}
		try {
			const result = await deployGeneration.mutateAsync({
				id: generation.id,
				suggestionId: selected.id,
				environmentId: envId,
				serverId: serverId === 'local' ? null : Number(serverId),
				deployNow,
			});
			toast.success(
				deployNow ? 'Deployment started' : `${selected.name} created`,
			);
			onClose();
			navigate({
				to: `/projects/${projectId}/compose/${result.compose_id}` as never,
			});
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	const canContinue =
		step === 'prompt'
			? !!aiSettingId && !!request.trim() && settings.length > 0
			: step === 'variants'
				? !!selected
				: true;

	const nextLabel =
		step === 'prompt'
			? createGeneration.isPending
				? 'Generating…'
				: 'Generate'
			: step === 'variants'
				? 'Review & Deploy'
				: deployGeneration.isPending
					? 'Deploying…'
					: 'Create';

	const handleNext = () => {
		if (step === 'prompt') void generate();
		else if (step === 'variants') void saveReview();
		else void deploy();
	};

	const stepDesc: Record<Step, string> = {
		prompt:
			'Describe what you want to run — the AI drafts deployable stacks.',
		variants: 'Pick a suggestion and fine-tune it before deploying.',
		deploy:
			'Choose where to run it. You can edit everything on the next screen.',
	};

	return (
		<Sheet open={open} onOpenChange={o => !o && (reset(), onClose())}>
			<SheetContent className="sm:data-[side=right]:w-[620px]">
				<SheetHeader>
					<SheetTitle>AI Assistant</SheetTitle>
					<SheetDescription>{stepDesc[step]}</SheetDescription>
				</SheetHeader>

				<div className="flex-1 overflow-y-auto px-6 py-6">
					<StepTimeline
						steps={STEPS.map(s => ({id: s, label: STEP_LABEL[s]}))}
						idx={stepIdx}>
						<div key={step}>
							{step === 'prompt' && (
								<div className="flex flex-col gap-5">
									{isPending ? (
										<p className="flex items-center gap-2 text-[13px] text-muted-foreground">
											<Loader className="size-3.5 animate-spin" />
											Loading AI providers…
										</p>
									) : settings.length === 0 ? (
										<div className="flex flex-col gap-2 rounded-xl border border-border/60 bg-muted/20 p-4">
											<p className="text-sm text-foreground">
												No AI provider enabled yet.
											</p>
											<p className="text-[13px] text-muted-foreground">
												Add a provider on the AI Providers page to use the
												assistant.
											</p>
											<Button
												variant="outline"
												size="sm"
												onClick={() => navigate({to: '/ai'} as never)}
												className="self-start">
												<Sparkles className="mr-1.5 size-3.5" />
												AI Providers
											</Button>
										</div>
									) : (
										<>
											<div className="flex flex-col gap-1.5">
												<FieldLabel>Provider</FieldLabel>
												<Select
													value={aiSettingId ?? ''}
													onValueChange={v =>
														setAiSettingId(v ? String(v) : null)
													}>
													<SelectTrigger>
														<SelectValue placeholder="Select an AI provider" />
													</SelectTrigger>
													<SelectContent className="bg-card">
														{settings.map(s => (
															<SelectItem
																key={s.id}
																value={String(s.id)}
																className="text-xs">
																{s.name} — {s.model}
															</SelectItem>
														))}
													</SelectContent>
												</Select>
											</div>
											<div className="flex flex-col gap-1.5">
												<FieldLabel>
													What do you want to deploy?
												</FieldLabel>
												<Textarea
													rows={5}
													value={request}
													placeholder="e.g. A Next.js app with Postgres and Redis on its own domain…"
													onChange={e => setRequest(e.target.value)}
													className="text-[13px]"
												/>
											</div>
											<div className="flex flex-col gap-2">
												<FieldLabel>Try an example</FieldLabel>
												<div className="flex flex-wrap gap-2">
													{EXAMPLE_PROMPTS.map(p => (
														<button
															key={p}
															type="button"
															onClick={() => setRequest(p)}
															className="cursor-pointer rounded-full border border-border/60 bg-muted/20 px-3 py-1.5 text-start text-[12px] text-muted-foreground transition-colors hover:border-border hover:bg-muted/40 hover:text-foreground">
															{p}
														</button>
													))}
												</div>
											</div>
										</>
									)}
								</div>
							)}

							{step === 'variants' && (
								<div className="flex flex-col gap-4">
									<div className="flex flex-col gap-2">
										{drafts.map(d => (
											<PlanCard
												key={d.id}
												selected={selected?.id === d.id}
												onClick={() => setSelectedId(d.id)}
												title={d.name}
												desc={d.short_description || d.description}
											/>
										))}
									</div>

									{selected && (
										<div className="flex flex-col gap-5 rounded-xl border border-border/60 bg-muted/10 p-4">
											<div className="flex flex-col gap-1.5">
												<FieldLabel>Docker Compose</FieldLabel>
												<Textarea
													rows={12}
													value={selected.docker_compose}
													onChange={e =>
														patchSelected({
															...selected,
															docker_compose: e.target.value,
														})
													}
													className="font-mono text-[12px]"
												/>
											</div>
											<div className="flex flex-col gap-1.5">
												<FieldLabel>Environment variables</FieldLabel>
												<EnvRows
													suggestion={selected}
													onChange={patchSelected}
												/>
											</div>
											<div className="flex flex-col gap-1.5">
												<FieldLabel>Domains</FieldLabel>
												<DomainRows
													suggestion={selected}
													onChange={patchSelected}
												/>
											</div>
											<div className="flex flex-col gap-1.5">
												<FieldLabel>Config files</FieldLabel>
												<FileRows
													suggestion={selected}
													onChange={patchSelected}
												/>
											</div>
										</div>
									)}
								</div>
							)}

							{step === 'deploy' && selected && (
								<div className="flex flex-col gap-5">
									<div className="rounded-xl border border-border/60 bg-muted/10 p-4">
										<p className="text-sm font-medium text-foreground">
											{selected.name}
										</p>
										{(selected.short_description ||
											selected.description) && (
											<p className="mt-1 text-[13px] text-muted-foreground">
												{selected.short_description ||
													selected.description}
											</p>
										)}
										<div className="mt-3 flex flex-wrap gap-x-4 gap-y-1 text-[12px] text-muted-foreground">
											<span className="flex items-center gap-1">
												<Check className="size-3 text-primary" />
												{selected.env_variables.length} env
											</span>
											<span className="flex items-center gap-1">
												<Check className="size-3 text-primary" />
												{selected.domains.length} domains
											</span>
											<span className="flex items-center gap-1">
												<Check className="size-3 text-primary" />
												{selected.config_files.length} files
											</span>
										</div>
									</div>

									<div className="flex flex-col gap-2">
										<FieldLabel>Server</FieldLabel>
										<div className="flex max-h-[min(40vh,360px)] flex-col gap-2 overflow-y-auto pr-0.5">
											<button
												type="button"
												onClick={() => setServerId('local')}
												className={cn(
													'flex w-full items-center gap-3.5 rounded-xl border px-4 py-3 text-start transition-all duration-150',
													serverId === 'local'
														? 'border-primary/40 bg-primary/10'
														: 'border-border/50 hover:border-border hover:bg-muted/30',
												)}>
												<ServerIcon className="size-5 text-muted-foreground" />
												<span className="min-w-0 flex-1">
													<span className="block text-sm font-medium text-foreground">
														Local
													</span>
													<span className="block text-[13px] text-muted-foreground">
														This server
													</span>
												</span>
												{serverId === 'local' && (
													<span className="flex size-5 shrink-0 items-center justify-center rounded-full bg-primary text-primary-foreground">
														<Check className="size-3" />
													</span>
												)}
											</button>
											{servers.map(s => (
												<button
													key={String(s.id)}
													type="button"
													onClick={() => setServerId(String(s.id))}
													className={cn(
														'flex w-full items-center gap-3.5 rounded-xl border px-4 py-3 text-start transition-all duration-150',
														serverId === String(s.id)
															? 'border-primary/40 bg-primary/10'
															: 'border-border/50 hover:border-border hover:bg-muted/30',
													)}>
													<ServerIcon className="size-5 text-muted-foreground" />
													<span className="min-w-0 flex-1">
														<span className="block text-sm font-medium text-foreground">
															{s.name}
														</span>
														<span className="block truncate text-[13px] text-muted-foreground">
															{s.ip_address}
														</span>
													</span>
													{serverId === String(s.id) && (
														<span className="flex size-5 shrink-0 items-center justify-center rounded-full bg-primary text-primary-foreground">
															<Check className="size-3" />
														</span>
													)}
												</button>
											))}
										</div>
									</div>

									<div className="flex items-center justify-between rounded-xl border border-border/60 px-4 py-3">
										<div>
											<p className="text-sm font-medium text-foreground">
												Deploy now
											</p>
											<p className="text-[13px] text-muted-foreground">
												Start the build immediately after creating.
											</p>
										</div>
										<Switch
											checked={deployNow}
											onCheckedChange={setDeployNow}
										/>
									</div>
								</div>
							)}
						</div>
					</StepTimeline>
				</div>

				<SheetFooter className="justify-between">
					{stepIdx > 0 ? (
						<Button
							variant="outline"
							disabled={busy}
							onClick={() => setStepIdx(i => Math.max(0, i - 1))}>
							Back
						</Button>
					) : (
						<span />
					)}
					<Button
						disabled={!canContinue || busy}
						onClick={handleNext}
						className="h-10 gap-1.5 px-5">
						{busy && <Loader className="size-3.5 animate-spin" />}
						{step === 'deploy' && !busy && (
							<Sparkles className="size-3.5" />
						)}
						{nextLabel}
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
