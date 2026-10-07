import {useEffect, useMemo, useRef, useState} from 'react';
import {
	Check,
	CheckCircle2,
	ChevronDown,
	Loader2,
	RefreshCw,
	Save,
	Search,
	Server,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from '#/components/ui/popover';
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
import {WizardField} from '#/components/shared/sheet-wizard';
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

/** The one model field — a searchable dropdown that also accepts a custom
 *  name: whatever you type that isn't in the list shows as “Use …”. */
function ModelSelect({
	value,
	options,
	onChange,
}: {
	value: string;
	options: string[];
	onChange: (model: string) => void;
}) {
	const [open, setOpen] = useState(false);
	const [query, setQuery] = useState('');
	const inputRef = useRef<HTMLInputElement>(null);

	const filtered = useMemo(() => {
		const q = query.trim().toLowerCase();
		if (!q) return options;
		return options.filter(model => model.toLowerCase().includes(q));
	}, [options, query]);

	const custom = query.trim();
	const customRow = custom && !options.includes(custom) ? custom : null;

	useEffect(() => {
		if (open) inputRef.current?.focus({preventScroll: true});
	}, [open]);

	const pick = (model: string) => {
		onChange(model);
		setOpen(false);
	};

	const onSearchKey = (event: React.KeyboardEvent) => {
		if (event.key !== 'Enter') return;
		event.preventDefault();
		const first = filtered[0];
		// Enter takes the best match, or the typed name when nothing matches.
		pick(first ?? (custom || ''));
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
				className={cn(
					'flex h-9 w-full cursor-pointer items-center gap-2 rounded-md border px-3 text-xs shadow-[0_1px_1px_rgba(0,0,0,0.02)] transition-colors duration-150 outline-none focus-visible:border-foreground/40 focus-visible:ring-2 focus-visible:ring-ring/30',
					value
						? 'border-input bg-card'
						: 'border-input bg-card text-muted-foreground',
				)}>
				<span
					className={cn(
						'min-w-0 flex-1 truncate text-left',
						value ? 'font-mono' : 'text-muted-foreground',
					)}>
					{value || 'Pick a model — search, or type your own'}
				</span>
				<ChevronDown className="size-4 shrink-0 text-muted-foreground opacity-70" />
			</PopoverTrigger>
			<PopoverContent className="p-0" align="start">
				<div className="flex flex-col">
					<div className="flex items-center gap-2 border-b border-border p-2">
						<Search className="size-3.5 shrink-0 text-muted-foreground" />
						<Input
							ref={inputRef}
							placeholder="Search models…"
							value={query}
							onChange={event => setQuery(event.target.value)}
							onKeyDown={onSearchKey}
							className="h-7 flex-1 border-0 bg-transparent px-0 text-[13px] shadow-none focus-visible:ring-0"
						/>
					</div>
					<div className="flex max-h-60 flex-col gap-px overflow-y-auto p-1.5">
						{customRow && (
							<button
								type="button"
								onClick={() => pick(customRow)}
								className="flex w-full cursor-pointer items-center gap-2.5 rounded-md px-2 py-1.5 text-left text-[13px] transition-colors outline-none hover:bg-accent focus-visible:bg-accent">
								<span className="min-w-0 flex-1 truncate">
									Use{' '}
									<span className="font-mono font-medium">
										{customRow}
									</span>
								</span>
								{customRow === value && (
									<Check className="size-3.5 shrink-0 text-foreground" />
								)}
							</button>
						)}
						{filtered.map(model => (
							<button
								key={model}
								type="button"
								onClick={() => pick(model)}
								className={cn(
									'flex w-full cursor-pointer items-center gap-2.5 rounded-md px-2 py-1.5 text-left text-[13px] transition-colors outline-none hover:bg-accent focus-visible:bg-accent',
									model === value && 'bg-accent',
								)}>
								<span className="min-w-0 flex-1 truncate font-mono">
									{model}
								</span>
								{model === value && (
									<Check className="size-3.5 shrink-0 text-foreground" />
								)}
							</button>
						))}
						{filtered.length === 0 && !customRow && (
							<p className="px-2 py-5 text-center text-[13px] text-muted-foreground">
								No models yet — run Discover above, or type a name and
								press Enter.
							</p>
						)}
					</div>
				</div>
			</PopoverContent>
		</Popover>
	);
}

/** Searchable provider dropdown — trigger shows the picked provider; the
 *  popover filters presets by name/description, with Custom endpoint always
 *  at the bottom. */
function ProviderSelect({
	value,
	onPickPreset,
	onPickCustom,
}: {
	value: string | null;
	onPickPreset: (name: string, url: string, defaultModel: string) => void;
	onPickCustom: () => void;
}) {
	const [open, setOpen] = useState(false);
	const [query, setQuery] = useState('');
	const inputRef = useRef<HTMLInputElement>(null);

	const filtered = useMemo(() => {
		const q = query.trim().toLowerCase();
		if (!q) return AI_PRESETS;
		return AI_PRESETS.filter(
			p =>
				p.name.toLowerCase().includes(q) ||
				p.description.toLowerCase().includes(q),
		);
	}, [query]);

	useEffect(() => {
		if (open) inputRef.current?.focus({preventScroll: true});
	}, [open]);

	const current = AI_PRESETS.find(p => p.name === value);

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
				{current ? (
					<>
						<AiProviderIcon
							name={current.name}
							className="size-4 shrink-0"
						/>
						<span className="min-w-0 flex-1 truncate text-left font-medium">
							{current.name}
						</span>
					</>
				) : value === 'custom' ? (
					<>
						<Server className="size-4 shrink-0 text-muted-foreground" />
						<span className="min-w-0 flex-1 truncate text-left font-medium">
							Custom endpoint
						</span>
					</>
				) : (
					<span className="min-w-0 flex-1 truncate text-left text-muted-foreground">
						Pick a provider — OpenAI, Anthropic, Ollama…
					</span>
				)}
				<ChevronDown className="size-4 shrink-0 text-muted-foreground opacity-70" />
			</PopoverTrigger>
			<PopoverContent className="p-0" align="start">
				<div className="flex flex-col">
					<div className="flex items-center gap-2 border-b border-border p-2">
						<Search className="size-3.5 shrink-0 text-muted-foreground" />
						<Input
							ref={inputRef}
							placeholder="Search providers…"
							value={query}
							onChange={event => setQuery(event.target.value)}
							className="h-7 flex-1 border-0 bg-transparent px-0 text-[13px] shadow-none focus-visible:ring-0"
						/>
					</div>
					<div className="flex max-h-60 flex-col gap-px overflow-y-auto p-1.5">
						{filtered.map(p => (
							<button
								key={p.name}
								type="button"
								onClick={() => {
									onPickPreset(p.name, p.url, p.defaultModel);
									setOpen(false);
								}}
								className={cn(
									'flex w-full cursor-pointer items-center gap-2.5 rounded-md px-2 py-1.5 text-left text-[13px] transition-colors outline-none hover:bg-accent focus-visible:bg-accent',
									p.name === value && 'bg-accent',
								)}>
								<AiProviderIcon
									name={p.name}
									className="size-4 shrink-0"
								/>
								<span className="flex min-w-0 flex-1 flex-col">
									<span className="font-medium text-foreground">
										{p.name}
									</span>
									<span className="truncate text-[11px] text-muted-foreground">
										{p.description}
									</span>
								</span>
								{p.name === value && (
									<Check className="size-3.5 shrink-0 text-foreground" />
								)}
							</button>
						))}
						{filtered.length === 0 && (
							<p className="px-2 py-3 text-center text-[13px] text-muted-foreground">
								No preset matches — use Custom endpoint below.
							</p>
						)}
						<button
							type="button"
							onClick={() => {
								onPickCustom();
								setOpen(false);
							}}
							className={cn(
								'flex w-full cursor-pointer items-center gap-2.5 rounded-md px-2 py-1.5 text-left text-[13px] transition-colors outline-none hover:bg-accent focus-visible:bg-accent',
								value === 'custom' && 'bg-accent',
							)}>
							<Server className="size-4 shrink-0 text-muted-foreground" />
							<span className="flex min-w-0 flex-1 flex-col">
								<span className="font-medium text-foreground">
									Custom endpoint
								</span>
								<span className="truncate text-[11px] text-muted-foreground">
									Any OpenAI-compatible base URL.
								</span>
							</span>
							{value === 'custom' && (
								<Check className="size-3.5 shrink-0 text-foreground" />
							)}
						</button>
					</div>
				</div>
			</PopoverContent>
		</Popover>
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
						<ProviderSelect
							value={picked}
							onPickPreset={pickPreset}
							onPickCustom={() => setPicked('custom')}
						/>
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
						<ModelSelect
							value={form.model}
							options={models}
							onChange={m => set('model', m)}
						/>
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
