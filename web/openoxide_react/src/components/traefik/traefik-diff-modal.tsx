import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
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
	const origLines = originalContent.split('\n');
	const modLines = modifiedContent.split('\n');
	const maxLines = Math.max(origLines.length, modLines.length);

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="max-h-[85vh] w-full max-w-4xl grid-cols-1 grid-rows-[auto_minmax(0,1fr)_auto] gap-0 overflow-hidden p-0 sm:max-w-4xl">
				<DialogHeader className="border-b border-border px-6 py-4">
					<DialogTitle className="text-base font-bold tracking-tight">
						Configuration Diff
					</DialogTitle>
					<p className="font-mono text-xs text-muted-foreground">
						/etc/traefik/dynamic/{filePath}
					</p>
				</DialogHeader>

				{/* Side-by-side Diff Body */}
				<div className="min-h-0 overflow-y-auto bg-muted/30 p-4 font-mono text-xs">
					<div className="grid grid-cols-2 gap-4">
						<div className="flex flex-col gap-0.5">
							<div className="border-b border-rose-500/20 pb-2 text-[10px] font-bold tracking-wider text-rose-500 uppercase">
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
												? 'border-l-2 border-rose-500 bg-rose-500/15 text-rose-500'
												: 'text-muted-foreground'
										}`}>
										{line}
									</div>
								);
							})}
						</div>

						<div className="flex flex-col gap-0.5">
							<div className="border-b border-emerald-500/20 pb-2 text-[10px] font-bold tracking-wider text-emerald-500 uppercase">
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
												? 'border-l-2 border-emerald-500 bg-emerald-500/15 text-emerald-500'
												: 'text-muted-foreground'
										}`}>
										{modLine}
									</div>
								);
							})}
						</div>
					</div>
				</div>

				<div className="flex justify-end border-t border-border bg-muted/20 px-6 py-3">
					<Button
						variant="outline"
						size="sm"
						onClick={onClose}
						className="h-9 cursor-pointer text-xs font-semibold">
						Close Diff View
					</Button>
				</div>
			</DialogContent>
		</Dialog>
	);
}
