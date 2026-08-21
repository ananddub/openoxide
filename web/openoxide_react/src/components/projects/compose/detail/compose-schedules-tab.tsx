import {useState, useMemo, useEffect} from 'react';
import {Calendar, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {useAppStore} from '#/stores/app-store';
import {CreateScheduleModal} from './schedules/create-schedule-modal';
import {ComposeSchedulesTable} from './schedules/compose-schedules-table';
import {
	buildRawGitUrl,
	getComposeServiceNames,
} from '#/utils/compose-services';

interface ComposeSchedulesTabProps {
	compose: any;
	schedules?: any[];
	isLoading?: boolean;
}

export function ComposeSchedulesTab({
	compose,
	schedules: passedSchedules,
	isLoading: passedIsLoading,
}: ComposeSchedulesTabProps) {
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [fetchedYaml, setFetchedYaml] = useState<string>('');

	useEffect(() => {
		if (compose?.compose_file && compose.compose_file.trim()) {
			setFetchedYaml(compose.compose_file);
			return;
		}
		const rawUrl = buildRawGitUrl(compose);
		if (rawUrl) {
			let isMounted = true;
			fetch(rawUrl)
				.then(res => (res.ok ? res.text() : ''))
				.then(text => {
					if (isMounted && text && text.trim()) {
						setFetchedYaml(text);
					}
				})
				.catch(() => {});
			return () => {
				isMounted = false;
			};
		}
	}, [compose]);

	const servicesList = useMemo(() => {
		return getComposeServiceNames(compose, fetchedYaml);
	}, [compose, fetchedYaml]);

	// Read schedules directly from Zustand RAM Store
	const composeId = compose?.id;
	const storeSchedules = useAppStore(state => state.schedules || []);

	// Safe array normalization
	const composeSchedules = useMemo(() => {
		if (passedSchedules && passedSchedules.length > 0)
			return passedSchedules;
		return storeSchedules.filter(
			(s: any) => Number(s.compose_id) === Number(composeId),
		);
	}, [passedSchedules, storeSchedules, composeId]);
	const isLoading = false;

	// Mutations
	const createMutation = $api.useMutation('post', '/schedules');
	const runMutation = $api.useMutation('post', '/schedules/{id}/trigger');
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
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleRun = async (id: number) => {
		try {
			await runMutation.mutateAsync({params: {path: {id}}});
			toast.success('Schedule task executed');
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleDelete = async (id: number) => {
		try {
			await deleteMutation.mutateAsync({params: {path: {id}}});
			toast.success('Schedule task deleted');
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Top Header Card */}
			<section className="flex flex-wrap items-center justify-between gap-4 rounded-xl border border-border bg-card p-5 shadow-sm">
				<div>
					<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
						<Calendar className="h-4 w-4 text-primary" /> Scheduled Tasks
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Configure scheduled tasks to execute recurring commands inside
						your compose service container
					</p>
				</div>
				<div className="flex items-center gap-3">
					<Badge variant="outline" className="px-3 py-1 font-mono text-xs">
						Active Tasks: {composeSchedules.length}
					</Badge>
					<Button
						onClick={() => setIsCreateOpen(true)}
						size="sm"
						className="flex h-8 items-center gap-1.5 text-xs font-semibold">
						<Plus className="h-4 w-4" /> Create Schedule
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
