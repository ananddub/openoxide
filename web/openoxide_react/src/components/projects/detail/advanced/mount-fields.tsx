import {
	AlertTriangle,
	FileText,
	FolderInput,
	HardDrive,
} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {Field, FieldGrid} from '#/components/shared/field';
import {cn} from '#/api/utils';

export type MountFormState = {
	type: string;
	source: string;
	target: string;
	content: string;
};

export const emptyMountForm: MountFormState = {
	type: 'VOLUME',
	source: '',
	target: '',
	content: '',
};

const MOUNT_TYPES = [
	{value: 'BIND', label: 'Bind Mount', icon: FolderInput},
	{value: 'VOLUME', label: 'Volume Mount', icon: HardDrive},
	{value: 'FILE', label: 'File Mount', icon: FileText},
] as const;

/** Seed a form from an existing mount (or a blank one). */
export function mountFormFrom(mount: any | null): MountFormState {
	return {
		type: mount?.mount_type || 'VOLUME',
		source:
			mount?.volume_name || mount?.host_path || mount?.file_path || '',
		target: mount?.mount_path || '',
		content: mount?.content || '',
	};
}

/** Build the create/upsert body the mount API expects. */
export function buildMountPayload(state: MountFormState) {
	const {type, source, target, content} = state;
	return {
		mount_type: type,
		mount_path: target.trim(),
		volume_name: type === 'VOLUME' ? source.trim() : null,
		host_path: type === 'BIND' ? source.trim() : null,
		file_path: type === 'FILE' ? source.trim() : null,
		content: type === 'FILE' ? content : null,
	};
}

/** Controlled volume form body shared by the App and Compose volume sheets. */
export function MountFields({
	state,
	set,
}: {
	state: MountFormState;
	set: (patch: Partial<MountFormState>) => void;
}) {
	const {type, source, target, content} = state;
	return (
		<div className="flex flex-col gap-5">
			{type === 'BIND' && (
				<div className="flex items-start gap-2.5 rounded-lg border border-amber-500/30 bg-amber-500/5 px-3.5 py-3 text-[13px] leading-snug text-amber-700 dark:text-amber-300">
					<AlertTriangle className="mt-0.5 size-4 shrink-0" />
					<span>
						The host path must exist on every node where this service can
						run. Named volumes are safer for clustered deployments.
					</span>
				</div>
			)}

			<div className="flex flex-col gap-2.5">
				<p className="text-sm leading-none font-medium text-foreground">
					Mount type
				</p>
				<div className="grid grid-cols-1 gap-3 sm:grid-cols-3">
					{MOUNT_TYPES.map(option => {
						const selected = type === option.value;
						return (
							<button
								key={option.value}
								type="button"
								onClick={() => set({type: option.value})}
								className={cn(
									'flex flex-col items-start gap-2 rounded-xl border px-4 py-3.5 text-start transition-all duration-150',
									selected
										? 'border-primary/40 bg-primary/10 text-primary'
										: 'border-border/50 text-foreground hover:border-border hover:bg-muted/30',
								)}>
								<option.icon className="size-5" />
								<span className="text-sm font-medium">{option.label}</span>
							</button>
						);
					})}
				</div>
			</div>

			<FieldGrid>
				<Field
					label={
						type === 'VOLUME'
							? 'Volume name'
							: type === 'BIND'
								? 'Host path'
								: 'File path'
					}
					hint={
						type === 'VOLUME'
							? 'A named Docker volume to attach.'
							: 'Absolute path on the host or in the container.'
					}>
					<Input
						value={source}
						onChange={event => set({source: event.target.value})}
						placeholder={
							type === 'VOLUME' ? 'my-volume' : '/absolute/path'
						}
					/>
				</Field>
				<Field
					label="Mount path"
					hint="Where it appears inside the container.">
					<Input
						value={target}
						onChange={event => set({target: event.target.value})}
						placeholder="/var/www/data"
					/>
				</Field>
			</FieldGrid>

			{type === 'FILE' && (
				<Field
					label="Content"
					hint="The file is written with exactly this content.">
					<Textarea
						value={content}
						onChange={event => set({content: event.target.value})}
						className="min-h-40 font-mono text-xs"
					/>
				</Field>
			)}
		</div>
	);
}
