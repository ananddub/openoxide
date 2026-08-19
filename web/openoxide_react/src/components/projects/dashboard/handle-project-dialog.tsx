import * as React from 'react';
import {Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Textarea} from '#/components/ui/textarea';
import {TagSelector} from '#/components/shared/tag-selector';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {getTagsFromDescription} from '#/hooks/projects/use-projects-list';
import type {components} from '#/types/api.d.ts';

type Project = components['schemas']['ProjectResponseDto'];

type HandleProjectDialogProps = {
	project?: Project | null;
	isOpen: boolean;
	onOpenChange: (open: boolean) => void;
	activeOrgId?: number;
	onSuccess?: () => void;
};

export const HandleProjectDialog: React.FC<HandleProjectDialogProps> = ({
	project,
	isOpen,
	onOpenChange,
	activeOrgId,
	onSuccess,
}) => {
	const isEditing = !!project;
	const [name, setName] = React.useState('');
	const [description, setDescription] = React.useState('');
	const [selectedTags, setSelectedTags] = React.useState<string[]>([]);
	const [isSubmitting, setIsSubmitting] = React.useState(false);

	const createMutation = $api.useMutation('post', '/projects');
	const updateMutation = $api.useMutation('patch', '/projects/{id}');

	// Reset & populate form when modal opens or project changes
	React.useEffect(() => {
		if (isOpen) {
			if (project) {
				setName(project.name || '');
				// Extract clean description (excluding hashtag block)
				const cleanDesc = (project.description || '')
					.replace(/#[\w-]+/g, '')
					.trim();
				setDescription(cleanDesc);

				// Extract existing tags
				const tags = getTagsFromDescription(project.description || '');
				setSelectedTags(tags);
			} else {
				setName('');
				setDescription('');
				setSelectedTags([]);
			}
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
					params: {path: {id: project.id}},
					body: {
						name: name.trim(),
						description: finalDescription || undefined,
					},
				});
				toast.success('Project updated successfully');
			} else {
				if (!activeOrgId) {
					toast.error('Active organization required');
					setIsSubmitting(false);
					return;
				}
				await createMutation.mutateAsync({
					body: {
						name: name.trim(),
						description: finalDescription || undefined,
						env_var: '',
						organization_id: activeOrgId,
					},
				});
				toast.success('Project created successfully');
			}

			onOpenChange(false);
			onSuccess?.();
		} catch (err: any) {
			toast.error(
				formatApiError(
					err,
					isEditing
						? 'Failed to update project'
						: 'Failed to create project',
				),
			);
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={onOpenChange}>
			<DialogContent className="border-border bg-card shadow-xl sm:max-w-lg">
				<DialogHeader>
					<DialogTitle className="text-xl font-bold text-foreground">
						{isEditing ? 'Update project' : 'Add a project'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						The home of something big!
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="space-y-4 pt-2">
					{/* Name */}
					<div className="flex flex-col gap-1.5">
						<Label
							htmlFor="name"
							className="text-xs font-semibold text-foreground">
							Name
						</Label>
						<Input
							id="name"
							placeholder="Vandelay Industries"
							value={name}
							onChange={e => setName(e.target.value)}
							disabled={isSubmitting}
							required
							className="h-9 border-border bg-card text-xs"
						/>
					</div>

					{/* Description Textarea (Shadcn UI Textarea with Dokploy exact min-h-[96px] styling) */}
					<div className="flex flex-col gap-1.5">
						<Label
							htmlFor="description"
							className="text-xs font-semibold text-foreground">
							Description
						</Label>
						<Textarea
							id="description"
							placeholder="Description about your project..."
							value={description}
							onChange={e => setDescription(e.target.value)}
							disabled={isSubmitting}
							className="min-h-[96px] border-border bg-card text-xs placeholder:text-muted-foreground focus-visible:ring-1 focus-visible:ring-ring"
						/>
					</div>

					{/* Multiselect Tags Dropdown */}
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold text-foreground">
							Tags
						</Label>
						<TagSelector
							selectedTags={selectedTags}
							onTagsChange={setSelectedTags}
							placeholder="Select tags..."
							disabled={isSubmitting}
						/>
					</div>

					{/* Dialog Footer */}
					<div className="mt-4 flex items-center justify-end border-t border-border/40 pt-3">
						<Button
							type="submit"
							disabled={isSubmitting || !name.trim()}
							className="h-9 cursor-pointer gap-2 bg-primary px-4 text-xs font-bold text-primary-foreground shadow-xs hover:bg-primary/90">
							{isSubmitting ? (
								<>
									<Loader2 className="size-4 animate-spin" />
									<span>{isEditing ? 'Updating...' : 'Creating...'}</span>
								</>
							) : (
								<span>{isEditing ? 'Update' : 'Create'}</span>
							)}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
};
