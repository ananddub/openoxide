import {useState, useMemo} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {Calendar, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {CreateScheduleModal} from './schedules/create-schedule-modal';
import {ComposeSchedulesTable} from './schedules/compose-schedules-table';

interface ComposeSchedulesTabProps {
	compose: any;
	schedules?: any[];
	isLoading?: boolean;
}

// Extract service names defined under 'services:' in docker-compose.yml content
const extractServicesFromYaml = (yamlStr?: string): string[] => {
	if (!yamlStr) return [];
	const lines = yamlStr.split('\n');
	const services: string[] = [];
	let inServicesBlock = false;
	let servicesIndent = 0;

	for (const line of lines) {
		const trimmed = line.trimEnd();
		if (!trimmed || trimmed.trimStart().startsWith('#')) continue;

		const indent = line.search(/\S/);
		const text = trimmed.trim();

		if (text === 'services:' || text.startsWith('services:')) {
			inServicesBlock = true;
			servicesIndent = indent;
			continue;
		}

		if (inServicesBlock) {
			if (indent <= servicesIndent && text.endsWith(':') && !text.startsWith('-')) {
				inServicesBlock = false;
			} else if (indent > servicesIndent && text.endsWith(':') && !text.includes(' ') && !text.includes('.')) {
				const serviceName = text.slice(0, -1).trim();
				if (serviceName && !services.includes(serviceName)) {
					services.push(serviceName);
				}
			}
		}
	}
	return services;
};

export function ComposeSchedulesTab({compose, schedules: passedSchedules, isLoading: passedIsLoading}: ComposeSchedulesTabProps) {
	const queryClient = useQueryClient();
	const [isCreateOpen, setIsCreateOpen] = useState(false);

	const availableServices = useMemo(() => {
		return extractServicesFromYaml(compose?.compose_file);
	}, [compose?.compose_file]);

	const servicesList = availableServices.length > 0 ? availableServices : ['app'];

	// Real-time compose schedules query (fallback if not passed from parent)
	const composeId = compose?.id;
	const {data: rawSchedules = [], isLoading: innerIsLoading} = $api.useQuery(
		'get',
		'/schedules/compose/{compose_id}',
		{
			params: {path: {compose_id: composeId || 0}},
			enabled: !passedSchedules && !!composeId,
		} as any
	);

	// Safe array normalization
	const composeSchedules = passedSchedules ?? (Array.isArray(rawSchedules) ? rawSchedules : []);
	const isLoading = passedIsLoading ?? innerIsLoading;

	// Mutations
	const createMutation = $api.useMutation('post', '/schedules');
	const runMutation = $api.useMutation('post', '/schedules/{id}/run');
	const deleteMutation = $api.useMutation('delete', '/schedules/{id}');

	const handleCreate = async (data: {
		name: string;
		serviceName: string;
		command: string;
		cronExpr: string;
	}) => {
		try {
			await createMutation.mutateAsync({
				body: {
					name: data.name,
					compose_id: compose?.id,
					service_name: data.serviceName,
					command: data.command,
					cron_expression: data.cronExpr,
					schedule_type: 'COMPOSE',
					schedule_action: 'EXEC',
				} as any,
			});
			toast.success('Compose schedule task created successfully');
			queryClient.invalidateQueries();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleRun = async (id: number) => {
		try {
			await runMutation.mutateAsync({params: {path: {id}}});
			toast.success('Schedule task executed');
			queryClient.invalidateQueries();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleDelete = async (id: number) => {
		try {
			await deleteMutation.mutateAsync({params: {path: {id}}});
			toast.success('Schedule task deleted');
			queryClient.invalidateQueries();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Top Header Card */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between flex-wrap gap-4 shadow-sm">
				<div>
					<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
						<Calendar className="w-4 h-4 text-primary" /> Scheduled Tasks
					</h3>
					<p className="text-xs text-muted-foreground mt-1">Configure scheduled tasks to execute recurring commands inside your compose service container</p>
				</div>
				<div className="flex items-center gap-3">
					<Badge variant="outline" className="text-xs font-mono px-3 py-1">
						Active Tasks: {composeSchedules.length}
					</Badge>
					<Button onClick={() => setIsCreateOpen(true)} size="sm" className="h-8 text-xs font-semibold flex items-center gap-1.5">
						<Plus className="w-4 h-4" /> Create Schedule
					</Button>
				</div>
			</section>

			{/* Schedules Table Component (< 200 lines) */}
			<ComposeSchedulesTable
				schedules={composeSchedules}
				isLoading={isLoading}
				onRun={handleRun}
				onDelete={handleDelete}
			/>

			{/* Create Modal Component (< 200 lines) */}
			<CreateScheduleModal
				isOpen={isCreateOpen}
				onClose={() => setIsCreateOpen(false)}
				servicesList={servicesList}
				onCreate={handleCreate}
			/>
		</div>
	);
}
