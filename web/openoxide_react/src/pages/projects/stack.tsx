import {useState} from 'react';
import {createFileRoute, Link} from '@tanstack/react-router';
import {FolderOpen, Layers} from 'lucide-react';
import {useAppStore} from '#/stores/app-store';
import {useEnvironmentListByProject} from 'virtual:openoxide-live';
import {EnvDropdown} from '#/components/projects/env/env-dropdown';
import {StackCanvas} from '#/components/projects/stack/stack-canvas';

export const Route = createFileRoute('/_app/projects/$id/stack')({
	component: BlueprintPage,
});

function BlueprintPage() {
	const {id} = Route.useParams();
	const projectId = Number(id);
	const [selectedEnvId, setSelectedEnvId] = useState<number | undefined>();

	const storeProjects = useAppStore(state => state.projects || []);
	const project = storeProjects.find(
		(p: any) => Number(p.id) === projectId,
	);
	const {data: rawEnvs} = useEnvironmentListByProject(BigInt(projectId));
	const envs = rawEnvs ?? [];

	const envId =
		selectedEnvId !== undefined
			? selectedEnvId
			: envs[0]
				? Number(envs[0].id)
				: undefined;

	return (
		<div
			style={{
				display: 'flex',
				flexDirection: 'column',
				height: 'calc(100vh - 56px)',
				background: '#0d1117',
			}}>
			{/* Header */}
			<div
				style={{
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'space-between',
					padding: '10px 20px',
					borderBottom: '1px solid rgba(255,255,255,0.07)',
					background: '#0d1117',
					flexShrink: 0,
					zIndex: 10,
				}}>
				<div
					style={{
						display: 'flex',
						alignItems: 'center',
						gap: 8,
						fontSize: 13,
					}}>
					<Link
						to="/projects"
						style={{
							color: '#6e7681',
							textDecoration: 'none',
							display: 'flex',
							alignItems: 'center',
							gap: 5,
						}}>
						<FolderOpen size={14} /> Projects
					</Link>
					<span style={{color: '#30363d'}}>/</span>
					<Link
						to="/projects/$id"
						params={{id}}
						style={{
							color: '#6e7681',
							textDecoration: 'none',
							fontWeight: 600,
						}}>
						{project?.name || '…'}
					</Link>
					<span style={{color: '#30363d'}}>/</span>
					<span
						style={{
							color: '#e6edf3',
							fontWeight: 700,
							display: 'flex',
							alignItems: 'center',
							gap: 5,
						}}>
						<Layers size={14} /> Blueprint
					</span>
					<span style={{color: '#30363d'}}>/</span>
					<EnvDropdown
						envs={envs}
						selectedId={envId ?? null}
						onSelect={setSelectedEnvId}
						onCreateNew={() => {}}
					/>
				</div>
			</div>

			{/* Canvas — full screen */}
			<div style={{flex: 1, overflow: 'hidden', position: 'relative'}}>
				{envId ? (
					<StackCanvas environmentId={envId} />
				) : (
					<div
						style={{
							display: 'flex',
							alignItems: 'center',
							justifyContent: 'center',
							height: '100%',
							color: '#6e7681',
							fontSize: 13,
						}}>
						Select an environment to continue
					</div>
				)}
			</div>
		</div>
	);
}
