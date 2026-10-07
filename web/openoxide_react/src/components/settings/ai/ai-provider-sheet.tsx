import {useEffect, useState} from 'react';
import {
	Check,
	CheckCircle2,
	Cpu,
	Loader2,
	RefreshCw,
	Save,
	Search,
	Server,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
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
import {OptionRow, WizardField} from '#/components/shared/sheet-wizard';
import {cn} from '#/api/utils';
import {AI_PRESETS, EMPTY_AI_FORM, type AiSetting} from './ai-types';
import {AiProviderIcon} from './ai-provider-icon';
import {useAiProviderForm} from './use-ai-provider-form';

type Props = {
	editing: AiSetting | null;
	open: boolean;
	onOpenChange: (open: boolean) => void;
	onSaved: () => Promise<void>;
};

const presetForUrl = (url?: string) =>
	AI_PRESETS.find(p => p.url === url)?.name ?? null;

/** Section label for one block of the single-page form. */
function SectionLabel({
	children,
	note,
}: {
	children: string;
	note?: string;
}) {
	return (
		<p className="text-xs font-semibold tracking-wider text-muted-foreground uppercase">
			{children}
			{note && (
				<span className="ml-2 font-normal tracking-normal text-muted-foreground/70 normal-case">
					{note}
				</span>
			)}
		</p>
	);
}

/** Add / edit an LLM provider — everything on one page, no steps. */
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

	const [picked, setPicked] = useState<string | null>(null);
	const [providerQuery, setProviderQuery] = useState('');
	const [modelQuery, setModelQuery] = useState('');

	// Sync the chosen provider each time the sheet opens — derived from the editing target / defaults, not the (async) form state, to avoid a stale read.
	useEffect(() => {
		if (!open) return;
		const url = editing?.api_url ?? EMPTY_AI_FORM.api_url;
		setPicked(presetForUrl(url) ?? (url ? 'custom' : null));
	}, [open, editing]);

	const pickPreset = (name: string, url: string, defaultModel: string) => {
		setPicked(name);
		selectPreset(name, url, defaultModel);
	};

	const q = providerQuery.trim().toLowerCase();
	const visiblePresets = q
		? AI_PRESETS.filter(
				p =>
					p.name.toLowerCase().includes(q) ||
					p.description.toLowerCase().includes(q),
			)
		: AI_PRESETS;

	const mq = modelQuery.trim().toLowerCase();
	const visibleModels = mq
		? models.filter(m => m.toLowerCase().includes(mq))
		: models;

	return (
		<Sheet open={open} onOpenChange={onOpenChange}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>
						{editing ? 'Edit AI Provider' : 'New AI Provider'}
					</SheetTitle>
					<SheetDescription>
						Pick a provider (or any OpenAI-compatible endpoint), connect,
						choose a model.
					</SheetDescription>
				</SheetHeader>

				<div className="flex min-h-0 flex-1 flex-col gap-6 overflow-y-auto px-6 py-6">
					{/* ── Provider ── */}
					<section className="flex flex-col gap-2">
						<SectionLabel note="presets fill the connection for you">
							Provider
						</SectionLabel>
						<div className="relative">
							<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
							<Input
								placeholder="Search providers…"
								value={providerQuery}
								onChange={e => setProviderQuery(e.target.value)}
								className="pl-9"
							/>
						</div>
						{visiblePresets.map(p => (
							<OptionRow
								key={p.name}
								selected={picked === p.name}
								onClick={() => pickPreset(p.name, p.url, p.defaultModel)}
								icon={<AiProviderIcon name={p.name} className="size-5" />}
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
						{visiblePresets.length === 0 && (
							<p className="py-2 text-center text-[13px] text-muted-foreground">
								No preset matches — pick Custom endpoint below.
							</p>
						)}
					</section>

					{/* ── Connection ── */}
					<section className="flex flex-col gap-4">
						<SectionLabel>Connection</SectionLabel>
						<WizardField
							label="API Base URL"
							required
							hint="Presets fill this in automatically — edit it only for a custom gateway.">
							<Input
								value={form.api_url}
								onChange={e => set('api_url', e.target.value)}
								placeholder="https://api.openai.com/v1"
								className="font-mono"
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
								onChange={e => set('api_key', e.target.value)}
								placeholder="sk-..."
								className="font-mono"
							/>
						</WizardField>
					</section>

					{/* ── Model ── */}
					<section className="flex flex-col gap-2">
						<SectionLabel note="Discover lists what the endpoint serves">
							Model
						</SectionLabel>
						<WizardField label="Model Name" required>
							<Input
								value={form.model}
								onChange={e => set('model', e.target.value)}
								placeholder="e.g. gpt-4o, claude-3-5-sonnet, llama3.2"
								className="font-mono"
							/>
						</WizardField>
						<Button
							type="button"
							variant="outline"
							onClick={() => void discover()}
							disabled={discovering || !form.api_url}
							className="h-9 w-fit gap-1.5 text-xs">
							<RefreshCw
								className={`size-3.5 ${discovering ? 'animate-spin' : ''}`}
							/>
							{discovering
								? 'Discovering…'
								: `Discover models${models.length ? ` (${models.length})` : ''}`}
						</Button>

						{models.length > 0 && (
							<div className="flex flex-col gap-2">
								<div className="relative">
									<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
									<Input
										placeholder="Filter models…"
										value={modelQuery}
										onChange={e => setModelQuery(e.target.value)}
										className="h-9 pl-9"
									/>
								</div>
								<div className="flex max-h-64 flex-col gap-2 overflow-y-auto pr-0.5">
									{visibleModels.map(m => (
										<button
											key={m}
											type="button"
											onClick={() => set('model', m)}
											className={cn(
												'flex w-full cursor-pointer items-center gap-3.5 rounded-xl border px-4 py-2.5 text-start transition-all duration-150',
												form.model === m
													? 'border-primary/40 bg-primary/10'
													: 'border-border/50 hover:border-border hover:bg-muted/30',
											)}>
											<Cpu className="size-4 shrink-0 text-muted-foreground" />
											<span className="min-w-0 flex-1 truncate font-mono text-[13px]">
												{m}
											</span>
											{form.model === m && (
												<span className="flex size-5 shrink-0 animate-in items-center justify-center rounded-full bg-primary text-primary-foreground duration-150 zoom-in-75">
													<Check className="size-3" />
												</span>
											)}
										</button>
									))}
									{visibleModels.length === 0 && (
										<p className="py-6 text-center text-[13px] text-muted-foreground">
											No model matches &ldquo;{modelQuery}&rdquo;.
										</p>
									)}
								</div>
							</div>
						)}
					</section>

					{/* ── Details ── */}
					<section className="flex flex-col gap-4">
						<SectionLabel>Details</SectionLabel>
						<WizardField label="Provider Name" required>
							<Input
								value={form.name}
								onChange={e => set('name', e.target.value)}
								placeholder="e.g. OpenAI Production"
							/>
						</WizardField>
						<div className="flex items-center justify-between gap-4 rounded-xl border border-border/60 px-4 py-3.5">
							<div className="min-w-0">
								<p className="text-[13px] font-medium text-foreground">
									Enable this provider
								</p>
								<p className="mt-0.5 text-[13px] text-muted-foreground">
									Used for log diagnosis and the assistant.
								</p>
							</div>
							<Switch
								checked={form.is_enabled}
								onCheckedChange={v => set('is_enabled', v)}
							/>
						</div>
					</section>
				</div>

				<SheetFooter className="justify-between">
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
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
