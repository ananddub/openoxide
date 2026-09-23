import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {WizardField} from '#/components/shared/sheet-wizard';
import type {ScheduleDialogController} from './use-schedule-dialog';

/** Target Type + Target Item selectors for the schedule sheet. */
export function ScheduleTargetFields({
	form,
}: {
	form: ScheduleDialogController;
}) {
	return (
		<>
			<WizardField label="Target type" required>
				<Select
					value={form.targetType}
					onValueChange={form.handleTargetTypeChange}>
					<SelectTrigger className="bg-background">
						<SelectValue />
					</SelectTrigger>
					<SelectContent className="bg-card">
						<SelectItem value="SERVER">Server</SelectItem>
						<SelectItem value="APPLICATION">Application</SelectItem>
					</SelectContent>
				</Select>
			</WizardField>

			<WizardField label="Target" required>
				{form.targetType === 'SERVER' ? (
					<Select
						value={form.targetId}
						onValueChange={val => form.setTargetId(val ?? '')}>
						<SelectTrigger className="bg-background">
							<SelectValue placeholder="Select server">
								{form.selectedServerName || 'Select server'}
							</SelectValue>
						</SelectTrigger>
						<SelectContent className="bg-card">
							{form.servers.map(srv => (
								<SelectItem key={srv.id} value={String(srv.id)}>
									{srv.name}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				) : (
					<Select
						value={form.targetId}
						onValueChange={val => form.setTargetId(val ?? '')}>
						<SelectTrigger className="bg-background">
							<SelectValue placeholder="Select application">
								{form.selectedAppName || 'Select application'}
							</SelectValue>
						</SelectTrigger>
						<SelectContent className="bg-card">
							{form.allApplications.length > 0 ? (
								form.allApplications.map(app => (
									<SelectItem key={app.id} value={String(app.id)}>
										{app.name}
									</SelectItem>
								))
							) : (
								<SelectItem value={form.targetId || '1'}>
									{form.targetId
										? `Application #${form.targetId}`
										: 'No applications found'}
								</SelectItem>
							)}
						</SelectContent>
					</Select>
				)}
			</WizardField>
		</>
	);
}
