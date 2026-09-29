import {Check} from 'lucide-react';
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
	/** Header-icon trigger: the caller owns visibility, no strip is painted. */
	open?: boolean;
	onOpenChange?: (open: boolean) => void;
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
	open,
	onOpenChange,
}: ComposeSourceCardProps) {
	const {provider} = source;
	const group = groupFor(provider);

	return (
		<SettingsSheet
			title="Source & Provider Configuration"
			description="Where this compose stack pulls its definition from."
			open={open}
			onOpenChange={onOpenChange}
			/* Editor-heavy form — roomier than the default 560px rail. */
			contentClassName="sm:data-[side=right]:w-[820px]"
			footer={
				<Button
					onClick={onSave}
					disabled={savingSource}
					className="h-8 gap-1.5 px-4 text-xs">
					{savingSource ? 'Saving…' : 'Save changes'}
				</Button>
			}>
			<SheetGroup title="Source">
				{/* Compact provider picker — six heavy radio rows read as
				    clutter; a 3-up chip grid stays scannable. */}
				<div className="grid grid-cols-2 gap-2 sm:grid-cols-3">
					{COMPOSE_SOURCE_PROVIDERS.map(p => {
						const selected = provider === p.id;
						return (
							<button
								key={p.id}
								type="button"
								onClick={() => onPatch({provider: p.id})}
								aria-pressed={selected}
								className={cn(
									'flex cursor-pointer items-center gap-2.5 rounded-lg border px-3 py-2.5 text-sm font-medium transition-colors',
									selected
										? 'border-primary/50 bg-primary/[0.08] text-foreground'
										: 'border-border/60 text-muted-foreground hover:bg-muted/30 hover:text-foreground',
								)}>
								<span className="shrink-0">{p.icon}</span>
								<span className="truncate">{p.label}</span>
								{selected && (
									<Check className="ml-auto size-3.5 shrink-0 text-primary" />
								)}
							</button>
						);
					})}
				</div>
			</SheetGroup>

			<SheetGroup
				title={group.title}
				description={group.description}
				className={provider === 'RAW' ? 'min-h-0 flex-1' : undefined}>
				{provider === 'RAW' ? (
					<div className="flex min-h-0 flex-1 flex-col gap-5">
						<ComposeYamlEditor
							value={source.composeFile}
							onChange={v => onPatch({composeFile: v})}
							/* Takes the sheet's remaining height — editing a stack needs no scrolling. */
							height="100%"
							className="min-h-[280px] flex-1"
							fileName="docker-compose.yml"
						/>
						<Field
							label="Command override"
							hint="Optional — replaces the default deploy command."
							className="shrink-0">
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
