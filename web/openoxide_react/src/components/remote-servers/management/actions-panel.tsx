import {useState} from 'react';
import {Loader2, Play, Plus, Save, X} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Checkbox} from '#/components/ui/checkbox';
import {Input} from '#/components/ui/input';
import {Skeleton} from '#/components/ui/skeleton';
import {Switch} from '#/components/ui/switch';
import {Textarea} from '#/components/ui/textarea';
import {formatApiError} from '#/api/utils';
import {usePermissions} from '#/hooks/permissions/use-permissions';
import {
	CLEANUP_SCOPES,
	CLEANUP_SCOPE_LABELS,
	MANAGEMENT_ACTIONS,
	MANAGEMENT_ACTION_HINTS,
	MANAGEMENT_ACTION_LABELS,
	isSlowAction,
	useCleanupHistory,
	useServerManagement,
	useServerManagementAction,
	useServerManagementSave,
	type CleanupExecutionPayload,
	type ManagementAction,
	type ServerActionPayload,
	type ServerManagementPayload,
} from '#/api/live/server-management';

import {
	DockerEmptyState,
	DockerTable,
	type DockerColumn,
} from '../docker/docker-shell';
import {Panel} from '../server-panel';
import {ServerAuditPanel} from './audit-panel';
import {
	formatEpochSeconds,
	formatRunDuration,
	managementDirty,
	summarizePolicy,
} from './management-metrics';

type CommandResult = {
	action: ManagementAction;
	payload: ServerActionPayload;
};

/** Actions tab: the server's management record (maintenance, labels, cleanup policy, GPU) plus the host-side maintenance commands. */
export function ServerManagementPanel({serverId}: {serverId: bigint}) {
	const {can} = usePermissions();
	const canAct = can('server:create');
	const settings = useServerManagement(serverId);
	const history = useCleanupHistory(serverId);
	const action = useServerManagementAction(serverId);
	const [result, setResult] = useState<CommandResult | null>(null);
	const [selected, setSelected] = useState<CleanupExecutionPayload | null>(
		null,
	);

	const run = (kind: ManagementAction) => {
		action.mutate(kind, {
			onSuccess: payload => {
				setResult({action: kind, payload});
				setSelected(null);
				if (payload.success) {
					toast.success(`${MANAGEMENT_ACTION_LABELS[kind]} finished`);
				} else {
					toast.error(
						payload.stderr || `${MANAGEMENT_ACTION_LABELS[kind]} failed`,
					);
				}
			},
			onError: error => toast.error(formatApiError(error)),
		});
	};

	const historyColumns: DockerColumn<CleanupExecutionPayload>[] = [
		{
			label: 'Status',
			cell: row => (
				<span
					className={`rounded-full px-2 py-0.5 text-[11px] font-medium ${
						row.succeeded
							? 'bg-emerald-500/10 text-emerald-500'
							: 'bg-rose-500/10 text-rose-500'
					}`}>
					{row.status}
				</span>
			),
		},
		{label: 'Started', cell: row => formatEpochSeconds(row.startedAt)},
		{
			label: 'Duration',
			align: 'right',
			cell: row => formatRunDuration(row.startedAt, row.finishedAt),
		},
		{
			label: 'Policy',
			cell: row => (
				<span className="text-muted-foreground">
					{summarizePolicy(row.policy)}
				</span>
			),
		},
	];

	return (
		<div className="flex flex-col gap-4">
			{settings.isLoading ? (
				<Skeleton className="h-48 w-full rounded-xl" />
			) : settings.isError ? (
				<Panel title="Management">
					<p className="py-8 text-center text-xs text-rose-500">
						{formatApiError(settings.error)}
					</p>
				</Panel>
			) : settings.data ? (
				<ManagementForm
					key={settings.data.updatedAt.toString()}
					canAct={canAct}
					initial={settings.data}
					serverId={serverId}
				/>
			) : null}

			<Panel title="Actions" caption="Host-side maintenance commands">
				<div className="grid gap-2 p-4 sm:grid-cols-2 lg:grid-cols-3">
					{MANAGEMENT_ACTIONS.map(kind => (
						<div
							key={kind}
							className="flex flex-col gap-2 rounded-lg border border-border/60 bg-muted/10 p-3">
							<span className="text-xs font-semibold text-foreground">
								{MANAGEMENT_ACTION_LABELS[kind]}
							</span>
							<span className="flex-1 text-[11px] leading-snug text-muted-foreground">
								{MANAGEMENT_ACTION_HINTS[kind]}
							</span>
							{isSlowAction(kind) && (
								<span className="text-[11px] text-amber-600 dark:text-amber-500">
									Can take a few minutes.
								</span>
							)}
							<Button
								size="sm"
								variant="outline"
								disabled={!canAct || action.isPending}
								onClick={() => run(kind)}
								className="h-8 cursor-pointer justify-center gap-1.5 text-xs disabled:cursor-default">
								{action.isPending && action.variables === kind ? (
									<Loader2 className="size-3.5 animate-spin" />
								) : (
									<Play className="size-3.5" />
								)}
								{action.isPending && action.variables === kind
									? 'Running…'
									: 'Run'}
							</Button>
						</div>
					))}
				</div>
				{result && (
					<CommandOutput
						title={MANAGEMENT_ACTION_LABELS[result.action]}
						payload={result.payload}
					/>
				)}
			</Panel>

			<ServerAuditPanel serverId={serverId} />

			<Panel
				title="Cleanup History"
				caption={`${history.data?.length ?? 0} run(s)`}
				isRefreshing={history.isFetching}
				onRefresh={() => void history.refetch()}>
				{history.isError ? (
					<p className="py-8 text-center text-xs text-rose-500">
						{formatApiError(history.error)}
					</p>
				) : (history.data?.length ?? 0) === 0 ? (
					<DockerEmptyState label="This server has never run a cleanup." />
				) : (
					<>
						<DockerTable
							columns={historyColumns}
							rows={history.data ?? []}
							keyOf={row => row.id}
							onSelect={row => setSelected(row)}
						/>
						{selected && (
							<CommandOutput
								title={`Cleanup #${selected.id} — ${selected.status}`}
								payload={{
									success: selected.succeeded,
									stdout: selected.stdout,
									stderr: selected.stderr,
								}}
							/>
						)}
					</>
				)}
			</Panel>
		</div>
	);
}

