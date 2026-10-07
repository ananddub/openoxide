import {useState, useRef, useEffect, useCallback} from 'react';
import {
	Sparkles,
	Loader,
	Send,
	ShieldAlert,
	Check,
	X,
	RotateCcw,
} from 'lucide-react';
import {toast} from 'sonner';
import {useAiEnabledSettings, streamAssistant} from '#/api/live/ai';
import {
	AiAssistantEventKind,
	AiInteractionKind,
} from '#/gen/connect/openoxide/api/v1/ai_pb';
import {formatApiError} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {Textarea} from '#/components/ui/textarea';
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
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {cn} from '#/api/utils';
import {MiniMarkdown} from './mini-markdown';

/* ── Types ────────────────────────────────────────────── */

type Option = {id: string; label: string; description: string};
type Interaction =
	| {kind: 'confirm'; question: string; pendingActions: string}
	| {kind: 'choice'; question: string; options: Option[]};

type ChatMessage = {
	role: 'user' | 'assistant';
	text: string;
	interaction?: Interaction;
	resolved?: boolean;
	/** Ephemeral thinking/status lines shown while the agent works. */
	steps?: string[];
};

const SESSION_KEY = 'openoxide-ai-session-id';

function loadSessionId(): string | undefined {
	try {
		return localStorage.getItem(SESSION_KEY) ?? undefined;
	} catch {
		return undefined;
	}
}
function saveSessionId(id: string) {
	try {
		localStorage.setItem(SESSION_KEY, id);
	} catch {}
}

/* ── Component ────────────────────────────────────────── */

