import {useState} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {useApplicationPortCreate} from '#/api/live';
import {formatApiError} from '#/api/utils';

/** "Add Port" modal: host/ingress port → container port mapping. */
export function PortAddDialog({
	open,
	onOpenChange,
	appId,
}: {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	appId: number;
}) {
	const create = useApplicationPortCreate();
	const [saving, setSaving] = useState(false);
	const [published, setPublished] = useState('');
	const [target, setTarget] = useState('');
	const [protocol, setProtocol] = useState('TCP');
	const [mode, setMode] = useState('HOST');

	const save = async () => {
		setSaving(true);
		try {
			await create.mutateAsync({
				application_id: appId,
				published_port: Number(published),
				target_port: Number(target),
				protocol,
				publish_mode: mode,
			});
			toast.success('Port added');
			onOpenChange(false);
			setPublished('');
			setTarget('');
			setProtocol('TCP');
			setMode('HOST');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<Sheet open={open} onOpenChange={onOpenChange}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>Add Port</SheetTitle>
					<SheetDescription>
						Map a host or ingress port to a container port.
					</SheetDescription>
				</SheetHeader>
				<div className="grid grid-cols-2 gap-3 px-6">
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-bold text-muted-foreground">
							Published Port
						</Label>
						<Input
							type="number"
							placeholder="8080"
							value={published}
							onChange={e => setPublished(e.target.value)}
							className="text-xs"
						/>
					</div>
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-bold text-muted-foreground">
							Target Port
						</Label>
						<Input
							type="number"
							placeholder="3000"
							value={target}
							onChange={e => setTarget(e.target.value)}
							className="text-xs"
						/>
					</div>
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-bold text-muted-foreground">
							Protocol
						</Label>
						<Select
							value={protocol}
							onValueChange={v => v && setProtocol(v)}>
							<SelectTrigger className="h-9 text-xs">
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="TCP">TCP</SelectItem>
								<SelectItem value="UDP">UDP</SelectItem>
							</SelectContent>
						</Select>
					</div>
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-bold text-muted-foreground">
							Publish Mode
						</Label>
						<Select value={mode} onValueChange={v => v && setMode(v)}>
							<SelectTrigger className="h-9 text-xs">
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="HOST">Host</SelectItem>
								<SelectItem value="INGRESS">Ingress</SelectItem>
							</SelectContent>
						</Select>
					</div>
				</div>
				<SheetFooter>
					<Button
						variant="outline"
						onClick={() => onOpenChange(false)}>
						Cancel
					</Button>
					<Button
						disabled={saving || !published || !target}
						onClick={save}>
						{saving ? 'Saving…' : 'Add'}
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
