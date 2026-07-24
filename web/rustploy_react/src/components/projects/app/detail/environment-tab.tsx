import {useState, useEffect, useRef} from 'react';
import {loader} from '@monaco-editor/react';
import {Button} from '#/components/ui/button';
import {Save, Info, ShieldAlert, Check} from 'lucide-react';
import {toast} from 'sonner';

interface EnvironmentTabProps {
	app: any;
	handleUpdate: (body: any) => Promise<void>;
}

export function EnvironmentTab({app, handleUpdate}: EnvironmentTabProps) {
	const originalEnv = app.env_var || '';
	const [envVars, setEnvVars] = useState(originalEnv);
	const [saving, setSaving] = useState(false);
	const [saved, setSaved] = useState(false);
	const containerRef = useRef<HTMLDivElement>(null);
	const editorRef = useRef<any>(null);

	// Sync environment variables when app updates
	useEffect(() => {
		setEnvVars(app.env_var || '');
		if (editorRef.current && typeof editorRef.current.getValue === 'function' && editorRef.current.getValue() !== (app.env_var || '')) {
			editorRef.current.setValue(app.env_var || '');
		}
	}, [app.env_var]);

	const isModified = envVars !== originalEnv;

	const handleSave = async () => {
		setSaving(true);
		try {
			await handleUpdate({env_var: envVars});
			toast.success('Environment variables saved successfully');
			setSaved(true);
			setTimeout(() => setSaved(false), 2000);
		} catch (err: any) {
			toast.error('Failed to save environment variables');
		} finally {
			setSaving(false);
		}
	};

	// Initialize Monaco Editor dynamically via vanilla JS on mount to bypass React 19 timing bugs
	useEffect(() => {
		let isMounted = true;
		let editorInstance: any;

		loader.init().then(monaco => {
			if (!isMounted || !containerRef.current) return;

			// Register standard monarch language configuration for INI/env files
			if (!monaco.languages.getLanguages().some((l: any) => l.id === 'ini')) {
				monaco.languages.register({ id: 'ini' });
				monaco.languages.setMonarchTokensProvider('ini', {
					tokenizer: {
						root: [
							[/#.*$/, 'comment'],
							[/;.*$/, 'comment'],
							[/^\s*\[.*\]/, 'tag'],
							[/[a-zA-Z_][\w]*(?==)/, 'key'],
							[/=/, 'delimiter'],
							[/"([^"\\]|\\.)*"/, 'string'],
							[/'([^'\\]|\\.)*'/, 'string']
						]
					}
				} as any);
			}

			editorInstance = monaco.editor.create(containerRef.current, {
				value: envVars,
				language: 'ini',
				theme: 'vs-dark',
				fontSize: 13,
				fontFamily: 'monospace',
				minimap: {enabled: false},
				scrollBeyondLastLine: false,
				lineNumbers: 'on',
				wordWrap: 'on',
				automaticLayout: true,
				padding: {top: 8, bottom: 8},
				renderLineHighlight: 'none',
				lineDecorationsWidth: 16,
				lineNumbersMinChars: 4,
				scrollbar: {
					vertical: 'visible',
					horizontal: 'hidden',
					verticalScrollbarSize: 8,
				},
			});

			editorRef.current = editorInstance;

			editorInstance.onDidChangeModelContent(() => {
				if (isMounted) {
					setEnvVars(editorInstance.getValue());
				}
			});
		}).catch(err => {
			console.error('Failed to load Monaco editor:', err);
		});

		return () => {
			isMounted = false;
			if (editorInstance) {
				editorInstance.dispose();
			}
		};
	}, []);

	return (
		<div className="flex flex-col gap-6 animate-in fade-in duration-200">
			{/* Info Warning Banner */}
			<div className="bg-primary/5 border border-primary/20 rounded-xl p-4 flex gap-3">
				<Info className="w-4.5 h-4.5 text-primary shrink-0 mt-0.5" />
				<div className="flex flex-col gap-1">
					<span className="text-xs font-bold text-foreground">Rebuild Required</span>
					<p className="text-[11px] text-muted-foreground leading-normal">
						Changes to environment variables are injected at runtime/build-time. You must <strong>redeploy or rebuild</strong> the application to apply the changes.
					</p>
				</div>
			</div>

			{/* Main Editor Card */}
			<div className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4">
				<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
					<div className="flex flex-wrap items-center gap-3">
						<div>
							<h3 className="text-sm font-bold text-foreground">Environment Variables</h3>
							<p className="text-[11px] text-muted-foreground mt-1">
								Define variables in <code className="font-mono bg-muted/40 px-1 py-0.5 rounded text-[10px]">KEY=VALUE</code> format.
							</p>
						</div>

						{isModified && (
							<span className="text-[9px] font-extrabold px-2 py-0.5 rounded-full bg-amber-500/10 text-amber-500 border border-amber-500/20 select-none animate-pulse">
								Unsaved Changes
							</span>
						)}
					</div>

					<div className="flex items-center gap-1.5 text-xs text-muted-foreground font-semibold bg-muted/20 border border-border/40 px-2 py-1 rounded-lg">
						<ShieldAlert className="w-3.5 h-3.5 text-amber-500" /> Secrets are encrypted at rest
					</div>
				</div>

				{/* Clear Editor Wrapper: height is strictly set to 420px inline to prevent any collapse bugs */}
				<div 
					ref={containerRef}
					className="rounded-xl border border-border bg-zinc-950 overflow-hidden relative shadow-inner" 
					style={{ height: '420px', width: '100%' }}
				/>

				<div className="flex justify-end items-center gap-3 mt-2">
					{saved && (
						<span className="text-xs font-bold text-emerald-500 flex items-center gap-1 animate-in fade-in slide-in-from-right-2 duration-200">
							<Check className="w-3.5 h-3.5" /> Saved!
						</span>
					)}

					<Button onClick={handleSave} disabled={saving} className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg text-xs">
						<Save className="w-3.5 h-3.5" /> {saving ? 'Saving...' : 'Save Environment'}
					</Button>
				</div>
			</div>
		</div>
	);
}
