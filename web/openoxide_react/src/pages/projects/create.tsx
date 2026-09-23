import {useState} from 'react';
import {createFileRoute, useSearch} from '@tanstack/react-router';
import {ArrowLeft, FolderOpen} from 'lucide-react';
import {Titlebar} from '#/components/shared/titlebar';
import {Button} from '#/components/ui/button';
import {
	StepBody,
	StepCheckpoints,
	type WizardStep,
} from '#/components/shared/sheet-wizard';
import {KindPicker} from '#/components/projects/create/kind-picker';
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

const STEPS: WizardStep[] = [
	{id: 'type', label: 'Type'},
	{id: 'configure', label: 'Configure'},
];

function CreateServicePage() {
	const {id} = Route.useParams();
	const projectId = Number(id);
	const {env: envParam, type: typeParam} = useSearch({
		from: '/_app/projects/$id/create',
	});

	const form = useCreateService({projectId, envParam, typeParam});
	const {project, chosen, pick} = form;

	// Deep-links that already carry a `?type=` skip straight to configuration.
	const [step, setStep] = useState<'type' | 'configure'>(
		typeParam ? 'configure' : 'type',
	);
	const idx = step === 'type' ? 0 : 1;

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<Titlebar
				breadcrumbs={[
					{label: 'Projects', to: '/projects', icon: FolderOpen},
					{
						label: project?.name || 'Project',
						to: '/projects/$id',
						params: {id: String(projectId)},
					},
					{label: step === 'type' ? 'Create Service' : 'Configure'},
				]}
			/>

			<StepCheckpoints steps={STEPS} idx={idx} />

			<StepBody key={step}>
				{step === 'type' ? (
					<div className="mx-auto flex w-full max-w-2xl flex-col gap-5">
						<div className="space-y-1">
							<h2 className="text-lg font-semibold text-foreground">
								What do you want to create?
							</h2>
							<p className="text-[13px] text-muted-foreground">
								Pick a service type, then configure it in the next
								step.
							</p>
						</div>

						<KindPicker
							chosen={chosen}
							onPick={(kind: ServiceKind) => pick(kind)}
						/>

						<div className="flex justify-end">
							<Button
								type="button"
								onClick={() => setStep('configure')}
								disabled={chosen === 'template'}>
								Continue
							</Button>
						</div>
					</div>
				) : (
					<div className="min-w-0 space-y-4">
						<button
							type="button"
							onClick={() => setStep('type')}
							className="flex items-center gap-1.5 text-[13px] font-medium text-muted-foreground transition-colors hover:text-foreground">
							<ArrowLeft className="size-3.5" />
							Change type
						</button>

						<div className="grid min-w-0 grid-cols-1 gap-6 lg:grid-cols-[1fr_340px]">
							<div className="min-w-0">
								<ServiceForm form={form} />
							</div>
							<InfoRail form={form} />
						</div>
					</div>
				)}
			</StepBody>
		</div>
	);
}
