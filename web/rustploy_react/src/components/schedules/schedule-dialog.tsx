import React, {useState, useEffect, useMemo} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle} from '#/components/ui/dialog';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {formatApiError} from '#/api/utils';
import type {Schedule} from '#/hooks/use-schedules';
import {Clock, Terminal} from 'lucide-react';
import {$api} from '#/api/query';

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

const CRON_PRESETS = [
	{label: 'Every 5m', value: '*/5 * * * *'},
	{label: 'Every 15m', value: '*/15 * * * *'},
	{label: 'Hourly', value: '0 * * * *'},
	{label: 'Daily', value: '0 0 * * *'},
	{label: 'Weekly', value: '0 0 * * 0'},
];

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

	// Fetch projects to list application names dynamically
	const {data: projectsList} = $api.useQuery('get', '/projects') as any;

	const allApplications = useMemo(() => {
		if (!Array.isArray(projectsList)) return [];
		const apps: { id: number; name: string }[] = [];
		(projectsList as any[]).forEach((proj: any) => {
			if (proj.applications) {
				proj.applications.forEach((app: any) => {
					apps.push({
						id: app.id || app.application_id,
						name: app.name || app.app_name || `App #${app.id}`,
					});
				});
			}
			if (proj.environments) {
				proj.environments.forEach((env: any) => {
					if (env.applications) {
						env.applications.forEach((app: any) => {
							apps.push({
								id: app.id || app.application_id,
								name: app.name || app.app_name || `App #${app.id}`,
							});
						});
					}
				});
			}
		});
		return apps;
	}, [projectsList]);

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
			toast.error('Please fill in all required fields including Target Item');
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
			<DialogContent className="sm:max-w-lg bg-card border-border p-6 rounded-xl shadow-2xl max-h-[90vh] overflow-y-auto">
				<DialogHeader className="flex flex-row items-center gap-3 space-y-0 border-b border-border/40 pb-4">
					<div className="size-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary shrink-0 border border-primary/20">
						<Clock className="size-5" />
					</div>
					<div>
						<DialogTitle className="text-base font-bold text-foreground">
							{editingSchedule ? 'Edit Schedule Task' : 'Create Schedule Task'}
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground mt-0.5">
							Configure automated cron jobs and command executions.
						</DialogDescription>
					</div>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="space-y-4 pt-2">
					{/* Name - Full Width */}
					<div className="space-y-1.5 w-full">
						<Label className="text-xs font-semibold">Name *</Label>
						<Input placeholder="e.g. Daily Database Backup" value={name} onChange={e => setName(e.target.value)} required className="h-9 text-xs w-full" />
					</div>

					{/* Description - Full Width */}
					<div className="space-y-1.5 w-full">
						<Label className="text-xs font-semibold">Description</Label>
						<Input placeholder="Optional task details..." value={description} onChange={e => setDescription(e.target.value)} className="h-9 text-xs w-full" />
					</div>

					{/* Target Type & Target Item - 2 Equal Columns (Full Width) */}
					<div className="grid grid-cols-2 gap-3 w-full">
						<div className="space-y-1.5 w-full">
							<Label className="text-xs font-semibold">Target Type *</Label>
							<Select value={targetType} onValueChange={val => {
								const newType = val as 'SERVER' | 'APPLICATION';
								setTargetType(newType);
								if (newType === 'SERVER') {
									setTargetId(servers[0]?.id ? String(servers[0].id) : '');
								} else {
									setTargetId(allApplications[0]?.id ? String(allApplications[0].id) : '');
								}
							}}>
								<SelectTrigger className="!h-9 text-xs w-full"><SelectValue /></SelectTrigger>
								<SelectContent className="bg-card border-border">
									<SelectItem value="SERVER" className="text-xs">Server</SelectItem>
									<SelectItem value="APPLICATION" className="text-xs">Application</SelectItem>
								</SelectContent>
							</Select>
						</div>
						<div className="space-y-1.5 w-full">
							<Label className="text-xs font-semibold">Target Item *</Label>
							{targetType === 'SERVER' ? (
								<Select value={targetId} onValueChange={val => setTargetId(val ?? '')}>
									<SelectTrigger className="!h-9 text-xs w-full"><SelectValue placeholder="Select server" /></SelectTrigger>
									<SelectContent className="bg-card border-border">
										{servers.map(srv => <SelectItem key={srv.id} value={String(srv.id)} className="text-xs">{srv.name}</SelectItem>)}
									</SelectContent>
								</Select>
							) : (
								<Select value={targetId} onValueChange={val => setTargetId(val ?? '')}>
									<SelectTrigger className="!h-9 text-xs w-full"><SelectValue placeholder="Select application" /></SelectTrigger>
									<SelectContent className="bg-card border-border">
										{allApplications.length > 0 ? (
											allApplications.map(app => (
												<SelectItem key={app.id} value={String(app.id)} className="text-xs">
													{app.name}
												</SelectItem>
											))
										) : (
											<SelectItem value={targetId || '1'} className="text-xs">
												{targetId ? `Application #${targetId}` : 'No applications found'}
											</SelectItem>
										)}
									</SelectContent>
								</Select>
							)}
						</div>
					</div>

					{/* Cron Expression - Full Width */}
					<div className="space-y-1.5 w-full">
						<div className="flex items-center justify-between">
							<Label className="text-xs font-semibold">Cron Expression *</Label>
							<div className="flex items-center gap-1">
								{CRON_PRESETS.map(preset => (
									<button
										key={preset.value}
										type="button"
										onClick={() => setCronExpression(preset.value)}
										className={`text-[10px] px-1.5 py-0.5 rounded font-mono transition-colors border ${
											cronExpression === preset.value
												? 'bg-primary text-primary-foreground border-primary'
												: 'bg-muted/40 text-muted-foreground border-border/40 hover:text-foreground hover:bg-muted'
										}`}
									>
										{preset.label}
									</button>
								))}
							</div>
						</div>
						<Input placeholder="*/5 * * * *" value={cronExpression} onChange={e => setCronExpression(e.target.value)} required className="h-9 text-xs font-mono w-full" />
					</div>

					{/* Shell Type - Full Width */}
					<div className="space-y-1.5 w-full">
						<Label className="text-xs font-semibold">Shell Type</Label>
						<Select value={shellType} onValueChange={val => setShellType(val ?? 'bash')}>
							<SelectTrigger className="!h-9 text-xs w-full"><SelectValue /></SelectTrigger>
							<SelectContent className="bg-card border-border">
								<SelectItem value="bash" className="text-xs">BASH</SelectItem>
								<SelectItem value="sh" className="text-xs">SH</SelectItem>
							</SelectContent>
						</Select>
					</div>

					{/* Command - Full Width */}
					<div className="space-y-1.5 w-full">
						<Label className="text-xs font-semibold flex items-center gap-1.5">
							<Terminal className="size-3.5 text-muted-foreground" />
							Command *
						</Label>
						<textarea
							placeholder="e.g. docker exec my_app npm run cleanup"
							value={command}
							onChange={e => setCommand(e.target.value)}
							required
							rows={3}
							className="flex w-full rounded-lg border border-border/80 bg-muted/20 px-3 py-2 text-xs font-mono text-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-primary"
						/>
					</div>

					<div className="flex justify-end pt-3 border-t border-border/30">
						<Button type="submit" disabled={isSubmitting} className="bg-primary hover:bg-primary/95 text-primary-foreground text-xs h-9 px-5 font-semibold shadow-sm w-full sm:w-auto">
							{isSubmitting ? 'Saving...' : editingSchedule ? 'Save Changes' : 'Create Schedule'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
