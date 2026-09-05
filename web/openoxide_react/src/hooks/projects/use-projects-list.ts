import * as React from 'react';
import {
	useProjectCreate,
	useProjectDelete,
	useProjectListByOrganization,
} from '#/api/live';
import {toast} from 'sonner';
import {useOrganizations} from '#/stores/session-context';
import {formatApiError} from '#/api/utils';

// Helper to extract hashtags (e.g. #prod, #api) from text (strips leading #)
export const getTagsFromDescription = (description?: string): string[] => {
	if (!description) return [];
	const matches = description.match(/#[\w-]+/g);
	return matches
		? matches.map(m => m.replace(/^#/, '').toLowerCase())
		: [];
};

export function useProjectsList() {
	const [isCreateOpen, setIsCreateOpen] = React.useState(false);
	const [isSubmitting, setIsSubmitting] = React.useState(false);

	// Filters & Sorting state
	const [searchQuery, setSearchQuery] = React.useState('');
	const [sortBy, setSortBy] = React.useState('newest');
	const [selectedTags, setSelectedTags] = React.useState<string[]>([]);

	// Get active organization from global layout switcher store
	const {activeOrg} = useOrganizations();
	const orgId = activeOrg?.id || 1;

	// Live query is the store: the table-watch stream invalidates the
	// `projects` prefix on table changes, refetching while mounted.
	const {data: liveProjects, isLoading: isLoadingProjects} =
		useProjectListByOrganization(BigInt(orgId));

	const rawProjectsList = liveProjects ?? [];
	const projects = Array.isArray(rawProjectsList) ? rawProjectsList : [];

	// Create Project Mutation
	const createProjectMutation = useProjectCreate();

	// Delete Project Mutation
	const deleteProjectMutation = useProjectDelete();

	// Extract all unique tags present across all projects
	const allTags = React.useMemo(() => {
		if (!projects) return [];
		const tagsSet = new Set<string>();
		projects.forEach(p => {
			getTagsFromDescription(p.description || '').forEach(t =>
				tagsSet.add(t),
			);
		});
		return Array.from(tagsSet);
	}, [projects]);

	// Filter and sort projects list
	const filteredAndSortedProjects = React.useMemo(() => {
		if (!projects) return [];

		// 1. Filter by search query and tags
		let result = projects.filter(project => {
			const matchesSearch =
				project.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				(project.description || '')
					.toLowerCase()
					.includes(searchQuery.toLowerCase());

			const projectTags = getTagsFromDescription(
				project.description || '',
			);
			const matchesTags =
				selectedTags.length === 0 ||
				selectedTags.every(t => projectTags.includes(t.toLowerCase()));

			return matchesSearch && matchesTags;
		});

		// 2. Sort projects
		return [...result].sort((a, b) => {
			if (sortBy === 'newest')
				return Number(b.created_at || 0) - Number(a.created_at || 0);
			if (sortBy === 'oldest')
				return Number(a.created_at || 0) - Number(b.created_at || 0);
			if (sortBy === 'alphabetical-asc')
				return a.name.localeCompare(b.name);
			if (sortBy === 'alphabetical-desc')
				return b.name.localeCompare(a.name);
			return 0;
		});
	}, [projects, searchQuery, sortBy, selectedTags]);

	const handleTagClick = (tag: string) => {
		setSelectedTags(prev =>
			prev.includes(tag) ? prev.filter(t => t !== tag) : [...prev, tag],
		);
	};

	const handleCreateProjectSubmit = async (
		name: string,
		description: string,
		envVar: string,
	) => {
		if (!name.trim()) {
			toast.error('Project name is required');
			return;
		}

		setIsSubmitting(true);
		try {
			await createProjectMutation.mutateAsync({
				name,
				description: description || undefined,
				environment_variables: envVar,
				organization_id: orgId,
			});

			toast.success('Project created successfully');
			setIsCreateOpen(false);
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleDeleteProject = async (id: number) => {
		try {
			await deleteProjectMutation.mutateAsync(id);
			toast.success('Project deleted');
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	return {
		projects,
		filteredAndSortedProjects,
		isLoadingProjects: isLoadingProjects && projects.length === 0,
		allTags,
		selectedTags,
		setSelectedTags,
		searchQuery,
		setSearchQuery,
		sortBy,
		setSortBy,
		handleTagClick,
		isCreateOpen,
		setIsCreateOpen,
		isSubmitting,
		handleCreateProjectSubmit,
		handleDeleteProject,
		activeOrg,
		activeOrgId: orgId,
	};
}
