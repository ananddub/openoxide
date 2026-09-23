import {useState, useEffect} from 'react';
import {Check, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {useDatabaseUpdate, useRemoteServerList} from '#/api/live';
import {cn, formatApiError} from '#/api/utils';
import {
	deriveDatabaseCredentials,
	buildConnectionString,
} from './database-credentials';
import {ReadonlyField, SecretField} from './credentials-fields';

interface DatabaseExternalCredentialsCardProps {
	database: any;
	onUpdated?: () => void;
	/** Inside a SettingsCollapse — no own shell/title. */
	embedded?: boolean;
}

export function DatabaseExternalCredentialsCard({
	database,
	onUpdated,
	embedded = false,
}: DatabaseExternalCredentialsCardProps) {
	const [extPortInput, setExtPortInput] = useState('');
	const [isSavingPort, setIsSavingPort] = useState(false);

	// Live servers query; the table-watch stream keeps it fresh.
	const {data: servers = []} = useRemoteServerList();
	const db = deriveDatabaseCredentials(database);

	const targetServer = (servers as any[]).find(
		(s: any) => String(s.id) === String(database?.server_id),
	);
	const serverIp =
		targetServer?.ip_address ||
		targetServer?.ip ||
		(typeof window !== 'undefined' && window.location.hostname
			? window.location.hostname
			: '127.0.0.1');

	const externalPort = database?.external_port || undefined;
	const externalConnStr = buildConnectionString(
		db.kind,
		db.userAuthPart,
		serverIp,
		externalPort || db.internalPort,
		db.dbNamePath,
	);
	const patchDatabase = useDatabaseUpdate();

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
				kind: db.dbKind,
				id: Number(database.id),
				external_port: parsedPort,
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
		<div
			className={cn(
				'flex flex-col gap-5',
				!embedded && 'rounded-xl bg-card p-6 shadow-xs',
			)}>
			{!embedded && (
				<div>
					<h3 className="text-xl font-bold tracking-tight text-foreground">
						External Credentials
					</h3>
					<p className="mt-1 text-xs leading-relaxed text-muted-foreground">
						In order to make the database reachable through the
						internet, you must set a port and ensure that the port
						is not being used by another application or database
					</p>
				</div>
			)}

			<div className="flex flex-col gap-5 border-t  pt-4">
				<div className="flex w-full flex-col gap-1.5">
					<label className="text-xs font-semibold text-muted-foreground">
						External Port (Internet)
					</label>
					<Input
						type="number"
						value={extPortInput}
						onChange={e => setExtPortInput(e.target.value)}
						placeholder={db.internalPort.toString()}
						className="h-9 w-full font-mono text-xs"
					/>
				</div>

				{externalPort ? (
					<div className="flex flex-col gap-4 border-t  pt-4">
						<ReadonlyField
							label="External Host"
							value={serverIp}
							className="w-full"
						/>
						<SecretField
							label="External Connection URL"
							value={externalConnStr}
							className="w-full"
						/>
					</div>
				) : null}

				<div className="flex justify-end border-t  pt-2">
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
