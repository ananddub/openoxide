import {useState, useEffect} from 'react';
import Editor from '@monaco-editor/react';
import {Lock, LockOpen, FileCode, Sparkles} from 'lucide-react';
import {Button} from '#/components/ui/button';

interface TraefikEditorProps {
	selectedFilePath: string | null;
	content: string;
	onChangeContent: (val: string) => void;
	isReadOnly: boolean;
	isLoading: boolean;
}

export function TraefikEditor({
	selectedFilePath,
	content,
	onChangeContent,
	isReadOnly,
	isLoading,
}: TraefikEditorProps) {
	const [isLocked, setIsLocked] = useState(true);

	const effectiveReadOnly = isReadOnly || isLocked;

	useEffect(() => {
		setIsLocked(true);
	}, [selectedFilePath]);

	if (!selectedFilePath) {
		return (
			<div className="flex h-full min-h-[400px] flex-1 flex-col items-center justify-center gap-3 p-12 text-center">
				<FileCode className="mb-1 size-8 text-muted-foreground/60" />
				<div className="space-y-1">
					<h3 className="text-sm font-bold text-foreground">
						Select a file to edit
					</h3>
					<p className="max-w-xs text-xs leading-relaxed text-muted-foreground">
						Choose a dynamic YAML configuration file from the sidebar to
						inspect or edit.
					</p>
				</div>
			</div>
		);
	}

	const fullPathDisplay = `/etc/openoxide/traefik/${selectedFilePath}`;

	return (
		<div className="flex h-full min-h-[450px] flex-1 flex-col overflow-hidden bg-[#1e1e1e]">
			{/* Clean Toolbar Header */}
			<div className="flex shrink-0 items-center justify-between border-b border-border/50 bg-muted/20 px-3.5 py-2">
				<div className="flex items-center gap-2.5 truncate">
					<FileCode className="size-4 shrink-0 text-primary" />
					<span
						className="truncate font-mono text-xs font-semibold text-foreground/90"
						title={fullPathDisplay}>
						{fullPathDisplay}
					</span>
					{isReadOnly && (
						<span className="rounded-md border border-amber-500/20 bg-amber-500/10 px-1.5 py-0.5 text-[10px] font-semibold text-amber-500">
							Read-Only
						</span>
					)}
				</div>

				<div className="flex shrink-0 items-center gap-2.5">
					<span className="flex items-center gap-1 font-mono text-[10px] text-muted-foreground/80">
						<Sparkles className="size-3 text-amber-400" /> YAML
					</span>
					{!isReadOnly && (
						<Button
							variant="outline"
							size="sm"
							onClick={() => setIsLocked(!isLocked)}
							className="h-7 cursor-pointer gap-1.5 border-border/60 px-2.5 text-xs font-medium shadow-2xs">
							{isLocked ? (
								<>
									<Lock className="size-3 text-amber-500" /> Unlock
								</>
							) : (
								<>
									<LockOpen className="size-3 text-emerald-500" /> Lock
									Editing
								</>
							)}
						</Button>
					)}
				</div>
			</div>

			{/* Full Height Monaco Code Editor */}
			<div className="relative min-h-[350px] flex-1 overflow-hidden bg-[#1e1e1e]">
				{isLoading && (
					<div className="absolute inset-0 z-10 flex items-center justify-center gap-2 bg-[#1e1e1e]/80 font-mono text-xs text-slate-300">
						Loading file content...
					</div>
				)}

				<Editor
					height="100%"
					language="yaml"
					theme="vs-dark"
					value={content}
					onChange={val => onChangeContent(val || '')}
					options={{
						readOnly: effectiveReadOnly,
						minimap: {enabled: true},
						scrollBeyondLastLine: false,
						fontSize: 13,
						fontFamily:
							"JetBrains Mono, Menlo, Monaco, 'Courier New', monospace",
						lineNumbers: 'on',
						renderLineHighlight: 'none',
						selectionHighlight: false,
						automaticLayout: true,
						tabSize: 2,
						padding: {top: 12, bottom: 12},
					}}
				/>
			</div>
		</div>
	);
}
