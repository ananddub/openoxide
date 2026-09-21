import {useEffect, useState} from 'react';
import {FileText, FolderInput, HardDrive} from 'lucide-react';
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

export type MountValue = {
	id?: number;
	mount_type: string;
	host_path?: string | null;
	volume_name?: string | null;
	file_path?: string | null;
	content?: string | null;
	mount_path: string;
};

const mountTypes = [
	{value: 'BIND', label: 'Bind Mount', icon: FolderInput},
	{value: 'VOLUME', label: 'Volume Mount', icon: HardDrive},
	{value: 'FILE', label: 'File Mount', icon: FileText},
] as const;

/** Add/edit dialog for application and compose mounts. */
export function MountDialog({
	open,
	mount,
	saving,
	onOpenChange,
	onSave,
}: {
	open: boolean;
	mount: MountValue | null;
	saving: boolean;
	onOpenChange: (open: boolean) => void;
	onSave: (value: Omit<MountValue, 'id'>) => Promise<void>;
}) {
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
			<DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-2xl">
				<DialogHeader>
					<DialogTitle>Volumes / Mounts</DialogTitle>
					<DialogDescription>
						{mount
							? 'Update the selected mount.'
							: 'Add persistent storage or an injected file.'}
					</DialogDescription>
				</DialogHeader>
				{type === 'BIND' && (
					<div className="rounded-lg bg-amber-500/5 p-3 text-xs text-amber-700 dark:text-amber-300">
						The host path must exist on every node where this service
						can run. Named volumes are safer for clustered
						deployments.
					</div>
				)}
				<div className="grid grid-cols-3 gap-2">
					{mountTypes.map(option => {
						const Icon = option.icon;
						const active = type === option.value;
						return (
							<button
								key={option.value}
								type="button"
								onClick={() => setType(option.value)}
								className={`flex flex-col items-center gap-1.5 rounded-xl border p-3 text-xs font-semibold transition-colors ${
									active
										? 'border-primary bg-primary/5 text-primary'
										: 'border-border/50 text-muted-foreground hover:border-border hover:text-foreground'
								}`}>
								<Icon className="size-4" />
								{option.label}
							</button>
						);
					})}
				</div>
				<div className="flex flex-col gap-1.5">
					<span className="text-xs font-bold text-muted-foreground">
						{type === 'VOLUME'
							? 'Volume Name'
							: type === 'BIND'
								? 'Host Path'
								: 'File Path'}
					</span>
					<Input
						placeholder={
							type === 'VOLUME'
								? 'my-volume'
								: type === 'BIND'
									? '/data/host/path'
									: 'config/app.env'
						}
						value={source}
						onChange={e => setSource(e.target.value)}
						className="text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<span className="text-xs font-bold text-muted-foreground">
						Mount Path (inside container)
					</span>
					<Input
						placeholder="/var/www/data"
						value={target}
						onChange={e => setTarget(e.target.value)}
						className="text-xs"
					/>
				</div>
				{type === 'FILE' && (
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">
							File Content
						</span>
						<Textarea
							rows={6}
							placeholder="File contents written into the container"
							value={content}
							onChange={e => setContent(e.target.value)}
							className="font-mono text-xs"
						/>
					</div>
				)}
				<DialogFooter>
					<Button
						variant="outline"
						onClick={() => onOpenChange(false)}
						className="text-xs">
						Cancel
					</Button>
					<Button
						disabled={saving || !source.trim() || !target.trim()}
						onClick={submit}
						className="text-xs">
						{saving ? 'Saving…' : mount ? 'Update' : 'Add'}
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
