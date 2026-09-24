import {useState} from 'react';
import {
	Copy,
	Download,
	Eye,
	EyeOff,
	FileDown,
	KeyRound,
	Plus,
	Trash2,
} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';

export type EnvRow = {key: string; value: string};

/** Parse a KEY=VALUE blob into rows — comments and blank lines dropped. */
export function parseEnv(text: string): EnvRow[] {
	const rows: EnvRow[] = [];
	for (const rawLine of text.split(/\r?\n/)) {
		const line = rawLine.trim();
		if (!line || line.startsWith('#') || line.startsWith(';')) continue;
		const eq = line.indexOf('=');
		if (eq === -1) continue;
		const key = line.slice(0, eq).replace(/^export\s+/, '').trim();
		if (!key) continue;
		let value = line.slice(eq + 1).trim();
		if (
			value.length >= 2 &&
			((value.startsWith('"') && value.endsWith('"')) ||
				(value.startsWith("'") && value.endsWith("'")))
		)
			value = value.slice(1, -1);
		rows.push({key, value});
	}
	return rows;
}

export const serializeEnv = (rows: EnvRow[]): string =>
	rows
		.filter(r => r.key.trim())
		.map(r => `${r.key.trim()}=${r.value}`)
		.join('\n');

/**
 * The KEY=VALUE row editor used by every environment surface: header strip
 * (file · count · import/export/show-hide), fixed-height grid rows and an
 * add-variable strip. Rows stay controlled by the caller so save state can
 * live alongside the sheet footer.
 */
