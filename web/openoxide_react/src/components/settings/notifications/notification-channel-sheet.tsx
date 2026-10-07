import {useEffect, useState} from 'react';
import {BellRing, Check, Loader2, Save} from 'lucide-react';
import {cn} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import type {NotificationPayload} from '#/api/live';
import {
	OptionRow,
	StepBody,
	StepCheckpoints,
	WizardField,
	type WizardStep,
} from '#/components/shared/sheet-wizard';
import {PROVIDERS, providerMeta} from './notification-providers';
import {
	TRIGGERS,
	TRIGGER_GROUPS,
	type DraftState,
} from './notification-triggers';
import {useNotificationSave} from './use-notification-save';

const CREATE_STEPS: WizardStep[] = [
	{id: 'provider', label: 'Provider'},
	{id: 'connection', label: 'Connection'},
	{id: 'events', label: 'Events'},
	{id: 'review', label: 'Review'},
];

/** Editing keeps the provider fixed but exposes its connection fields — blanks mean "keep what is stored", the patch only carries re-typed values. */
const EDIT_STEPS: WizardStep[] = [
	{id: 'connection', label: 'Connection'},
	{id: 'events', label: 'Events'},
	{id: 'review', label: 'Review'},
];

const STEP_DESC: Record<string, string> = {
	provider: 'Where should OpenOxide deliver the alerts?',
	connection: 'The address and credentials this provider needs.',
	events: 'Pick the events that fire a message to this channel.',
	review:
		'Name the channel — that is all that is stored besides the events.',
};

const STEP_DESC_EDIT: Record<string, string> = {
	...STEP_DESC,
	connection:
		'Optional — fill only what you want to change. Blank fields keep their current values.',
	review: 'Rename the channel or leave it as is.',
};

