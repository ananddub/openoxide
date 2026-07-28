import {useState, useEffect, useRef} from 'react';
import {toast} from 'sonner';
import {Server} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
	MongodbIcon,
	RedisIcon,
	LibsqlIcon,
} from '#/components/icons/db-icons';
import {$api} from '#/api/query';
import {formatApiError, cn} from '#/api/utils';

import type {DatabaseResponse, RemoteServerResponse} from '#/types/api-helpers';

interface CreateDatabaseDialogProps {
	isOpen: boolean;
	onClose: () => void;
	environmentId: number;
	servers: RemoteServerResponse[];
	onCreated: (db: DatabaseResponse) => void;
}

const DB_KINDS = [
	{ id: 'postgres', label: 'PostgreSQL', defaultImage: 'postgres:17', Icon: PostgresqlIcon },
	{ id: 'mysql', label: 'MySQL', defaultImage: 'mysql:8', Icon: MysqlIcon },
	{ id: 'mariadb', label: 'MariaDB', defaultImage: 'mariadb:11', Icon: MariadbIcon },
	{ id: 'mongo', label: 'MongoDB', defaultImage: 'mongo:7', Icon: MongodbIcon },
	{ id: 'redis', label: 'Redis', defaultImage: 'redis:7', Icon: RedisIcon },
	{ id: 'libsql', label: 'LibSQL', defaultImage: 'ghcr.io/tursodatabase/libsql-server:latest', Icon: LibsqlIcon },
];

