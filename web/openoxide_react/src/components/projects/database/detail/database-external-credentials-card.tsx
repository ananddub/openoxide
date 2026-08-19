import {useState, useEffect} from 'react';
import {Eye, EyeOff, Check, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {useAppStore} from '#/stores/app-store';

interface DatabaseExternalCredentialsCardProps {
	database: any;
	onUpdated?: () => void;
}

export function DatabaseExternalCredentialsCard({
	database,
	onUpdated,
}: DatabaseExternalCredentialsCardProps) {
	const [showExternalUrl, setShowExternalUrl] = useState(false);
	const [extPortInput, setExtPortInput] = useState('');
	const [isSavingPort, setIsSavingPort] = useState(false);

	const servers = useAppStore(state => state.servers || []);
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

	const targetServer = servers.find(
		(s: any) => String(s.id) === String(database?.server_id),
	);
	const serverIp =
		targetServer?.ip_address ||
		targetServer?.ip ||
		(typeof window !== 'undefined' && window.location.hostname
			? window.location.hostname
			: '127.0.0.1');

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
	const externalPort = database?.external_port || undefined;
	const dbNamePath = isRedis || isLibsql ? '' : `/${dbName || 'db'}`;
	const userAuthPart = isRedis
		? currentPassword
			? `:${currentPassword}`
			: ''
		: `${dbUser}:${currentPassword}`;
	const externalConnStr = `${kind}://${userAuthPart}@${serverIp}:${externalPort || internalPort}${dbNamePath}`;

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
		setExtPortInput(externalPort ? externalPort.toString() : '');
	}, [externalPort]);

	const handleSavePort = async () => {
		if (!database?.id) return;
		const parsedPort = extPortInput.trim()
			? Number(extPortInput)
			: undefined;
		if (
			extPortInput.trim() &&
			(isNaN(Number(extPortInput)) ||
				Number(extPortInput) < 1 ||
				Number(extPortInput) > 65535)
		) {
			toast.error('Please enter a valid port number between 1 and 65535');
			return;
		}
		setIsSavingPort(true);
		try {
			await patchDatabase.mutateAsync({
				params: {path: {id: Number(database.id)}},
				body: {external_port: parsedPort} as any,
			});
			toast.success('External Port updated');
			if (onUpdated) onUpdated();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsSavingPort(false);
		}
	};

	return (
		<div className="flex flex-col gap-5 rounded-2xl border border-border/80 bg-card p-6 shadow-xs">
			<div>
				<h3 className="text-xl font-bold tracking-tight text-foreground">
					External Credentials
				</h3>
				<p className="mt-1 text-xs leading-relaxed text-muted-foreground">
					In order to make the database reachable through the internet, you
					must set a port and ensure that the port is not being used by
					another application or database
				</p>
			</div>

			<div className="flex flex-col gap-5 border-t border-border/40 pt-4">
				<div className="flex w-full flex-col gap-1.5">
					<label className="text-xs font-semibold text-muted-foreground">
						External Port (Internet)
					</label>
					<Input
						type="number"
						value={extPortInput}
						onChange={e => setExtPortInput(e.target.value)}
						placeholder={internalPort.toString()}
						className="h-9 w-full border-border bg-background font-mono text-xs"
					/>
				</div>

				{externalPort ? (
					<div className="flex flex-col gap-4 border-t border-border/30 pt-4">
						<div className="flex w-full flex-col gap-1.5">
							<label className="text-xs font-semibold text-muted-foreground">
								External Host
							</label>
							<Input
								readOnly
								value={serverIp}
								className="h-9 w-full bg-muted/30 font-mono text-xs"
							/>
						</div>
						<div className="flex w-full flex-col gap-1.5">
							<label className="text-xs font-semibold text-muted-foreground">
								External Connection URL
							</label>
							<div className="flex w-full items-center gap-2">
								<Input
									readOnly
									type={showExternalUrl ? 'text' : 'password'}
									value={externalConnStr}
									className="h-9 w-full bg-muted/30 font-mono text-xs"
								/>
								<Button
									variant="outline"
									size="sm"
									type="button"
									onClick={() => setShowExternalUrl(!showExternalUrl)}
									className="h-9 shrink-0 px-2.5">
									{showExternalUrl ? (
										<EyeOff className="size-3.5" />
									) : (
										<Eye className="size-3.5" />
									)}
								</Button>
							</div>
						</div>
					</div>
				) : null}

				<div className="flex justify-end border-t border-border/30 pt-2">
					<Button
						variant="default"
						size="sm"
						disabled={isSavingPort}
						onClick={handleSavePort}
						className="h-9 cursor-pointer gap-1.5 bg-primary px-6 text-xs font-semibold text-primary-foreground">
						{isSavingPort ? (
							<RefreshCw className="size-3.5 animate-spin" />
						) : (
							<Check className="size-3.5" />
						)}{' '}
						Save
					</Button>
				</div>
			</div>
		</div>
	);
}
