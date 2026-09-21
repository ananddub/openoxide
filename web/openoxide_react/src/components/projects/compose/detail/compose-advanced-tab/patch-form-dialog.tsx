import {useState} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Textarea} from '#/components/ui/textarea';
import {Switch} from '#/components/ui/switch';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {useComposePatchUpsert, type ComposePatchPayload} from '#/api/live';
import {formatApiError} from '#/api/utils';

const patchTypes = [
	{value: 'UPDATE', label: 'Update (merge YAML)'},
	{value: 'DELETE', label: 'Delete (remove keys)'},
];

export function PatchFormDialog({
	composeId,
	patch,
	open,
	onOpenChange,
}: {
	composeId: number;
	patch: ComposePatchPayload | null;
	open: boolean;
	onOpenChange: (open: boolean) => void;
}) {
	const upsert = useComposePatchUpsert();
	const [saving, setSaving] = useState(false);
	const [type, setType] = useState(patch?.patch_type || 'UPDATE');
	const [filePath, setFilePath] = useState(patch?.file_path || '');
	const [enabled, setEnabled] = useState(patch ? patch.enabled : true);
	const [content, setContent] = useState(patch?.content || '');

	const save = async () => {
		setSaving(true);
		try {
			await upsert.mutateAsync({
				compose_id: composeId,
				id: patch?.id,
				patch_type: type,
				payload: JSON.stringify({
					file_path: filePath,
					enabled,
					content,
				}),
			});
			toast.success(patch ? 'Patch updated' : 'Patch added');
			onOpenChange(false);
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<Dialog open={open} onOpenChange={onOpenChange}>
			<DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-2xl">
				<DialogHeader>
					<DialogTitle>Compose Patch</DialogTitle>
					<DialogDescription>
						Target one service file in the compose project with a patch
						overlay.
					</DialogDescription>
				</DialogHeader>
				<div className="grid grid-cols-2 gap-3">
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-bold text-muted-foreground">
							Patch Type
						</Label>
						<Select value={type} onValueChange={v => v && setType(v)}>
							<SelectTrigger className="h-9 text-xs">
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								{patchTypes.map(option => (
									<SelectItem key={option.value} value={option.value}>
										{option.label}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</div>
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-bold text-muted-foreground">
							Service File
						</Label>
						<Input
							placeholder="docker-compose.yml"
							value={filePath}
							onChange={e => setFilePath(e.target.value)}
							className="font-mono text-xs"
						/>
					</div>
				</div>
				<div className="flex items-center justify-between rounded-lg border border-border/40 px-3 py-2.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Enabled
					</Label>
					<Switch checked={enabled} onCheckedChange={setEnabled} />
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Content (YAML)
					</Label>
					<Textarea
						rows={8}
						placeholder={'environment:\n  - FOO=bar'}
						value={content}
						onChange={e => setContent(e.target.value)}
						className="font-mono text-xs"
					/>
				</div>
				<DialogFooter>
					<Button
						variant="outline"
						onClick={() => onOpenChange(false)}
						className="text-xs">
						Cancel
					</Button>
					<Button
						disabled={saving || !filePath.trim() || !content.trim()}
						onClick={save}
						className="text-xs">
						{saving ? 'Saving…' : patch ? 'Update' : 'Add'}
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
