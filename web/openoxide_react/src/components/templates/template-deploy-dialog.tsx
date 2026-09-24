import {useEffect, useState} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {toast} from 'sonner';
import {Rocket} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Field} from '#/components/projects/create/field';
import {
	parseTemplateEnv,
	templateFileUrl,
	useActiveOrgId,
	useComposeInstallTemplate,
	useEnvironmentListByProject,
	useProjectListByOrganization,
	useRemoteServerList,
	type TemplateMeta,
} from '#/api/live';
import {formatApiError} from '#/api/utils';

/** One-click deploy: picks project → environment → server, then installs the template's compose file with env vars seeded from its [variables] table. */
export function TemplateDeployDialog({
	template,
	open,
	onOpenChange,
}: {
	template: TemplateMeta | null;
	open: boolean;
	onOpenChange: (open: boolean) => void;
}) {
	const navigate = useNavigate();
	const orgId = useActiveOrgId();

	const [projectId, setProjectId] = useState<string>('');
	const [environmentId, setEnvironmentId] = useState<string>('');
	const [serverId, setServerId] = useState<string>('local');
	const [name, setName] = useState('');
	const [submitting, setSubmitting] = useState(false);

	const {data: rawProjects} = useProjectListByOrganization(orgId);
	const projects = rawProjects ?? [];
	const {data: rawEnvs} = useEnvironmentListByProject(
		BigInt(projectId || 0),
	);
	const envs = rawEnvs ?? [];
	const {data: rawServers} = useRemoteServerList();
	const servers = rawServers ?? [];
	const install = useComposeInstallTemplate();

	useEffect(() => {
		if (!open || !template) return;
		setName(template.name.toLowerCase().replace(/\s+/g, '-'));
		if (!projectId && projects[0]) setProjectId(String(projects[0].id));
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [open, template]);

	useEffect(() => {
		if (envs.length && !envs.some(e => String(e.id) === environmentId)) {
			setEnvironmentId(String(envs[0].id));
		}
	}, [envs, environmentId]);

	const handleSubmit = async () => {
		if (!template || !projectId || !environmentId) return;
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
				env_var: parseTemplateEnv(toml),
			});
			toast.success(`${template.name} installed`);
			onOpenChange(false);
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
		<Sheet open={open} onOpenChange={onOpenChange}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>Deploy {template?.name}</SheetTitle>
					<SheetDescription className="text-xs">
						Installs the template as a compose stack. Environment
						variables from the template are pre-filled; adjust them
						on the compose page before deploying.
					</SheetDescription>
				</SheetHeader>

				<div className="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-6 py-6">
					<Field label="Project">
						<Select
							value={projectId}
							onValueChange={value => {
								if (!value) return;
								setProjectId(value);
								setEnvironmentId('');
							}}>
							<SelectTrigger className="w-full bg-background">
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

					<Field label="Environment">
						<Select
							value={environmentId}
							onValueChange={value => {
								if (value) setEnvironmentId(value);
							}}>
							<SelectTrigger className="w-full bg-background">
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

					<Field label="Server">
						<Select
							value={serverId}
							onValueChange={value => {
								if (value) setServerId(value);
							}}>
							<SelectTrigger className="w-full bg-background">
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
										{server.name} · {server.ip_address}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</Field>

					<Field label="Service name">
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder={template?.id}
							className="bg-background"
						/>
					</Field>
				</div>

				<SheetFooter>
					<Button
						onClick={handleSubmit}
						disabled={submitting || !projectId || !environmentId}
						className="h-9 gap-1.5">
						<Rocket className="size-3.5" />
						{submitting ? 'Installing…' : 'Install'}
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