export function CreateDatabaseDialog({
	isOpen,
	onClose,
	environmentId,
	servers,
	onCreated,
}: CreateDatabaseDialogProps) {
	const [selectedKind, setSelectedKind] = useState('postgres');
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [dbName, setDbName] = useState('');
	const [dbUser, setDbUser] = useState('');
	const [dbPassword, setDbPassword] = useState('');
	const [dockerImage, setDockerImage] = useState('postgres:17');
	const [externalPort, setExternalPort] = useState('');
	const [serverId, setServerId] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);
	const submittingRef = useRef(false);

	const availableServers = (servers || []).some(
		s => String(s.name).toLowerCase().includes('local') || String(s.id) === 'local'
	)
		? servers
		: [{ id: 'local', name: 'Local Server', ip_address: 'localhost' }, ...(servers || [])];

	const createPostgres = $api.useMutation('post', '/postgres');
	const createMysql = $api.useMutation('post', '/mysql');
	const createMariadb = $api.useMutation('post', '/mariadb');
	const createMongo = $api.useMutation('post', '/mongo');
	const createRedis = $api.useMutation('post', '/redis');
	const createLibsql = $api.useMutation('post', '/libsql');

	useEffect(() => {
		if (isOpen) {
			setName('');
			setDescription('');
			setDbName('');
			setDbUser('');
			setDbPassword('');
			setExternalPort('');
			setSelectedKind('postgres');
			setDockerImage('postgres:17');
			setServerId(availableServers[0]?.id ? String(availableServers[0].id) : 'local');
		}
	}, [isOpen, servers]);

	const selectKind = (kind: string) => {
		setSelectedKind(kind);
		const k = DB_KINDS.find(d => d.id === kind);
		if (k) setDockerImage(k.defaultImage);
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (submittingRef.current || isSubmitting) return;

		if (!name.trim() || !serverId) {
			toast.error('Please fill in database name and target server');
			return;
		}

		submittingRef.current = true;
		setIsSubmitting(true);
		try {
			const parsedServerId = serverId && serverId !== 'local' && !isNaN(Number(serverId)) ? Number(serverId) : undefined;
			const body: Record<string, unknown> = {
				name: name.trim(),
				description: description.trim() || undefined,
				environment_id: Number(environmentId),
				docker_image: dockerImage || undefined,
				external_port: externalPort && !isNaN(Number(externalPort)) ? Number(externalPort) : undefined,
				server_id: parsedServerId,
			};

			if (selectedKind !== 'libsql') {
				body.database_password = dbPassword || undefined;
			}
			if (selectedKind !== 'redis' && selectedKind !== 'libsql') {
				body.database_name = dbName.trim() || undefined;
				body.database_user = dbUser.trim() || undefined;
			}

			let res: unknown;
			switch (selectedKind) {
				case 'postgres': res = await createPostgres.mutateAsync({ body: body as unknown as {environment_id: number; name: string} }); break;
				case 'mysql': res = await createMysql.mutateAsync({ body: body as unknown as {environment_id: number; name: string} }); break;
				case 'mariadb': res = await createMariadb.mutateAsync({ body: body as unknown as {environment_id: number; name: string} }); break;
				case 'mongo': res = await createMongo.mutateAsync({ body: body as unknown as {environment_id: number; name: string} }); break;
				case 'redis': res = await createRedis.mutateAsync({ body: body as unknown as {environment_id: number; name: string} }); break;
				case 'libsql': res = await createLibsql.mutateAsync({ body: body as unknown as {environment_id: number; name: string} }); break;
			}

			const resObj = res as Record<string, unknown>;
			if (resObj?.error) {
				toast.error(formatApiError(resObj.error));
				return;
			}

			const rawData = resObj?.data || res;
			const dbObj = typeof rawData === 'object' && rawData !== null ? (rawData as Record<string, unknown>) : {};
			const dbId = (dbObj.id as number | string) || (dbObj.database_id as number | string) || (resObj?.id as number | string);
			const finalDb = { ...dbObj, id: dbId, kind: selectedKind } as unknown as DatabaseResponse;

			toast.success('Database created successfully');
			onClose();
			onCreated(finalDb);
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			submittingRef.current = false;
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-lg bg-card border-border max-h-[90vh] overflow-y-auto">
				<DialogHeader>
					<DialogTitle className="text-base font-bold">Create Managed Database</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Deploy containerized high-performance databases directly on your servers.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="space-y-4 mt-2">
					{/* Database Engine Cards Grid */}
					<div className="space-y-2">
						<label className="text-xs font-semibold text-foreground">Select a Database Engine *</label>
						<div className="grid grid-cols-3 gap-2.5">
							{DB_KINDS.map(k => {
								const IconComponent = k.Icon;
								const isSelected = selectedKind === k.id;
								return (
									<button
										key={k.id}
										type="button"
										onClick={() => selectKind(k.id)}
										className={cn(
											'flex flex-col items-center justify-center p-2.5 h-20 rounded-xl border-2 transition-all cursor-pointer gap-1.5 hover:bg-accent/40',
											isSelected
												? 'border-primary bg-primary/5 text-foreground shadow-xs ring-1 ring-primary/20'
												: 'border-border/70 bg-popover/40 text-muted-foreground hover:text-foreground hover:border-border'
										)}>
										<IconComponent className="size-7 shrink-0" />
										<span className="text-[11px] font-semibold tracking-wide">{k.label}</span>
									</button>
								);
							})}
						</div>
					</div>

					{/* Service Name (Full Width) */}
					<div className="space-y-1">
						<label className="text-xs font-semibold text-foreground">Service Name *</label>
						<Input
							placeholder="e.g. prod-db"
							value={name}
							onChange={e => setName(e.target.value)}
							required
							className="h-9 w-full"
						/>
					</div>

					{/* Target Server (Full Width) */}
					<div className="space-y-1">
						<label className="text-xs font-semibold text-foreground flex items-center gap-1.5">
							<Server className="w-3.5 h-3.5 text-muted-foreground" /> Target Server *
						</label>
						<Select value={serverId} onValueChange={val => setServerId(val ?? '')}>
							<SelectTrigger className="h-9 w-full"><SelectValue placeholder="Select server" /></SelectTrigger>
							<SelectContent className="bg-card border-border">
								{availableServers.map(srv => (
									<SelectItem key={srv.id} value={String(srv.id)}>
										{srv.name} {srv.ip_address ? `(${srv.ip_address})` : ''}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</div>

					{/* Description (Full Width) */}
					<div className="space-y-1">
						<label className="text-xs font-semibold text-foreground">Description</label>
						<textarea
							rows={3}
							placeholder="Brief description..."
							value={description}
							onChange={e => setDescription(e.target.value)}
							className="flex w-full rounded-lg border border-input bg-transparent dark:bg-input/30 px-3 py-2 text-xs text-foreground placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 outline-none resize-none leading-relaxed"
						/>
					</div>

					{/* Database Credentials (if applicable) */}
					{selectedKind !== 'redis' && selectedKind !== 'libsql' && (
						<>
							<div className="space-y-1">
								<label className="text-xs font-semibold text-foreground">Database Name</label>
								<Input
									placeholder={name || 'mydb'}
									value={dbName}
									onChange={e => setDbName(e.target.value)}
									className="h-9 w-full"
								/>
							</div>
							<div className="space-y-1">
								<label className="text-xs font-semibold text-foreground">Username</label>
								<Input
									placeholder={name || 'admin'}
									value={dbUser}
									onChange={e => setDbUser(e.target.value)}
									className="h-9 w-full"
								/>
							</div>
							<div className="space-y-1">
								<label className="text-xs font-semibold text-foreground">Password</label>
								<Input
									type="password"
									placeholder="••••••••"
									value={dbPassword}
									onChange={e => setDbPassword(e.target.value)}
									className="h-9 w-full"
								/>
							</div>
						</>
					)}

					{/* Docker Image (Full Width) */}
					<div className="space-y-1">
						<label className="text-xs font-semibold text-foreground">Docker Image</label>
						<Input
							value={dockerImage}
							onChange={e => setDockerImage(e.target.value)}
							className="h-9 font-mono text-xs w-full"
						/>
					</div>

					{/* External Port (Full Width) */}
					<div className="space-y-1">
						<label className="text-xs font-semibold text-foreground">External Port</label>
						<Input
							type="number"
							placeholder="Leave empty to auto-assign"
							value={externalPort}
							onChange={e => setExternalPort(e.target.value)}
							className="h-9 w-full"
						/>
					</div>

					<div className="flex justify-end gap-3 pt-3 border-t border-border/20">
						<Button
							type="button"
							variant="outline"
							onClick={onClose}
							disabled={isSubmitting}
							className="h-9 text-xs">
							Cancel
						</Button>
						<Button
							type="submit"
							disabled={isSubmitting}
							className="bg-primary hover:bg-primary/95 text-primary-foreground text-xs h-9 px-4 font-semibold shadow-xs">
							{isSubmitting ? 'Creating...' : 'Create Database'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
