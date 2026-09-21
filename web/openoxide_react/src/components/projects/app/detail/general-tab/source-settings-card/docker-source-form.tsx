import {LabeledInput} from './labeled-input';
import type {SourceForm} from './source-form';

export function DockerSourceForm({
	form,
	onPatch,
}: {
	form: SourceForm;
	onPatch: (patch: Partial<SourceForm>) => void;
}) {
	return (
		<div className="grid grid-cols-1 gap-4 md:grid-cols-2">
			<LabeledInput
				label="Docker Image"
				placeholder="node:16"
				mono
				value={form.dockerImage}
				onValueChange={v => onPatch({dockerImage: v})}
				className="col-span-1 md:col-span-2"
			/>
			<LabeledInput
				label="Registry URL"
				placeholder="Registry URL (Optional)"
				mono
				value={form.registryUrl}
				onValueChange={v => onPatch({registryUrl: v})}
				className="col-span-1 md:col-span-2"
			/>
			<LabeledInput
				label="Username"
				placeholder="Username (Optional)"
				value={form.dockerUsername}
				onValueChange={v => onPatch({dockerUsername: v})}
			/>
			<LabeledInput
				label="Password"
				type="password"
				placeholder="Password (Optional)"
				value={form.dockerPassword}
				onValueChange={v => onPatch({dockerPassword: v})}
			/>
		</div>
	);
}
