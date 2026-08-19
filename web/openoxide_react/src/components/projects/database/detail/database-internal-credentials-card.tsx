import {useState, useEffect} from 'react';
import {Eye, EyeOff, Pencil, Check, X, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface DatabaseInternalCredentialsCardProps {
	database: any;
	onUpdated?: () => void;
}

export function DatabaseInternalCredentialsCard({
	database,
	onUpdated,
}: DatabaseInternalCredentialsCardProps) {
	const [showPassword, setShowPassword] = useState(false);
	const [showInternalUrl, setShowInternalUrl] = useState(false);
	const [isEditingPassword, setIsEditingPassword] = useState(false);
	const [newPassword, setNewPassword] = useState('');
	const [isSavingPassword, setIsSavingPassword] = useState(false);

	const kind = (database?.kind || 'postgres').toLowerCase();
	const isRedis = kind.includes('redis');
	const isLibsql = kind.includes('libsql');

	const rawDb = database as unknown as Record<string, unknown>;
	const defaultUser = kind.includes('postgres')
		? 'postgres'
		: kind.includes('mysql') ||
			  kind.includes('maria') ||
			  kind.includes('mongo')
			? 'root'
			: isRedis
				? 'default'
				: 'admin';

	const dbUser = String(
		database?.database_user ||
			rawDb?.database_user ||
			rawDb?.databaseUser ||
			rawDb?.db_user ||
			defaultUser,
	);
	const dbName = String(
		database?.database_name ||
			rawDb?.database_name ||
			rawDb?.databaseName ||
			database?.name ||
			'',
	);
	const currentPassword = String(
		rawDb?.database_password ||
			rawDb?.databasePassword ||
			rawDb?.password ||
			rawDb?.database_root_password ||
			rawDb?.db_password ||
			rawDb?.postgres_password ||
			rawDb?.mysql_password ||
			rawDb?.mongo_password ||
			'',
	);

	const internalPort =
		kind.includes('mysql') || kind.includes('maria')
			? 3306
			: kind.includes('mongo')
				? 27017
				: isRedis
					? 6379
					: isLibsql
						? 8080
						: 5432;
	const host = database?.app_name || database?.name || 'localhost';
	const dbNamePath = isRedis || isLibsql ? '' : `/${dbName || 'db'}`;
	const userAuthPart = isRedis
		? currentPassword
			? `:${currentPassword}`
			: ''
		: `${dbUser}:${currentPassword}`;
	const internalConnStr = `${kind}://${userAuthPart}@${host}:${internalPort}${dbNamePath}`;

	let endpoint:
		| '/postgres/{id}'
		| '/mysql/{id}'
		| '/mariadb/{id}'
		| '/mongo/{id}'
		| '/redis/{id}'
		| '/libsql/{id}' = '/postgres/{id}';
	if (kind.includes('mysql')) endpoint = '/mysql/{id}';
	else if (kind.includes('mariadb')) endpoint = '/mariadb/{id}';
	else if (kind.includes('mongo')) endpoint = '/mongo/{id}';
	else if (kind.includes('redis')) endpoint = '/redis/{id}';
	else if (kind.includes('libsql')) endpoint = '/libsql/{id}';

	const patchDatabase = $api.useMutation('patch', endpoint as any);

	useEffect(() => {
		setNewPassword(currentPassword);
	}, [currentPassword]);

	const handleSavePassword = async () => {
		if (!database?.id) return;
		if (!newPassword.trim()) {
			toast.error('Password cannot be empty');
			return;
		}
		setIsSavingPassword(true);
		try {
			await patchDatabase.mutateAsync({
				params: {path: {id: Number(database.id)}},
				body: {database_password: newPassword.trim()} as any,
			});
			toast.success('Password updated successfully');
			setIsEditingPassword(false);
			if (onUpdated) onUpdated();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsSavingPassword(false);
		}
	};

	return (
		<div className="flex flex-col gap-5 rounded-2xl border border-border/80 bg-card p-6 shadow-xs">
			<h3 className="text-xl font-bold tracking-tight text-foreground">
				Internal Credentials
			</h3>

			<div className="grid grid-cols-1 gap-4 border-t border-border/40 pt-4 md:grid-cols-2 md:gap-6">
				<div className="flex flex-col gap-1.5">
					<label className="text-xs font-semibold text-muted-foreground">
						User
					</label>
					<Input
						readOnly
						value={dbUser}
						className="h-9 bg-muted/30 font-mono text-xs"
					/>
				</div>

				{!isRedis && !isLibsql && (
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-muted-foreground">
							Database Name
						</label>
						<Input
							readOnly
							value={dbName}
							className="h-9 bg-muted/30 font-mono text-xs"
						/>
					</div>
				)}

				<div className="flex flex-col gap-1.5">
					<label className="text-xs font-semibold text-muted-foreground">
						Password
					</label>
					<div className="flex items-center gap-2">
						{isEditingPassword ? (
							<>
								<Input
									type={showPassword ? 'text' : 'password'}
									value={newPassword}
									onChange={e => setNewPassword(e.target.value)}
									placeholder="Enter new password"
									className="h-9 border-primary/50 bg-background font-mono text-xs"
								/>
								<Button
									variant="outline"
									size="sm"
									type="button"
									onClick={() => setShowPassword(!showPassword)}
									className="h-9 shrink-0 px-2.5">
									{showPassword ? (
										<EyeOff className="size-3.5" />
									) : (
										<Eye className="size-3.5" />
									)}
								</Button>
								<Button
									variant="default"
									size="sm"
									type="button"
									disabled={isSavingPassword}
									onClick={handleSavePassword}
									className="h-9 shrink-0 gap-1.5 bg-primary px-3 text-xs font-semibold text-primary-foreground">
									{isSavingPassword ? (
										<RefreshCw className="size-3.5 animate-spin" />
									) : (
										<Check className="size-3.5" />
									)}{' '}
									Save
								</Button>
								<Button
									variant="ghost"
									size="sm"
									type="button"
									onClick={() => {
										setIsEditingPassword(false);
										setNewPassword(currentPassword);
									}}
									className="h-9 shrink-0 px-2 text-xs">
									<X className="size-3.5" />
								</Button>
							</>
						) : (
							<>
								<Input
									readOnly
									type={showPassword ? 'text' : 'password'}
									value={currentPassword || ''}
									className="h-9 bg-muted/30 font-mono text-xs"
								/>
								<Button
									variant="outline"
									size="sm"
									type="button"
									onClick={() => setShowPassword(!showPassword)}
									className="h-9 shrink-0 px-2.5">
									{showPassword ? (
										<EyeOff className="size-3.5" />
									) : (
										<Eye className="size-3.5" />
									)}
								</Button>
								<Button
									variant="outline"
									size="sm"
									type="button"
									onClick={() => setIsEditingPassword(true)}
									className="h-9 shrink-0 px-2.5">
									<Pencil className="size-3.5 text-muted-foreground" />
								</Button>
							</>
						)}
					</div>
				</div>

				<div className="flex flex-col gap-1.5">
					<label className="text-xs font-semibold text-muted-foreground">
						Internal Port (Container)
					</label>
					<Input
						readOnly
						value={internalPort.toString()}
						className="h-9 bg-muted/30 font-mono text-xs"
					/>
				</div>

				<div className="flex flex-col gap-1.5">
					<label className="text-xs font-semibold text-muted-foreground">
						Internal Host
					</label>
					<Input
						readOnly
						value={host}
						className="h-9 bg-muted/30 font-mono text-xs"
					/>
				</div>

				<div className="flex flex-col gap-1.5 md:col-span-2">
					<label className="text-xs font-semibold text-muted-foreground">
						Internal Connection URL
					</label>
					<div className="flex items-center gap-2">
						<Input
							readOnly
							type={showInternalUrl ? 'text' : 'password'}
							value={internalConnStr}
							className="h-9 bg-muted/30 font-mono text-xs"
						/>
						<Button
							variant="outline"
							size="sm"
							type="button"
							onClick={() => setShowInternalUrl(!showInternalUrl)}
							className="h-9 shrink-0 px-2.5">
							{showInternalUrl ? (
								<EyeOff className="size-3.5" />
							) : (
								<Eye className="size-3.5" />
							)}
						</Button>
					</div>
				</div>
			</div>
		</div>
	);
}