/** Add / edit a notification channel as a checkpoint wizard. */
export function NotificationChannelSheet({
	draft,
	setDraft,
	editing,
	orgId,
	onClose,
}: {
	draft: DraftState | null;
	setDraft: (d: DraftState | null) => void;
	editing: NotificationPayload | null;
	orgId: bigint;
	onClose: () => void;
}) {
	const open = draft !== null;
	const steps = editing ? EDIT_STEPS : CREATE_STEPS;

	const [stepIdx, setStepIdx] = useState(0);
	const [saving, setSaving] = useState(false);

	// Every open starts at the first checkpoint.
	useEffect(() => {
		if (open) setStepIdx(0);
	}, [open, editing]);

	const provider = draft ? providerMeta(draft.provider) : null;
	const save = useNotificationSave({
		draft,
		editing,
		provider,
		orgId,
		onSaved: onClose,
	});
	const patch = (d: Partial<DraftState>) =>
		draft && setDraft({...draft, ...d});

	const step = steps[stepIdx]?.id ?? 'review';
	const last = stepIdx === steps.length - 1;
	const ProviderIcon = provider?.icon ?? BellRing;

	const selectedTriggers = TRIGGERS.filter(
		t => draft?.triggers[t.key] === true,
	);
	const allSelected = selectedTriggers.length === TRIGGERS.length;

	// Shared blocks: the wizard shows them one per step, the edit page shows
	// them stacked on a single scrollable form. Both are guarded on `draft`
	// being non-null — they run on every render, including while closed.
	const connectionFields = provider?.fields.length ? (
		<div className="flex flex-col gap-5">
			{provider?.fields.map(f => (
				<WizardField key={f.key} label={f.label} optional={f.optional}>
					<Input
						type={f.type ?? 'text'}
						placeholder={f.placeholder}
						value={draft?.config[f.key] ?? ''}
						onChange={e =>
							patch({
								config: {
									...draft!.config,
									[f.key]: e.target.value,
								},
							})
						}
						className="bg-background font-mono"
					/>
				</WizardField>
			))}
		</div>
	) : (
		<p className="text-[13px] text-muted-foreground">
			{provider?.label} needs no connection details.
		</p>
	);

	const eventsSection = draft ? (
		<div className="flex flex-col gap-6">
			<div className="flex items-center justify-between gap-3">
				<p className="text-[13px] font-medium text-foreground">
					{selectedTriggers.length} of {TRIGGERS.length} selected
				</p>
				<button
					type="button"
					onClick={() =>
						patch({
							triggers: Object.fromEntries(
								TRIGGERS.map(t => [t.key, !allSelected]),
							),
						})
					}
					className="cursor-pointer text-[13px] font-medium text-primary hover:underline">
					{allSelected ? 'Clear all' : 'Select all'}
				</button>
			</div>

			{TRIGGER_GROUPS.map(group => (
				<div key={group} className="flex flex-col gap-2">
					<p className="text-xs font-semibold tracking-wider text-muted-foreground uppercase">
						{group}
					</p>
					<div className="grid grid-cols-1 gap-2 sm:grid-cols-2">
						{TRIGGERS.filter(t => t.group === group).map(t => {
							const selected = draft.triggers[t.key] === true;
							return (
								<button
									key={t.key}
									type="button"
									onClick={() =>
										patch({
											triggers: {
												...draft.triggers,
												[t.key]: !selected,
											},
										})
									}
									aria-pressed={selected}
									className={cn(
										'flex cursor-pointer items-center gap-2.5 rounded-lg border px-3 py-2.5 text-sm font-medium transition-all duration-150',
										selected
											? 'border-primary/50 bg-primary/[0.08] text-foreground'
											: 'border-border/60 text-muted-foreground hover:bg-muted/30',
									)}>
									<span className="truncate">{t.label}</span>
									{selected && (
										<Check className="ml-auto size-3.5 shrink-0 text-primary" />
									)}
								</button>
							);
						})}
					</div>
				</div>
			))}
		</div>
	) : null;

	const connectionReady =
		!provider ||
		provider.fields.every(
			f => f.optional || (draft?.config[f.key] ?? '').trim() !== '',
		);
	const canContinue =
		step === 'connection'
			? editing || connectionReady
			: step === 'events'
				? selectedTriggers.length > 0
				: true;

	const submit = async () => {
		setSaving(true);
		try {
			await save();
		} finally {
			setSaving(false);
		}
	};

	return (
		<Sheet open={open} onOpenChange={o => !o && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>
						{editing ? 'Edit channel' : 'New notification channel'}
					</SheetTitle>
					<SheetDescription>
						{editing
							? 'Everything on one page — blanks keep their stored values.'
							: (editing ? STEP_DESC_EDIT : STEP_DESC)[step]}
					</SheetDescription>
					{!editing && <StepCheckpoints steps={steps} idx={stepIdx} />}
				</SheetHeader>

				{draft && (
					<div className="flex min-h-0 flex-1 flex-col overflow-y-auto px-6 py-6">
						{editing ? (
							<div className="flex flex-col gap-8">
								{connectionFields}
								{eventsSection}
								<WizardField label="Channel Name" required>
									<Input
										placeholder="e.g. Ops alerts"
										value={draft.name}
										onChange={e => patch({name: e.target.value})}
									/>
								</WizardField>
							</div>
						) : (
							<StepBody key={step}>
								{step === 'provider' && (
									<div className="flex flex-col gap-2">
										{PROVIDERS.map(p => {
											const Icon = p.icon;
											return (
												<OptionRow
													key={p.id}
													selected={draft.provider === p.id}
													onClick={() =>
														patch({provider: p.id, config: {}})
													}
													icon={<Icon className="size-5" />}
													title={p.label}
													subtitle={p.description}
												/>
											);
										})}
									</div>
								)}

								{step === 'connection' && connectionFields}

								{step === 'events' && eventsSection}

								{step === 'review' && (
									<div className="flex flex-col gap-5">
										<div className="flex items-center gap-3.5 rounded-xl border border-border/60 bg-muted/20 px-4 py-3.5">
											<span className="flex size-10 shrink-0 items-center justify-center rounded-lg bg-background text-foreground">
												<ProviderIcon className="size-5" />
											</span>
											<div className="min-w-0 flex-1">
												<p className="truncate text-sm font-medium text-foreground">
													{provider?.label ?? draft.provider}
												</p>
												<p className="mt-0.5 truncate text-[13px] text-muted-foreground">
													{selectedTriggers.length === 0
														? 'No events selected yet'
														: selectedTriggers.length === TRIGGERS.length
															? 'All events'
															: selectedTriggers
																	.slice(0, 3)
																	.map(t => t.label)
																	.join(', ') +
																(selectedTriggers.length > 3
																	? ` +${selectedTriggers.length - 3} more`
																	: '')}
												</p>
											</div>
										</div>

										<WizardField label="Channel Name" required>
											<Input
												placeholder="e.g. Ops alerts"
												value={draft.name}
												onChange={e => patch({name: e.target.value})}
												autoFocus
											/>
										</WizardField>
									</div>
								)}
							</StepBody>
						)}
					</div>
				)}

				{editing ? (
					<SheetFooter>
						<Button
							onClick={() => void submit()}
							disabled={
								saving ||
								!draft?.name.trim() ||
								selectedTriggers.length === 0
							}
							className="h-10 gap-1.5 px-5">
							{saving ? (
								<Loader2 className="size-3.5 animate-spin" />
							) : (
								<Save className="size-3.5" />
							)}
							Save changes
						</Button>
					</SheetFooter>
				) : last ? (
					<SheetFooter>
						{stepIdx > 0 && (
							<Button
								variant="outline"
								className="mr-auto"
								onClick={() => setStepIdx(i => i - 1)}>
								Back
							</Button>
						)}
						<Button
							onClick={() => void submit()}
							disabled={
								saving ||
								!draft?.name.trim() ||
								selectedTriggers.length === 0
							}
							className="h-10 gap-1.5 px-5">
							{saving ? (
								<Loader2 className="size-3.5 animate-spin" />
							) : (
								<Save className="size-3.5" />
							)}
							{editing ? 'Save changes' : 'Save'}
						</Button>
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
								setStepIdx(i => Math.min(i + 1, steps.length - 1))
							}>
							Continue
						</Button>
					</SheetFooter>
				)}
			</SheetContent>
		</Sheet>
	);
}
