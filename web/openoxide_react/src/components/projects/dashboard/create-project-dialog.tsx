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
	const [envVar, setEnvVar] = React.useState('');

	// Reset form when modal opens/closes
	React.useEffect(() => {
		if (isOpen) {
			setName('');
			setDescription('');
			setEnvVar('');
		}
	}, [isOpen]);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		await onSubmit(name, description, envVar);
	};

	return (
		<Dialog
			open={isOpen}
			onOpenChange={onOpenChange}>
			<DialogContent className="sm:max-w-md bg-card border-border">
				<DialogHeader>
					<DialogTitle className="text-xl font-bold">Create New Project</DialogTitle>
					<DialogDescription className="text-muted-foreground text-sm">
						Provide a name, optional description, and environment variables for your new project.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit}>
					<div className="grid gap-5 py-4">
						<div className="flex flex-col gap-2">
							<Label
								htmlFor="name"
								className="text-sm font-semibold text-foreground">
								Project Name
							</Label>
							<Input
								id="name"
								placeholder="e.g. My Production API"
								value={name}
								onChange={e => setName(e.target.value)}
								disabled={isSubmitting}
								required
								className="bg-card border-border"
							/>
						</div>

						<div className="flex flex-col gap-2">
							<Label
								htmlFor="description"
								className="text-sm font-semibold text-foreground">
								Description
							</Label>
							<textarea
								id="description"
								placeholder="Describe the purpose of this project..."
								value={description}
								onChange={e => setDescription(e.target.value)}
								disabled={isSubmitting}
								rows={3}
								className="flex w-full rounded-md border border-input bg-card px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 border-border resize-none"
							/>
						</div>

						<div className="flex flex-col gap-2">
							<Label
								htmlFor="envVar"
								className="text-sm font-semibold text-foreground">
								Environment Variables
							</Label>
							<textarea
								id="envVar"
								placeholder="KEY=VALUE&#10;PORT=8080"
								value={envVar}
								onChange={e => setEnvVar(e.target.value)}
								disabled={isSubmitting}
								rows={4}
								className="flex w-full rounded-md border border-input bg-card px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 border-border font-mono resize-none"
							/>
						</div>
					</div>

					<div className="flex justify-end mt-2">
						<Button
							type="submit"
							disabled={isSubmitting}
							className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-2">
							{isSubmitting ? (
								<>
									<Loader2 className="animate-spin size-4" />
									Creating...
								</>
							) : (
								'Create Project'
							)}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
};
