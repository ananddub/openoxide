import {useState} from 'react';
import {useNavigate} from '@tanstack/react-router';
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
import {type ServiceKind, DB_KINDS} from './constants';

/** State + submit orchestration for the create-service page: picks the service kind, gathers name/server/db fields, and routes to the matching create mutation, then navigates to the new resource's detail page. */
export function useCreateService(args: {
	projectId: number;
	envParam?: number;
	typeParam?: string;
	onCreated?: (newId?: number | bigint) => void;
}) {
	const {projectId, envParam, typeParam, onCreated} = args;
	const navigate = useNavigate();

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
	// Templates open as a gallery side panel on the current page — never a
	// navigation to /templates.
	const [templateGalleryOpen, setTemplateGalleryOpen] = useState(false);

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
		if (chosen === 'application') {
			navigate({to: `/projects/${projectId}/app/${newId}` as never});
		} else if (chosen === 'compose') {
			navigate({to: `/projects/${projectId}/compose/${newId}` as never});
		} else {
			navigate({
				to: '/projects/$id/database/$dbId',
				params: {id: String(projectId), dbId: String(newId)},
				search: {kind: dbKind, tab: undefined},
			});
		}
	};

	const finish = (newId?: number | bigint) => {
		if (onCreated) {
			onCreated(newId);
			return;
		}
		goDetail(newId);
	};

	const pick = (kind: ServiceKind) => {
		if (kind === 'template') {
			setTemplateGalleryOpen(true);
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
				toast.success('Application created');
				finish((((res as any)?.data || res) as any)?.id);
			} else if (chosen === 'compose') {
				const res = await createCompose.mutateAsync({
					name: name.trim(),
					compose_type: composeType,
					description: description.trim() || undefined,
					environment_id: selectedEnvId,
					server_id: serverId === 'local' ? undefined : Number(serverId),
				});
				toast.success('Compose stack created');
				finish((((res as any)?.data || res) as any)?.id);
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
				toast.success('Database created');
				finish((((res as any)?.data || res) as any)?.id);
			}
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return {
		chosen,
		submitting,
		name,
		setName,
		description,
		setDescription,
		serverId,
		setServerId,
		composeType,
		setComposeType,
		dbKind,
		setDbKind,
		dbName,
		setDbName,
		dbUser,
		setDbUser,
		dbPassword,
		setDbPassword,
		dockerImage,
		setDockerImage,
		project,
		servers,
		selectedEnv,
		selectedEnvId,
		pick,
		handleSubmit,
		templateGalleryOpen,
		setTemplateGalleryOpen,
	};
}
