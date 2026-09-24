import * as React from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useProjectCreate, useProjectUpdate} from '#/api/live';
import {getTagsFromDescription} from '#/utils/project-tags';

type Project = Record<string, any>;

/** Owns the create/update project form state, tag parsing and submission. */
export function useProjectForm({
	project,
	isOpen,
	activeOrgId,
	onOpenChange,
	onSuccess,
}: {
	project?: Project | null;
	isOpen: boolean;
	activeOrgId?: number;
	onOpenChange: (open: boolean) => void;
	onSuccess?: () => void;
}) {
	const isEditing = !!project;
	const [name, setName] = React.useState('');
	const [description, setDescription] = React.useState('');
	const [selectedTags, setSelectedTags] = React.useState<string[]>([]);
	const [isSubmitting, setIsSubmitting] = React.useState(false);

	const createMutation = useProjectCreate();
	const updateMutation = useProjectUpdate();

	// Reset & populate form when modal opens or project changes
	React.useEffect(() => {
		if (!isOpen) return;
		if (project) {
			setName(project.name || '');
			// Extract clean description (excluding hashtag block)
			setDescription(
				(project.description || '').replace(/#[\w-]+/g, '').trim(),
			);
			setSelectedTags(getTagsFromDescription(project.description || ''));
		} else {
			setName('');
			setDescription('');
			setSelectedTags([]);
		}
	}, [isOpen, project]);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim()) {
			toast.error('Project name is required');
			return;
		}

		setIsSubmitting(true);

		// Format description with hashtags block
		let finalDescription = description.trim();
		if (selectedTags.length > 0) {
			const hashtagsString = selectedTags.map(t => `#${t}`).join(' ');
			finalDescription = finalDescription
				? `${finalDescription}\n\n${hashtagsString}`
				: hashtagsString;
		}

		try {
			if (isEditing && project) {
				await updateMutation.mutateAsync({
					id: project.id,
					name: name.trim(),
					description: finalDescription || undefined,
				});
				toast.success('Project updated successfully');
			} else {
				if (!activeOrgId) {
					toast.error('Active organization required');
					setIsSubmitting(false);
					return;
				}
				await createMutation.mutateAsync({
					name: name.trim(),
					description: finalDescription || undefined,
					environment_variables: '',
					organization_id: activeOrgId,
				});
				toast.success('Project created successfully');
			}

			onOpenChange(false);
			onSuccess?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	return {
		isEditing,
		name,
		setName,
		description,
		setDescription,
		selectedTags,
		setSelectedTags,
		isSubmitting,
		handleSubmit,
	};
}
