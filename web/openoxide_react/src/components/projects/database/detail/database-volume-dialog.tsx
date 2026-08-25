import {useEffect, useState} from 'react';
import {FileText, FolderInput, HardDrive, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';

export type DatabaseMount = {
	id: number;
	mount_type: string;
	volume_name?: string | null;
	host_path?: string | null;
	file_path?: string | null;
	content?: string | null;
	mount_path: string;
};
type Props = {
	open: boolean;
	mount: DatabaseMount | null;
	saving: boolean;
	onOpenChange: (open: boolean) => void;
	onSave: (value: Omit<DatabaseMount, 'id'>) => Promise<void>;
};
const mountTypes = [
	{value: 'BIND', label: 'Bind Mount', icon: FolderInput},
	{value: 'VOLUME', label: 'Volume Mount', icon: HardDrive},
	{value: 'FILE', label: 'File Mount', icon: FileText},
] as const;

export function DatabaseVolumeDialog({
	open,
	mount,
	saving,
	onOpenChange,
	onSave,
}: Props) {
	const [type, setType] = useState('VOLUME');
	const [source, setSource] = useState('');
	const [target, setTarget] = useState('');
	const [content, setContent] = useState('');
	useEffect(() => {
		setType(mount?.mount_type || 'VOLUME');
		setSource(
			mount?.volume_name || mount?.host_path || mount?.file_path || '',
		);
		setTarget(mount?.mount_path || '');
		setContent(mount?.content || '');
	}, [mount, open]);
	const submit = () =>
		onSave({
			mount_type: type,
			mount_path: target.trim(),
			volume_name: type === 'VOLUME' ? source.trim() : null,
			host_path: type === 'BIND' ? source.trim() : null,
			file_path: type === 'FILE' ? source.trim() : null,
			content: type === 'FILE' ? content : null,
		});
	return (
		<Dialog open={open} onOpenChange={onOpenChange}>
			<DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-3xl">
				<DialogHeader>
					<DialogTitle>Volumes / Mounts</DialogTitle>
					<DialogDescription>
						{mount
							? 'Update the selected database mount.'
							: 'Add persistent storage or a file to this database service.'}
					</DialogDescription>
				</DialogHeader>
				{type === 'BIND' && (
					<div className="rounded-lg border border-amber-500/30 bg-amber-500/5 p-3 text-xs text-amber-700 dark:text-amber-300">
						The host path must exist on every Swarm node where this
						database can run. Named volumes are safer for clustered
						deployments.
					</div>
				)}
				<div className="flex flex-col gap-3">
					<p className="text-xs font-medium text-muted-foreground">
						Select the Mount Type
					</p>
					<div className="grid grid-cols-1 gap-3 sm:grid-cols-3">
						{mountTypes.map(option => (
							<button
								key={option.value}
								type="button"
								onClick={() => setType(option.value)}
								className={`flex h-20 flex-col items-center justify-center gap-2 rounded-md border-2 text-sm font-medium ${type === option.value ? 'border-primary bg-primary/5 text-primary' : 'border-border hover:bg-muted/40'}`}>
								<option.icon className="size-5" />
								{option.label}
							</button>
						))}
					</div>
				</div>
				<div className="grid gap-4 sm:grid-cols-2">
					<label className="flex flex-col gap-1.5 text-xs font-medium">
						{type === 'VOLUME'
							? 'Volume Name'
							: type === 'BIND'
								? 'Host Path'
								: 'File Path'}
						<Input
							value={source}
							onChange={event => setSource(event.target.value)}
							placeholder={
								type === 'VOLUME' ? 'database-data' : '/absolute/path'
							}
						/>
					</label>
					<label className="flex flex-col gap-1.5 text-xs font-medium">
						Mount Path
						<Input
							value={target}
							onChange={event => setTarget(event.target.value)}
							placeholder="/var/lib/database/data"
						/>
					</label>
					{type === 'FILE' && (
						<label className="flex flex-col gap-1.5 text-xs font-medium sm:col-span-2">
							Content
							<Textarea
								value={content}
								onChange={event => setContent(event.target.value)}
								className="min-h-48 font-mono text-xs"
							/>
						</label>
					)}
				</div>
				<DialogFooter>
					<Button
						onClick={submit}
						disabled={saving || !source.trim() || !target.trim()}
						className="gap-1.5">
						<Save className="size-4" />
						{saving ? 'Saving...' : mount ? 'Update Volume' : 'Add Volume'}
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
