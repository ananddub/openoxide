import * as React from 'react';
import {$api} from '#/api/query';
import {useQueryClient} from '@tanstack/react-query';
import {toast} from 'sonner';
import {useOrganizationStore} from '#/stores/organization-store';
import {formatApiError} from '#/api/utils';

// Helper to extract hashtags (e.g. #prod, #api) from text
export const getTagsFromDescription = (description?: string): string[] => {
	if (!description) return [];
	const matches = description.match(/#[\w-]+/g);
	return matches ? matches.map(m => m.toLowerCase()) : [];
};

export function useProjectsList() {
	const queryClient = useQueryClient();
	const [isCreateOpen, setIsCreateOpen] = React.useState(false);
	const [isSubmitting, setIsSubmitting] = React.useState(false);

	// Filters & Sorting state
	const [searchQuery, setSearchQuery] = React.useState('');
	const [sortBy, setSortBy] = React.useState('newest');
	const [selectedTags, setSelectedTags] = React.useState<string[]>([]);

	// Get active organization from global layout switcher store
	const activeOrg = useOrganizationStore(state => state.activeOrg);

	// Fetch Projects for the active organization
	const {data: projects, isLoading: isLoadingProjects} = $api.useQuery(
		'get',
		'/projects/organization/{organization_id}',
		{
			params: {
				path: {
					organization_id: activeOrg?.id || 0,
				},
			},
		},
		{
			enabled: activeOrg !== null,
		},
	);

	// Create Project Mutation
	const createProjectMutation = $api.useMutation('post', '/projects');

	// Delete Project Mutation
	const deleteProjectMutation = $api.useMutation('delete', '/projects/{id}');

	// Extract all unique tags present across all projects
	const allTags = React.useMemo(() => {
		if (!projects) return [];
		const tagsSet = new Set<string>();
		projects.forEach(p => {
			getTagsFromDescription(p.description).forEach(t => tagsSet.add(t));
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
				(project.description || '').toLowerCase().includes(searchQuery.toLowerCase());

			const projectTags = getTagsFromDescription(project.description);
			const matchesTags =
				selectedTags.length === 0 ||
				selectedTags.every(t => projectTags.includes(t));

			return matchesSearch && matchesTags;
		});

		// 2. Sort projects
		return [...result].sort((a, b) => {
			if (sortBy === 'newest') return b.created_at - a.created_at;
			if (sortBy === 'oldest') return a.created_at - b.created_at;
			if (sortBy === 'alphabetical-asc') return a.name.localeCompare(b.name);
			if (sortBy === 'alphabetical-desc') return b.name.localeCompare(a.name);
			return 0;
		});
	}, [projects, searchQuery, sortBy, selectedTags]);

	const handleTagClick = (tag: string) => {
		setSelectedTags(prev =>
			prev.includes(tag) ? prev.filter(t => t !== tag) : [...prev, tag]
		);
	};

	const handleCreateProjectSubmit = async (
		name: string,
		description: string,
		envVar: string,
	) => {
		if (!name.trim() || !activeOrg) {
			toast.error('Project name is required');
			return;
		}

		setIsSubmitting(true);
		try {
			await createProjectMutation.mutateAsync({
				body: {
					name,
					description: description || undefined,
					env_var: envVar,
					organization_id: activeOrg.id,
				},
			});

			toast.success('Project created successfully!');
			setIsCreateOpen(false);
			// Refresh projects list
			queryClient.invalidateQueries({
				queryKey: [
					'get',
					'/projects/organization/{organization_id}',
					{params: {path: {organization_id: activeOrg.id}}},
				],
			});
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleDeleteProject = async (projectId: number) => {
		if (!activeOrg) return;

		try {
			await deleteProjectMutation.mutateAsync({
				params: {
					path: {
						id: projectId,
					},
				},
			});

			toast.success('Project deleted successfully');
			queryClient.invalidateQueries({
				queryKey: [
					'get',
					'/projects/organization/{organization_id}',
					{params: {path: {organization_id: activeOrg.id}}},
				],
			});
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	return {
		projects,
		isLoadingProjects,
		filteredAndSortedProjects,
		allTags,
		searchQuery,
		setSearchQuery,
		sortBy,
		setSortBy,
		selectedTags,
		setSelectedTags,
		isCreateOpen,
		setIsCreateOpen,
		isSubmitting,
		activeOrg,
		handleTagClick,
		handleCreateProjectSubmit,
		handleDeleteProject,
	};
}
