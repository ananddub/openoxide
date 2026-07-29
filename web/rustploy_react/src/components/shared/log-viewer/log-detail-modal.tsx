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
		const textToCopy = entry.isJson && entry.jsonObject
			? JSON.stringify(entry.jsonObject, null, 2)
			: entry.raw;
		navigator.clipboard.writeText(textToCopy);
		setCopied(true);
		setTimeout(() => setCopied(false), 2000);
	};

	return (
		<Dialog open={Boolean(entry)} onOpenChange={onClose}>
			<DialogContent className="max-w-2xl bg-card border-border font-mono">
				<DialogHeader className="flex flex-row items-center justify-between border-b border-border pb-3">
					<DialogTitle className="text-sm font-semibold flex items-center gap-2 text-foreground">
						{entry.isJson ? (
							<Code className="w-4 h-4 text-primary" />
						) : (
							<FileText className="w-4 h-4 text-primary" />
						)}
						Log Entry Details
					</DialogTitle>
					<Button
						variant="outline"
						size="sm"
						onClick={handleCopy}
						className="h-7 text-xs font-semibold gap-1.5 border-border"
					>
						{copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
						{copied ? 'Copied' : 'Copy'}
					</Button>
				</DialogHeader>

				<div className="space-y-3 pt-2">
					<div className="flex flex-wrap gap-2 text-xs font-mono">
						<span className="bg-muted px-2 py-0.5 rounded border border-border text-muted-foreground">
							Level: <strong className="text-foreground">{entry.level}</strong>
						</span>
						<span className="bg-muted px-2 py-0.5 rounded border border-border text-muted-foreground">
							Stream: <strong className="text-foreground">{entry.stream}</strong>
						</span>
						{entry.timestamp && (
							<span className="bg-muted px-2 py-0.5 rounded border border-border text-muted-foreground">
								Timestamp: <strong className="text-foreground">{entry.timestamp}</strong>
							</span>
						)}
						{entry.stage && (
							<span className="bg-primary/10 text-primary px-2 py-0.5 rounded border border-primary/20">
								Stage: <strong>{entry.stage}</strong>
							</span>
						)}
					</div>

					<div className="bg-[#090d16] border border-border/80 rounded-lg p-3 max-h-[360px] overflow-y-auto">
						<pre className="text-xs text-emerald-300 font-mono whitespace-pre-wrap break-all leading-relaxed">
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
