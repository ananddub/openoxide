import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Field, FieldGrid} from '#/components/shared/field';
import {useSshKeyItems} from '#/api/live';
import type {ComposeSourceState} from '../../use-compose-source';

const MONO = 'font-mono';

export function ComposeGitForm({
	source,
	onPatch,
}: {
	source: ComposeSourceState;
	onPatch: (patch: Partial<ComposeSourceState>) => void;
}) {
	const sshKeys = useSshKeyItems().data ?? [];
	const {gitUrl, gitBranch, gitBuildPath, sshKeyId} = source;
	const selectedKey = (sshKeys as any[])?.find(
		(k: any) => Number(k.id) === Number(sshKeyId),
	);
	const selectedName = sshKeyId
		? selectedKey?.name || `SSH Key #${sshKeyId}`
		: 'None (Public repo)';

	return (
		<FieldGrid>
			<Field
				label="Repository URL"
				hint="HTTPS or SSH clone URL for the repository."
				className="sm:col-span-2">
				<Input
					value={gitUrl}
					onChange={e => onPatch({gitUrl: e.target.value})}
					placeholder="git@github.com:acme/webstack.git"
					className={MONO}
				/>
			</Field>
			<Field label="Branch">
				<Input
					value={gitBranch}
					onChange={e => onPatch({gitBranch: e.target.value})}
					placeholder="main"
					className={MONO}
				/>
			</Field>
			<Field label="Compose file path" hint="Path inside the repository.">
				<Input
					value={gitBuildPath}
					onChange={e => onPatch({gitBuildPath: e.target.value})}
					placeholder="docker-compose.yml"
					className={MONO}
				/>
			</Field>
			<Field
				label="SSH key"
				hint="Only needed for private repositories."
				className="sm:col-span-2">
				<Select
					value={sshKeyId ? String(sshKeyId) : 'none'}
					onValueChange={v =>
						onPatch({sshKeyId: v === 'none' ? undefined : Number(v)})
					}>
					<SelectTrigger className="h-9 w-full bg-card font-sans text-sm">
						<SelectValue placeholder="No SSH key selected">
							{selectedName}
						</SelectValue>
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="none" className="font-sans text-sm">
							None (Public repo)
						</SelectItem>
						{sshKeys?.map((key: any) => (
							<SelectItem
								key={key.id}
								value={String(key.id)}
								className="font-sans text-sm">
								{key.name}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Field>
		</FieldGrid>
	);
}

export function ComposeRepoForm({
	source,
	onPatch,
}: {
	source: ComposeSourceState;
	onPatch: (patch: Partial<ComposeSourceState>) => void;
}) {
	const {repoOwner, repoName, gitBranch, gitBuildPath} = source;
	return (
		<FieldGrid>
			<Field label="Owner / username">
				<Input
					value={repoOwner}
					onChange={e => onPatch({repoOwner: e.target.value})}
					placeholder="ananddub"
					className={MONO}
				/>
			</Field>
			<Field label="Repository name">
				<Input
					value={repoName}
					onChange={e => onPatch({repoName: e.target.value})}
					placeholder="sample-node-project"
					className={MONO}
				/>
			</Field>
			<Field label="Branch">
				<Input
					value={gitBranch}
					onChange={e => onPatch({gitBranch: e.target.value})}
					placeholder="main"
					className={MONO}
				/>
			</Field>
			<Field label="Compose file path">
				<Input
					value={gitBuildPath}
					onChange={e => onPatch({gitBuildPath: e.target.value})}
					placeholder="docker-compose.yml"
					className={MONO}
				/>
			</Field>
		</FieldGrid>
	);
}
