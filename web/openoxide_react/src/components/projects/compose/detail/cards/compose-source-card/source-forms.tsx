import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {useSshKeyItems} from '#/api/live';
import type {ComposeSourceState} from '../../use-compose-source';

function Field({
	label,
	children,
	className = '',
}: {
	label: string;
	children: React.ReactNode;
	className?: string;
}) {
	return (
		<div className={`flex flex-col gap-1.5 ${className}`}>
			<label className="text-xs font-semibold text-foreground">
				{label}
			</label>
			{children}
		</div>
	);
}

const INPUT = 'h-9 font-mono text-xs';

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
		: 'None (Public Repo)';

	return (
		<div className="grid grid-cols-1 items-end gap-4 md:grid-cols-3">
			<Field label="Repository URL *" className="md:col-span-2">
				<Input
					value={gitUrl}
					onChange={e => onPatch({gitUrl: e.target.value})}
					placeholder="https://github.com/user/repo.git or git@github.com:user/repo.git"
					className={INPUT}
				/>
			</Field>
			<Field label="SSH Key (Optional for private repos)">
				<Select
					value={sshKeyId ? String(sshKeyId) : 'none'}
					onValueChange={v =>
						onPatch({sshKeyId: v === 'none' ? undefined : Number(v)})
					}>
					<SelectTrigger className="border- h-9 w-full bg-card font-sans text-xs">
						<SelectValue placeholder="No SSH Key Selected">
							{selectedName}
						</SelectValue>
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="none" className="font-sans text-xs">
							None (Public Repo)
						</SelectItem>
						{sshKeys?.map((key: any) => (
							<SelectItem
								key={key.id}
								value={String(key.id)}
								className="font-sans text-xs">
								{key.name}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Field>
			<Field label="Git Branch *">
				<Input
					value={gitBranch}
					onChange={e => onPatch({gitBranch: e.target.value})}
					placeholder="main"
					className={INPUT}
				/>
			</Field>
			<Field label="Compose File Path *" className="md:col-span-2">
				<Input
					value={gitBuildPath}
					onChange={e => onPatch({gitBuildPath: e.target.value})}
					placeholder="docker-compose.yml"
					className={INPUT}
				/>
			</Field>
		</div>
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
		<div className="grid grid-cols-1 items-end gap-4 md:grid-cols-4">
			<Field label="Owner / Username *" className="md:col-span-2">
				<Input
					value={repoOwner}
					onChange={e => onPatch({repoOwner: e.target.value})}
					placeholder="e.g. ananddub"
					className={INPUT}
				/>
			</Field>
			<Field label="Repository Name *" className="md:col-span-2">
				<Input
					value={repoName}
					onChange={e => onPatch({repoName: e.target.value})}
					placeholder="e.g. sample-node-project"
					className={INPUT}
				/>
			</Field>
			<Field label="Git Branch *" className="md:col-span-2">
				<Input
					value={gitBranch}
					onChange={e => onPatch({gitBranch: e.target.value})}
					placeholder="main"
					className={INPUT}
				/>
			</Field>
			<Field label="Compose File Path *" className="md:col-span-2">
				<Input
					value={gitBuildPath}
					onChange={e => onPatch({gitBuildPath: e.target.value})}
					placeholder="docker-compose.yml"
					className={INPUT}
				/>
			</Field>
		</div>
	);
}
