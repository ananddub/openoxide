import {useEffect, useState} from 'react';
import {createPortal} from 'react-dom';
import Editor from '@monaco-editor/react';
import {ChevronDown, Code2, Maximize2, Minimize2} from 'lucide-react';
import {useTheme} from '#/hooks/ui/use-theme';

const EDITOR_OPTIONS = {
	minimap: {enabled: false},
	fontSize: 12,
	scrollBeyondLastLine: false,
	lineNumbers: 'on' as const,
};

/** Theme-following Monaco YAML editor in a token-colored well. The header bar's maximize button lifts the editor into a fullscreen overlay so long compose files are actually editable. */
export function ComposeYamlEditor({
	value,
	onChange,
	height,
	fileName = 'docker-compose.yml',
}: {
	value: string;
	onChange: (v: string) => void;
	height: string;
	fileName?: string;
}) {
	const {theme} = useTheme();
	const [maximized, setMaximized] = useState(false);

	useEffect(() => {
		if (!maximized) return;
		const onKey = (e: KeyboardEvent) => {
			if (e.key === 'Escape') setMaximized(false);
		};
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	}, [maximized]);

	const chrome = (
		<div className="flex shrink-0 items-center gap-2 border-b border-border bg-muted/40 px-3 py-1.5">
			<Code2 className="size-3.5 text-muted-foreground" />
			<span className="min-w-0 flex-1 truncate font-mono text-[11px] text-muted-foreground">
				{fileName}
			</span>
			<button
				type="button"
				onClick={() => setMaximized(m => !m)}
				title={maximized ? 'Restore (Esc)' : 'Maximize'}
				aria-label={maximized ? 'Restore editor' : 'Maximize editor'}
				className="flex size-6 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground">
				{maximized ? (
					<Minimize2 className="size-3.5" />
				) : (
					<Maximize2 className="size-3.5" />
				)}
			</button>
		</div>
	);

	const editor = (fill: boolean) => (
		<Editor
			height={fill ? '100%' : height}
			defaultLanguage="yaml"
			theme={theme === 'dark' ? 'vs-dark' : 'light'}
			value={value}
			onChange={val => onChange(val || '')}
			options={EDITOR_OPTIONS}
		/>
	);

	if (maximized) {
		return createPortal(
			<div className="fixed inset-0 z-[70] flex animate-in flex-col gap-3 bg-black/40 p-4 backdrop-blur-[2px] duration-150 fade-in-0 sm:p-10">
				<div className="flex min-h-0 flex-1 flex-col overflow-hidden rounded-xl border border-border bg-card shadow-2xl">
					{chrome}
					<div className="min-h-0 flex-1">{editor(true)}</div>
				</div>
			</div>,
			document.body,
		);
	}

	return (
		<div className="overflow-hidden rounded-lg border border-border bg-card">
			{chrome}
			{editor(false)}
		</div>
	);
}

/** Collapsible paste-to-preview editor used by the repo forms. */
export function YamlPreviewDetails({
	value,
	onChange,
}: {
	value: string;
	onChange: (v: string) => void;
}) {
	return (
		<details className="group border-t border-border pt-4">
			<summary className="flex cursor-pointer items-center gap-2 py-1 text-xs font-semibold text-muted-foreground select-none hover:text-foreground">
				<ChevronDown className="size-3.5 -rotate-90 transition-transform duration-200 group-open:rotate-0" />
				<Code2 className="size-3.5 text-primary" />
				<span>Paste compose YAML to preview the dependency graph</span>
			</summary>
			<div className="flex flex-col gap-2 pt-3">
				<ComposeYamlEditor
					value={value}
					onChange={onChange}
					height="220px"
					fileName="docker-compose.yml"
				/>
				<p className="text-[11px] text-muted-foreground">
					No build needed — pasting the file here instantly renders the
					service graph.
				</p>
			</div>
		</details>
	);
}
