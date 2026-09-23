import {useState, useMemo, useEffect} from 'react';
import {Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
import {toast} from 'sonner';
import {
	useScheduleCreate,
	useScheduleDelete,
	useScheduleRun,
	useScheduleListByOrganization,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {useOrganizations} from '#/stores/session-context';
import {CreateScheduleModal} from './schedules/create-schedule-modal';
import {ComposeSchedulesTable} from './schedules/compose-schedules-table';
import {
	buildRawGitUrl,
	getComposeServiceNames,
} from '#/utils/compose-services';

interface ComposeSchedulesTabProps {
	compose: any;
	schedules?: any[];
}

export function ComposeSchedulesTab({
	compose,
	schedules: passedSchedules,
}: ComposeSchedulesTabProps) {
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [query, setQuery] = useState('');
	const [view, setView] = useListView('compose-schedules-view');
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

	// Live schedules query; the table-watch stream (schedules tables) keeps
	// it fresh, so no zustand slice involved.
	const composeId = compose?.id;
	const {activeOrg} = useOrganizations();
	const {data: storeSchedules = []} = useScheduleListByOrganization(
		BigInt(activeOrg?.id || 1),
	);

	// Safe array normalization
	const composeSchedules = useMemo(() => {
		if (passedSchedules && passedSchedules.length > 0)
			return passedSchedules;
		return storeSchedules.filter(
			(s: any) => Number(s.compose_id) === Number(composeId),
		);
	}, [passedSchedules, storeSchedules, composeId]);
	const isLoading = false;

	const q = query.trim().toLowerCase();
	const visibleSchedules = useMemo(
		() =>
			q
				? composeSchedules.filter((s: any) =>
						[s?.name, s?.command, s?.cron_expression, s?.service_name]
							.filter(Boolean)
							.join(' ')
							.toLowerCase()
							.includes(q),
					)
				: composeSchedules,
		[composeSchedules, q],
	);

	// Mutations
	const createMutation = useScheduleCreate();
	const runMutation = useScheduleRun();
	const deleteMutation = useScheduleDelete();

	const handleCreate = async (data: {
		name: string;
		serviceName: string;
		command: string;
		cronExpr: string;
	}) => {
		try {
			await createMutation.mutateAsync({
				name: data.name,
				compose_id: compose?.id,
				organization_id: activeOrg?.id || 1,
				service_name: data.serviceName,
				command: data.command,
				cron_expression: data.cronExpr,
				schedule_type: 'COMPOSE',
				schedule_action: 'EXEC',
			});
			toast.success('Compose schedule task created successfully');
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleRun = async (id: number) => {
		try {
			await runMutation.mutateAsync(id);
			toast.success('Schedule task executed');
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleDelete = async (id: number) => {
		try {
			await deleteMutation.mutateAsync(id);
			toast.success('Schedule task deleted');
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const createButton = (
		<Button onClick={() => setIsCreateOpen(true)} className="h-9 gap-1.5">
			<Plus className="size-3.5" />
			New Schedule
		</Button>
	);

	return (
		<div className="flex w-full flex-col gap-5">
			<SectionHeader
				title="Schedules"
				description="Cron jobs that run inside this stack's service containers."
				action={createButton}
			/>

			<Toolbar action={<ViewToggle view={view} onChange={setView} />}>
				<ToolbarSearch
					value={query}
					onChange={setQuery}
					placeholder="Search schedules…"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			{/* Schedules Table Component (< 200 lines) */}
			<ComposeSchedulesTable
				schedules={visibleSchedules}
				isLoading={isLoading}
				hasQuery={q !== ''}
				view={view}
				onRun={handleRun}
				onDelete={handleDelete}
				createAction={createButton}
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
