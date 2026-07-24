import {DeploySettingsCard} from './general-tab/deploy-settings-card';
import {SourceSettingsCard} from './general-tab/source-settings-card';
import {BuildSettingsCard} from './general-tab/build-settings-card';

interface GeneralTabProps {
	app: any;
	onUpdated: () => void;
	handleAction: (action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'cancel') => Promise<void>;
}

export function GeneralTab({app, onUpdated, handleAction}: GeneralTabProps) {
	return (
		<div className="flex flex-col gap-6">
			{/* Deploy Controls */}
			<DeploySettingsCard app={app} handleAction={handleAction} onUpdated={onUpdated} />

			{/* Source Settings */}
			<SourceSettingsCard app={app} onUpdated={onUpdated} />

			{/* Build Type */}
			<BuildSettingsCard app={app} onUpdated={onUpdated} />
		</div>
	);
}
