import * as React from 'react';
import { Loader2 } from 'lucide-react';
import { Button } from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import { Input } from '#/components/ui/input';
import { Label } from '#/components/ui/label';
import { TagSelector } from '#/components/shared/tag-selector';

type CreateProjectDialogProps = {
	isOpen: boolean;
	onOpenChange: (open: boolean) => void;
	onSubmit: (name: string, description: string, envVar: string) => Promise<void>;
	isSubmitting: boolean;
};

export const CreateProjectDialog: React.FC<CreateProjectDialogProps> = ({
	isOpen,
	onOpenChange,
	onSubmit,
	isSubmitting,
}) => {
	const [name, setName] = React.useState('');
	const [description, setDescription] = React.useState('');
	const [selectedTags, setSelectedTags] = React.useState<string[]>([]);

	// Reset form when modal opens/closes
	React.useEffect(() => {
		if (isOpen) {
			setName('');
			setDescription('');
			setSelectedTags([]);
		}
	}, [isOpen]);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		let finalDescription = description.trim();
		if (selectedTags.length > 0) {
			const hashtagsString = selectedTags.map((t) => `#${t}`).join(' ');
			finalDescription = finalDescription ? `${finalDescription}\n\n${hashtagsString}` : hashtagsString;
		}
		await onSubmit(name, finalDescription, '');
	};

	return (
		<Dialog open={isOpen} onOpenChange={onOpenChange}>
			<DialogContent className="sm:max-w-lg bg-card border-border shadow-xl">
				<DialogHeader>
					<DialogTitle className="text-xl font-bold text-foreground">Add a project</DialogTitle>
					<DialogDescription className="text-muted-foreground text-xs">
						The home of something big!
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="space-y-4 pt-2">
					{/* Name */}
					<div className="flex flex-col gap-1.5">
						<Label htmlFor="name" className="text-xs font-semibold text-foreground">
							Name
						</Label>
						<Input
							id="name"
							placeholder="Vandelay Industries"
							value={name}
							onChange={(e) => setName(e.target.value)}
							disabled={isSubmitting}
							required
							className="bg-card border-border text-xs h-9"
						/>
					</div>

					{/* Description */}
					<div className="flex flex-col gap-1.5">
						<Label htmlFor="description" className="text-xs font-semibold text-foreground">
							Description
						</Label>
						<textarea
							id="description"
							placeholder="Description about your project..."
							value={description}
							onChange={(e) => setDescription(e.target.value)}
							disabled={isSubmitting}
							rows={3}
							className="flex w-full rounded-md border border-input bg-card px-3 py-2 text-xs ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 border-border resize-none"
						/>
					</div>

					{/* Multiselect Tags Dropdown (with custom colors from Settings Tags) */}
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
					<div className="flex items-center justify-end pt-3 border-t border-border/40 mt-4">
						<Button
							type="submit"
							disabled={isSubmitting || !name.trim()}
							className="text-xs font-bold px-4 h-9 gap-2 bg-primary hover:bg-primary/90 text-primary-foreground shadow-xs cursor-pointer"
						>
							{isSubmitting ? (
								<>
									<Loader2 className="size-4 animate-spin" />
									<span>Creating...</span>
								</>
							) : (
								<span>Create</span>
							)}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
};
