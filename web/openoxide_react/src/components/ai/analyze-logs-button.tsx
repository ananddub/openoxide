import {useState} from 'react';
import {Copy, Loader, Sparkles, Wrench} from 'lucide-react';
import {Link} from '@tanstack/react-router';
import {toast} from 'sonner';
import {
	fetchBuildDiagnosis,
	useAiAnalyzeLogs,
	useAiApplyFix,
	useAiEnabledSettings,
	type AiBuildDiagnosis,
	type AiLogAnalysisKind,
} from '#/api/live/ai';
import {formatApiError} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from '#/components/ui/popover';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

/** only the tail of the corpus goes to the model. */
const MAX_LOG_LINES = 200;

/** AI log-analysis popover for log surfaces: pick an enabled provider, send the last 200 lines, read the analysis. */
export function AnalyzeLogsButton({
	lines,
	context,
	deploymentId,
	failed = false,
}: {
	lines: string[];
	context: AiLogAnalysisKind;
	/** When set + failed, Analyze also fetches a build diagnosis so a
	 *  "Fix with AI" button can appear next to Re-analyze when fixable. */
	deploymentId?: number;
	failed?: boolean;
}) {
	const {settings, isPending} = useAiEnabledSettings();
	const analyze = useAiAnalyzeLogs();
	const applyFix = useAiApplyFix();
	const [open, setOpen] = useState(false);
	const [pickedId, setPickedId] = useState<string | null>(null);
	const [analysis, setAnalysis] = useState('');
	const [diagnosis, setDiagnosis] = useState<AiBuildDiagnosis | null>(
		null,
	);
	const [fixResult, setFixResult] = useState<string | null>(null);

	const selected =
		settings.find(s => String(s.id) === pickedId) ?? settings[0];
	const canAnalyze = !!selected && lines.length > 0;
	const canOfferFix = failed && !!deploymentId && deploymentId > 0;

	const run = async () => {
		if (!selected || !canAnalyze) return;
		// Reset per-run state so a previous fix result can't permanently hide
		// the "Fix with AI" button on the next Analyze.
		setFixResult(null);
		setDiagnosis(null);
		try {
			const result = await analyze.mutateAsync({
				aiSettingId: selected.id,
				context,
				logs: lines.slice(-MAX_LOG_LINES).join('\n'),
			});
			setAnalysis(result.analysis);
			// Force a fresh diagnosis on every Analyze of a failed deploy: a
			// stale stored verdict (e.g. from before a provider was enabled,
			// or a wrong "code_error" call) used to hide "Fix with AI"
			// forever, since the backend reused stored diagnoses.
			if (canOfferFix) {
				try {
					setDiagnosis(await fetchBuildDiagnosis(deploymentId!, true));
				} catch {
					setDiagnosis(null);
				}
			}
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	return (
		<Popover open={open} onOpenChange={setOpen}>
			<PopoverTrigger
				render={
					<Button
						variant="ghost"
						size="sm"
						title="Analyze logs with AI"
						onClick={() => setAnalysis('')}
						className="h-7 gap-1.5 rounded-md border border-border/60 px-2 text-[11px] font-medium text-muted-foreground hover:text-foreground">
						<Sparkles className="size-3" />
						AI
					</Button>
				}
			/>
			<PopoverContent
				side="bottom"
				align="end"
				className="w-[420px] max-w-[calc(100vw-2rem)] p-3">
				{isPending ? (
					<p className="flex items-center gap-2 px-1 py-2 text-[13px] text-muted-foreground">
						<Loader className="size-3.5 animate-spin" />
						Loading AI providers…
					</p>
				) : settings.length === 0 ? (
					<div className="flex flex-col gap-2 px-1 py-2">
						<p className="text-[13px] text-muted-foreground">
							No AI provider enabled yet.
						</p>
						<Button
							variant="outline"
							size="sm"
							render={<Link to="/ai" />}
							className="self-start">
							<Sparkles className="mr-1.5 size-3.5" />
							Configure AI Providers
						</Button>
					</div>
				) : (
					<div className="flex flex-col gap-2.5">
						<div className="flex items-center gap-2">
							<Select
								value={String(selected.id)}
								onValueChange={v => v && setPickedId(String(v))}>
								<SelectTrigger className="h-8 flex-1 bg-muted/20 text-xs">
									<SelectValue placeholder="Provider" />
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
							<Button
								size="sm"
								variant={analysis ? 'outline' : 'default'}
								disabled={!canAnalyze || analyze.isPending}
								onClick={() => void run()}
								className="h-8 shrink-0 gap-1.5 text-xs">
								{analyze.isPending ? (
									<Loader className="size-3.5 animate-spin" />
								) : (
									<Sparkles className="size-3.5" />
								)}
								{analyze.isPending
									? 'Analyzing…'
									: analysis
										? 'Re-analyze'
										: 'Analyze'}
							</Button>
							{diagnosis?.fixable && !fixResult && (
								<Button
									size="sm"
									disabled={applyFix.isPending}
									title={`Apply the AI fix to the ${diagnosis.fixTarget || 'config'} and redeploy`}
									onClick={() => {
										// Close the popover first so the redeploy's
										// re-render can't tear it down mid-request
										// (that left the panel visually "hung").
										setOpen(false);
										setFixResult('applying');
										toast.promise(applyFix.mutateAsync(deploymentId!), {
											loading: 'Applying AI fix…',
											success: r =>
												r.applied
													? r.message
													: `Could not apply: ${r.message}`,
											error: e => `Fix failed: ${e.message}`,
										});
									}}
									className="h-8 shrink-0 gap-1.5 border border-violet-500/40 bg-violet-500/10 text-xs text-violet-400 hover:bg-violet-500/20">
									{applyFix.isPending ? (
										<Loader className="size-3.5 animate-spin" />
									) : (
										<Wrench className="size-3.5" />
									)}
									{applyFix.isPending ? 'Fixing…' : 'Fix with AI'}
								</Button>
							)}
						</div>
						{fixResult && (
							<p className="text-[12px] text-emerald-400">{fixResult}</p>
						)}
						{diagnosis && !diagnosis.available && (
							<p className="text-[11px] text-amber-400">
								No AI diagnosis available for this deployment — check that
								an AI provider is enabled and the deploy recorded an error.
							</p>
						)}
						{diagnosis?.available && !diagnosis.fixable && (
							<div className="rounded-lg border border-border bg-muted/20 p-2.5">
								<p className="text-[11px] font-medium text-foreground">
									Diagnosis: {diagnosis.classification.replace('_', ' ')}
								</p>
								<p className="mt-1 text-[11px] leading-relaxed text-muted-foreground">
									{diagnosis.rootCause || diagnosis.explanation}
								</p>
							</div>
						)}
						<p className="text-[11px] text-muted-foreground">
							{Math.min(lines.length, MAX_LOG_LINES)} of {lines.length}{' '}
							lines · {context === 'BUILD' ? 'build' : 'runtime'} context
						</p>
						{analysis && (
							<div className="relative max-h-[320px] overflow-y-auto rounded-lg border border-border bg-muted/20">
								<pre className="p-3 pr-10 font-mono text-[12px] leading-relaxed whitespace-pre-wrap text-foreground">
									{analysis}
								</pre>
								<button
									type="button"
									aria-label="Copy analysis"
									onClick={() => {
										void navigator.clipboard
											.writeText(analysis)
											.then(() =>
												toast.success('Analysis copied to clipboard'),
											);
									}}
									className="absolute top-2 right-2 flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted/60 hover:text-foreground">
									<Copy className="size-3.5" />
								</button>
							</div>
						)}
					</div>
				)}
			</PopoverContent>
		</Popover>
	);
}
