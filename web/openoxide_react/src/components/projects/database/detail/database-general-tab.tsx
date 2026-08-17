import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {
	Rocket,
	RefreshCw,
	Play,
	Ban,
	Terminal,
	Eye,
	EyeOff,
	X,
	Check,
	Pencil,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {TerminalModal} from '#/components/projects/common/terminal-modal';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import type {DatabaseResponse} from '#/types/api-helpers';
import { useAppStore } from '#/stores/app-store';
interface DatabaseGeneralTabProps {
	database: DatabaseResponse | null;
	actionLoading?: string | null;
	isBuilding?: boolean;
	onAction: (action: 'deploy' | 'reload' | 'start' | 'stop' | 'cancel') => Promise<void>;
	onUpdated?: () => void;
}

type ActionType = 'deploy' | 'reload' | 'start' | 'stop' | 'cancel';

export function DatabaseGeneralTab({
	database,
	actionLoading: propActionLoading,
	isBuilding: _propIsBuilding,
	onAction,
	onUpdated,
}: DatabaseGeneralTabProps) {
	const [showPassword, setShowPassword] = useState(false);
	const [showInternalUrl, setShowInternalUrl] = useState(false);
	const [showExternalUrl, setShowExternalUrl] = useState(false);
	const [isTerminalOpen, setIsTerminalOpen] = useState(false);
	const [confirmAction, setConfirmAction] = useState<ActionType | null>(null);

	// Password edit state
	const [isEditingPassword, setIsEditingPassword] = useState(false);
	const [newPassword, setNewPassword] = useState('');
	const [isSavingPassword, setIsSavingPassword] = useState(false);

	// External Port state
	const [extPortInput, setExtPortInput] = useState('');
	const [isSavingPort, setIsSavingPort] = useState(false);

	const servers = useAppStore((state) => state.servers || []);

	const dbStatusStr = database?.status || database?.app_status || '';
	const rawDbStatus = dbStatusStr.toUpperCase();
	const isStoppingOrCancelling = rawDbStatus === 'STOPPING' || rawDbStatus === 'CANCELLING' || propActionLoading === 'stop' || propActionLoading === 'cancel';
	const activeLoading = propActionLoading || null;
	const isProcessing = activeLoading !== null || isStoppingOrCancelling;

	const rawStatus = dbStatusStr.toLowerCase();
	const isRunning = ['running', 'done', 'healthy', 'deployed', 'success', 'up', 'active', 'ok'].includes(rawStatus);
	const isBuilding = ['starting', 'building', 'queued', 'preparing'].includes(rawStatus) || activeLoading === 'deploy' || activeLoading === 'reload';

	const kind = (database?.kind || 'postgres').toLowerCase();
	const isRedis = kind.includes('redis');
	const isLibsql = kind.includes('libsql');

	const rawDb = database as unknown as Record<string, unknown>;
	const defaultUser = kind.includes('postgres')
		? 'postgres'
		: kind.includes('mysql') || kind.includes('maria') || kind.includes('mongo')
		? 'root'
		: isRedis
		? 'default'
		: 'admin';

	const dbUser = String(database?.database_user || rawDb?.database_user || rawDb?.databaseUser || rawDb?.db_user || defaultUser);
	const dbName = String(database?.database_name || rawDb?.database_name || rawDb?.databaseName || database?.name || '');
	const rawPassword = String(
		rawDb?.database_password ||
		rawDb?.databasePassword ||
		rawDb?.password ||
		rawDb?.database_root_password ||
		rawDb?.db_password ||
		rawDb?.postgres_password ||
		rawDb?.mysql_password ||
		rawDb?.mongo_password ||
		''
	);
	const currentPassword = rawPassword;

	const targetServer = servers.find((s: any) => String(s.id) === String(database?.server_id));
	const serverIp = targetServer?.ip_address || targetServer?.ip || (typeof window !== 'undefined' && window.location.hostname ? window.location.hostname : '127.0.0.1');

	const internalPort = kind.includes('mysql') || kind.includes('maria') ? 3306 : kind.includes('mongo') ? 27017 : isRedis ? 6379 : isLibsql ? 8080 : 5432;
	const externalPort = database?.external_port || undefined;
	const host = database?.app_name || database?.name || 'localhost';

	const dbNamePath = isRedis || isLibsql ? '' : `/${dbName || 'db'}`;
	const userAuthPart = isRedis
		? (currentPassword ? `:${currentPassword}` : '')
		: `${dbUser}:${currentPassword}`;
	const internalConnStr = `${kind}://${userAuthPart}@${host}:${internalPort}${dbNamePath}`;
	const externalConnStr = `${kind}://${userAuthPart}@${serverIp}:${externalPort || internalPort}${dbNamePath}`;

	// Mutations for patching password & external port
	const patchPostgres = $api.useMutation('patch', '/postgres/{id}');
	const patchMysql = $api.useMutation('patch', '/mysql/{id}');
	const patchMariadb = $api.useMutation('patch', '/mariadb/{id}');
	const patchMongo = $api.useMutation('patch', '/mongo/{id}');
	const patchRedis = $api.useMutation('patch', '/redis/{id}');
	const patchLibsql = $api.useMutation('patch', '/libsql/{id}');

	useEffect(() => {
		setNewPassword(currentPassword);
	}, [currentPassword]);

	useEffect(() => {
		setExtPortInput(externalPort ? externalPort.toString() : '');
	}, [externalPort]);

	const executeActionClick = async (action: ActionType) => {
		setConfirmAction(null);
		try {
			await onAction(action);
			if (onUpdated) {
				onUpdated();
			}
		} catch (err: unknown) {
			// handled upstream
		}
	};

	const handleSavePassword = async () => {
		if (!database?.id) return;
		if (!newPassword.trim()) {
			toast.error('Password cannot be empty');
			return;
		}
		setIsSavingPassword(true);
		try {
			const dbId = Number(database.id);
			const body = {database_password: newPassword.trim()} as unknown as {database_password?: string};
			if (isRedis) {
				await patchRedis.mutateAsync({params: {path: {id: dbId}}, body});
			} else if (kind.includes('postgres')) {
				await patchPostgres.mutateAsync({params: {path: {id: dbId}}, body});
			} else if (kind.includes('mysql')) {
				await patchMysql.mutateAsync({params: {path: {id: dbId}}, body});
			} else if (kind.includes('mariadb')) {
				await patchMariadb.mutateAsync({params: {path: {id: dbId}}, body});
			} else if (kind.includes('mongo')) {
				await patchMongo.mutateAsync({params: {path: {id: dbId}}, body});
			} else if (isLibsql) {
				await patchLibsql.mutateAsync({params: {path: {id: dbId}}, body});
			}
			toast.success('Password updated successfully');
			setIsEditingPassword(false);
			if (onUpdated) onUpdated();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsSavingPassword(false);
		}
	};

	const handleSavePort = async () => {
		if (!database?.id) return;
		const parsedPort = extPortInput.trim() ? Number(extPortInput) : undefined;
		if (extPortInput.trim() && (isNaN(Number(extPortInput)) || Number(extPortInput) < 1 || Number(extPortInput) > 65535)) {
			toast.error('Please enter a valid port number between 1 and 65535');
			return;
		}
		setIsSavingPort(true);
		try {
			const dbId = Number(database.id);
			const body = {external_port: parsedPort} as unknown as {external_port?: number};
			if (isRedis) {
				await patchRedis.mutateAsync({params: {path: {id: dbId}}, body});
			} else if (kind.includes('postgres')) {
				await patchPostgres.mutateAsync({params: {path: {id: dbId}}, body});
			} else if (kind.includes('mysql')) {
				await patchMysql.mutateAsync({params: {path: {id: dbId}}, body});
			} else if (kind.includes('mariadb')) {
				await patchMariadb.mutateAsync({params: {path: {id: dbId}}, body});
			} else if (kind.includes('mongo')) {
				await patchMongo.mutateAsync({params: {path: {id: dbId}}, body});
			} else if (isLibsql) {
				await patchLibsql.mutateAsync({params: {path: {id: dbId}}, body});
			}
			toast.success('External Port updated');
			if (onUpdated) onUpdated();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsSavingPort(false);
		}
	};

	const getActionTitle = (action: ActionType) => {
		if (action === 'deploy') return 'Deploy Database';
		if (action === 'reload') return 'Reload Database';
		if (action === 'start') return 'Start Database';
		return 'Stop Database';
	};

	const getActionDesc = (action: ActionType) => {
		if (action === 'deploy') return 'Are you sure you want to deploy this database? This will provision/update the database stack.';
		if (action === 'reload') return 'Are you sure you want to reload this database service?';
		if (action === 'start') return 'Are you sure you want to start this database container?';
		return 'Are you sure you want to stop this database container?';
	};

	return (
		<div className="flex flex-col gap-6 w-full animate-in fade-in duration-200">
			{/* Deploy Settings Card */}
			<div className="bg-card border border-border/80 rounded-2xl p-6 shadow-xs flex flex-col gap-5">
				<div>
					<h3 className="text-base font-bold text-foreground tracking-tight flex items-center gap-2">
						<Rocket className="size-4 text-primary" /> Deploy Settings
					</h3>
					<p className="text-xs text-muted-foreground mt-0.5">
						Control database provisioning, service reload, lifecycle state, and open interactive container shell.
					</p>
				</div>

				<div className="flex items-center gap-3 flex-wrap border-t border-border/40 pt-4">
					{/* Deploy Button */}
					<Button
						variant="default"
						onClick={() => executeActionClick('deploy')}
						disabled={isProcessing || isBuilding}
						className="h-9 px-4 text-xs font-semibold bg-primary hover:bg-primary/90 text-primary-foreground shadow-xs gap-2 rounded-lg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed">
						{activeLoading === 'deploy' ? <RefreshCw className="size-4 animate-spin" /> : <Rocket className="size-4" />}
						{activeLoading === 'deploy' ? 'Deploying...' : 'Deploy'}
					</Button>

					{/* Reload Button */}
					<Button
						variant="secondary"
						onClick={() => executeActionClick('reload')}
						disabled={isProcessing || isBuilding}
						className="h-9 px-4 text-xs font-semibold bg-muted hover:bg-muted/80 text-foreground border border-border/80 gap-2 rounded-lg cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed">
						{activeLoading === 'reload' ? <RefreshCw className="size-4 animate-spin" /> : <RefreshCw className="size-4" />}
						{activeLoading === 'reload' ? 'Reloading...' : 'Reload'}
					</Button>

					{/* Lifecycle Action Buttons */}
					{/* 4-State Action Button: Stopping, Cancelling, Cancel (Building), Stop (Running), Start (Idle/Error/Stopped) */}
					{activeLoading === 'stop' || rawDbStatus === 'STOPPING' ? (
						<Button
							disabled
							variant="outline"
							size="sm"
							className="border-border text-destructive font-semibold flex items-center gap-1.5 h-9 rounded-lg opacity-80"
						>
							<RefreshCw className="w-4 h-4 animate-spin text-destructive" />
							Stopping...
						</Button>
					) : activeLoading === 'cancel' || rawDbStatus === 'CANCELLING' ? (
						<Button
							disabled
							variant="outline"
							size="sm"
							className="border-destructive/50 text-destructive font-bold flex items-center gap-1.5 h-9 rounded-lg px-4 opacity-80"
						>
							<RefreshCw className="w-4 h-4 animate-spin text-destructive" />
							Cancelling...
						</Button>
					) : isBuilding ? (
						<Button
							onClick={() => executeActionClick('cancel')}
							disabled={activeLoading === 'cancel'}
							variant="outline"
							size="sm"
							className="border-destructive/50 text-destructive hover:bg-destructive/10 font-bold flex items-center gap-1.5 h-9 rounded-lg px-4 shadow-xs cursor-pointer"
						>
							<RefreshCw className="w-4 h-4 animate-spin text-destructive" />
							{activeLoading === 'cancel' ? 'Cancelling...' : 'Cancel'}
						</Button>
					) : isRunning ? (
						<Button
							onClick={() => executeActionClick('stop')}
							disabled={isProcessing}
							variant="destructive"
							size="sm"
							className="h-9 px-4 text-xs font-semibold gap-1.5 rounded-lg flex items-center cursor-pointer"
						>
							{activeLoading === 'stop' ? <RefreshCw className="size-4 animate-spin" /> : <Ban className="size-4" />}
							{activeLoading === 'stop' ? 'Stopping...' : 'Stop'}
						</Button>
					) : (
						<Button
							onClick={() => executeActionClick('start')}
							disabled={isProcessing}
							variant="outline"
							size="sm"
							className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-9 rounded-lg cursor-pointer"
						>
							{activeLoading === 'start' ? <RefreshCw className="size-4 animate-spin" /> : <Play className="w-4 h-4" />}
							{activeLoading === 'start' ? 'Starting...' : 'Start'}
						</Button>
					)}

					{/* Terminal Button */}
					<Button
						variant="outline"
						onClick={() => setIsTerminalOpen(true)}
						disabled={isProcessing || !isRunning}
						className="h-9 px-4 text-xs font-semibold border-border hover:bg-accent text-foreground gap-2 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed">
						<Terminal className="size-4 text-primary" /> Open Terminal
					</Button>
				</div>
			</div>

			{/* Internal Credentials Card (Dokploy Exact Style - NO Title Icon) */}
			<div className="bg-card border border-border/80 rounded-2xl p-6 shadow-xs flex flex-col gap-5">
				<div>
					<h3 className="text-xl font-bold text-foreground tracking-tight">
						Internal Credentials
					</h3>
				</div>

				<div className="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6 border-t border-border/40 pt-4">
					{/* User */}
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-muted-foreground">User</label>
						<Input readOnly value={dbUser} className="font-mono text-xs bg-muted/30 h-9" />
					</div>

					{/* Database Name (Only if not Redis / LibSQL) */}
					{!isRedis && !isLibsql && (
						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-muted-foreground">Database Name</label>
							<Input readOnly value={dbName} className="font-mono text-xs bg-muted/30 h-9" />
						</div>
					)}

					{/* Password with Eye toggle and Pencil icon */}
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-muted-foreground">Password</label>
						<div className="flex items-center gap-2">
							{isEditingPassword ? (
								<>
									<Input
										type={showPassword ? 'text' : 'password'}
										value={newPassword}
										onChange={(e) => setNewPassword(e.target.value)}
										placeholder="Enter new password"
										className="font-mono text-xs bg-background h-9 border-primary/50"
									/>
									<Button
										variant="outline"
										size="sm"
										type="button"
										onClick={() => setShowPassword(!showPassword)}
										className="shrink-0 h-9 px-2.5">
										{showPassword ? <EyeOff className="size-3.5" /> : <Eye className="size-3.5" />}
									</Button>
									<Button
										variant="default"
										size="sm"
										type="button"
										disabled={isSavingPassword}
										onClick={handleSavePassword}
										className="shrink-0 h-9 px-3 bg-primary hover:bg-primary/90 text-primary-foreground font-semibold text-xs gap-1.5">
										{isSavingPassword ? <RefreshCw className="size-3.5 animate-spin" /> : <Check className="size-3.5" />}
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
										className="shrink-0 h-9 px-2 text-xs">
										<X className="size-3.5" />
									</Button>
								</>
							) : (
								<>
									<Input
										readOnly
										type={showPassword ? 'text' : 'password'}
										value={currentPassword || ''}
										className="font-mono text-xs bg-muted/30 h-9"
									/>
									<Button
										variant="outline"
										size="sm"
										type="button"
										onClick={() => setShowPassword(!showPassword)}
										title="Toggle Password Visibility"
										className="shrink-0 h-9 px-2.5">
										{showPassword ? <EyeOff className="size-3.5" /> : <Eye className="size-3.5" />}
									</Button>
									<Button
										variant="outline"
										size="sm"
										type="button"
										onClick={() => setIsEditingPassword(true)}
										title="Change Password"
										className="shrink-0 h-9 px-2.5">
										<Pencil className="size-3.5 text-muted-foreground" />
									</Button>
								</>
							)}
						</div>
					</div>

					{/* Internal Port (Container) */}
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-muted-foreground">Internal Port (Container)</label>
						<Input readOnly value={internalPort.toString()} className="font-mono text-xs bg-muted/30 h-9" />
					</div>

					{/* Internal Host */}
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-muted-foreground">Internal Host</label>
						<Input readOnly value={host} className="font-mono text-xs bg-muted/30 h-9" />
					</div>

					{/* Internal Connection URL */}
					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-muted-foreground">Internal Connection URL</label>
						<div className="flex items-center gap-2">
							<Input
								readOnly
								type={showInternalUrl ? 'text' : 'password'}
								value={internalConnStr}
								className="font-mono text-xs bg-muted/30 h-9"
							/>
							<Button
								variant="outline"
								size="sm"
								type="button"
								onClick={() => setShowInternalUrl(!showInternalUrl)}
								className="shrink-0 h-9 px-2.5">
								{showInternalUrl ? <EyeOff className="size-3.5" /> : <Eye className="size-3.5" />}
							</Button>
						</div>
					</div>
				</div>
			</div>

			{/* External Credentials Card (Dokploy Exact Style - Full Width Inputs, Bottom Rightmost Save Button) */}
			<div className="bg-card border border-border/80 rounded-2xl p-6 shadow-xs flex flex-col gap-5">
				<div>
					<h3 className="text-xl font-bold text-foreground tracking-tight">
						External Credentials
					</h3>
					<p className="text-xs text-muted-foreground mt-1 leading-relaxed">
						In order to make the database reachable through the internet, you must set a port and ensure that the port is not being used by another application or database
					</p>
				</div>

				<div className="flex flex-col gap-5 border-t border-border/40 pt-4">
					<div className="flex flex-col gap-1.5 w-full">
						<label className="text-xs font-semibold text-muted-foreground">External Port (Internet)</label>
						<Input
							type="number"
							value={extPortInput}
							onChange={(e) => setExtPortInput(e.target.value)}
							placeholder={internalPort.toString()}
							className="font-mono text-xs bg-background h-9 border-border w-full"
						/>
					</div>

					{externalPort ? (
						<div className="flex flex-col gap-4 border-t border-border/30 pt-4">
							<div className="flex flex-col gap-1.5 w-full">
								<label className="text-xs font-semibold text-muted-foreground">External Host</label>
								<Input readOnly value={serverIp} className="font-mono text-xs bg-muted/30 h-9 w-full" />
							</div>
							<div className="flex flex-col gap-1.5 w-full">
								<label className="text-xs font-semibold text-muted-foreground">External Connection URL</label>
								<div className="flex items-center gap-2 w-full">
									<Input
										readOnly
										type={showExternalUrl ? 'text' : 'password'}
										value={externalConnStr}
										className="font-mono text-xs bg-muted/30 h-9 w-full"
									/>
									<Button
										variant="outline"
										size="sm"
										type="button"
										onClick={() => setShowExternalUrl(!showExternalUrl)}
										className="shrink-0 h-9 px-2.5">
										{showExternalUrl ? <EyeOff className="size-3.5" /> : <Eye className="size-3.5" />}
									</Button>
								</div>
							</div>
						</div>
					) : null}

					{/* Save button at bottom rightmost */}
					<div className="flex justify-end pt-2 border-t border-border/30">
						<Button
							variant="default"
							size="sm"
							disabled={isSavingPort}
							onClick={handleSavePort}
							className="h-9 px-6 bg-primary hover:bg-primary/90 text-primary-foreground font-semibold text-xs gap-1.5 cursor-pointer">
							{isSavingPort ? <RefreshCw className="size-3.5 animate-spin" /> : <Check className="size-3.5" />}
							Save
						</Button>
					</div>
				</div>
			</div>

			{/* Interactive Container Terminal Modal */}
			{isTerminalOpen && (
				<TerminalModal
					app={database}
					open={isTerminalOpen}
					onClose={() => setIsTerminalOpen(false)}
				/>
			)}

			{/* Confirmation Modal */}
			{confirmAction && (
				<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
					<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm p-5 flex flex-col gap-4 animate-in fade-in zoom-in-95 duration-150">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<h3 className="text-sm font-bold text-foreground">{getActionTitle(confirmAction)}</h3>
							<Button variant="ghost" size="sm" onClick={() => setConfirmAction(null)} className="h-7 w-7 p-0 text-muted-foreground">
								<X className="w-4 h-4" />
							</Button>
						</div>
						<p className="text-xs text-muted-foreground leading-relaxed">{getActionDesc(confirmAction)}</p>
						<div className="flex justify-end gap-2 border-t border-border/60 pt-3">
							<Button variant="outline" size="sm" onClick={() => setConfirmAction(null)} className="h-8 text-xs font-semibold">
								Cancel
							</Button>
							<Button
								size="sm"
								variant={confirmAction === 'stop' ? 'destructive' : 'default'}
								onClick={() => executeActionClick(confirmAction)}
								className="h-8 text-xs font-semibold"
							>
								Confirm
							</Button>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}

