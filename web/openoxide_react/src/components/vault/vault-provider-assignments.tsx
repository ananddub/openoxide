import {useState} from 'react';
import {ChevronDown, ChevronRight, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Checkbox} from '#/components/ui/checkbox';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {useEnvironmentListByProject} from '#/api/live';
import {useProjectsList} from '#/hooks/projects/use-projects-list';
import {parseVaultAssignments} from './vault-provider-catalog';

interface AssignmentRow {
	project_id: number;
	environment_ids: number[];
}

/**
 * Project/environment scoping editor for a vault provider. Stored as the
 * same JSON array the backend expects; an empty selection means the provider
 * is available to every project in the organization.
 */
export function VaultProviderAssignments({
	value,
	onChange,
}: {
	value: string;
	onChange: (json: string) => void;
}) {
	const {projects} = useProjectsList();
	const [expanded, setExpanded] = useState<number | null>(null);
	const rows = parseVaultAssignments(value) as AssignmentRow[];
	const environmentsQuery = useEnvironmentListByProject(
		BigInt(expanded ?? 0),
	);

	const write = (next: AssignmentRow[]) =>
		onChange(
			JSON.stringify(
				next.map(row => ({
					project_id: Number(row.project_id),
					environment_ids: (row.environment_ids || []).map(Number),
				})),
			),
		);

	const rowFor = (projectId: number) => {
		const existing = rows.find(
			row => Number(row.project_id) === projectId,
		);
		return (
			existing ?? {
				project_id: projectId,
				environment_ids: [],
			}
		);
	};

	const addProject = (projectId: number) => {
		if (rows.some(row => Number(row.project_id) === projectId)) return;
		write([...rows, {project_id: projectId, environment_ids: []}]);
		setExpanded(projectId);
	};

	const removeProject = (projectId: number) => {
		write(rows.filter(row => Number(row.project_id) !== projectId));
		if (expanded === projectId) setExpanded(null);
	};

	const toggleEnvironment = (projectId: number, environmentId: number) => {
		const current = rowFor(projectId);
		const ids = current.environment_ids.map(Number);
		write(
			rows
				.filter(row => Number(row.project_id) !== projectId)
				.concat({
					...current,
					project_id: projectId,
					environment_ids: ids.includes(environmentId)
						? ids.filter(id => id !== environmentId)
						: [...ids, environmentId],
				}),
		);
	};

	const assignedIds = rows.map(row => Number(row.project_id));
	const unassigned = (projects as any[]).filter(
		project => !assignedIds.includes(Number(project.id)),
	);

	return (
		<div className="flex flex-col gap-3">
			<p className="text-xs leading-relaxed text-muted-foreground">
				{rows.length === 0
					? 'No scope selected — this provider is available to every project in the organization.'
					: 'Only the projects below can resolve this provider’s secrets.'}
			</p>

			<Select
				value=""
				onValueChange={val => val && addProject(Number(val))}>
				<SelectTrigger className="h-9 text-xs">
					<SelectValue placeholder="Assign to a project…" />
				</SelectTrigger>
				<SelectContent>
					{unassigned.map(project => (
						<SelectItem key={project.id} value={String(project.id)}>
							{project.name}
						</SelectItem>
					))}
				</SelectContent>
			</Select>

			{rows.length > 0 && (
				<div className="w-full divide-y overflow-hidden rounded-lg border bg-card">
					{rows.map(row => {
						const projectId = Number(row.project_id);
						const project = (projects as any[]).find(
							item => Number(item.id) === projectId,
						);
						const isOpen = expanded === projectId;
						const envCount = (row.environment_ids || []).length;
						return (
							<div key={projectId}>
								<div className="flex items-center gap-2 px-3 py-2">
									<button
										type="button"
										onClick={() => setExpanded(isOpen ? null : projectId)}
										className="flex min-w-0 flex-1 cursor-pointer items-center gap-2 text-start">
										{isOpen ? (
											<ChevronDown className="size-3.5 shrink-0 text-muted-foreground" />
										) : (
											<ChevronRight className="size-3.5 shrink-0 text-muted-foreground" />
										)}
										<span className="min-w-0 flex-1">
											<span className="block truncate text-xs font-medium text-foreground">
												{project?.name ?? `Project ${projectId}`}
											</span>
											<span className="block truncate text-[11px] text-muted-foreground">
												{envCount
													? `${envCount} environment${envCount > 1 ? 's' : ''}`
													: 'All environments'}
											</span>
										</span>
									</button>
									<Button
										variant="ghost"
										size="icon"
										title="Remove project"
										onClick={() => removeProject(projectId)}
										className="size-7 shrink-0 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
										<X className="size-3.5" />
									</Button>
								</div>

								{isOpen && (
									<div className="flex flex-col gap-1.5 bg-muted/20 px-3 py-2.5">
										<p className="mb-1 text-[11px] text-muted-foreground">
											Pick environments — none selected means every
											environment in this project.
										</p>
										{environmentsQuery.isPending ? (
											<p className="text-[11px] text-muted-foreground">
												Loading environments…
											</p>
										) : environmentsQuery.isError ? (
											<p className="text-[11px] text-destructive">
												Could not load this project’s environments.
											</p>
										) : environmentsQuery.isFetched &&
										  environmentsQuery.data?.length === 0 ? (
											<p className="text-[11px] text-muted-foreground">
												This project has no environments yet.
											</p>
										) : (
											((environmentsQuery.data ?? []) as any[]).map(
												environment => {
													const checked = (row.environment_ids || [])
														.map(Number)
														.includes(Number(environment.id));
													return (
														<label
															key={environment.id}
															className="flex cursor-pointer items-center gap-2.5 text-xs text-foreground">
															<Checkbox
																checked={checked}
																onCheckedChange={() =>
																	toggleEnvironment(
																		projectId,
																		Number(environment.id),
																	)
																}
															/>
															{environment.name}
														</label>
													);
												},
											)
										)}
									</div>
								)}
							</div>
						);
					})}
				</div>
			)}
		</div>
	);
}
