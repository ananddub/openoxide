import {useState} from 'react';
import {Copy, Loader, Sparkles} from 'lucide-react';
import {Link} from '@tanstack/react-router';
import {toast} from 'sonner';
import {
	useAiAnalyzeLogs,
	useAiEnabledSettings,
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
}: {
	lines: string[];
	context: AiLogAnalysisKind;
}) {
	const {settings, isPending} = useAiEnabledSettings();
	const analyze = useAiAnalyzeLogs();
	const [open, setOpen] = useState(false);
	const [pickedId, setPickedId] = useState<string | null>(null);
	const [analysis, setAnalysis] = useState('');

	const selected =
		settings.find(s => String(s.id) === pickedId) ?? settings[0];
	const canAnalyze = !!selected && lines.length > 0;

	const run = async () => {
		if (!selected || !canAnalyze) return;
		try {
			const result = await analyze.mutateAsync({
				aiSettingId: selected.id,
				context,
				logs: lines.slice(-MAX_LOG_LINES).join('\n'),
			});
			setAnalysis(result.analysis);
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
						</div>
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
