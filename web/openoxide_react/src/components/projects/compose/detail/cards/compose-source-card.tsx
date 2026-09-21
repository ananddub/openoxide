import {Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	COMPOSE_SOURCE_PROVIDERS,
	ProviderTabs,
} from '#/components/projects/detail/provider-tabs';
import type {ComposeSourceState} from '../use-compose-source';
import {
	ComposeGitForm,
	ComposeRepoForm,
} from './compose-source-card/source-forms';
import {
	ComposeYamlEditor,
	YamlPreviewDetails,
} from './compose-source-card/yaml-editor';

interface ComposeSourceCardProps {
	source: ComposeSourceState;
	onPatch: (patch: Partial<ComposeSourceState>) => void;
	savingSource: boolean;
	onSave: () => void;
}

export function ComposeSourceCard({
	source,
	onPatch,
	savingSource,
	onSave,
}: ComposeSourceCardProps) {
	const {provider} = source;
	return (
		<section className="flex flex-col gap-5 rounded-xl bg-card p-5 shadow-sm">
			<div className="flex items-center justify-between border-b  pb-4">
				<h3 className="text-sm font-bold text-foreground">
					Compose Source & Provider Configuration
				</h3>
				<Button
					size="sm"
					onClick={onSave}
					disabled={savingSource}
					className="flex h-8 items-center gap-1.5 text-xs font-semibold">
					<Save className="h-3.5 w-3.5" /> Save Source Settings
				</Button>
			</div>

			<ProviderTabs
				value={provider}
				onChange={id => onPatch({provider: id})}
				providers={COMPOSE_SOURCE_PROVIDERS}
			/>

			{provider === 'RAW' ? (
				<div className="flex flex-col gap-3">
					<ComposeYamlEditor
						value={source.composeFile}
						onChange={v => onPatch({composeFile: v})}
						height="360px"
					/>
					<div className="flex flex-col gap-1">
						<label className="text-xs font-semibold text-foreground">
							Compose Command Override (Optional)
						</label>
						<Input
							value={source.command}
							onChange={e => onPatch({command: e.target.value})}
							placeholder="docker compose up -d --build"
							className="h-9 font-mono text-xs"
						/>
					</div>
				</div>
			) : provider === 'GIT' ? (
				<ComposeGitForm source={source} onPatch={onPatch} />
			) : (
				<div className="flex flex-col gap-2">
					<ComposeRepoForm source={source} onPatch={onPatch} />
					<YamlPreviewDetails
						value={source.composeFile}
						onChange={v => onPatch({composeFile: v})}
					/>
				</div>
			)}
		</section>
	);
}

export type {ComposeSourceState};
