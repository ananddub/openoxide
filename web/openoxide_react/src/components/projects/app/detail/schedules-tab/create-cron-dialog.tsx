import {useState} from 'react';
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
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useOrganizations} from '#/stores/session-context';
import {useScheduleCreate} from '#/api/live';

export function CreateCronDialog({
	app,
	open,
	onClose,
	onCreated,
}: {
	app: any;
	open: boolean;
	onClose: () => void;
	onCreated?: () => void;
}) {
	const {activeOrg} = useOrganizations();
	const createMutation = useScheduleCreate();
	const [name, setName] = useState('');
	const [cronExpression, setCronExpression] = useState('0 2 * * *');
	const [command, setCommand] = useState('echo "running scheduled task"');
	const [creating, setCreating] = useState(false);

	const handleCreate = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name || !cronExpression || !command) {
			toast.error('Please fill in all required fields');
			return;
		}
		setCreating(true);
		try {
			await createMutation.mutateAsync({
				name,
				cron_expression: cronExpression,
				command,
				application_id: app.id,
				organization_id: activeOrg?.id || 1,
				app_name: app.app_name || app.appName || app.name || 'application',
				enabled: 1,
				schedule_type: 'APPLICATION',
				schedule_action: 'EXEC',
				shell_type: 'bash',
			});
			toast.success('Schedule created successfully');
			setName('');
			onClose();
			onCreated?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setCreating(false);
		}
	};

	return (
		<Sheet open={open} onOpenChange={o => !o && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>Create Cron Job</SheetTitle>
					<SheetDescription>
						Run a command inside this application on a schedule.
					</SheetDescription>
				</SheetHeader>

				<form
					id="create-cron-form"
					onSubmit={handleCreate}
					className="flex flex-1 flex-col gap-4 overflow-y-auto px-6 py-5">
					<div className="flex flex-col gap-1.5">
						<Label htmlFor="cron-name">Job Name</Label>
						<Input
							id="cron-name"
							placeholder="Nightly Database Backup"
							value={name}
							onChange={e => setName(e.target.value)}
						/>
					</div>
					<div className="flex flex-col gap-1.5">
						<Label htmlFor="cron-expr">Cron Expression</Label>
						<Input
							id="cron-expr"
							placeholder="0 2 * * *"
							value={cronExpression}
							onChange={e => setCronExpression(e.target.value)}
							className="font-mono"
						/>
						<span className="text-xs text-muted-foreground">
							Standard 5-part cron syntax (e.g. 0 2 * * * for 2 AM daily)
						</span>
					</div>
					<div className="flex flex-col gap-1.5">
						<Label htmlFor="cron-cmd">Command to Execute</Label>
						<Input
							id="cron-cmd"
							placeholder="npm run backup"
							value={command}
							onChange={e => setCommand(e.target.value)}
							className="font-mono"
						/>
					</div>
				</form>

				<SheetFooter>
					<Button type="button" variant="outline" onClick={onClose}>
						Cancel
					</Button>
					<Button
						type="submit"
						form="create-cron-form"
						disabled={creating}>
						{creating ? 'Creating...' : 'Create Job'}
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
