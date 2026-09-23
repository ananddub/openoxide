import {useEffect, useState} from 'react';
import {
	CheckCircle2,
	Cpu,
	Link2,
	Loader2,
	RefreshCw,
	Save,
	Server,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
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
import {Input} from '#/components/ui/input';
import {Switch} from '#/components/ui/switch';
import {
	OptionRow,
	StepBody,
	StepCheckpoints,
	WizardField,
	type WizardStep,
} from '#/components/shared/sheet-wizard';
import {AI_PRESETS, EMPTY_AI_FORM, type AiSetting} from './ai-types';
import {AiProviderIcon} from './ai-provider-icon';
import {useAiProviderForm} from './use-ai-provider-form';

type Props = {
	editing: AiSetting | null;
	open: boolean;
	onOpenChange: (open: boolean) => void;
	onSaved: () => Promise<void>;
};

const STEPS: WizardStep[] = [
	{id: 'provider', label: 'Provider'},
	{id: 'credentials', label: 'Connection'},
	{id: 'model', label: 'Model'},
	{id: 'review', label: 'Review'},
];

const STEP_DESC: Record<string, string> = {
	provider: 'Pick a provider, or point at any OpenAI-compatible endpoint.',
	credentials: 'Where OpenOxide connects and how it authenticates.',
	model: 'Choose the model to use — Discover lists what the endpoint serves.',
	review: 'Name it, then decide whether it is enabled.',
};

const presetForUrl = (url?: string) =>
	AI_PRESETS.find(p => p.url === url)?.name ?? null;

/** Add / edit an LLM provider as a four-checkpoint wizard. */
export function AiProviderSheet({
	editing,
	open,
	onOpenChange,
	onSaved,
}: Props) {
	const {
		form,
		set,
		models,
		selectPreset,
		discover,
		test,
		save,
		saving,
		testing,
		discovering,
	} = useAiProviderForm({open, editing, onOpenChange, onSaved});

	const [stepIdx, setStepIdx] = useState(0);
	const [picked, setPicked] = useState<string | null>(null);
	const step = STEPS[stepIdx].id;
	const last = stepIdx === STEPS.length - 1;

	// Sync the chosen provider each time the sheet opens — derived from the
	// editing target / defaults, not the (async) form state, to avoid a stale read.
	useEffect(() => {
		if (!open) return;
		setStepIdx(0);
		const url = editing?.api_url ?? EMPTY_AI_FORM.api_url;
		setPicked(presetForUrl(url) ?? (url ? 'custom' : null));
	}, [open, editing]);

	const pickPreset = (name: string, url: string, defaultModel: string) => {
		setPicked(name);
		selectPreset(name, url, defaultModel);
	};

	const canContinue =
		step === 'provider'
			? picked !== null
			: step === 'credentials'
				? !!form.api_url.trim()
				: step === 'model'
					? !!form.model.trim()
					: true;

	const currentProviderName =
		picked && picked !== 'custom' ? picked : form.name;

	return (
		<Sheet open={open} onOpenChange={onOpenChange}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>
						{editing ? 'Edit AI Provider' : 'New AI Provider'}
					</SheetTitle>
					<SheetDescription>{STEP_DESC[step]}</SheetDescription>
					<StepCheckpoints steps={STEPS} idx={stepIdx} />
				</SheetHeader>

				<div className="flex min-h-0 flex-1 flex-col overflow-y-auto px-6 py-6">
					<StepBody key={step}>
						{step === 'provider' && (
							<div className="flex flex-col gap-2">
								{AI_PRESETS.map(p => (
									<OptionRow
										key={p.name}
										selected={picked === p.name}
										onClick={() =>
											pickPreset(
												p.name,
												p.url,
												p.defaultModel,
											)
										}
										icon={
											<AiProviderIcon
												name={p.name}
												className="size-5"
											/>
										}
										title={p.name}
										subtitle={p.description}
									/>
								))}
								<OptionRow
									selected={picked === 'custom'}
									onClick={() => setPicked('custom')}
									icon={<Server className="size-5" />}
									title="Custom endpoint"
									subtitle="Any OpenAI-compatible base URL."
								/>
							</div>
						)}

						{step === 'credentials' && (
							<div className="flex flex-col gap-5">
								<WizardField
									label="API Base URL"
									required
									hint="Presets fill this in automatically — edit it only for a custom gateway.">
									<Input
										value={form.api_url}
										onChange={e =>
											set('api_url', e.target.value)
										}
										placeholder="https://api.openai.com/v1"
										className="bg-background font-mono"
									/>
								</WizardField>

								<WizardField
									label="API Key / Token"
									optional
									hint={
										editing?.has_api_key
											? 'Leave blank to keep the existing key.'
											: 'Stored encrypted; never sent back to the browser.'
									}>
									<Input
										type="password"
										value={form.api_key}
										onChange={e =>
											set('api_key', e.target.value)
										}
										placeholder="sk-..."
										className="bg-background font-mono"
									/>
								</WizardField>
							</div>
						)}

						{step === 'model' && (
							<div className="flex flex-col gap-5">
								<WizardField
									label="Model Name"
									required
									action={
										<Button
											type="button"
											variant="ghost"
											size="sm"
											onClick={() => void discover()}
											disabled={
												discovering || !form.api_url
											}
											className="h-7 gap-1.5 px-2 text-[13px] font-medium text-primary">
											<RefreshCw
												className={`size-3.5 ${
													discovering
														? 'animate-spin'
														: ''
												}`}
											/>
											Discover
										</Button>
									}>
									<Input
										value={form.model}
										onChange={e =>
											set('model', e.target.value)
										}
										placeholder="e.g. gpt-4o, claude-3-5-sonnet, llama3.2"
										className="bg-background font-mono"
									/>
								</WizardField>

								{models.length > 0 && (
									<WizardField label="Discovered Models">
										<Select
											value={form.model}
											onValueChange={v =>
												v && set('model', v)
											}>
											<SelectTrigger className="bg-background font-mono">
												<SelectValue placeholder="Choose a model" />
											</SelectTrigger>
											<SelectContent className="bg-card">
												{models.map(m => (
													<SelectItem
														key={m}
														value={m}>
														<span className="font-mono text-[13px]">
															{m}
														</span>
													</SelectItem>
												))}
											</SelectContent>
										</Select>
									</WizardField>
								)}
							</div>
						)}

						{step === 'review' && (
							<div className="flex flex-col gap-5">
								<div className="flex items-center gap-3.5 rounded-xl border border-border/60 bg-muted/20 px-4 py-3.5">
									<span className="flex size-10 shrink-0 items-center justify-center rounded-lg bg-background text-foreground">
										<AiProviderIcon
											name={currentProviderName}
											className="size-5"
										/>
									</span>
									<div className="min-w-0 flex-1">
										<p className="truncate text-sm font-medium text-foreground">
											{picked && picked !== 'custom'
												? picked
												: 'Custom endpoint'}
										</p>
										<div className="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1">
											<span className="flex items-center gap-1 truncate font-mono text-[11px] text-muted-foreground">
												<Link2 className="size-3 shrink-0 opacity-60" />
												{form.api_url}
											</span>
											<span className="flex items-center gap-1 font-mono text-[11px] text-muted-foreground">
												<Cpu className="size-3 opacity-70" />
												{form.model}
											</span>
										</div>
									</div>
								</div>

								<WizardField label="Provider Name" required>
									<Input
										value={form.name}
										onChange={e =>
											set('name', e.target.value)
										}
										placeholder="e.g. OpenAI Production"
										className="bg-background"
									/>
								</WizardField>

								<div className="flex items-center justify-between gap-4 rounded-xl border border-border/60 px-4 py-3.5">
									<div className="min-w-0">
										<p className="text-[13px] font-medium text-foreground">
											Enable this provider
										</p>
										<p className="mt-0.5 text-[13px] text-muted-foreground">
											Used for log diagnosis and the
											assistant.
										</p>
									</div>
									<Switch
										checked={form.is_enabled}
										onCheckedChange={v =>
											set('is_enabled', v)
										}
									/>
								</div>
							</div>
						)}
					</StepBody>
				</div>

				{last ? (
					<SheetFooter className="justify-between">
						<Button
							variant="outline"
							onClick={() => setStepIdx(i => i - 1)}>
							Back
						</Button>
						<div className="flex items-center gap-3">
							<Button
								variant="outline"
								onClick={() => void test()}
								disabled={testing || !form.model}
								className="h-10 gap-1.5 px-4">
								{testing ? (
									<Loader2 className="size-3.5 animate-spin" />
								) : (
									<CheckCircle2 className="size-3.5 text-emerald-500" />
								)}
								Test
							</Button>
							<Button
								onClick={() => void save()}
								disabled={
									saving ||
									!form.name.trim() ||
									!form.api_url.trim() ||
									!form.model.trim()
								}
								className="h-10 gap-1.5 px-5">
								{saving ? (
									<Loader2 className="size-3.5 animate-spin" />
								) : (
									<Save className="size-3.5" />
								)}
								{editing ? 'Save changes' : 'Add provider'}
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
						<Button
							disabled={!canContinue}
							onClick={() =>
								setStepIdx(i => Math.min(i + 1, STEPS.length - 1))
							}>
							Continue
						</Button>
					</SheetFooter>
				)}
			</SheetContent>
		</Sheet>
	);
}