export function AiChatSheet({
	open,
	onClose,
}: {
	open: boolean;
	onClose: () => void;
}) {
	const {settings, isPending} = useAiEnabledSettings();
	const [messages, setMessages] = useState<ChatMessage[]>([]);
	const [input, setInput] = useState('');
	const [aiSettingId, setAiSettingId] = useState<number | null>(null);
	const [busy, setBusy] = useState(false);
	const scrollRef = useRef<HTMLDivElement>(null);
	const sessionRef = useRef<string | undefined>(loadSessionId());

	useEffect(() => {
		if (aiSettingId === null && settings.length > 0)
			setAiSettingId(settings[0].id);
	}, [settings, aiSettingId]);

	useEffect(() => {
		scrollRef.current?.scrollTo({
			top: scrollRef.current.scrollHeight,
			behavior: 'smooth',
		});
	}, [messages, busy]);

	const send = useCallback(
		async (
			prompt: string,
			prior?: ChatMessage[],
			pendingActions?: string,
		) => {
			const text = prompt.trim();
			if (!text || busy) return;
			const base = prior ?? messages;
			const next: ChatMessage[] = [...base, {role: 'user', text}];
			setMessages(next);
			setInput('');
			setBusy(true);

			// Add a placeholder assistant message that will accumulate steps + reply.
			const assistantIdx = next.length;
			setMessages(m => [...m, {role: 'assistant', text: '', steps: []}]);

			const controller = new AbortController();
			try {
				const events = streamAssistant({
					prompt: text,
					aiSettingId: aiSettingId ?? undefined,
					pendingActions,
					sessionId: sessionRef.current,
					signal: controller.signal,
				});

				for await (const event of events) {
					// Persist session id from the server.
					if (event.sessionId) {
						sessionRef.current = event.sessionId;
						saveSessionId(event.sessionId);
					}

					setMessages(prev => {
						const updated = [...prev];
						const msg = {...updated[assistantIdx]};
						const steps = [...(msg.steps ?? [])];

						switch (event.kind) {
							case AiAssistantEventKind.THINKING:
							case AiAssistantEventKind.STATUS:
								if (event.text) steps.push(event.text);
								msg.steps = steps;
								break;
							case AiAssistantEventKind.ACTION_RESULT:
								if (event.text) steps.push(`✓ ${event.text}`);
								msg.steps = steps;
								break;
							case AiAssistantEventKind.REPLY:
								msg.text = event.text;
								break;
							case AiAssistantEventKind.INTERACTION:
								msg.text = event.text || msg.text;
								msg.interaction = toInteraction(event.interaction);
								break;
							case AiAssistantEventKind.DONE:
								break;
						}
						updated[assistantIdx] = msg;
						return updated;
					});
				}
			} catch (err: unknown) {
				const isAbort = (err as {name?: string})?.name === 'AbortError';
				if (!isAbort) {
					const message = formatApiError(err);
					setMessages(m => {
						const updated = [...m];
						updated[assistantIdx] = {
							role: 'assistant',
							text: `⚠️ ${message}`,
						};
						return updated;
					});
					toast.error(message);
				}
			} finally {
				setBusy(false);
			}
		},
		[busy, messages, aiSettingId],
	);

	const resolve = (index: number): ChatMessage[] => {
		const updated = messages.map((m, i) =>
			i === index ? {...m, resolved: true} : m,
		);
		setMessages(updated);
		return updated;
	};

	const resetSession = () => {
		sessionRef.current = undefined;
		try {
			localStorage.removeItem(SESSION_KEY);
		} catch {}
		setMessages([]);
	};

	const noProvider = !isPending && settings.length === 0;

	// The latest message carrying an unresolved interaction drives the
	// dedicated action panel above the input.
	let pending: Interaction | undefined;
	let pendingIdx: number | null = null;
	for (let i = messages.length - 1; i >= 0; i--) {
		if (messages[i].interaction && !messages[i].resolved) {
			pending = messages[i].interaction;
			pendingIdx = i;
			break;
		}
	}

	return (
		<Sheet open={open} onOpenChange={o => !o && onClose()}>
			<SheetContent className="flex flex-col sm:data-[side=right]:w-[560px]">
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2">
						<Sparkles className="size-4 text-primary" />
						AI Assistant
						<button
							type="button"
							aria-label="New chat"
							onClick={resetSession}
							className="ml-auto flex size-7 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground">
							<RotateCcw className="size-3.5" />
						</button>
					</SheetTitle>
					<SheetDescription>
						Ask in plain language — actions stream live as the agent works.
					</SheetDescription>
					{settings.length > 0 && (
						<div className="pt-2">
							<Select
								value={aiSettingId !== null ? String(aiSettingId) : ''}
								onValueChange={v => setAiSettingId(v ? Number(v) : null)}>
								<SelectTrigger className="h-8 text-xs">
									<SelectValue placeholder="Select a provider" />
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
					)}
				</SheetHeader>

				<div ref={scrollRef} className="flex-1 overflow-y-auto px-6 py-4">
					{noProvider ? (
						<p className="text-[13px] text-muted-foreground">
							No AI provider enabled. Add one on the AI Providers page.
						</p>
					) : messages.length === 0 ? (
						<div className="flex flex-col gap-2 pt-6 text-center">
							<p className="text-sm text-muted-foreground">
								What would you like to do?
							</p>
							<div className="mt-2 flex flex-col gap-2">
								{[
									'List my projects',
									'Deploy blinko',
									'Show my services',
								].map(ex => (
									<button
										key={ex}
										type="button"
										onClick={() => void send(ex)}
										className="mx-auto cursor-pointer rounded-full border border-border/60 bg-muted/20 px-3 py-1.5 text-[12px] text-muted-foreground transition-colors hover:border-border hover:text-foreground">
										{ex}
									</button>
								))}
							</div>
						</div>
					) : (
						<div className="flex flex-col gap-3">
							{messages.map((m, i) => (
								<div
									key={i}
									className={cn(
										'flex',
										m.role === 'user' ? 'justify-end' : 'justify-start',
									)}>
									<div
										className={cn(
											'max-w-[85%] rounded-2xl px-3.5 py-2 text-[13px]',
											m.role === 'user'
												? 'bg-primary whitespace-pre-wrap text-primary-foreground'
												: 'bg-muted/40 text-foreground',
										)}>
										{/* Live steps (thinking/status) — compact, truncated, contained */}
										{m.steps && m.steps.length > 0 && (
											<div className="mb-1.5 flex flex-col gap-0.5 overflow-hidden border-l-2 border-border/50 pl-2 text-[11px] text-muted-foreground">
												{m.steps.map((s, si) => (
													<span key={si} className="truncate" title={s}>
														{truncateStep(s)}
													</span>
												))}
											</div>
										)}
										{/* Reply text — markdown rendered */}
										{m.text &&
											(m.role === 'assistant' ? (
												<MiniMarkdown text={m.text} />
											) : (
												<span className="whitespace-pre-wrap">
													{m.text}
												</span>
											))}
										{/* Busy indicator for the current assistant turn */}
										{m.role === 'assistant' &&
											busy &&
											i === messages.length - 1 &&
											!m.text && (
												<span className="flex items-center gap-1.5 text-muted-foreground">
													<Loader className="size-3.5 animate-spin" />
													Working…
												</span>
											)}
									</div>
								</div>
							))}
						</div>
					)}
				</div>

				{/* Dedicated interaction panel — sits above the input so Yes/No
				    and option cards read as a professional action bar, not as
				    buttons buried inside a chat bubble. */}
				{pending && pendingIdx !== null && pending.kind === 'confirm' && (
					<div className="border-t bg-muted/20 px-6 py-3">
						<div className="mb-2 flex items-start gap-2 text-[13px] text-foreground">
							<ShieldAlert className="mt-0.5 size-4 shrink-0 text-amber-500" />
							<span>{pending.question}</span>
						</div>
						<div className="flex gap-2">
							<Button
								variant="destructive"
								disabled={busy}
								onClick={() => {
									const pa =
										pending.kind === 'confirm'
											? pending.pendingActions
											: undefined;
									const prior = resolve(pendingIdx);
									void send('yes', prior, pa);
								}}
								className="h-9 flex-1 gap-1.5">
								<Check className="size-4" />
								Yes, do it
							</Button>
							<Button
								variant="outline"
								disabled={busy}
								onClick={() => {
									const prior = resolve(pendingIdx);
									void send('no', prior);
								}}
								className="h-9 flex-1 gap-1.5">
								<X className="size-4" />
								No, cancel
							</Button>
						</div>
					</div>
				)}
				{pending && pendingIdx !== null && pending.kind === 'choice' && (
					<div className="border-t bg-muted/20 px-6 py-3">
						<p className="mb-2 text-[13px] font-medium text-foreground">
							{pending.question}
						</p>
						<div className="flex max-h-[40vh] flex-col gap-1.5 overflow-y-auto">
							{pending.options.map(opt => (
								<button
									key={opt.id}
									type="button"
									disabled={busy}
									onClick={() => {
										const prior = resolve(pendingIdx);
										void send(opt.label, prior);
									}}
									className="flex flex-col rounded-lg border border-border/60 bg-background px-3 py-2 text-start transition-colors hover:border-primary/60 hover:bg-primary/5 disabled:opacity-50">
									<span className="text-[13px] font-medium text-foreground">
										{opt.label}
									</span>
									{opt.description && (
										<span className="text-[12px] text-muted-foreground">
											{opt.description}
										</span>
									)}
								</button>
							))}
						</div>
					</div>
				)}

				<div className="border-t px-6 py-4">
					<div className="flex items-end gap-2">
						<Textarea
							rows={2}
							value={input}
							disabled={noProvider || busy}
							placeholder="Ask the assistant…"
							onChange={e => setInput(e.target.value)}
							onKeyDown={e => {
								if (e.key === 'Enter' && !e.shiftKey) {
									e.preventDefault();
									void send(input);
								}
							}}
							className="min-h-0 resize-none text-[13px]"
						/>
						<Button
							disabled={noProvider || busy || !input.trim()}
							onClick={() => void send(input)}
							className="h-10 shrink-0 gap-1.5">
							<Send className="size-3.5" />
						</Button>
					</div>
				</div>
			</SheetContent>
		</Sheet>
	);
}

