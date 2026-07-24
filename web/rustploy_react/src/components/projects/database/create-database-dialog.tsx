import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {Database} from 'lucide-react';
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
import {$api} from '#/api/query';
import {formatApiError, cn} from '#/api/utils';

interface CreateDatabaseDialogProps {
	isOpen: boolean;
	onClose: () => void;
	environmentId: number;
	servers: any[];
	onCreated: (db: any) => void;
}

const DB_KINDS = [
	{ id: 'postgres',  label: 'PostgreSQL',  defaultImage: 'postgres:17', color: 'text-blue-400' },
	{ id: 'mysql',     label: 'MySQL',        defaultImage: 'mysql:8',    color: 'text-orange-400' },
	{ id: 'mariadb',   label: 'MariaDB',      defaultImage: 'mariadb:11', color: 'text-orange-400' },
	{ id: 'mongo',     label: 'MongoDB',      defaultImage: 'mongo:7',    color: 'text-green-500' },
	{ id: 'redis',     label: 'Redis',        defaultImage: 'redis:7',    color: 'text-red-400' },
	{ id: 'libsql',    label: 'LibSQL/Turso', defaultImage: 'ghcr.io/tursodatabase/libsql-server:latest', color: 'text-purple-400' },
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
			setServerId(servers[0]?.id ? String(servers[0].id) : '');
		}
	}, [isOpen, servers]);

	const selectKind = (kind: string) => {
		setSelectedKind(kind);
		const k = DB_KINDS.find(d => d.id === kind);
		if (k) setDockerImage(k.defaultImage);
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim() || !serverId) {
			toast.error('Please fill in database name and target server');
			return;
		}

		setIsSubmitting(true);
		try {
			const body: any = {
				name: name.trim(),
				description: description.trim() || undefined,
				environment_id: environmentId,
				docker_image: dockerImage || undefined,
				external_port: externalPort ? Number(externalPort) : undefined,
				server_id: Number(serverId),
			};

			if (selectedKind !== 'redis' && selectedKind !== 'libsql') {
				body.database_name = dbName.trim() || undefined;
				body.database_user = dbUser.trim() || undefined;
				body.database_password = dbPassword || undefined;
			}

			let res: any;
			switch (selectedKind) {
				case 'postgres': res = await createPostgres.mutateAsync({ body }); break;
				case 'mysql': res = await createMysql.mutateAsync({ body }); break;
				case 'mariadb': res = await createMariadb.mutateAsync({ body }); break;
				case 'mongo': res = await createMongo.mutateAsync({ body }); break;
				case 'redis': res = await createRedis.mutateAsync({ body }); break;
				case 'libsql': res = await createLibsql.mutateAsync({ body }); break;
			}

			toast.success('Database created successfully');
			onCreated({...res, kind: selectedKind});
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
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
					<div className="space-y-1.5">
						<label className="text-xs font-semibold text-foreground">Database Engine *</label>
						<div className="grid grid-cols-3 gap-2">
							{DB_KINDS.map(k => (
								<button
									key={k.id}
									type="button"
									onClick={() => selectKind(k.id)}
									className={cn(
										'flex items-center gap-2 px-3 py-2 rounded-lg border text-xs font-semibold transition-all',
										selectedKind === k.id
											? 'border-primary bg-primary/10 text-primary'
											: 'border-border text-muted-foreground hover:border-foreground/30 hover:text-foreground'
									)}>
									<Database className={cn('size-3.5', k.color)} />
									{k.label}
								</button>
							))}
						</div>
					</div>

					<div className="grid grid-cols-2 gap-4">
						<div className="space-y-1">
							<label className="text-xs font-semibold text-foreground">Service Name *</label>
							<Input
								placeholder="e.g. prod-db"
								value={name}
								onChange={e => setName(e.target.value)}
								required
								className="h-9"
							/>
						</div>
						<div className="space-y-1">
							<label className="text-xs font-semibold text-foreground">Description</label>
							<Input
								placeholder="Brief description"
								value={description}
								onChange={e => setDescription(e.target.value)}
								className="h-9"
							/>
						</div>
					</div>

					<div className="grid grid-cols-2 gap-4">
						<div className="space-y-1">
							<label className="text-xs font-semibold text-foreground">Docker Image</label>
							<Input
								value={dockerImage}
								onChange={e => setDockerImage(e.target.value)}
								className="h-9 font-mono text-xs"
							/>
						</div>
						<div className="space-y-1">
							<label className="text-xs font-semibold text-foreground">Target Server *</label>
							<Select value={serverId} onValueChange={val => setServerId(val ?? '')}>
								<SelectTrigger className="h-9"><SelectValue placeholder="Select server" /></SelectTrigger>
								<SelectContent className="bg-card border-border">
									{servers.map(srv => (
										<SelectItem key={srv.id} value={String(srv.id)}>
											{srv.name}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						</div>
					</div>

					{selectedKind !== 'redis' && selectedKind !== 'libsql' && (
						<>
							<div className="grid grid-cols-2 gap-4">
								<div className="space-y-1">
									<label className="text-xs font-semibold text-foreground">Database Name</label>
									<Input
										placeholder={name || 'mydb'}
										value={dbName}
										onChange={e => setDbName(e.target.value)}
										className="h-9"
									/>
								</div>
								<div className="space-y-1">
									<label className="text-xs font-semibold text-foreground">Username</label>
									<Input
										placeholder={name || 'admin'}
										value={dbUser}
										onChange={e => setDbUser(e.target.value)}
										className="h-9"
									/>
								</div>
							</div>
							<div className="space-y-1">
								<label className="text-xs font-semibold text-foreground">Password</label>
								<Input
									type="password"
									placeholder="••••••••"
									value={dbPassword}
									onChange={e => setDbPassword(e.target.value)}
									className="h-9"
								/>
							</div>
						</>
					)}

					<div className="space-y-1">
						<label className="text-xs font-semibold text-foreground">External Port</label>
						<Input
							type="number"
							placeholder="Leave empty to auto-assign"
							value={externalPort}
							onChange={e => setExternalPort(e.target.value)}
							className="h-9"
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
							className="bg-primary hover:bg-primary/95 text-primary-foreground text-xs h-9 px-4">
							{isSubmitting ? 'Creating...' : 'Create Database'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
