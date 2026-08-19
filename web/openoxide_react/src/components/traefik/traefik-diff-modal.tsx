import {X, GitCompare} from 'lucide-react';
import {Button} from '#/components/ui/button';

interface TraefikDiffModalProps {
	isOpen: boolean;
	onClose: () => void;
	filePath: string;
	originalContent: string;
	modifiedContent: string;
}

export function TraefikDiffModal({
	isOpen,
	onClose,
	filePath,
	originalContent,
	modifiedContent,
}: TraefikDiffModalProps) {
	if (!isOpen) return null;

	const origLines = originalContent.split('\n');
	const modLines = modifiedContent.split('\n');
	const maxLines = Math.max(origLines.length, modLines.length);

	return (
		<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 p-4 backdrop-blur-sm">
			<div className="flex max-h-[85vh] w-full max-w-4xl animate-in flex-col overflow-hidden rounded-2xl border border-border bg-card shadow-2xl duration-150 zoom-in-95 fade-in">
				{/* Modal Header */}
				<div className="flex items-center justify-between border-b border-border/60 px-6 py-4">
					<div className="flex items-center gap-2">
						<GitCompare className="size-5 text-primary" />
						<div>
							<h3 className="text-base font-bold tracking-tight text-foreground">
								Configuration Diff
							</h3>
							<p className="font-mono text-xs text-muted-foreground">
								/etc/traefik/dynamic/{filePath}
							</p>
						</div>
					</div>
					<Button
						variant="ghost"
						size="sm"
						onClick={onClose}
						className="h-8 w-8 p-0 text-muted-foreground">
						<X className="size-4" />
					</Button>
				</div>

				{/* Side-by-side Diff Body */}
				<div className="flex-1 overflow-y-auto bg-[#181818] p-4 font-mono text-xs text-slate-300">
					<div className="grid grid-cols-2 gap-4">
						<div className="flex flex-col gap-0.5">
							<div className="border-b border-rose-500/20 pb-2 text-[10px] font-bold tracking-wider text-rose-400 uppercase">
								Original File
							</div>
							{Array.from({length: maxLines}).map((_, i) => {
								const line = origLines[i] ?? '';
								const modLine = modLines[i] ?? '';
								const isRemoved = line !== modLine;
								return (
									<div
										key={i}
										className={`rounded px-2 py-0.5 font-mono whitespace-pre ${
											isRemoved
												? 'border-l-2 border-rose-500 bg-rose-500/20 text-rose-300'
												: 'text-slate-400'
										}`}>
										{line}
									</div>
								);
							})}
						</div>

						<div className="flex flex-col gap-0.5">
							<div className="border-b border-emerald-500/20 pb-2 text-[10px] font-bold tracking-wider text-emerald-400 uppercase">
								Modified File
							</div>
							{Array.from({length: maxLines}).map((_, i) => {
								const line = origLines[i] ?? '';
								const modLine = modLines[i] ?? '';
								const isAdded = line !== modLine;
								return (
									<div
										key={i}
										className={`rounded px-2 py-0.5 font-mono whitespace-pre ${
											isAdded
												? 'border-l-2 border-emerald-500 bg-emerald-500/20 text-emerald-300'
												: 'text-slate-400'
										}`}>
										{modLine}
									</div>
								);
							})}
						</div>
					</div>
				</div>

				{/* Modal Footer */}
				<div className="flex justify-end border-t border-border/60 bg-muted/20 px-6 py-3">
					<Button
						variant="outline"
						size="sm"
						onClick={onClose}
						className="h-8 text-xs font-semibold">
						Close Diff View
					</Button>
				</div>
			</div>
		</div>
	);
}
