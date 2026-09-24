import {useEffect, useState} from 'react';
import {createPortal} from 'react-dom';
import Editor, {type EditorProps} from '@monaco-editor/react';
import {FileCode, Maximize2, Minimize2} from 'lucide-react';
import {useTheme} from '#/hooks/ui/use-theme';
import {cn} from '#/api/utils';

/** Read-only Monaco preview of a template file — same maximize overlay as the compose YAML editor. */
export function TemplateFilePreview({
	filename,
	language,
	data,
	isLoading,
	fill,
}: {
	filename: string;
	language: string;
	data?: string;
	isLoading?: boolean;
	/** Grow to the parent's remaining height instead of a fixed 288px well. */
	fill?: boolean;
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

	const options: EditorProps['options'] = {
		readOnly: true,
		domReadOnly: true,
		minimap: {enabled: false},
		scrollBeyondLastLine: false,
		folding: true,
		renderLineHighlight: 'none',
		overviewRulerLanes: 0,
		automaticLayout: true,
		fontSize: 12,
		lineDecorationsWidth: 6,
		lineNumbersMinChars: 3,
	};

	const chrome = (
		<div className="flex shrink-0 items-center gap-2 border-b border-border bg-muted/40 px-3 py-1.5">
			<FileCode className="size-3.5 text-muted-foreground" />
			<span className="min-w-0 flex-1 truncate font-mono text-[11px] text-muted-foreground">
				{filename}
			</span>
			<button
				type="button"
				onClick={() => setMaximized(m => !m)}
				title={maximized ? 'Restore (Esc)' : 'Maximize'}
				aria-label={maximized ? 'Restore preview' : 'Maximize preview'}
				className="flex size-6 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground">
				{maximized ? (
					<Minimize2 className="size-3.5" />
				) : (
					<Maximize2 className="size-3.5" />
				)}
			</button>
		</div>
	);

	const body = (expand: boolean) =>
		isLoading && !data ? (
			<div
				className={
					expand ? 'flex h-full items-center justify-center' : 'h-72'
				}>
				<span className="font-mono text-xs text-muted-foreground">
					Loading…
				</span>
			</div>
		) : (
			<Editor
				height={expand ? '100%' : '288px'}
				language={language}
				theme={theme === 'dark' ? 'vs-dark' : 'light'}
				value={data ?? ''}
				options={options}
			/>
		);

	if (maximized) {
		return createPortal(
			<div className="fixed inset-0 z-[70] flex animate-in flex-col gap-3 bg-black/40 p-4 backdrop-blur-[2px] duration-150 fade-in-0 sm:p-10">
				<div className="flex min-h-0 flex-1 flex-col overflow-hidden rounded-xl border border-border bg-card shadow-2xl">
					{chrome}
					<div className="min-h-0 flex-1">{body(true)}</div>
				</div>
			</div>,
			document.body,
		);
	}

	return (
		<div
			className={cn(
				'flex flex-col overflow-hidden rounded-lg border border-border bg-card',
				fill && 'min-h-0 grow',
			)}>
			{chrome}
			<div className={cn(!fill && 'shrink-0', fill && 'min-h-0 flex-1')}>
				{body(!!fill)}
			</div>
		</div>
	);
}
