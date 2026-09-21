import {useMemo} from 'react';
import {useTagListAll, useOverviewServices} from '#/api/live';
import {getTagsFromDescription} from '#/hooks/projects/use-projects-list';

type Project = Record<string, any>;

const PALETTE = [
	'#3b82f6',
	'#10b981',
	'#f59e0b',
	'#8b5cf6',
	'#ec4899',
	'#06b6d4',
];

/**
 * Derives the dashboard project card display data (service/env counts,
 * deployment server names, tag names and badge colors) from the live store.
 */
export function useProjectCardData(project: Project) {
	const overviewServices = useOverviewServices().data ?? [];
	const availableTags = useTagListAll().data ?? [];

	// created_at is a Connect proto int64 (bigint at runtime); coerce before
	// arithmetic to avoid a TypeError crashing the card render.
	const formatDate = (timestamp?: number | bigint) => {
		const seconds = Number(timestamp ?? 0);
		if (!seconds) return 'recently';
		return new Date(seconds * 1000).toLocaleDateString();
	};

	const projectServices = overviewServices.filter(
		s => Number(s.project_id) === Number(project.id),
	);
	const totalServices = projectServices.length;
	const envIds = new Set(
		projectServices.map(s => s.environment_id).filter(Boolean),
	);
	const totalEnvironments = envIds.size > 0 ? envIds.size : 1;

	const projectServerNames = useMemo(() => {
		const names = new Map<string, string>();
		projectServices.forEach((service: any) => {
			const name = String(
				service.server_name ||
					(service.server_id == null
						? 'Localhost (Main server)'
						: `Server ${service.server_id}`),
			).trim();
			if (name) names.set(name.toLowerCase(), name);
		});
		return [...names.values()];
	}, [projectServices]);

	const projectTagNames = getTagsFromDescription(
		project.description || '',
	);

	// Match tag with exact custom color from Settings Tags, else deterministic.
	const getTagColor = (tagName: string) => {
		const clean = tagName.replace(/^#/, '').trim().toLowerCase();
		const found = availableTags.find(
			(at: any) => (at.name || '').trim().toLowerCase() === clean,
		);
		if (found && found.color) return found.color;
		let hash = 0;
		for (let i = 0; i < clean.length; i++)
			hash = clean.charCodeAt(i) + ((hash << 5) - hash);
		return PALETTE[Math.abs(hash) % PALETTE.length];
	};

	return {
		formatDate,
		totalServices,
		totalEnvironments,
		projectServerNames,
		projectTagNames,
		getTagColor,
	};
}
