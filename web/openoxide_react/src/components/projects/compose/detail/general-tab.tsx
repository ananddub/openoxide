import {useState} from 'react';
import {TerminalCard} from '#/components/shared/terminal-card';
import {EnvironmentSheet} from '#/components/projects/common/environment-sheet';
import {formatTimestamp} from '#/components/projects/detail/deployments/build-status';
import {ComposeHeaderActions} from './cards/compose-header-actions';
import {ComposeSourceCard} from './cards/compose-source-card';
import {BuildLogsSection} from './cards/architecture-build-logs';
import {
	ArchitecturePreviewCard,
	statusToneFor,
} from './cards/architecture-preview-card';
import {useComposeSource} from './use-compose-source';

interface GeneralTabProps {
	compose: any;
	onAction: (
		action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'cancel',
	) => Promise<void>;
	handleUpdate: (body: any) => Promise<void>;
	onUpdated: () => void;
	domains?: any[];
	schedules?: any[];
	backups?: any[];
	onOpenArchitecture: () => void;
}

export function ComposeGeneralTab({
	compose,
	handleUpdate,
	onUpdated,
	domains,
	schedules,
	backups,
	onOpenArchitecture,
}: GeneralTabProps) {
	const [showTerminal, setShowTerminal] = useState(false);
	const [showEnv, setShowEnv] = useState(false);
	const [showSourceCfg, setShowSourceCfg] = useState(false);
	const {source, onPatch, savingSource, handleSaveSource} =
		useComposeSource(compose, onUpdated);

	return (
		<div className="flex flex-col gap-6">
			{/* Read-only topology — what a general visitor looks at first */}
			<ArchitecturePreviewCard
				compose={compose}
				domains={domains}
				schedules={schedules}
				backups={backups}
				status={{
					label: compose?.compose_status || compose?.status || 'STOPPED',
					tone: statusToneFor(compose?.compose_status || compose?.status),
				}}
				rows={[
					{
						label: 'Source',
						value: (
							<span className="font-mono text-[13px]">
								{compose?.repository ||
									compose?.custom_git_url ||
									compose?.gitlab_repository ||
									compose?.gitea_repository ||
									compose?.bitbucket_repository ||
									'docker-compose.yml'}
							</span>
						),
					},
					{
						label: 'Created',
						value: (
							<span className="text-[13px] font-semibold">
								{formatTimestamp(compose?.created_at)}
							</span>
						),
					},
				]}
				onOpen={onOpenArchitecture}
				monitor={{
					id: Number(compose?.id ?? 0),
					type: 'compose',
				}}
				errorDetail={compose?.last_error}
				sections={
					<BuildLogsSection
						entityId={Number(compose?.id ?? 0)}
						kind="compose"
					/>
				}
				actions={
					<ComposeHeaderActions
						compose={compose}
						onUpdated={onUpdated}
						onOpenTerminal={() => setShowTerminal(true)}
						onOpenEnv={() => setShowEnv(true)}
						onOpenSource={() => setShowSourceCfg(true)}
					/>
				}
			/>

			{/* Configuration sheet — opened from the header icon, no inline strip */}
			<ComposeSourceCard
				source={source}
				onPatch={onPatch}
				savingSource={savingSource}
				onSave={handleSaveSource}
				open={showSourceCfg}
				onOpenChange={setShowSourceCfg}
			/>

			{/* Environment variables — opened from the key icon */}
			<EnvironmentSheet
				open={showEnv}
				onOpenChange={setShowEnv}
				app={compose}
				handleUpdate={handleUpdate}
			/>

			{/* Interactive terminal for compose containers — inline card */}
			{showTerminal && (
				<TerminalCard
					app={compose}
					enabled={true}
					onClose={() => setShowTerminal(false)}
				/>
			)}
		</div>
	);
}