/* ── Helpers ──────────────────────────────────────────── */

/** Truncate a step line: keep the emoji + first 80 chars, hide raw JSON. */
function truncateStep(s: string): string {
	// Raw JSON results → just "✓ done".
	if (s.startsWith('✓ {') || s.startsWith('✓ [')) return '✓ done';
	// Verbose tool results → clean summaries.
	if (s.includes('Actions matching')) return '✓ found matching actions';
	if (s.includes('Available templates')) return '✓ loaded templates';
	if (s.startsWith('✓ action failed') || s.startsWith('✓ No such'))
		return s.slice(0, 70) + (s.length > 70 ? '…' : '');
	if (s.startsWith('✓ ')) return '✓ done';
	if (s.length > 70) return s.slice(0, 68) + '…';
	return s;
}

function toInteraction(
	raw:
		| {
				kind: AiInteractionKind;
				question: string;
				options: {
					id: string;
					label: string;
					description: string;
				}[];
				pendingActions: string;
		  }
		| undefined,
): Interaction | undefined {
	if (!raw) return undefined;
	if (raw.kind === AiInteractionKind.CONFIRM) {
		return {
			kind: 'confirm',
			question: raw.question,
			pendingActions: raw.pendingActions ?? '',
		};
	}
	if (raw.kind === AiInteractionKind.CHOICE) {
		return {
			kind: 'choice',
			question: raw.question,
			options: raw.options.map(o => ({
				id: o.id,
				label: o.label,
				description: o.description,
			})),
		};
	}
	return undefined;
}
