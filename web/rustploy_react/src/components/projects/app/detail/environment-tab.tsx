import {useState, useEffect, useRef} from 'react';
import {loader} from '@monaco-editor/react';
import {Button} from '#/components/ui/button';
import {Save, Info, ShieldAlert, Check, Eye, EyeOff} from 'lucide-react';
import {toast} from 'sonner';

interface EnvironmentTabProps {
	app: any;
	handleUpdate: (body: any) => Promise<void>;
}

export function EnvironmentTab({app, handleUpdate}: EnvironmentTabProps) {
	const originalEnv = app.env_var || '';
	const [envVars, setEnvVars] = useState(originalEnv);
	const [showSecrets, setShowSecrets] = useState(false);
	const [saving, setSaving] = useState(false);
	const [saved, setSaved] = useState(false);
	const containerRef = useRef<HTMLDivElement>(null);
	const editorRef = useRef<any>(null);

	// Sync environment variables when app updates
	useEffect(() => {
		setEnvVars(app.env_var || '');
	}, [app.env_var]);

	const isModified = envVars !== originalEnv;

	// Mask environment variable values when secrets are hidden
	const maskEnvText = (raw: string): string => {
		return raw
			.split('\n')
			.map(line => {
				const trimmed = line.trim();
				if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith(';')) return line;
				const eqIdx = line.indexOf('=');
				if (eqIdx === -1) return line;
				const key = line.slice(0, eqIdx + 1);
				const val = line.slice(eqIdx + 1);
				if (!val) return line;
				return `${key}${'•'.repeat(val.length)}`;
			})
			.join('\n');
	};

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

	// Update Monaco Editor content & readOnly state dynamically when showSecrets or envVars change
	useEffect(() => {
		if (!editorRef.current) return;
		const targetVal = showSecrets ? envVars : maskEnvText(envVars);
		if (editorRef.current.getValue() !== targetVal) {
			editorRef.current.setValue(targetVal);
		}
		editorRef.current.updateOptions({readOnly: !showSecrets});
	}, [showSecrets, envVars]);

	// Initialize Monaco Editor dynamically on mount
	useEffect(() => {
		let isMounted = true;
		let editorInstance: any;

		loader.init().then(monaco => {
			if (!isMounted || !containerRef.current) return;

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
				value: showSecrets ? envVars : maskEnvText(envVars),
				readOnly: !showSecrets,
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
				if (isMounted && showSecrets) {
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

					<div className="flex items-center gap-2">
						<Button
							variant="outline"
							size="sm"
							onClick={() => setShowSecrets(!showSecrets)}
							className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-8 text-xs rounded-lg"
						>
							{showSecrets ? <EyeOff className="w-3.5 h-3.5" /> : <Eye className="w-3.5 h-3.5 text-primary" />}
							{showSecrets ? 'Hide Values' : 'Show Values'}
						</Button>

						<div className="hidden md:flex items-center gap-1.5 text-xs text-muted-foreground font-semibold bg-muted/20 border border-border/40 px-2 py-1 rounded-lg">
							<ShieldAlert className="w-3.5 h-3.5 text-amber-500" /> Secrets encrypted
						</div>
					</div>
				</div>

				{/* Notice when values are hidden */}
				{!showSecrets && (
					<div className="bg-muted/30 border border-border/60 rounded-lg px-3 py-2 text-[11px] text-muted-foreground flex items-center justify-between">
						<span>Secret values are masked for security. Click <strong>Show Values</strong> (eye icon) to edit or view exact values.</span>
					</div>
				)}

				{/* Monaco Editor Container */}
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

					<Button
						onClick={handleSave}
						disabled={saving || !showSecrets}
						title={!showSecrets ? 'Click Show Values (eye icon) to edit and save' : ''}
						className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg text-xs"
					>
						<Save className="w-3.5 h-3.5" /> {saving ? 'Saving...' : 'Save Environment'}
					</Button>
				</div>
			</div>
		</div>
	);
}
