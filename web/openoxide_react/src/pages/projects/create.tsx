import {createFileRoute, useSearch} from '@tanstack/react-router';
import {Rocket, FolderOpen} from 'lucide-react';
import {Titlebar} from '#/components/shared/titlebar';
import {TypeCards} from '#/components/projects/create/type-cards';
import {ServiceForm} from '#/components/projects/create/service-form';
import {InfoRail} from '#/components/projects/create/info-rail';
import {useCreateService} from '#/components/projects/create/use-create-service';
import type {ServiceKind} from '#/components/projects/create/constants';

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

function CreateServicePage() {
	const {id} = Route.useParams();
	const projectId = Number(id);
	const {env: envParam, type: typeParam} = useSearch({
		from: '/_app/projects/$id/create',
	});

	const form = useCreateService({projectId, envParam, typeParam});
	const {project, chosen, pick} = form;

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			<Titlebar
				breadcrumbs={[
					{label: 'Projects', to: '/projects', icon: FolderOpen},
					{
						label: project?.name || 'Project',
						to: '/projects/$id',
						params: {id: String(projectId)},
					},
					{label: 'Create Service'},
				]}
				icon={<Rocket className="text-foreground/70" />}
				title="Create Service"
				description="Pick a service type and give it a name — everything else can be configured later."
			/>

			<div className="grid min-w-0 grid-cols-1 gap-6 lg:grid-cols-[1fr_340px]">
				<div className="min-w-0 space-y-4">
					<TypeCards
						chosen={chosen}
						onPick={(kind: ServiceKind) => pick(kind)}
					/>
					<ServiceForm form={form} />
				</div>
				<InfoRail form={form} />
			</div>
		</div>
	);
}
