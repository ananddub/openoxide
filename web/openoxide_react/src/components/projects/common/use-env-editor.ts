import {useState, useEffect, useRef} from 'react';
import {loader} from '@monaco-editor/react';
import {toast} from 'sonner';

/** Masks env var values (`KEY=•••`) when secrets are hidden. */
export const maskEnvText = (raw: string): string =>
	raw
		.split('\n')
		.map(line => {
			const trimmed = line.trim();
			if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith(';'))
				return line;
			const eqIdx = line.indexOf('=');
			if (eqIdx === -1) return line;
			const val = line.slice(eqIdx + 1);
			if (!val) return line;
			return `${line.slice(0, eqIdx + 1)}${'•'.repeat(val.length)}`;
		})
		.join('\n');

/**
 * Owns the Monaco-backed environment editor: masking, secret toggle, live
 * sync with the server value, and save handling.
 */
export function useEnvEditor(
	app: any,
	handleUpdate: (body: any) => Promise<void>,
) {
	const originalEnv = app.env_var || '';
	const [envVars, setEnvVars] = useState(originalEnv);
	const [showSecrets, setShowSecrets] = useState(true);
	const [saving, setSaving] = useState(false);
	const [saved, setSaved] = useState(false);
	const containerRef = useRef<HTMLDivElement>(null);
	const editorRef = useRef<any>(null);

	// Ref trackers to avoid stale closures inside Monaco callbacks.
	const showSecretsRef = useRef(showSecrets);
	showSecretsRef.current = showSecrets;
	const envVarsRef = useRef(envVars);
	envVarsRef.current = envVars;

	const syncEditor = (value: string) => {
		if (!editorRef.current) return;
		const target = showSecretsRef.current ? value : maskEnvText(value);
		if (editorRef.current.getValue() !== target)
			editorRef.current.setValue(target);
	};

	useEffect(() => {
		const raw = app.env_var || '';
		setEnvVars(raw);
		syncEditor(raw);
	}, [app.env_var]);

	useEffect(() => {
		if (!editorRef.current) return;
		syncEditor(envVarsRef.current);
		editorRef.current.updateOptions({readOnly: !showSecrets});
	}, [showSecrets]);

	useEffect(() => {
		let isMounted = true;
		let editorInstance: any;
		loader
			.init()
			.then(monaco => {
				if (!isMounted || !containerRef.current) return;
				registerIniLanguage(monaco);
				editorInstance = monaco.editor.create(containerRef.current, {
					value: showSecretsRef.current
						? envVarsRef.current
						: maskEnvText(envVarsRef.current),
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
					if (isMounted && showSecretsRef.current)
						setEnvVars(editorInstance.getValue());
				});
			})
			.catch(err => console.error('Failed to load Monaco editor:', err));
		return () => {
			isMounted = false;
			if (editorInstance) editorInstance.dispose();
		};
	}, []);

	const handleSave = async () => {
		setSaving(true);
		try {
			await handleUpdate({env_var: envVars});
			toast.success('Environment variables saved successfully');
			setSaved(true);
			setTimeout(() => setSaved(false), 2000);
		} catch {
			toast.error('Failed to save environment variables');
		} finally {
			setSaving(false);
		}
	};

	return {
		envVars,
		showSecrets,
		setShowSecrets,
		saved,
		saving,
		isModified: envVars !== originalEnv,
		containerRef,
		handleSave,
	};
}

function registerIniLanguage(monaco: any) {
	if (monaco.languages.getLanguages().some((l: any) => l.id === 'ini'))
		return;
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