export function EnvRowsCard({
	rows,
	onRowsChange,
	fileName,
	isModified,
}: {
	rows: EnvRow[];
	onRowsChange: (updater: (prev: EnvRow[]) => EnvRow[]) => void;
	fileName: string;
	isModified: boolean;
}) {
	const [showValues, setShowValues] = useState(true);
	const [importOpen, setImportOpen] = useState(false);
	const [importText, setImportText] = useState('');

	const setAt = (i: number, patch: Partial<EnvRow>) =>
		onRowsChange(prev =>
			prev.map((r, idx) => (idx === i ? {...r, ...patch} : r)),
		);

	const removeAt = (i: number) =>
		onRowsChange(prev => prev.filter((_, idx) => idx !== i));

	const addRow = () => onRowsChange(prev => [...prev, {key: '', value: ''}]);

	const handleImport = () => {
		const incoming = parseEnv(importText);
		if (incoming.length === 0) {
			toast.error('No KEY=VALUE lines found in the pasted text');
			return;
		}
		onRowsChange(prev => {
			const next = [...prev];
			for (const {key, value} of incoming) {
				const at = next.findIndex(r => r.key === key);
				if (at === -1) next.push({key, value});
				else next[at] = {key, value};
			}
			return next;
		});
		setImportOpen(false);
		setImportText('');
		toast.success(
			`Imported ${incoming.length} variable${incoming.length === 1 ? '' : 's'}`,
		);
	};

	/** Export the current rows as a .env file download. */
	const handleExportFile = () => {
		const blob = new Blob([`${serializeEnv(rows)}\n`], {
			type: 'text/plain;charset=utf-8',
		});
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = fileName;
		document.body.appendChild(a);
		a.click();
		a.remove();
		URL.revokeObjectURL(url);
		toast.success(`Downloaded ${fileName}`);
	};

	const handleExportCopy = async () => {
		try {
			await navigator.clipboard.writeText(serializeEnv(rows));
			toast.success('Copied to clipboard');
		} catch {
			toast.error('Could not copy to clipboard');
		}
	};

	return (
		<div className="flex min-h-0 flex-1 flex-col overflow-hidden rounded-xl border border-border bg-card">
			<div className="flex shrink-0 items-center justify-between gap-3 border-b border-border bg-muted/40 px-3 py-2">
				<div className="flex min-w-0 items-center gap-2">
					<KeyRound className="size-3.5 shrink-0 text-muted-foreground/70" />
					<span className="truncate font-mono text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
						{fileName}
					</span>
					<span className="shrink-0 rounded-full bg-foreground/[0.06] px-1.5 py-px font-mono text-[10px] tabular-nums text-muted-foreground">
						{rows.filter(r => r.key.trim()).length}
					</span>
					{isModified && (
						<span className="shrink-0 rounded-full bg-foreground/[0.07] px-2 py-0.5 text-[10px] font-medium text-foreground/70">
							Unsaved
						</span>
					)}
				</div>
				<div className="flex shrink-0 items-center gap-1.5">
					<Button
						type="button"
						variant="ghost"
						size="sm"
						onClick={() => setImportOpen(true)}
						className="h-7 gap-1.5 rounded-lg px-2 text-xs font-medium text-muted-foreground hover:text-foreground">
						<FileDown className="size-3.5" />
						Import
					</Button>
					<Button
						type="button"
						variant="ghost"
						size="sm"
						onClick={handleExportFile}
						className="h-7 gap-1.5 rounded-lg px-2 text-xs font-medium text-muted-foreground hover:text-foreground">
						<Download className="size-3.5" />
						Export
					</Button>
					<button
						type="button"
						onClick={() => void handleExportCopy()}
						title="Copy as .env text"
						aria-label="Copy variables as .env text"
						className="flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground">
						<Copy className="size-3.5" />
					</button>
					<button
						type="button"
						onClick={() => setShowValues(v => !v)}
						title={showValues ? 'Hide values' : 'Show values'}
						aria-label={showValues ? 'Hide values' : 'Show values'}
						className="flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground">
						{showValues ? (
							<EyeOff className="size-3.5" />
						) : (
							<Eye className="size-3.5" />
						)}
					</button>
				</div>
			</div>

			<div className="min-h-0 flex-1 overflow-y-auto">
				{rows.length === 0 ? (
					<div className="flex h-full flex-col items-center justify-center gap-2.5 px-4 py-10 text-center">
						<div className="flex size-9 items-center justify-center rounded-full bg-muted/60">
							<KeyRound className="size-4 text-muted-foreground" />
						</div>
						<p className="text-xs text-muted-foreground">
							No variables yet — add one below or import a .env
							file.
						</p>
					</div>
				) : (
					rows.map((r, i) => (
						<div
							key={i}
							className="group grid h-12 grid-cols-[minmax(0,2fr)_1px_minmax(0,3fr)_auto] items-stretch border-b border-border/60 transition-colors hover:bg-accent/30 focus-within:bg-accent/40">
							<Input
								value={r.key}
								spellCheck={false}
								onChange={e =>
									setAt(i, {
										key: e.target.value.toUpperCase(),
									})
								}
								placeholder="KEY"
								className="h-full w-full min-w-0 rounded-none border-0 bg-transparent px-3 font-mono text-xs font-semibold tracking-wide uppercase shadow-none focus-visible:ring-0"
							/>
							<div className="self-stretch bg-border/60" />
							<Input
								value={r.value}
								type={showValues ? 'text' : 'password'}
								spellCheck={false}
								onChange={e =>
									setAt(i, {value: e.target.value})
								}
								placeholder="value"
								className="h-full w-full min-w-0 rounded-none border-0 bg-transparent px-3 font-mono text-xs shadow-none focus-visible:ring-0"
							/>
							<div className="flex items-center pr-1.5">
								<button
									type="button"
									onClick={() => removeAt(i)}
									aria-label={`Remove ${r.key || 'variable'}`}
									className="flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground/50 opacity-0 transition-all group-hover:text-muted-foreground group-hover:opacity-100 hover:bg-destructive/10 hover:text-destructive focus-visible:opacity-100">
									<Trash2 className="size-3.5" />
								</button>
							</div>
						</div>
					))
				)}
			</div>

			<button
				type="button"
				onClick={addRow}
				className="flex shrink-0 cursor-pointer items-center gap-2 bg-muted/20 px-3 py-2.5 text-xs font-medium text-muted-foreground transition-colors hover:bg-muted/40 hover:text-foreground">
				<span className="flex size-[18px] items-center justify-center rounded-[5px] border border-dashed border-border">
					<Plus className="size-3" />
					<span className="sr-only">Add</span>
				</span>
				Add variable
			</button>

			{/* Paste-a-file dialog → merged into the rows above */}
			<Dialog open={importOpen} onOpenChange={setImportOpen}>
				<DialogContent className="sm:max-w-md">
					<DialogHeader>
						<DialogTitle>Import from .env</DialogTitle>
						<DialogDescription>
							Paste the whole file — comments and blank lines are
							ignored, and keys that already exist get updated.
						</DialogDescription>
					</DialogHeader>
					<Textarea
						value={importText}
						onChange={e => setImportText(e.target.value)}
						placeholder={'DB_HOST=localhost\nDB_PORT=5432\n# a comment'}
						spellCheck={false}
						className="min-h-44 resize-y font-mono text-xs"
					/>
					<DialogFooter>
						<Button
							type="button"
							variant="ghost"
							onClick={() => setImportOpen(false)}
							className="h-8 text-xs">
							Cancel
						</Button>
						<Button
							type="button"
							onClick={handleImport}
							disabled={!importText.trim()}
							className="h-8 px-4 text-xs">
							Import
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</div>
	);
}
