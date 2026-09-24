import * as React from 'react';
import {Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Textarea} from '#/components/ui/textarea';
import {TagSelector} from '#/components/shared/tag-selector';
import {useProjectForm} from './use-project-form';
type Project = Record<string, any>;

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
	const {
		isEditing,
		name,
		setName,
		description,
		setDescription,
		selectedTags,
		setSelectedTags,
		isSubmitting,
		handleSubmit,
	} = useProjectForm({
		project,
		isOpen,
		activeOrgId,
		onOpenChange,
		onSuccess,
	});

	return (
		<Sheet open={isOpen} onOpenChange={onOpenChange}>
			<SheetContent side="right">
				<SheetHeader>
					<SheetTitle>
						{isEditing ? 'Update project' : 'Add a project'}
					</SheetTitle>
					<SheetDescription>The home of something big!</SheetDescription>
				</SheetHeader>

				<form
					id="project-form"
					onSubmit={handleSubmit}
					className="flex flex-1 flex-col gap-4 overflow-y-auto px-6 py-6">
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
						/>
					</div>

					{/* Description */}
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
				</form>

				<SheetFooter>
					<Button
						type="submit"
						form="project-form"
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
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
};
