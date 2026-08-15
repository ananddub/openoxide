import * as React from 'react';
import { Loader2, X, Tag } from 'lucide-react';
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
import { Badge } from '#/components/ui/badge';

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
	const [tags, setTags] = React.useState<string[]>([]);
	const [tagInput, setTagInput] = React.useState('');

	// Reset form when modal opens/closes
	React.useEffect(() => {
		if (isOpen) {
			setName('');
			setDescription('');
			setTags([]);
			setTagInput('');
		}
	}, [isOpen]);

	const handleAddTag = (tagToAdd: string) => {
		const cleanTag = tagToAdd.trim().replace(/^#/, '').toLowerCase();
		if (cleanTag && !tags.includes(cleanTag)) {
			setTags([...tags, cleanTag]);
		}
		setTagInput('');
	};

	const handleRemoveTag = (tagToRemove: string) => {
		setTags(tags.filter((t) => t !== tagToRemove));
	};

	const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
		if (e.key === 'Enter' || e.key === ',') {
			e.preventDefault();
			handleAddTag(tagInput);
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		let finalDescription = description.trim();
		if (tags.length > 0) {
			const hashtagsString = tags.map((t) => `#${t}`).join(' ');
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

					{/* Tags */}
					<div className="flex flex-col gap-1.5">
						<div className="flex items-center justify-between">
							<Label htmlFor="tags" className="text-xs font-semibold text-foreground flex items-center gap-1.5">
								<Tag className="size-3.5 text-muted-foreground" />
								<span>Tags</span>
							</Label>
							<span className="text-[10px] text-muted-foreground">Press Enter or comma to add</span>
						</div>

						<div className="flex flex-col gap-2 border border-border/80 bg-background/60 rounded-md p-2 min-h-[42px]">
							{tags.length > 0 && (
								<div className="flex flex-wrap gap-1.5">
									{tags.map((t) => (
										<Badge key={t} variant="secondary" className="text-[10px] font-semibold px-2 py-0.5 gap-1 bg-muted/80 border border-border/40">
											<span>#{t}</span>
											<button
												type="button"
												onClick={() => handleRemoveTag(t)}
												className="text-muted-foreground hover:text-foreground shrink-0 rounded-full cursor-pointer"
											>
												<X className="size-3" />
											</button>
										</Badge>
									))}
								</div>
							)}

							<Input
								id="tags"
								placeholder={tags.length === 0 ? "Add tags (e.g. prod, api)..." : "Add more tags..."}
								value={tagInput}
								onChange={(e) => setTagInput(e.target.value)}
								onKeyDown={handleKeyDown}
								onBlur={() => tagInput.trim() && handleAddTag(tagInput)}
								disabled={isSubmitting}
								className="h-7 text-xs border-0 bg-transparent p-0 focus-visible:ring-0 shadow-none placeholder:text-muted-foreground/60"
							/>
						</div>

						{/* Quick Tag Suggestions */}
						<div className="flex items-center gap-1.5 flex-wrap pt-0.5">
							<span className="text-[10px] text-muted-foreground font-medium">Quick suggestions:</span>
							{['prod', 'staging', 'api', 'frontend', 'backend', 'database'].map((sugg) => (
								<button
									key={sugg}
									type="button"
									onClick={() => handleAddTag(sugg)}
									disabled={tags.includes(sugg)}
									className="text-[10px] font-semibold px-2 py-0.5 rounded-md bg-muted/40 hover:bg-muted text-muted-foreground hover:text-foreground disabled:opacity-40 transition-colors border border-border/40 cursor-pointer"
								>
									+{sugg}
								</button>
							))}
						</div>
					</div>

					{/* Dialog Footer (Dokploy exact: Submit button only, no cancel button) */}
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
