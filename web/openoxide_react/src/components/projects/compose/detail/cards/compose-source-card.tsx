import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {cn} from '#/api/utils';
import {Field, SheetGroup} from '#/components/shared/field';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {COMPOSE_SOURCE_PROVIDERS} from '#/components/projects/detail/provider-tabs';
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
	/** Kept for call-site compatibility; the card always renders its own sheet. */
	embedded?: boolean;
}

/** Titled config group per provider family. */
function groupFor(provider: string): {title: string; description: string} {
	if (provider === 'RAW')
		return {
			title: 'Compose file',
			description: 'Paste the full definition used to build this stack.',
		};
	if (provider === 'GIT')
		return {
			title: 'Git repository',
			description: 'Deploy a compose file from any Git host by URL.',
		};
	return {
		title: 'Repository',
		description: 'Pick the repository, branch and compose file path.',
	};
}

/** One compact sheet: provider chips on top, and the selected provider's config right below them — no Continue/Back step dance, one Save. */
export function ComposeSourceCard({
	source,
	onPatch,
	savingSource,
	onSave,
}: ComposeSourceCardProps) {
	const {provider} = source;
	const group = groupFor(provider);

	return (
		<SettingsSheet
			title="Source & Provider Configuration"
			description="Where this compose stack pulls its definition from."
			/* Editor-heavy form — roomier than the default 560px rail. */
			contentClassName="sm:data-[side=right]:w-[820px]"
			footer={
				<Button
					onClick={onSave}
					disabled={savingSource}
					className="h-9 gap-1.5 px-4 text-sm">
					{savingSource ? 'Saving…' : 'Save Source Settings'}
				</Button>
			}>
			<SheetGroup
				title="Provider"
				description="Choose where this stack pulls its compose file from.">
				<div className="flex flex-col gap-2">
					{COMPOSE_SOURCE_PROVIDERS.map(p => {
						const selected = provider === p.id;
						return (
							<button
								key={p.id}
								type="button"
								onClick={() => onPatch({provider: p.id})}
								aria-pressed={selected}
								className={cn(
									'flex w-full cursor-pointer items-center gap-3 rounded-xl border px-4 py-3.5 text-start transition-all duration-150',
									selected
										? 'border-primary/40 bg-primary/10'
										: 'border-border/50 hover:border-border hover:bg-muted/30',
								)}>
								<span className="shrink-0">{p.icon}</span>
								<span
									className={cn(
										'truncate text-sm font-medium',
										selected ? 'text-foreground' : 'text-muted-foreground',
									)}>
									{p.label}
								</span>
								<span
									className={cn(
										'ml-auto flex size-5 shrink-0 items-center justify-center rounded-full border-2 transition-colors',
										selected
											? 'border-primary'
											: 'border-muted-foreground/40',
									)}>
									{selected && (
										<span className="size-2.5 rounded-full bg-primary" />
									)}
								</span>
							</button>
						);
					})}
				</div>
			</SheetGroup>

			<SheetGroup title={group.title} description={group.description}>
				{provider === 'RAW' ? (
					<div className="flex flex-col gap-5">
						<ComposeYamlEditor
							value={source.composeFile}
							onChange={v => onPatch({composeFile: v})}
							height="260px"
							fileName="docker-compose.yml"
						/>
						<Field
							label="Command override"
							hint="Optional — replaces the default deploy command.">
							<Input
								value={source.command}
								onChange={e => onPatch({command: e.target.value})}
								placeholder="docker compose up -d --build"
								className="font-mono"
							/>
						</Field>
					</div>
				) : provider === 'GIT' ? (
					<ComposeGitForm source={source} onPatch={onPatch} />
				) : (
					<div className="flex flex-col gap-5">
						<ComposeRepoForm source={source} onPatch={onPatch} />
						<YamlPreviewDetails
							value={source.composeFile}
							onChange={v => onPatch({composeFile: v})}
						/>
					</div>
				)}
			</SheetGroup>
		</SettingsSheet>
	);
}

export type {ComposeSourceState};
