import {LabeledInput} from './labeled-input';
import type {SourceForm} from './source-form';

/** Owner/repository/branch/path form for GitHub, GitLab, Gitea & Bitbucket. */
export function RepoSourceForm({
	form,
	onPatch,
}: {
	form: SourceForm;
	onPatch: (patch: Partial<SourceForm>) => void;
}) {
	return (
		<div className="grid grid-cols-1 items-end gap-4 md:grid-cols-4">
			<LabeledInput
				label="Owner / Username"
				placeholder="e.g. facebook"
				value={form.repoOwner}
				onValueChange={v => onPatch({repoOwner: v})}
				className="md:col-span-2"
			/>
			<LabeledInput
				label="Repository Name"
				placeholder="e.g. react"
				value={form.repoName}
				onValueChange={v => onPatch({repoName: v})}
				className="md:col-span-2"
			/>
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
