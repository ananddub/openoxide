import {useMemo, useState} from 'react';
import {TerminalCard} from '#/components/shared/terminal-card';
import {EnvironmentSheet} from '#/components/projects/common/environment-sheet';
import {
	ArchitecturePreviewCard,
	statusToneFor,
} from '#/components/projects/compose/detail/cards/architecture-preview-card';
import {formatTimestamp} from '#/components/projects/detail/deployments/build-status';
import {AppHeaderActions} from './general-tab/app-header-actions';
import {rawAppStatus} from './general-tab/deploy-settings-card/actions';
import {SourceSettingsCard} from './general-tab/source-settings-card';
import {BuildSettingsCard} from './general-tab/build-settings-card';
import {BuildLogsSection} from '#/components/projects/compose/detail/cards/architecture-build-logs';
import {
	buildAppArchService,
	filterAppArchBackups,
} from './arch-visual-data';

interface GeneralTabProps {
	app: any;
	onUpdated: () => void;
	handleAction: (
		action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'cancel',
	) => Promise<void>;
	handleUpdate: (body: any) => Promise<void>;
	domains?: any[];
	schedules?: any[];
	backups?: any[];
	onOpenArchitecture: () => void;
}

export function GeneralTab({
	app,
	onUpdated,
	handleAction,
	handleUpdate,
	domains,
	schedules,
	backups,
	onOpenArchitecture,
}: GeneralTabProps) {
	const [showTerminal, setShowTerminal] = useState(false);
	const [showEnv, setShowEnv] = useState(false);
	const archServices = useMemo(() => buildAppArchService(app), [app]);
	const archBackups = useMemo(
		() => filterAppArchBackups(backups, app),
		[backups, app],
	);

	return (
		<div className="flex flex-col gap-6">
			{/* Read-only topology — what a general visitor looks at first */}
			<ArchitecturePreviewCard
				customServices={archServices}
				domains={domains}
				schedules={schedules}
				backups={archBackups}
				status={{
					label: rawAppStatus(app).toUpperCase() || 'UNKNOWN',
					tone: statusToneFor(rawAppStatus(app)),
				}}
				rows={[
					{
						label: 'Source',
						value: (
							<span className="font-mono text-[13px]">
								{app?.repository || app?.build_type || 'No repository'}
							</span>
						),
					},
					{
						label: 'Created',
						value: (
							<span className="text-[13px] font-semibold">
								{formatTimestamp(app?.created_at)}
							</span>
						),
					},
				]}
				onOpen={onOpenArchitecture}
				monitor={{
					id: Number(app?.application_id ?? app?.id ?? 0),
					type: 'application',
				}}
				errorDetail={app?.last_error}
				sections={
					<>
						{/* Source Settings — own strip + right sheet */}
						<SourceSettingsCard
							app={app}
							onUpdated={onUpdated}
							embedded
						/>

						{/* Build Type — own strip + right sheet */}
						<BuildSettingsCard
							app={app}
							onUpdated={onUpdated}
							embedded
						/>

						<BuildLogsSection
							entityId={Number(app?.application_id ?? app?.id ?? 0)}
							kind="app"
						/>
					</>
				}
				actions={
					<AppHeaderActions
						app={app}
						handleAction={handleAction}
						onOpenTerminal={() => setShowTerminal(true)}
						onOpenEnv={() => setShowEnv(true)}
					/>
				}
			/>

			{/* Environment variables — opened from the key icon */}
			<EnvironmentSheet
				open={showEnv}
				onOpenChange={setShowEnv}
				app={app}
				handleUpdate={handleUpdate}
			/>

			{/* Container terminal opened from the header icon */}
			{showTerminal && (
				<TerminalCard
					app={app}
					enabled={true}
					onClose={() => setShowTerminal(false)}
				/>
			)}
		</div>
	);
}
