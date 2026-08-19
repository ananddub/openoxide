import {useState, useEffect, useRef} from 'react';
import {Palette} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {TagBadge} from '#/components/shared/tag-badge';
import {toast} from 'sonner';
import {$api} from '#/api/query';

interface TagItem {
	id: number;
	name: string;
	color: string;
}

interface HandleTagModalProps {
	tag?: TagItem | null;
	isOpen: boolean;
	onClose: () => void;
	onSuccess: () => void;
}

export function HandleTagModal({
	tag,
	isOpen,
	onClose,
	onSuccess,
}: HandleTagModalProps) {
	const [name, setName] = useState('');
	const [color, setColor] = useState('#3b82f6');
	const [isSubmitting, setIsSubmitting] = useState(false);
	const colorInputRef = useRef<HTMLInputElement>(null);

	const createMutation = $api.useMutation('post', '/tags' as any);
	const patchMutation = $api.useMutation('patch', '/tags/{id}' as any);

	useEffect(() => {
		if (tag) {
			setName(tag.name || '');
			setColor(tag.color || '#3b82f6');
		} else {
			setName('');
			setColor('#3b82f6');
		}
	}, [tag, isOpen]);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		const trimmedName = name.trim();
		if (!trimmedName) {
			toast.error('Tag name is required');
			return;
		}

		setIsSubmitting(true);
		try {
			if (tag?.id) {
				await patchMutation.mutateAsync({
					params: {path: {id: tag.id}},
					body: {
						name: trimmedName,
						color: color || '#3b82f6',
					},
				});
				toast.success('Tag Updated');
			} else {
				await createMutation.mutateAsync({
					body: {
						name: trimmedName,
						color: color || '#3b82f6',
					},
				});
				toast.success('Tag Created');
			}
			onSuccess();
			onClose();
		} catch {
			toast.error(tag?.id ? 'Error updating tag' : 'Error creating tag');
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-lg">
				<DialogHeader>
					<DialogTitle>{tag ? 'Update' : 'Create'} Tag</DialogTitle>
					<DialogDescription>
						{tag
							? 'Update the tag name and color'
							: 'Create a new tag to organize your projects'}
					</DialogDescription>
				</DialogHeader>

				<form
					id="hook-form-tag"
					onSubmit={handleSubmit}
					className="grid w-full gap-4">
					{/* Name Field */}
					<div className="flex flex-col gap-2">
						<label className="text-sm leading-none font-medium peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
							Name
						</label>
						<Input
							placeholder="e.g., Production, Client, Internal"
							value={name}
							onChange={e => setName(e.target.value)}
							required
							autoFocus
						/>
					</div>

					{/* Color Field */}
					<div className="flex flex-col gap-2">
						<label className="text-sm leading-none font-medium peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
							Color (Optional)
						</label>
						<div className="flex items-center gap-3">
							<label
								className="relative flex h-12 w-12 cursor-pointer items-center justify-center rounded-md border-2 transition-opacity hover:opacity-80"
								style={{
									backgroundColor: color || '#3b82f6',
								}}
								onClick={() => colorInputRef.current?.click()}>
								<div className="flex items-center justify-center">
									{!color && <Palette className="h-5 w-5 text-white" />}
								</div>
								<input
									ref={colorInputRef}
									type="color"
									className="pointer-events-none absolute top-0 left-0 h-12 w-12 opacity-0"
									value={color || '#3b82f6'}
									onChange={e => setColor(e.target.value)}
								/>
							</label>
							<div className="flex-1">
								<Input
									placeholder="#3b82f6"
									value={color || ''}
									onChange={e => {
										const value = e.target.value;
										if (value.startsWith('#') || value === '') {
											setColor(value);
										}
									}}
								/>
								<p className="mt-1 text-xs text-muted-foreground">
									Choose a color to easily identify this tag
								</p>
							</div>
						</div>
					</div>

					{/* Live Preview */}
					{color && (
						<div className="flex items-center gap-2">
							<span className="text-sm text-muted-foreground">
								Preview:
							</span>
							<TagBadge name={name || 'Tag Name'} color={color} />
						</div>
					)}
				</form>

				<DialogFooter>
					<Button
						disabled={isSubmitting}
						form="hook-form-tag"
						type="submit">
						{tag ? 'Update' : 'Create'}
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
