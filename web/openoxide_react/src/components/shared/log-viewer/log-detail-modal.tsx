import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {Copy, Check, Code, FileText} from 'lucide-react';
import {useState} from 'react';
import type {ParsedLogEntry} from './types';

interface LogDetailModalProps {
	entry: ParsedLogEntry | null;
	onClose: () => void;
}

export function LogDetailModal({entry, onClose}: LogDetailModalProps) {
	const [copied, setCopied] = useState(false);

	if (!entry) return null;

	const handleCopy = () => {
		const textToCopy =
			entry.isJson && entry.jsonObject
				? JSON.stringify(entry.jsonObject, null, 2)
				: entry.raw;
		navigator.clipboard.writeText(textToCopy);
		setCopied(true);
		setTimeout(() => setCopied(false), 2000);
	};

	return (
		<Dialog open={Boolean(entry)} onOpenChange={onClose}>
			<DialogContent className="max-w-2xl border-border bg-card font-mono">
				<DialogHeader className="flex flex-row items-center justify-between border-b border-border pb-3">
					<DialogTitle className="flex items-center gap-2 text-sm font-semibold text-foreground">
						{entry.isJson ? (
							<Code className="h-4 w-4 text-primary" />
						) : (
							<FileText className="h-4 w-4 text-primary" />
						)}
						Log Entry Details
					</DialogTitle>
					<Button
						variant="outline"
						size="sm"
						onClick={handleCopy}
						className="h-7 gap-1.5 border-border text-xs font-semibold">
						{copied ? (
							<Check className="h-3.5 w-3.5 text-emerald-400" />
						) : (
							<Copy className="h-3.5 w-3.5" />
						)}
						{copied ? 'Copied' : 'Copy'}
					</Button>
				</DialogHeader>

				<div className="space-y-3 pt-2">
					<div className="flex flex-wrap gap-2 font-mono text-xs">
						<span className="rounded border border-border bg-muted px-2 py-0.5 text-muted-foreground">
							Level:{' '}
							<strong className="text-foreground">{entry.level}</strong>
						</span>
						<span className="rounded border border-border bg-muted px-2 py-0.5 text-muted-foreground">
							Stream:{' '}
							<strong className="text-foreground">{entry.stream}</strong>
						</span>
						{entry.timestamp && (
							<span className="rounded border border-border bg-muted px-2 py-0.5 text-muted-foreground">
								Timestamp:{' '}
								<strong className="text-foreground">
									{entry.timestamp}
								</strong>
							</span>
						)}
						{entry.stage && (
							<span className="rounded border border-primary/20 bg-primary/10 px-2 py-0.5 text-primary">
								Stage: <strong>{entry.stage}</strong>
							</span>
						)}
					</div>

					<div className="max-h-[360px] overflow-y-auto rounded-lg border border-border/80 bg-[#090d16] p-3">
						<pre className="font-mono text-xs leading-relaxed break-all whitespace-pre-wrap text-emerald-300">
							{entry.isJson && entry.jsonObject
								? JSON.stringify(entry.jsonObject, null, 2)
								: entry.clean || entry.raw}
						</pre>
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
