import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {useSshKeyItems} from '#/api/live';
import {LabeledInput} from './labeled-input';
import type {SourceForm} from './source-form';

export function GitSourceForm({
	form,
	onPatch,
}: {
	form: SourceForm;
	onPatch: (patch: Partial<SourceForm>) => void;
}) {
	const sshKeys = useSshKeyItems().data ?? [];
	const selectedKey = sshKeys?.find(
		(k: any) => Number(k.id) === Number(form.sshKeyId),
	);
	const selectedName = form.sshKeyId
		? selectedKey?.name || `SSH Key #${form.sshKeyId}`
		: 'None (Public Repo)';

	return (
		<div className="grid grid-cols-1 items-end gap-4 md:grid-cols-4">
			<LabeledInput
				label="Repository URL"
				placeholder="https://github.com/username/repo.git"
				mono
				value={form.gitUrl}
				onValueChange={v => onPatch({gitUrl: v})}
				className="md:col-span-3"
			/>
			<div className="flex flex-col gap-1.5 md:col-span-1">
				<span className="text-xs font-bold text-foreground">SSH Key</span>
				<Select
					value={form.sshKeyId ? String(form.sshKeyId) : 'none'}
					onValueChange={val =>
						onPatch({sshKeyId: val === 'none' ? undefined : Number(val)})
					}>
					<SelectTrigger className="border- h-9 w-full bg-card text-xs">
						<SelectValue placeholder="No SSH Key Selected">
							{selectedName}
						</SelectValue>
					</SelectTrigger>
					<SelectContent className="bg-popover">
						<SelectItem
							value="none"
							className="cursor-pointer text-xs font-bold text-primary hover:bg-muted">
							None (Public Repo)
						</SelectItem>
						{sshKeys?.map(key => (
							<SelectItem
								key={key.id}
								value={String(key.id)}
								className="cursor-pointer font-mono text-xs hover:bg-muted">
								{key.name}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>
			<LabeledInput
				label="Branch"
				placeholder="main"
				value={form.gitBranch}
				onValueChange={v => onPatch({gitBranch: v})}
				className="md:col-span-2"
			/>
			<LabeledInput
				label="Build Path"
				placeholder="/"
				value={form.gitBuildPath}
				onValueChange={v => onPatch({gitBuildPath: v})}
				className="md:col-span-2"
			/>
		</div>
	);
}
