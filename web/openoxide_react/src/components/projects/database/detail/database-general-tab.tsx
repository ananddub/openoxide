import {useMemo, useState} from 'react';
import type {DatabaseResponse} from '#/types/api-helpers';
import {TerminalCard} from '#/components/shared/terminal-card';
import {EnvironmentSheet} from '#/components/projects/common/environment-sheet';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {SheetGroup} from '#/components/shared/field';
import {
	ArchitecturePreviewCard,
	statusToneFor,
} from '#/components/projects/compose/detail/cards/architecture-preview-card';
import {DatabaseHeaderActions} from './database-header-actions';
import type {DatabaseActionType} from './database-deploy-settings-card';
import {DatabaseInternalCredentialsCard} from './database-internal-credentials-card';
import {DatabaseExternalCredentialsCard} from './database-external-credentials-card';
import {BuildLogsSection} from '#/components/projects/compose/detail/cards/architecture-build-logs';
import {
	buildDbArchService,
	dbInternalPort,
	dbSupportsVolumeBackup,
	filterDbArchBackups,
} from './arch-visual-data';

interface DatabaseGeneralTabProps {
	database: DatabaseResponse | null;
	actionLoading?: string | null;
	isBuilding?: boolean;
	onAction: (action: DatabaseActionType) => Promise<void>;
	handleUpdate?: (body: any) => Promise<void>;
	onUpdated?: () => void;
	databaseId?: number;
	kind?: string;
	backups?: any[];
	onOpenArchitecture: () => void;
}

export function DatabaseGeneralTab({
	database,
	onAction,
	handleUpdate,
	onUpdated,
	databaseId,
	kind: kindProp,
	backups,
	onOpenArchitecture,
}: DatabaseGeneralTabProps) {
	const [showTerminal, setShowTerminal] = useState(false);
	const [showEnv, setShowEnv] = useState(false);
	const [showCreds, setShowCreds] = useState(false);
	const kind = (kindProp || database?.kind || 'postgres').toLowerCase();
	const archServices = useMemo(
		() => buildDbArchService(database, kind),
		[database, kind],
	);
	const archBackups = useMemo(
		() =>
			dbSupportsVolumeBackup(kind)
				? filterDbArchBackups(backups, databaseId, database)
				: [],
		[backups, databaseId, database, kind],
	);

	return (
		<div className="flex w-full animate-in flex-col gap-6 duration-200 fade-in">
			{/* Read-only topology — what a general visitor looks at first */}
			<ArchitecturePreviewCard
				customServices={archServices}
				backups={archBackups}
				status={{
					label:
						(database as any)?.database_status ||
						(database as any)?.status ||
						'UNKNOWN',
					tone: statusToneFor(
						(database as any)?.database_status ||
							(database as any)?.status,
					),
				}}
				rows={[
					{
						label: 'Engine',
						value: (
							<span className="font-mono text-[13px] capitalize">
								{kind}
							</span>
						),
					},
					{
						label: 'Port',
						value: (
							<span className="font-mono text-[13px]">
								{(database as any)?.external_port || dbInternalPort(kind)}
							</span>
						),
					},
				]}
				onOpen={onOpenArchitecture}
				monitor={{
					id: Number(database?.id ?? 0),
					type: 'database',
				}}
				errorDetail={(database as any)?.last_error}
				sections={
					<BuildLogsSection
						entityId={Number(database?.id ?? 0)}
						kind="database"
					/>
				}
				actions={
					<DatabaseHeaderActions
						database={database}
						onAction={onAction}
						onOpenTerminal={() => setShowTerminal(true)}
						onOpenEnv={handleUpdate ? () => setShowEnv(true) : undefined}
						onOpenCredentials={() => setShowCreds(true)}
					/>
				}
			/>

			{/* Credentials — opened from the header icon, no inline strip */}
			<SettingsSheet
				title="Credentials"
				description="How apps and the internet reach this database."
				open={showCreds}
				onOpenChange={setShowCreds}>
				<SheetGroup
					title="Internal"
					description="Credentials apps on the private network use to connect.">
					<DatabaseInternalCredentialsCard
						database={database}
						onUpdated={onUpdated}
						embedded
					/>
				</SheetGroup>

				<SheetGroup
					title="External"
					description="Optionally expose the database on a public port.">
					<DatabaseExternalCredentialsCard
						database={database}
						onUpdated={onUpdated}
						embedded
					/>
				</SheetGroup>
			</SettingsSheet>

			{/* Environment variables — opened from the key icon */}
			{handleUpdate && database && (
				<EnvironmentSheet
					open={showEnv}
					onOpenChange={setShowEnv}
					app={database}
					handleUpdate={handleUpdate}
				/>
			)}

			{/* Container terminal opened from the header icon */}
			{showTerminal && database && (
				<TerminalCard
					app={database}
					enabled={true}
					onClose={() => setShowTerminal(false)}
				/>
			)}
		</div>
	);
}