interface ManagementFormProps {
	canAct: boolean;
	initial: ServerManagementPayload;
	serverId: bigint;
}

/** Editable half of the management record; Save PUTs the whole record because the backend has no PATCH semantics here. */
function ManagementForm({canAct, initial, serverId}: ManagementFormProps) {
	const [form, setForm] = useState<ServerManagementPayload>(initial);
	const save = useServerManagementSave(serverId);
	const dirty = managementDirty(form, initial);

	const patch = (changes: Partial<ServerManagementPayload>) =>
		setForm(prev => ({...prev, ...changes}));

	const onSave = () => {
		save.mutate(form, {
			onSuccess: () => toast.success('Management settings saved'),
			onError: error => toast.error(formatApiError(error)),
		});
	};

	return (
		<Panel
			title="Maintenance & Policy"
			caption={`Updated ${formatEpochSeconds(form.updatedAt)}`}>
			<div className="flex flex-col gap-4 p-4">
				<div className="flex items-start justify-between gap-4">
					<div className="min-w-0">
						<span className="text-xs font-semibold text-foreground">
							Maintenance mode
						</span>
						<p className="text-[11px] text-muted-foreground">
							Stops new deployments to this server and shows visitors your
							message.
						</p>
					</div>
					<Switch
						checked={form.maintenanceMode}
						disabled={!canAct}
						onCheckedChange={checked =>
							patch({maintenanceMode: checked === true})
						}
					/>
				</div>
				{form.maintenanceMode && (
					<Textarea
						value={form.maintenanceMessage}
						maxLength={1000}
						placeholder="Shown while this server is in maintenance."
						onChange={event =>
							patch({maintenanceMessage: event.target.value})
						}
						className="min-h-20 text-xs"
					/>
				)}

				<div className="flex flex-col gap-2">
					<span className="text-xs font-semibold text-foreground">
						Cleanup policy
					</span>
					<div className="flex flex-wrap gap-x-5 gap-y-2">
						{CLEANUP_SCOPES.map(scope => (
							<label
								key={scope}
								className="flex cursor-pointer items-center gap-2 text-xs text-foreground/80">
								<Checkbox
									checked={form.cleanupPolicy[scope]}
									disabled={!canAct}
									onCheckedChange={checked =>
										patch({
											cleanupPolicy: {
												...form.cleanupPolicy,
												[scope]: checked === true,
											},
										})
									}
								/>
								{CLEANUP_SCOPE_LABELS[scope]}
							</label>
						))}
					</div>
				</div>

				<div className="flex items-start justify-between gap-4">
					<div className="min-w-0">
						<span className="text-xs font-semibold text-foreground">
							GPU support
						</span>
						<p className="text-[11px] text-muted-foreground">
							Allows services on this server to request GPU resources. Run
							“Configure GPU” first to install the container toolkit.
						</p>
					</div>
					<Switch
						checked={form.gpuEnabled}
						disabled={!canAct}
						onCheckedChange={checked =>
							patch({gpuEnabled: checked === true})
						}
					/>
				</div>

				<div className="flex flex-col gap-2">
					<span className="text-xs font-semibold text-foreground">
						Labels
					</span>
					{form.labels.map((entry, index) => (
						<div key={index} className="flex items-center gap-2">
							<Input
								value={entry[0]}
								placeholder="key"
								disabled={!canAct}
								onChange={event =>
									patch({
										labels: form.labels.map((row, position) =>
											position === index
												? ([event.target.value, row[1]] as [
														string,
														string,
													])
												: row,
										),
									})
								}
								className="h-8 flex-1 text-xs"
							/>
							<Input
								value={entry[1]}
								placeholder="value"
								disabled={!canAct}
								onChange={event =>
									patch({
										labels: form.labels.map((row, position) =>
											position === index
												? ([row[0], event.target.value] as [
														string,
														string,
													])
												: row,
										),
									})
								}
								className="h-8 flex-1 text-xs"
							/>
							<Button
								variant="ghost"
								size="sm"
								disabled={!canAct}
								onClick={() =>
									patch({
										labels: form.labels.filter((_, p) => p !== index),
									})
								}
								title="Remove label"
								className="size-8 shrink-0 cursor-pointer p-0 text-rose-500 disabled:cursor-default">
								<X className="size-3.5" />
							</Button>
						</div>
					))}
					<Button
						variant="outline"
						size="sm"
						disabled={!canAct}
						onClick={() =>
							patch({
								labels: [...form.labels, ['', ''] as [string, string]],
							})
						}
						className="h-8 w-fit cursor-pointer gap-1.5 text-xs">
						<Plus className="size-3.5" />
						Add label
					</Button>
				</div>

				<Button
					size="sm"
					disabled={!canAct || !dirty || save.isPending}
					onClick={onSave}
					className="h-8 w-fit cursor-pointer gap-1.5 text-xs disabled:cursor-default">
					{save.isPending ? (
						<Loader2 className="size-3.5 animate-spin" />
					) : (
						<Save className="size-3.5" />
					)}
					Save Settings
				</Button>
			</div>
		</Panel>
	);
}

/** Verbatim stdout/stderr of one management command or cleanup execution. */
function CommandOutput({
	title,
	payload,
}: {
	title: string;
	payload: ServerActionPayload;
}) {
	return (
		<div className="border-t border-border/50 px-5 py-4">
			<div className="mb-2 flex items-center gap-2">
				<span className="text-xs font-semibold text-foreground">
					{title}
				</span>
				<span
					className={`rounded-full px-2 py-0.5 text-[11px] font-medium ${
						payload.success
							? 'bg-emerald-500/10 text-emerald-500'
							: 'bg-rose-500/10 text-rose-500'
					}`}>
					{payload.success ? 'success' : 'failed'}
				</span>
			</div>
			<pre className="max-h-64 scrollbar-thin scrollbar-thumb-border/40 overflow-auto rounded-lg bg-muted/40 p-4 font-mono text-[11px] leading-relaxed">
				{[payload.stdout, payload.stderr].filter(Boolean).join('\n\n') ||
					'No output.'}
			</pre>
		</div>
	);
}
