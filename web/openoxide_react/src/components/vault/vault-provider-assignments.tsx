import {useState} from 'react';
import {useEnvironmentListByProject} from '#/api/live';
import {useProjectsList} from '#/hooks/projects/use-projects-list';
import {Badge} from '#/components/ui/badge';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {FieldLabel} from '#/components/shared/field-label';
import {parseVaultAssignments} from './vault-provider-catalog';

interface AssignmentRow {
	project_id: number;
	environment_ids: number[];
}

/**
 * Project/environment scoping editor for a vault provider. Stores the
 * selection as the same JSON array payload the backend expects; an empty
 * selection means "available to all projects".
 */
export function VaultProviderAssignments({
	value,
	onChange,
}: {
	value: string;
	onChange: (json: string) => void;
}) {
	const {projects} = useProjectsList();
	const [assignmentProject, setAssignmentProject] = useState('');
	const environmentsQuery = useEnvironmentListByProject(
		BigInt(Number(assignmentProject) || 0),
	);
	const rows = parseVaultAssignments(value) as AssignmentRow[];

	const toggleAssignment = (projectId: number, environmentId?: number) => {
		const next: AssignmentRow[] = rows.map(row => ({
			project_id: Number(row.project_id),
			environment_ids: (row.environment_ids || []).map(Number),
		}));
		let row = next.find(item => item.project_id === projectId);
		if (!row) {
			row = {project_id: projectId, environment_ids: []};
			next.push(row);
		}
		if (environmentId) {
			row.environment_ids = row.environment_ids.includes(environmentId)
				? row.environment_ids.filter(id => id !== environmentId)
				: [...row.environment_ids, environmentId];
		}
		onChange(JSON.stringify(next));
	};

	return (
		<div className="space-y-2 rounded-lg bg-muted/10 p-3">
			<div className="flex items-center justify-between">
				<FieldLabel>Project Assignments</FieldLabel>
				<Badge variant="outline">{rows.length} selected</Badge>
			</div>
			<Select
				value={assignmentProject}
				onValueChange={v => {
					if (!v) return;
					setAssignmentProject(v);
					toggleAssignment(Number(v));
				}}>
				<SelectTrigger className="h-9 text-xs">
					<SelectValue placeholder="Assign to a project" />
				</SelectTrigger>
				<SelectContent>
					{(projects as any[]).map(project => (
						<SelectItem key={project.id} value={String(project.id)}>
							{project.name}
						</SelectItem>
					))}
				</SelectContent>
			</Select>
			{assignmentProject && (
				<div className="space-y-1">
					<p className="text-[11px] text-muted-foreground">
						Select environments (empty means every environment in this
						project).
					</p>
					{environmentsQuery.isLoading ? (
						<p className="text-xs text-muted-foreground">
							Loading environments...
						</p>
					) : (
						((environmentsQuery.data || []) as any[]).map(environment => {
							const row = rows.find(
								item =>
									Number(item.project_id) === Number(assignmentProject),
							);
							const checked = !!row?.environment_ids
								?.map(Number)
								.includes(Number(environment.id));
							return (
								<label
									key={environment.id}
									className="flex items-center gap-2 text-xs">
									<input
										type="checkbox"
										checked={checked}
										onChange={() =>
											toggleAssignment(
												Number(assignmentProject),
												Number(environment.id),
											)
										}
									/>
									{environment.name}
								</label>
							);
						})
					)}
				</div>
			)}
			{rows.length > 0 && (
				<div className="flex flex-wrap gap-1">
					{rows.map(row => (
						<Badge key={row.project_id} variant="secondary">
							Project {row.project_id}
							{row.environment_ids?.length
								? ` (${row.environment_ids.length} env)`
								: ' (all envs)'}
						</Badge>
					))}
				</div>
			)}
			<p className="text-[11px] text-muted-foreground">
				No assignments means this provider is available to all projects.
			</p>
		</div>
	);
}
