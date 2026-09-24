import {toast} from 'sonner';
import {useDatabaseUpdate} from '#/api/live';
import {cn, formatApiError} from '#/api/utils';
import {
	deriveDatabaseCredentials,
	buildConnectionString,
} from './database-credentials';
import {ReadonlyField, SecretField} from './credentials-fields';
import {PasswordEditorField} from './password-editor-field';

interface DatabaseInternalCredentialsCardProps {
	database: any;
	onUpdated?: () => void;
	/** Inside a SettingsSheet — no own shell/title. */
	embedded?: boolean;
}

export function DatabaseInternalCredentialsCard({
	database,
	onUpdated,
	embedded = false,
}: DatabaseInternalCredentialsCardProps) {
	const db = deriveDatabaseCredentials(database);
	const host = database?.app_name || database?.name || 'localhost';
	const internalConnStr = buildConnectionString(
		db.kind,
		db.userAuthPart,
		host,
		db.internalPort,
		db.dbNamePath,
	);
	const patchDatabase = useDatabaseUpdate();

	const handleSavePassword = async (next: string) => {
		if (!database?.id) return;
		if (!next.trim()) {
			toast.error('Password cannot be empty');
			return;
		}
		try {
			await patchDatabase.mutateAsync({
				kind: db.dbKind,
				id: Number(database.id),
				database_password: next.trim(),
			});
			toast.success('Password updated successfully');
			if (onUpdated) onUpdated();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<div
			className={cn(
				'flex flex-col gap-5',
				!embedded && 'rounded-xl bg-card p-6 shadow-xs',
			)}>
			{!embedded && (
				<h3 className="text-xl font-bold tracking-tight text-foreground">
					Internal Credentials
				</h3>
			)}

			<div className="grid grid-cols-1 gap-4 md:grid-cols-2 md:gap-6">
				<ReadonlyField label="User" value={db.dbUser} />

				{!db.isRedis && !db.isLibsql && (
					<ReadonlyField label="Database Name" value={db.dbName} />
				)}

				<ReadonlyField
					label="Internal Port (Container)"
					value={db.internalPort.toString()}
				/>

				<ReadonlyField label="Internal Host" value={host} />

				<PasswordEditorField
					currentPassword={db.currentPassword}
					onSave={handleSavePassword}
					className="md:col-span-2"
				/>

				<SecretField
					label="Internal Connection URL"
					value={internalConnStr}
					className="md:col-span-2"
				/>
			</div>
		</div>
	);
}
