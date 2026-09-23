import Editor from '@monaco-editor/react';
import {Code2} from 'lucide-react';
import {useTheme} from '#/hooks/use-theme';

const EDITOR_OPTIONS = {
	minimap: {enabled: false},
	fontSize: 12,
	scrollBeyondLastLine: false,
	lineNumbers: 'on' as const,
};

/** Theme-following Monaco YAML editor in a rounded well. */
export function ComposeYamlEditor({
	value,
	onChange,
	height,
}: {
	value: string;
	onChange: (v: string) => void;
	height: string;
}) {
	const {theme} = useTheme();
	return (
		<div className="overflow-hidden rounded-xl border border-border bg-white shadow-inner dark:bg-zinc-950">
			<Editor
				height={height}
				defaultLanguage="yaml"
				theme={theme === 'dark' ? 'vs-dark' : 'light'}
				value={value}
				onChange={val => onChange(val || '')}
				options={EDITOR_OPTIONS}
			/>
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
		<div className="flex flex-col gap-2 border-t  pt-2 md:col-span-4">
			<details className="group">
				<summary className="flex cursor-pointer items-center gap-1.5 py-1 text-xs font-semibold text-muted-foreground select-none hover:text-foreground">
					<Code2 className="size-3.5 text-primary" />
					<span>
						Compose YAML Content (Paste code here to preview Dependency
						Graph without building)
					</span>
				</summary>
				<div className="flex flex-col gap-2 pt-3">
					<ComposeYamlEditor
						value={value}
						onChange={onChange}
						height="240px"
					/>
					<p className="text-[11px] text-muted-foreground italic">
						Pasting your docker-compose.yml content here instantly renders
						the service graph below.
					</p>
				</div>
			</details>
		</div>
	);
}
