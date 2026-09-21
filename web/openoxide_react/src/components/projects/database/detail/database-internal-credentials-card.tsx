import {toast} from 'sonner';
import {useDatabaseUpdate} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {
	deriveDatabaseCredentials,
	buildConnectionString,
} from './database-credentials';
import {ReadonlyField, SecretField} from './credentials-fields';
import {PasswordEditorField} from './password-editor-field';

interface DatabaseInternalCredentialsCardProps {
	database: any;
	onUpdated?: () => void;
}

export function DatabaseInternalCredentialsCard({
	database,
	onUpdated,
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
		<div className="flex flex-col gap-5 rounded-xl bg-card p-6 shadow-xs">
			<h3 className="text-xl font-bold tracking-tight text-foreground">
				Internal Credentials
			</h3>

			<div className="grid grid-cols-1 gap-4 border-t  pt-4 md:grid-cols-2 md:gap-6">
				<ReadonlyField label="User" value={db.dbUser} />

				{!db.isRedis && !db.isLibsql && (
					<ReadonlyField label="Database Name" value={db.dbName} />
				)}

				<PasswordEditorField
					currentPassword={db.currentPassword}
					onSave={handleSavePassword}
				/>

				<ReadonlyField
					label="Internal Port (Container)"
					value={db.internalPort.toString()}
				/>

				<ReadonlyField label="Internal Host" value={host} />

				<SecretField
					label="Internal Connection URL"
					value={internalConnStr}
					className="md:col-span-2"
				/>
			</div>
		</div>
	);
}
