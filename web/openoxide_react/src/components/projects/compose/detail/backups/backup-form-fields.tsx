import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Switch} from '#/components/ui/switch';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

interface BackupFormFieldsProps {
	destinations: any[];
	servicesList: string[];
	destinationId: string;
	onDestinationId: (v: string) => void;
	name: string;
	onName: (v: string) => void;
	serviceName: string;
	onServiceName: (v: string) => void;
	volumeName: string;
	onVolumeName: (v: string) => void;
	cronExpr: string;
	onCronExpr: (v: string) => void;
	prefix: string;
	onPrefix: (v: string) => void;
	turnOff: boolean;
	onTurnOff: (v: boolean) => void;
	hideServiceAndVolumeSelect?: boolean;
}

const Field = ({
	label,
	children,
}: {
	label: string;
	children: React.ReactNode;
}) => (
	<div className="flex flex-col gap-1.5">
		<Label className="text-xs font-semibold">{label}</Label>
		{children}
	</div>
);

/** All input controls for the create/edit volume-backup rule form. */
export function BackupFormFields(p: BackupFormFieldsProps) {
	return (
		<>
			<Field label="S3 Destination *">
				<Select
					value={p.destinationId}
					onValueChange={v => v && p.onDestinationId(v)}>
					<SelectTrigger className="h-9 w-full text-xs">
						<SelectValue placeholder="Select S3 destination" />
					</SelectTrigger>
					<SelectContent>
						{p.destinations.map((d: any) => (
							<SelectItem key={d.id} value={String(d.id)}>
								{d.name} ({d.bucket})
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Field>

			<Field label="Backup Rule Name *">
				<Input
					value={p.name}
					onChange={e => p.onName(e.target.value)}
					placeholder="Daily Postgres Data Backup"
					className="h-9 text-xs"
				/>
			</Field>

			{!p.hideServiceAndVolumeSelect && (
				<div className="grid w-full grid-cols-1 gap-4 sm:grid-cols-2">
					<Field label="Service *">
						<Select
							value={p.serviceName}
							onValueChange={v => v && p.onServiceName(v)}>
							<SelectTrigger className="border- h-9 w-full bg-muted/20 text-xs font-semibold">
								<SelectValue placeholder="Select service" />
							</SelectTrigger>
							<SelectContent className="border- bg-card">
								{p.servicesList.map(srv => (
									<SelectItem
										key={srv}
										value={srv}
										className="text-xs font-semibold">
										{srv}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</Field>
					<Field label="Volume Name *">
						<Input
							value={p.volumeName}
							onChange={e => p.onVolumeName(e.target.value)}
							placeholder="postgres_data"
							className="h-9 w-full font-mono text-xs"
						/>
					</Field>
				</div>
			)}

			<Field label="Cron Schedule (UTC) *">
				<Input
					value={p.cronExpr}
					onChange={e => p.onCronExpr(e.target.value)}
					placeholder="0 2 * * * (Daily at 2 AM)"
					className="h-9 font-mono text-xs"
				/>
			</Field>

			<Field label="Backup File Prefix">
				<Input
					value={p.prefix}
					onChange={e => p.onPrefix(e.target.value)}
					placeholder="db_backup"
					className="h-9 font-mono text-xs"
				/>
			</Field>

			<div className="flex items-center justify-between rounded-lg bg-muted/20 p-3">
				<div>
					<Label className="text-xs font-semibold">
						Stop Container During Backup
					</Label>
					<p className="text-[11px] text-muted-foreground">
						Ensures zero database corruption during snapshot
					</p>
				</div>
				<Switch checked={p.turnOff} onCheckedChange={p.onTurnOff} />
			</div>
		</>
	);
}
