import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle} from '#/components/ui/dialog';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {formatApiError} from '#/api/utils';
import type {Schedule} from '#/hooks/use-schedules';

interface ScheduleDialogProps {
	isOpen: boolean;
	onClose: () => void;
	editingSchedule: Schedule | null;
	servers: any[];
	refetch: () => void;
	activeOrgId?: number;
	createMutation: any;
	patchMutation: any;
}

export function ScheduleDialog({
	isOpen, onClose, editingSchedule, servers, refetch, activeOrgId, createMutation, patchMutation
}: ScheduleDialogProps) {
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [cronExpression, setCronExpression] = useState('');
	const [command, setCommand] = useState('');
	const [shellType, setShellType] = useState('bash');
	const [targetType, setTargetType] = useState<'SERVER' | 'APPLICATION'>('SERVER');
	const [targetId, setTargetId] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	useEffect(() => {
		if (editingSchedule) {
			setName(editingSchedule.name);
			setDescription(editingSchedule.description || '');
			setCronExpression(editingSchedule.cron_expression);
			setCommand(editingSchedule.command);
			setShellType(editingSchedule.shell_type || 'bash');
			if (editingSchedule.application_id) {
				setTargetType('APPLICATION');
				setTargetId(String(editingSchedule.application_id));
			} else {
				setTargetType('SERVER');
				setTargetId(editingSchedule.server_id ? String(editingSchedule.server_id) : '');
			}
		} else {
			setName('');
			setDescription('');
			setCronExpression('*/5 * * * *');
			setCommand('');
			setShellType('bash');
			setTargetType('SERVER');
			setTargetId(servers[0]?.id ? String(servers[0].id) : '');
		}
	}, [editingSchedule, isOpen, servers]);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim() || !cronExpression.trim() || !command.trim() || !targetId) {
			toast.error('Please fill in all required fields including Target');
			return;
		}

		setIsSubmitting(true);
		try {
			const body: any = {
				name,
				description,
				cron_expression: cronExpression,
				command,
				shell_type: shellType.toUpperCase(),
				schedule_type: targetType,
				schedule_action: 'EXEC',
				organization_id: activeOrgId,
				enabled: editingSchedule ? editingSchedule.enabled : 1,
			};

			if (targetType === 'SERVER') {
				body.server_id = Number(targetId);
			} else {
				body.application_id = Number(targetId);
			}

			if (editingSchedule?.id !== undefined) {
				await patchMutation.mutateAsync({
					params: { path: { id: editingSchedule.id } },
					body,
				});
				toast.success('Schedule updated');
			} else {
				await createMutation.mutateAsync({ body });
				toast.success('Schedule created');
			}
			refetch();
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-lg bg-card border-border p-6">
				<DialogHeader>
					<DialogTitle className="text-lg font-bold">
						{editingSchedule ? 'Edit Schedule' : 'Create Schedule'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Configure automated command execution tasks.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="space-y-4 mt-4">
					<div className="grid grid-cols-2 gap-4">
						<div className="space-y-1">
							<label className="text-xs font-semibold text-foreground">Name *</label>
							<Input placeholder="Daily Backup" value={name} onChange={e => setName(e.target.value)} required className="h-9" />
						</div>
						<div className="space-y-1">
							<label className="text-xs font-semibold text-foreground">Description</label>
							<Input placeholder="Details" value={description} onChange={e => setDescription(e.target.value)} className="h-9" />
						</div>
					</div>

					<div className="grid grid-cols-2 gap-4">
						<div className="space-y-1">
							<label className="text-xs font-semibold text-foreground">Cron Expression *</label>
							<Input placeholder="*/5 * * * *" value={cronExpression} onChange={e => setCronExpression(e.target.value)} required className="h-9 font-mono" />
						</div>
						<div className="space-y-1">
							<label className="text-xs font-semibold text-foreground">Shell Type</label>
							<Select value={shellType} onValueChange={val => setShellType(val ?? 'bash')}>
								<SelectTrigger className="h-9"><SelectValue /></SelectTrigger>
								<SelectContent className="bg-card border-border">
									<SelectItem value="bash">BASH</SelectItem>
									<SelectItem value="sh">SH</SelectItem>
								</SelectContent>
							</Select>
						</div>
					</div>

					<div className="grid grid-cols-2 gap-4">
						<div className="space-y-1">
							<label className="text-xs font-semibold text-foreground">Target Type *</label>
							<Select value={targetType} onValueChange={val => {
								setTargetType(val as any);
								setTargetId(val === 'SERVER' && servers[0]?.id ? String(servers[0].id) : '');
							}}>
								<SelectTrigger className="h-9"><SelectValue /></SelectTrigger>
								<SelectContent className="bg-card border-border">
									<SelectItem value="SERVER">Server</SelectItem>
									<SelectItem value="APPLICATION">Application</SelectItem>
								</SelectContent>
							</Select>
						</div>
						<div className="space-y-1">
							<label className="text-xs font-semibold text-foreground">Target Selection *</label>
							{targetType === 'SERVER' ? (
								<Select value={targetId} onValueChange={val => setTargetId(val ?? '')}>
									<SelectTrigger className="h-9"><SelectValue placeholder="Select server" /></SelectTrigger>
									<SelectContent className="bg-card border-border">
										{servers.map(srv => <SelectItem key={srv.id} value={String(srv.id)}>{srv.name}</SelectItem>)}
									</SelectContent>
								</Select>
							) : (
								<Input type="number" placeholder="Application ID" value={targetId} onChange={e => setTargetId(e.target.value)} required className="h-9" />
							)}
						</div>
					</div>

					<div className="space-y-1">
						<label className="text-xs font-semibold text-foreground">Command *</label>
						<textarea placeholder="Command to run..." value={command} onChange={e => setCommand(e.target.value)} required rows={3} className="flex w-full rounded-lg border border-border bg-card/50 px-3 py-2 text-xs font-mono focus-visible:outline-none" />
					</div>

					<div className="flex justify-end gap-3 pt-3 border-t border-border/20">
						<Button type="button" variant="outline" onClick={onClose} disabled={isSubmitting} className="h-9 text-xs">Cancel</Button>
						<Button type="submit" disabled={isSubmitting} className="bg-primary hover:bg-primary/95 text-primary-foreground text-xs h-9 px-4">
							{isSubmitting ? 'Saving...' : editingSchedule ? 'Save Changes' : 'Create Schedule'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
