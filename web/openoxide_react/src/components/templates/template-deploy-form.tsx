import {useEffect, useState} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {toast} from 'sonner';
import {ChevronLeft, Loader2, Rocket} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Field} from '#/components/shared/field';
import {TemplateLogo} from './template-logo';
import {
	templateFileUrl,
	useActiveOrgId,
	useComposeInstallTemplate,
	useEnvironmentListByProject,
	useProjectListByOrganization,
	useRemoteServerList,
	type TemplateMeta,
} from '#/api/live';
import {formatApiError, isLocalServerRow} from '#/api/utils';

/**
 * One-click deploy: picks project → environment → server, then installs the
 * template's compose file with env vars seeded from its [variables] table.
 * Rendered inline as the third view of the panel that showed the template —
 * no second sheet stacks on top. When the panel was opened from inside a
 * project (create page / dashboard), `projectId`/`environmentId` are pinned
 * and those selectors are not shown again.
 */
export function TemplateDeployForm({
	template,
	onCancel,
	projectId: pinnedProjectId,
	environmentId: pinnedEnvironmentId,
}: {
	template: TemplateMeta;
	onCancel: () => void;
	projectId?: number | string;
	environmentId?: number | string;
}) {
	const navigate = useNavigate();
	const orgId = useActiveOrgId();

	const [projectId, setProjectId] = useState(() =>
		pinnedProjectId != null ? String(pinnedProjectId) : '',
	);
	const [environmentId, setEnvironmentId] = useState(() =>
		pinnedEnvironmentId != null ? String(pinnedEnvironmentId) : '',
	);
	const [serverId, setServerId] = useState<string>('local');
	const [name, setName] = useState(() =>
		template.name.toLowerCase().replace(/\s+/g, '-'),
	);
	const [submitting, setSubmitting] = useState(false);

	const {data: rawProjects} = useProjectListByOrganization(orgId);
	const projects = rawProjects ?? [];
	const {data: rawEnvs} = useEnvironmentListByProject(
		BigInt(projectId || 0),
	);
	const envs = rawEnvs ?? [];
	const {data: rawServers} = useRemoteServerList();
	// The panel host has its own row in `servers` too; the hardcoded "Local"
	// option already covers it, so drop it here to avoid listing it twice.
	const servers = (rawServers ?? []).filter(
		server => !isLocalServerRow(server.ip_address),
	);
	const install = useComposeInstallTemplate();

	const pinnedProject =
		pinnedProjectId != null
			? projects.find(p => String(p.id) === String(pinnedProjectId))
			: undefined;
	const pinnedEnv =
		pinnedEnvironmentId != null
			? envs.find(e => String(e.id) === String(pinnedEnvironmentId))
			: undefined;

	useEffect(() => {
		if (pinnedProjectId) return;
		if (!projectId && projects[0]) setProjectId(String(projects[0].id));
	}, [projects, projectId, pinnedProjectId]);

	useEffect(() => {
		if (pinnedEnvironmentId != null) return;
		if (envs.length && !envs.some(e => String(e.id) === environmentId)) {
			setEnvironmentId(String(envs[0].id));
		}
	}, [envs, environmentId, pinnedEnvironmentId]);

	const handleSubmit = async () => {
		if (!projectId || !environmentId) return;
		setSubmitting(true);
		try {
			const [composeResponse, tomlResponse] = await Promise.all([
				fetch(templateFileUrl(template.id, 'docker-compose.yml')),
				fetch(templateFileUrl(template.id, 'template.toml')),
			]);
			if (!composeResponse.ok) {
				throw new Error(
					`Could not download compose file (${composeResponse.status})`,
				);
			}
			const composeFile = await composeResponse.text();
			const toml = tomlResponse.ok ? await tomlResponse.text() : undefined;
			const response = await install.mutateAsync({
				template: composeFile,
				environment_id: BigInt(environmentId),
				name: name.trim() || undefined,
				server_id: serverId === 'local' ? undefined : Number(serverId),
				// Send the raw template.toml: the backend resolves both the
				// [variables] generators (${password:N}, ${domain}) and the
				// [config].env entries that reference them (BB_MINIO_ACCESS_KEY
				// etc.), so `${...}` interpolation in the compose file gets real
				// values instead of blanks.
				env_var: toml,
			});
			toast.success(`${template.name} installed`);
			const composeId = (response as {id?: bigint})?.id;
			if (composeId) {
				navigate({
					to: `/projects/${projectId}/compose/${composeId}` as never,
				});
			} else {
				navigate({
					to: '/projects/$id',
					params: {id: String(projectId)},
				});
			}
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<div
			key={template.id}
			className="flex min-h-0 w-full flex-1 animate-in flex-col gap-5 duration-200 ease-out fade-in-0 slide-in-from-right-4">
			<div className="flex shrink-0 items-center gap-3">
				<TemplateLogo template={template} className="size-9 shrink-0" />
				<div className="min-w-0">
					<div className="flex items-center gap-2">
						<p className="truncate text-sm font-semibold text-foreground">
							{template.name}
						</p>
						<Badge
							variant="outline"
							className="shrink-0 font-mono text-[10px] font-normal">
							{template.version}
						</Badge>
					</div>
					{(pinnedProject || pinnedEnv) && (
						<p className="mt-0.5 truncate text-xs text-muted-foreground">
							Installs into{' '}
							<span className="font-medium text-foreground">
								{pinnedProject?.name ?? 'this project'}
							</span>
							{pinnedEnv && (
								<>
									{' · '}
									<span className="font-medium text-foreground">
										{pinnedEnv.name}
									</span>
								</>
							)}
						</p>
					)}
				</div>
			</div>

			<div className="flex min-h-0 flex-col gap-5 overflow-y-auto pt-1">
				{!pinnedProjectId && (
					<Field
						label="Project"
						hint="The project that will own this stack.">
						<Select
							value={projectId}
							onValueChange={value => {
								if (!value) return;
								setProjectId(value);
								setEnvironmentId('');
							}}>
							<SelectTrigger className="w-full">
								<SelectValue placeholder="Select a project" />
							</SelectTrigger>
							<SelectContent>
								{projects.map(project => (
									<SelectItem
										key={String(project.id)}
										value={String(project.id)}
										className="text-sm">
										{project.name}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</Field>
				)}

				{!pinnedEnvironmentId && (
					<Field
						label="Environment"
						hint="Deploy settings and secrets live per environment.">
						<Select
							value={environmentId}
							onValueChange={value => {
								if (value) setEnvironmentId(value);
							}}>
							<SelectTrigger className="w-full">
								<SelectValue placeholder="Select an environment" />
							</SelectTrigger>
							<SelectContent>
								{envs.map(environment => (
									<SelectItem
										key={String(environment.id)}
										value={String(environment.id)}
										className="text-sm">
										{environment.name}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</Field>
				)}

				<Field
					label="Server"
					hint="The panel's own host or a connected remote.">
					<Select
						value={serverId}
						onValueChange={value => {
							if (value) setServerId(value);
						}}>
						<SelectTrigger className="w-full">
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value="local" className="text-sm">
								Local (this server)
							</SelectItem>
							{servers.map(server => (
								<SelectItem
									key={String(server.id)}
									value={String(server.id)}
									className="text-sm">
									{server.name} ({server.ip_address})
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</Field>

				<Field
					label="Service name"
					hint="Compose project name. Keep it lowercase.">
					<Input
						value={name}
						onChange={e => setName(e.target.value)}
						placeholder={template.id}
						className="font-mono"
					/>
				</Field>
			</div>

			<div className="flex shrink-0 flex-col gap-2 pt-2">
				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={onCancel}
						disabled={submitting}
						className="gap-1.5">
						<ChevronLeft className="size-3.5" />
						Back
					</Button>
					<Button
						onClick={handleSubmit}
						disabled={submitting || !projectId || !environmentId}
						className="h-8 flex-1 gap-2 text-sm">
						{submitting ? (
							<Loader2 className="size-4 animate-spin" />
						) : (
							<Rocket className="size-4" />
						)}
						{submitting ? 'Installing…' : `Install ${template.name}`}
					</Button>
				</div>
				{submitting && (
					<p className="text-center text-[11px] text-muted-foreground">
						Fetching the compose file and generating template secrets…
					</p>
				)}
			</div>
		</div>
	);
}
