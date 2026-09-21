import {Label} from '#/components/ui/label';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import type {ScheduleDialogController} from './use-schedule-dialog';

/** Target Type + Target Item selectors for the schedule form. */
export function ScheduleTargetFields({
	form,
}: {
	form: ScheduleDialogController;
}) {
	return (
		<>
			<div className="w-full space-y-1.5">
				<Label className="text-xs font-semibold">Target Type *</Label>
				<Select
					value={form.targetType}
					onValueChange={form.handleTargetTypeChange}>
					<SelectTrigger className="!h-9 w-full text-xs">
						<SelectValue />
					</SelectTrigger>
					<SelectContent className="border- bg-card">
						<SelectItem value="SERVER" className="text-xs">
							Server
						</SelectItem>
						<SelectItem value="APPLICATION" className="text-xs">
							Application
						</SelectItem>
					</SelectContent>
				</Select>
			</div>

			<div className="w-full space-y-1.5">
				<Label className="text-xs font-semibold">Target Item *</Label>
				{form.targetType === 'SERVER' ? (
					<Select
						value={form.targetId}
						onValueChange={val => form.setTargetId(val ?? '')}>
						<SelectTrigger className="!h-9 w-full text-xs">
							<SelectValue placeholder="Select server">
								{form.selectedServerName || 'Select server'}
							</SelectValue>
						</SelectTrigger>
						<SelectContent className="border- bg-card">
							{form.servers.map(srv => (
								<SelectItem
									key={srv.id}
									value={String(srv.id)}
									className="text-xs">
									{srv.name}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				) : (
					<Select
						value={form.targetId}
						onValueChange={val => form.setTargetId(val ?? '')}>
						<SelectTrigger className="!h-9 w-full text-xs">
							<SelectValue placeholder="Select application">
								{form.selectedAppName || 'Select application'}
							</SelectValue>
						</SelectTrigger>
						<SelectContent className="border- bg-card">
							{form.allApplications.length > 0 ? (
								form.allApplications.map(app => (
									<SelectItem
										key={app.id}
										value={String(app.id)}
										className="text-xs">
										{app.name}
									</SelectItem>
								))
							) : (
								<SelectItem
									value={form.targetId || '1'}
									className="text-xs">
									{form.targetId
										? `Application #${form.targetId}`
										: 'No applications found'}
								</SelectItem>
							)}
						</SelectContent>
					</Select>
				)}
			</div>
		</>
	);
}
