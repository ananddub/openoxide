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

	// Ref trackers to eliminate React closure stale state bugs inside Monaco callbacks
	const showSecretsRef = useRef(showSecrets);
	showSecretsRef.current = showSecrets;

	const envVarsRef = useRef(envVars);
	envVarsRef.current = envVars;

	// Mask environment variable values when secrets are hidden
	const maskEnvText = (raw: string): string => {
		return raw
			.split('\n')
			.map(line => {
				const trimmed = line.trim();
				if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith(';'))
					return line;
				const eqIdx = line.indexOf('=');
				if (eqIdx === -1) return line;
				const key = line.slice(0, eqIdx + 1);
				const val = line.slice(eqIdx + 1);
				if (!val) return line;
				return `${key}${'•'.repeat(val.length)}`;
			})
			.join('\n');
	};

	// Sync environment variables when app updates
	useEffect(() => {
		const raw = app.env_var || '';
		setEnvVars(raw);
		if (editorRef.current) {
			const targetVal = showSecretsRef.current ? raw : maskEnvText(raw);
			if (editorRef.current.getValue() !== targetVal) {
				editorRef.current.setValue(targetVal);
			}
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

	// Update Monaco Editor content & readOnly state dynamically when showSecrets toggles
	useEffect(() => {
		if (!editorRef.current) return;
		const targetVal = showSecrets
			? envVarsRef.current
			: maskEnvText(envVarsRef.current);
		if (editorRef.current.getValue() !== targetVal) {
			editorRef.current.setValue(targetVal);
		}
		editorRef.current.updateOptions({readOnly: !showSecrets});
	}, [showSecrets]);

	// Initialize Monaco Editor dynamically on mount
	useEffect(() => {
		let isMounted = true;
		let editorInstance: any;

		loader
			.init()
			.then(monaco => {
				if (!isMounted || !containerRef.current) return;

				if (
					!monaco.languages.getLanguages().some((l: any) => l.id === 'ini')
				) {
					monaco.languages.register({id: 'ini'});
					monaco.languages.setMonarchTokensProvider('ini', {
						tokenizer: {
							root: [
								[/#.*$/, 'comment'],
								[/;.*$/, 'comment'],
								[/^\s*\[.*\]/, 'tag'],
								[/[a-zA-Z_][\w]*(?==)/, 'key'],
								[/=/, 'delimiter'],
								[/"([^"\\]|\\.)*"/, 'string'],
								[/'([^'\\]|\\.)*'/, 'string'],
							],
						},
					} as any);
				}

				const initialContent = showSecretsRef.current
					? envVarsRef.current
					: maskEnvText(envVarsRef.current);

				editorInstance = monaco.editor.create(containerRef.current, {
					value: initialContent,
					readOnly: !showSecretsRef.current,
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
					if (isMounted && showSecretsRef.current) {
						const val = editorInstance.getValue();
						setEnvVars(val);
					}
				});
			})
			.catch(err => {
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
		<div className="flex animate-in flex-col gap-6 duration-200 fade-in">
			{/* Info Warning Banner */}
			<div className="flex gap-3 rounded-xl border border-primary/20 bg-primary/5 p-4">
				<Info className="mt-0.5 h-4.5 w-4.5 shrink-0 text-primary" />
				<div className="flex flex-col gap-1">
					<span className="text-xs font-bold text-foreground">
						Rebuild Required
					</span>
					<p className="text-[11px] leading-normal text-muted-foreground">
						Changes to environment variables are injected at
						runtime/build-time. You must{' '}
						<strong>redeploy or rebuild</strong> the application to apply
						the changes.
					</p>
				</div>
			</div>

			{/* Main Editor Card */}
			<div className="flex flex-col gap-4 rounded-xl border border-border bg-card p-5">
				<div className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
					<div className="flex flex-wrap items-center gap-3">
						<div>
							<h3 className="text-sm font-bold text-foreground">
								Environment Variables
							</h3>
							<p className="mt-1 text-[11px] text-muted-foreground">
								Define variables in{' '}
								<code className="rounded bg-muted/40 px-1 py-0.5 font-mono text-[10px]">
									KEY=VALUE
								</code>{' '}
								format.
							</p>
						</div>

						{isModified && (
							<span className="animate-pulse rounded-full border border-amber-500/20 bg-amber-500/10 px-2 py-0.5 text-[9px] font-extrabold text-amber-500 select-none">
								Unsaved Changes
							</span>
						)}
					</div>

					<div className="flex items-center gap-2">
						<Button
							variant="outline"
							size="sm"
							onClick={() => setShowSecrets(!showSecrets)}
							className="flex h-8 items-center gap-1.5 rounded-lg border-border text-xs font-semibold text-foreground hover:bg-muted">
							{showSecrets ? (
								<EyeOff className="h-3.5 w-3.5" />
							) : (
								<Eye className="h-3.5 w-3.5 text-primary" />
							)}
							{showSecrets ? 'Hide Values' : 'Show Values'}
						</Button>

						<div className="hidden items-center gap-1.5 rounded-lg border border-border/40 bg-muted/20 px-2 py-1 text-xs font-semibold text-muted-foreground md:flex">
							<ShieldAlert className="h-3.5 w-3.5 text-amber-500" />{' '}
							Secrets encrypted
						</div>
					</div>
				</div>

				{/* Notice when values are hidden */}
				{!showSecrets && (
					<div className="flex items-center justify-between rounded-lg border border-border/60 bg-muted/30 px-3 py-2 text-[11px] text-muted-foreground">
						<span>
							Secret values are masked for security. Click{' '}
							<strong>Show Values</strong> (eye icon) to edit or view exact
							values.
						</span>
					</div>
				)}

				{/* Monaco Editor Container */}
				<div
					ref={containerRef}
					className={`relative overflow-hidden rounded-xl border border-border bg-zinc-950 shadow-inner transition-all duration-300 ${!showSecrets ? 'opacity-60 grayscale-[30%]' : 'opacity-100'}`}
					style={{height: '420px', width: '100%'}}
				/>

				<div className="mt-2 flex items-center justify-end gap-3">
					{saved && (
						<span className="flex animate-in items-center gap-1 text-xs font-bold text-emerald-500 duration-200 fade-in slide-in-from-right-2">
							<Check className="h-3.5 w-3.5" /> Saved!
						</span>
					)}

					<Button
						onClick={handleSave}
						disabled={saving || !showSecrets}
						title={
							!showSecrets
								? 'Click Show Values (eye icon) to edit and save'
								: ''
						}
						className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
						<Save className="h-3.5 w-3.5" />{' '}
						{saving ? 'Saving...' : 'Save Environment'}
					</Button>
				</div>
			</div>
		</div>
	);
}
