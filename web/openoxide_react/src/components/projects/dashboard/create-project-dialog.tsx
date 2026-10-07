import * as React from 'react';
import {Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {Label} from '#/components/ui/label';
import {TagSelector} from '#/components/shared/tag-selector';
import {
	StepTimeline,
	type WizardStep,
} from '#/components/shared/sheet-wizard';

type CreateProjectDialogProps = {
	isOpen: boolean;
	onOpenChange: (open: boolean) => void;
	onSubmit: (
		name: string,
		description: string,
		envVar: string,
	) => Promise<void>;
	isSubmitting: boolean;
};

const STEPS: WizardStep[] = [
	{id: 'identity', label: 'Identity'},
	{id: 'details', label: 'Details'},
];

const FORM_ID = 'create-project-form';

const STEP_DESCRIPTIONS: Record<string, string> = {
	identity: 'Give the project a name — the home of something big!',
	details: 'Describe it and tag it so it stays findable.',
};

/** New Project — checkpoint wizard: Identity → Details, the same StepTimeline grammar as the new-server sheet. */
export const CreateProjectDialog: React.FC<CreateProjectDialogProps> = ({
	isOpen,
	onOpenChange,
	onSubmit,
	isSubmitting,
}) => {
	const [name, setName] = React.useState('');
	const [description, setDescription] = React.useState('');
	const [selectedTags, setSelectedTags] = React.useState<string[]>([]);
	const [stepIdx, setStepIdx] = React.useState(0);

	// Reset form when modal opens/closes
	React.useEffect(() => {
		if (isOpen) {
			setName('');
			setDescription('');
			setSelectedTags([]);
			setStepIdx(0);
		}
	}, [isOpen]);

	const last = stepIdx === STEPS.length - 1;
	const step = STEPS[stepIdx].id;

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		let finalDescription = description.trim();
		if (selectedTags.length > 0) {
			const hashtagsString = selectedTags.map(t => `#${t}`).join(' ');
			finalDescription = finalDescription
				? `${finalDescription}\n\n${hashtagsString}`
				: hashtagsString;
		}
		await onSubmit(name, finalDescription, '');
	};

	return (
		<Sheet open={isOpen} onOpenChange={onOpenChange}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>New Project</SheetTitle>
					<SheetDescription>{STEP_DESCRIPTIONS[step]}</SheetDescription>
				</SheetHeader>

				<form
					id={FORM_ID}
					onSubmit={handleSubmit}
					className="flex min-h-0 flex-1 flex-col">
					<div className="flex min-h-0 flex-1 flex-col overflow-y-auto px-6 py-6">
						<StepTimeline steps={STEPS} idx={stepIdx}>
							<div
								key={step}
								className="animate-in duration-200 ease-out fade-in-0 slide-in-from-right-4">
								{step === 'identity' ? (
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
											autoFocus
										/>
									</div>
								) : (
									<div className="flex flex-col gap-4">
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
												rows={3}
												className="min-h-0 resize-none"
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
									</div>
								)}
							</div>
						</StepTimeline>
					</div>

					{/* Dialog Footer */}
					<div className="flex shrink-0 items-center justify-between border-t px-6 py-4">
						{stepIdx > 0 ? (
							<Button
								type="button"
								variant="outline"
								onClick={() => setStepIdx(i => i - 1)}>
								Back
							</Button>
						) : (
							<span />
						)}
						{last ? (
							<Button
								type="submit"
								form={FORM_ID}
								disabled={isSubmitting || !name.trim()}
								className="h-9 cursor-pointer gap-2 bg-primary px-4 text-xs font-bold text-primary-foreground shadow-xs hover:bg-primary/90">
								{isSubmitting ? (
									<>
										<Loader2 className="size-4 animate-spin" />
										<span>Creating...</span>
									</>
								) : (
									<span>Create</span>
								)}
							</Button>
						) : (
							<Button
								type="button"
								disabled={!name.trim()}
								onClick={() => setStepIdx(i => i + 1)}>
								Continue
							</Button>
						)}
					</div>
				</form>
			</SheetContent>
		</Sheet>
	);
};
