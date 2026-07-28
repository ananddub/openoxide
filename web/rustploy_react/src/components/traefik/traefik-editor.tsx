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
			<div className="flex-1 p-12 flex flex-col items-center justify-center text-center gap-3 h-full min-h-[400px]">
				<FileCode className="size-8 text-muted-foreground/60 mb-1" />
				<div className="space-y-1">
					<h3 className="text-sm font-bold text-foreground">Select a file to edit</h3>
					<p className="text-xs text-muted-foreground max-w-xs leading-relaxed">
						Choose a dynamic YAML configuration file from the sidebar to inspect or edit.
					</p>
				</div>
			</div>
		);
	}

	const fullPathDisplay = `/etc/rustploy/traefik/${selectedFilePath}`;

	return (
		<div className="flex-1 flex flex-col overflow-hidden h-full min-h-[450px] bg-[#1e1e1e]">
			{/* Clean Toolbar Header */}
			<div className="flex items-center justify-between px-3.5 py-2 border-b border-border/50 bg-muted/20 shrink-0">
				<div className="flex items-center gap-2.5 truncate">
					<FileCode className="size-4 text-primary shrink-0" />
					<span className="text-xs font-mono font-semibold text-foreground/90 truncate" title={fullPathDisplay}>
						{fullPathDisplay}
					</span>
					{isReadOnly && (
						<span className="text-[10px] font-semibold bg-amber-500/10 text-amber-500 border border-amber-500/20 px-1.5 py-0.5 rounded-md">
							Read-Only
						</span>
					)}
				</div>

				<div className="flex items-center gap-2.5 shrink-0">
					<span className="text-[10px] font-mono text-muted-foreground/80 flex items-center gap-1">
						<Sparkles className="size-3 text-amber-400" /> YAML
					</span>
					{!isReadOnly && (
						<Button
							variant="outline"
							size="sm"
							onClick={() => setIsLocked(!isLocked)}
							className="h-7 px-2.5 text-xs font-medium gap-1.5 border-border/60 cursor-pointer shadow-2xs">
							{isLocked ? (
								<>
									<Lock className="size-3 text-amber-500" /> Unlock
								</>
							) : (
								<>
									<LockOpen className="size-3 text-emerald-500" /> Lock Editing
								</>
							)}
						</Button>
					)}
				</div>
			</div>

			{/* Full Height Monaco Code Editor */}
			<div className="relative flex-1 bg-[#1e1e1e] overflow-hidden min-h-[350px]">
				{isLoading && (
					<div className="absolute inset-0 z-10 bg-[#1e1e1e]/80 flex items-center justify-center text-xs text-slate-300 font-mono gap-2">
						Loading file content...
					</div>
				)}

				<Editor
					height="100%"
					language="yaml"
					theme="vs-dark"
					value={content}
					onChange={(val) => onChangeContent(val || '')}
					options={{
						readOnly: effectiveReadOnly,
						minimap: {enabled: true},
						scrollBeyondLastLine: false,
						fontSize: 13,
						fontFamily: "JetBrains Mono, Menlo, Monaco, 'Courier New', monospace",
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
