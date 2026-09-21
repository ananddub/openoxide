import {useState} from 'react';
import {
	createFileRoute,
	useNavigate,
	useSearch,
} from '@tanstack/react-router';
import {
	ArrowLeft,
	Box,
	Layers2,
	Database,
	LayoutTemplate,
	Save,
	Check,
	Rocket,
	ShieldCheck,
} from 'lucide-react';
import {toast} from 'sonner';
import {
	useProjectGet,
	useEnvironmentListByProject,
	useRemoteServerList,
	useApplicationCreate,
	useComposeCreate,
	useDatabaseCreate,
	parseDatabaseKind,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {Badge} from '#/components/ui/badge';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {cn} from '#/api/utils';

type ServiceKind = 'application' | 'compose' | 'database' | 'template';

export const Route = createFileRoute('/_app/projects/$id/create')({
	component: CreateServicePage,
	validateSearch: (
		search: Record<string, unknown>,
	): {env?: number; type?: string} => ({
		env:
			typeof search.env === 'string' || typeof search.env === 'number'
				? Number(search.env)
				: undefined,
		type:
			search.type === 'application' ||
			search.type === 'compose' ||
			search.type === 'database'
				? search.type
				: undefined,
	}),
});

const OPTIONS: {
	id: ServiceKind;
	title: string;
	description: string;
	icon: typeof Box;
	soon?: boolean;
}[] = [
	{
		id: 'application',
		title: 'Application',
		description: 'Git repo, Dockerfile, Nixpacks ya static build.',
		icon: Box,
	},
	{
		id: 'compose',
		title: 'Compose Stack',
		description: 'Multi-container app from docker-compose.yml.',
		icon: Layers2,
	},
	{
		id: 'database',
		title: 'Database',
		description: 'PostgreSQL, MySQL, MariaDB, MongoDB, Redis.',
		icon: Database,
	},
	{
		id: 'template',
		title: 'Template',
		description: 'Ready-made app blueprints — one-click deploy.',
		icon: LayoutTemplate,
		soon: true,
	},
];

const DB_KINDS = [
	{id: 'postgres', label: 'PostgreSQL', defaultImage: 'postgres:17'},
	{id: 'mysql', label: 'MySQL', defaultImage: 'mysql:8'},
	{id: 'mariadb', label: 'MariaDB', defaultImage: 'mariadb:11'},
	{id: 'mongodb', label: 'MongoDB', defaultImage: 'mongo:7'},
	{id: 'redis', label: 'Redis', defaultImage: 'redis:7'},
	{id: 'libsql', label: 'LibSQL', defaultImage: 'ghcr.io/turso-labs/libsql:latest'},
];

const TYPE_INFO: Record<ServiceKind, {heading: string; steps: string[]}> = {
	application: {
		heading: 'Before you deploy an app',
		steps: [
			'Connect a Git provider or keep the repo public for auto-deploy.',
			'Add environment variables later in the service Environment tab.',
			'After creation, attach a domain from the Domains tab.',
		],
	},
	compose: {
		heading: 'About Compose Stacks',
		steps: [
			'Paste a docker-compose file or point at a repo path.',
			'Each service in the file gets its own container and logs.',
			'Swarm mode runs the stack across your cluster nodes.',
		],
	},
	database: {
		heading: 'About Databases',
		steps: [
			'Credentials are generated inside the service env file.',
			'Databases are private by default — publish a port only if needed.',
			'Schedule backups from the Backups tab after creation.',
		],
	},
	template: {
		heading: 'Templates',
		steps: [
			'Template gallery is coming soon.',
			'You can deploy the same apps today via Compose Stack.',
		],
	},
};

function Field({
	label,
	children,
}: {
	label: string;
	children: React.ReactNode;
}) {
	return (
		<div className="flex flex-col gap-1.5">
			<span className="text-xs font-bold text-muted-foreground">{label}</span>
			{children}
		</div>
	);
}

function CreateServicePage() {
	const {id} = Route.useParams();
	const projectId = Number(id);
	const navigate = useNavigate();
	const {env: envParam, type: typeParam} = useSearch({
		from: '/_app/projects/$id/create',
	});

	const [chosen, setChosen] = useState<ServiceKind>(
		(typeParam as ServiceKind) ?? 'application',
	);
	const [submitting, setSubmitting] = useState(false);

	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [serverId, setServerId] = useState('local');
	const [composeType, setComposeType] = useState('DOCKER-COMPOSE');
	const [dbKind, setDbKind] = useState('postgres');
	const [dbName, setDbName] = useState('');
	const [dbUser, setDbUser] = useState('');
	const [dbPassword, setDbPassword] = useState('');
	const [dockerImage, setDockerImage] = useState('postgres:17');

	const {data: project} = useProjectGet(BigInt(projectId));
	const {data: rawEnvs} = useEnvironmentListByProject(BigInt(projectId));
	const envs = rawEnvs ?? [];
	const {data: rawServers} = useRemoteServerList();
	const servers = rawServers ?? [];

	const selectedEnvId =
		envParam !== undefined
			? Number(envParam)
			: envs[0]
				? Number(envs[0].id)
				: null;
	const selectedEnv = envs.find(e => Number(e.id) === selectedEnvId);

	const createApplication = useApplicationCreate();
	const createCompose = useComposeCreate();
	const createDatabase = useDatabaseCreate();

	const goDetail = (newId?: number | bigint) => {
		if (!newId) {
			navigate({to: '/projects/$id', params: {id: String(projectId)}});
			return;
		}
		if (chosen === 'application')
			navigate({to: `/projects/${projectId}/app/${newId}` as any});
		else if (chosen === 'compose')
			navigate({to: `/projects/${projectId}/compose/${newId}` as any});
		else
			navigate({
				to: '/projects/$id/database/$dbId',
				params: {id: String(projectId), dbId: String(newId)},
				search: {kind: dbKind},
			});
	};

	const pick = (kind: ServiceKind) => {
		if (kind === 'template') {
			toast.info('Template gallery is coming soon — try Compose Stack meanwhile.');
			return;
		}
		setChosen(kind);
		setName('');
		setDescription('');
		if (kind === 'database') {
			setDockerImage(
				DB_KINDS.find(k => k.id === dbKind)?.defaultImage || 'postgres:17',
			);
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim() || !selectedEnvId) return;
		setSubmitting(true);
		try {
			if (chosen === 'application') {
				const res = await createApplication.mutateAsync({
					name: name.trim(),
					description: description.trim() || undefined,
					environment_id: selectedEnvId,
					server_id: serverId === 'local' ? undefined : Number(serverId),
				});
				const obj = ((res as any)?.data || res) as any;
				toast.success('Application created');
				goDetail(obj?.id);
			} else if (chosen === 'compose') {
				const res = await createCompose.mutateAsync({
					name: name.trim(),
					compose_type: composeType,
					description: description.trim() || undefined,
					environment_id: selectedEnvId,
					server_id: serverId === 'local' ? undefined : Number(serverId),
				});
				const obj = ((res as any)?.data || res) as any;
				toast.success('Compose stack created');
				goDetail(obj?.id);
			} else {
				const res = await createDatabase.mutateAsync({
					kind: parseDatabaseKind(dbKind),
					name: name.trim(),
					description: description.trim() || undefined,
					environment_id: selectedEnvId,
					docker_image: dockerImage || undefined,
					server_id:
						serverId && serverId !== 'local'
							? Number(serverId)
							: undefined,
					database_password:
						dbKind !== 'libsql' ? dbPassword || undefined : undefined,
					database_name:
						dbKind !== 'redis' && dbKind !== 'libsql'
							? dbName.trim() || undefined
							: undefined,
					database_user:
						dbKind !== 'redis' && dbKind !== 'libsql'
							? dbUser.trim() || undefined
							: undefined,
				});
				const obj = ((res as any)?.data || res) as any;
				toast.success('Database created');
				goDetail(obj?.id);
			}
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	const option = OPTIONS.find(o => o.id === chosen);
	const isDatabase = chosen === 'database';
	const info = TYPE_INFO[chosen];

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* Header — same quiet style as the rest of the app */}
			<div className="flex flex-col gap-1">
				<button
					type="button"
					onClick={() =>
						navigate({to: '/projects/$id', params: {id: String(projectId)}})
					}
					className="mb-2 flex w-fit items-center gap-1.5 text-xs font-medium text-muted-foreground transition-colors hover:text-foreground">
					<ArrowLeft className="size-3.5" />
					Back to {project?.name || 'project'}
				</button>
				<h1 className="text-2xl font-medium tracking-tight text-foreground/80">
					Create Service
				</h1>
				<p className="text-sm text-muted-foreground/70">
					Pick a service type and give it a name — everything else can be
					configured later.
				</p>
			</div>

			{/* Left: form · Right: rail */}
			<div className="grid min-w-0 grid-cols-1 gap-6 lg:grid-cols-[1fr_340px]">
				<div className="min-w-0 space-y-4">
					{/* Type cards — 2x2 (4 options) */}
					<div className="grid grid-cols-2 gap-3 xl:grid-cols-4">
						{OPTIONS.map(o => {
							const isActive = o.id === chosen;
							return (
								<button
									key={o.id}
									type="button"
									onClick={() => pick(o.id)}
									className={cn(
										'group relative flex flex-col items-start gap-2 rounded-2xl border bg-card p-4 text-start transition-colors',
										isActive
											? 'border-primary/50 bg-primary/[0.04]'
											: 'border-border/50 hover:bg-muted/25',
									)}>
									<span className="flex size-9 items-center justify-center rounded-xl bg-muted/45 transition-colors group-hover:bg-muted/65">
										<o.icon
											className={cn(
												'size-5',
												isActive ? 'text-primary' : 'text-muted-foreground',
											)}
										/>
									</span>
									<span>
										<span className="block text-sm font-semibold text-foreground">
											{o.title}
										</span>
										<span className="mt-0.5 block text-[11px] leading-snug text-muted-foreground">
											{o.description}
										</span>
									</span>
									{o.soon && (
										<Badge
											variant="secondary"
											className="absolute top-3 end-3 px-1.5 py-0 text-[9px] font-semibold">
											SOON
										</Badge>
									)}
									{isActive && !o.soon && (
										<span className="absolute top-3 end-3 flex size-5 items-center justify-center rounded-full bg-primary text-primary-foreground">
											<Check className="size-3" />
										</span>
									)}
								</button>
							);
						})}
					</div>

					{/* Form card */}
					<form
						onSubmit={handleSubmit}
						className="space-y-5 rounded-2xl border border-border/50 bg-card p-6">
						{isDatabase && (
							<Field label="Database Engine">
								<div className="flex flex-wrap gap-2">
									{DB_KINDS.map(k => (
										<button
											key={k.id}
											type="button"
											onClick={() => {
												setDbKind(k.id);
												setDockerImage(k.defaultImage);
											}}
											className={cn(
												'inline-flex h-8 items-center rounded-lg border px-3 text-xs font-medium transition-colors',
												dbKind === k.id
													? 'border-primary/40 bg-primary/10 text-primary'
													: 'border-border/50 text-muted-foreground hover:bg-muted/40 hover:text-foreground',
											)}>
											{k.label}
										</button>
									))}
								</div>
							</Field>
						)}

						<div className="grid grid-cols-1 gap-5 sm:grid-cols-2">
							<Field label="Name">
								<Input
									autoFocus
									placeholder={
										chosen === 'database'
											? 'my-database'
											: chosen === 'compose'
												? 'my-stack'
												: 'my-application'
									}
									value={name}
									onChange={e => setName(e.target.value)}
									className="h-10 rounded-xl border-border/50 bg-background text-sm"
								/>
							</Field>
							<Field label="Server">
								<Select
									value={serverId}
									onValueChange={v => v && setServerId(v)}>
									<SelectTrigger className="!h-10 w-full rounded-xl border-border/50 bg-background text-sm">
										<SelectValue />
									</SelectTrigger>
									<SelectContent>
										<SelectItem value="local" className="text-sm">
											Local (this server)
										</SelectItem>
										{servers.map((s: any) => (
											<SelectItem
												key={String(s.id)}
												value={String(s.id)}
												className="text-sm">
												{s.name} · {s.ip_address}
											</SelectItem>
										))}
									</SelectContent>
								</Select>
							</Field>
						</div>

						<Field label="Description (optional)">
							<Textarea
								rows={2}
								placeholder="What is this service for?"
								value={description}
								onChange={e => setDescription(e.target.value)}
								className="rounded-xl border-border/50 bg-background text-sm"
							/>
						</Field>

						{chosen === 'compose' && (
							<Field label="Compose Type">
								<Select
									value={composeType}
									onValueChange={v => v && setComposeType(v)}>
									<SelectTrigger className="!h-10 w-full max-w-xs rounded-xl border-border/50 bg-background text-sm">
										<SelectValue />
									</SelectTrigger>
									<SelectContent>
										<SelectItem value="DOCKER-COMPOSE" className="text-sm">
											docker-compose
										</SelectItem>
										<SelectItem value="STACK" className="text-sm">
											docker stack (swarm)
										</SelectItem>
									</SelectContent>
								</Select>
							</Field>
						)}

						{isDatabase && (
							<div className="grid grid-cols-1 gap-5 sm:grid-cols-2">
								{dbKind !== 'redis' && dbKind !== 'libsql' && (
									<Field label="Database Name">
										<Input
											placeholder="app_db"
											value={dbName}
											onChange={e => setDbName(e.target.value)}
											className="h-10 rounded-xl border-border/50 bg-background text-sm"
										/>
									</Field>
								)}
								{dbKind !== 'redis' && dbKind !== 'libsql' && (
									<Field label="Database User">
										<Input
											placeholder="admin"
											value={dbUser}
											onChange={e => setDbUser(e.target.value)}
											className="h-10 rounded-xl border-border/50 bg-background text-sm"
										/>
									</Field>
								)}
								{dbKind !== 'libsql' && (
									<Field label="Password">
										<Input
											type="password"
											placeholder="••••••••"
											value={dbPassword}
											onChange={e => setDbPassword(e.target.value)}
											className="h-10 rounded-xl border-border/50 bg-background text-sm"
										/>
									</Field>
								)}
								<Field label="Docker Image">
									<Input
										placeholder={dockerImage}
										value={dockerImage}
										onChange={e => setDockerImage(e.target.value)}
										className="h-10 rounded-xl border-border/50 bg-background font-mono text-sm"
									/>
								</Field>
							</div>
						)}

						<div className="flex items-center justify-between border-t border-border/40 pt-4">
							<p className="text-xs text-muted-foreground">
								Deploying into{' '}
								<span className="font-semibold text-foreground">
									{selectedEnv?.name || 'default'}
								</span>
							</p>
							<Button
								type="submit"
								disabled={submitting || !name.trim() || !selectedEnvId}
								className="flex h-9 items-center gap-1.5 rounded-xl text-xs font-semibold">
								<Save className="size-3.5" />
								{submitting
									? 'Creating…'
									: `Create ${option?.title.toLowerCase() || 'service'}`}
							</Button>
						</div>
					</form>
				</div>

				{/* Right rail */}
				<div className="hidden space-y-4 lg:sticky lg:top-6 lg:block lg:self-start">
					{/* Selected type info */}
					<div className="rounded-2xl border border-border/50 bg-card p-5">
						<div className="mb-3 flex items-center gap-2">
							<Rocket className="size-4 text-primary" />
							<h3 className="text-sm font-semibold text-foreground">
								{info.heading}
							</h3>
						</div>
						<ul className="space-y-2.5">
							{info.steps.map(step => (
								<li
									key={step}
									className="flex items-start gap-2 text-xs leading-relaxed text-muted-foreground">
									<Check className="mt-0.5 size-3.5 shrink-0 text-emerald-500" />
									{step}
								</li>
							))}
						</ul>
					</div>

					{/* Environment context */}
					<div className="rounded-2xl border border-border/50 bg-card p-5">
						<div className="mb-3 flex items-center gap-2">
							<ShieldCheck className="size-4 text-muted-foreground" />
							<h3 className="text-sm font-semibold text-foreground">
								Where it lands
							</h3>
						</div>
						<div className="space-y-2 text-xs text-muted-foreground">
							<p className="flex items-center justify-between">
								<span>Project</span>
								<span className="font-medium text-foreground">
									{project?.name || '—'}
								</span>
							</p>
							<p className="flex items-center justify-between">
								<span>Environment</span>
								<span className="font-medium text-foreground">
									{selectedEnv?.name || 'default'}
								</span>
							</p>
							<p className="flex items-center justify-between">
								<span>Service</span>
								<span className="font-medium text-foreground">
									{option?.title}
								</span>
							</p>
						</div>
					</div>
				</div>
			</div>
		</div>
	);
}
