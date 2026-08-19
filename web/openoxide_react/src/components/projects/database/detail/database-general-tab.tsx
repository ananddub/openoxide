import type {DatabaseResponse} from '#/types/api-helpers';
import {
	DatabaseDeploySettingsCard,
	type DatabaseActionType,
} from './database-deploy-settings-card';
import {DatabaseInternalCredentialsCard} from './database-internal-credentials-card';
import {DatabaseExternalCredentialsCard} from './database-external-credentials-card';

interface DatabaseGeneralTabProps {
	database: DatabaseResponse | null;
	actionLoading?: string | null;
	isBuilding?: boolean;
	onAction: (action: DatabaseActionType) => Promise<void>;
	onUpdated?: () => void;
}

export function DatabaseGeneralTab({
	database,
	actionLoading,
	onAction,
	onUpdated,
}: DatabaseGeneralTabProps) {
	return (
		<div className="flex w-full animate-in flex-col gap-6 duration-200 fade-in">
			{/* Deploy & Lifecycle Control Card */}
			<DatabaseDeploySettingsCard
				database={database}
				actionLoading={actionLoading}
				onAction={onAction}
				onUpdated={onUpdated}
			/>

			{/* Internal Credentials Card */}
			<DatabaseInternalCredentialsCard
				database={database}
				onUpdated={onUpdated}
			/>

			{/* External Port & Internet Access Credentials Card */}
			<DatabaseExternalCredentialsCard
				database={database}
				onUpdated={onUpdated}
			/>
		</div>
	);
}
