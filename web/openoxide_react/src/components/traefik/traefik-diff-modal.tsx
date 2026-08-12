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
		<div className="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4">
			<div className="bg-card border border-border rounded-2xl shadow-2xl w-full max-w-4xl max-h-[85vh] flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-150">
				{/* Modal Header */}
				<div className="flex items-center justify-between px-6 py-4 border-b border-border/60">
					<div className="flex items-center gap-2">
						<GitCompare className="size-5 text-primary" />
						<div>
							<h3 className="text-base font-bold text-foreground tracking-tight">Configuration Diff</h3>
							<p className="text-xs text-muted-foreground font-mono">/etc/traefik/dynamic/{filePath}</p>
						</div>
					</div>
					<Button variant="ghost" size="sm" onClick={onClose} className="h-8 w-8 p-0 text-muted-foreground">
						<X className="size-4" />
					</Button>
				</div>

				{/* Side-by-side Diff Body */}
				<div className="flex-1 overflow-y-auto p-4 bg-[#181818] font-mono text-xs text-slate-300">
					<div className="grid grid-cols-2 gap-4">
						<div className="flex flex-col gap-0.5">
							<div className="text-[10px] font-bold uppercase tracking-wider text-rose-400 pb-2 border-b border-rose-500/20">
								Original File
							</div>
							{Array.from({length: maxLines}).map((_, i) => {
								const line = origLines[i] ?? '';
								const modLine = modLines[i] ?? '';
								const isRemoved = line !== modLine;
								return (
									<div
										key={i}
										className={`px-2 py-0.5 whitespace-pre font-mono rounded ${
											isRemoved ? 'bg-rose-500/20 text-rose-300 border-l-2 border-rose-500' : 'text-slate-400'
										}`}>
										{line}
									</div>
								);
							})}
						</div>

						<div className="flex flex-col gap-0.5">
							<div className="text-[10px] font-bold uppercase tracking-wider text-emerald-400 pb-2 border-b border-emerald-500/20">
								Modified File
							</div>
							{Array.from({length: maxLines}).map((_, i) => {
								const line = origLines[i] ?? '';
								const modLine = modLines[i] ?? '';
								const isAdded = line !== modLine;
								return (
									<div
										key={i}
										className={`px-2 py-0.5 whitespace-pre font-mono rounded ${
											isAdded ? 'bg-emerald-500/20 text-emerald-300 border-l-2 border-emerald-500' : 'text-slate-400'
										}`}>
										{modLine}
									</div>
								);
							})}
						</div>
					</div>
				</div>

				{/* Modal Footer */}
				<div className="flex justify-end px-6 py-3 border-t border-border/60 bg-muted/20">
					<Button variant="outline" size="sm" onClick={onClose} className="h-8 text-xs font-semibold">
						Close Diff View
					</Button>
				</div>
			</div>
		</div>
	);
}
