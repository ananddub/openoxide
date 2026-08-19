import {Palette, PenBoxIcon, PlusIcon, Check} from 'lucide-react';
import {useEffect, useRef, useState} from 'react';
import {toast} from 'sonner';
import {AlertBlock} from '#/components/shared/alert-block';
import {TagBadge} from '#/components/shared/tag-badge';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from '#/components/ui/dialog';
import {Input} from '#/components/ui/input';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

const PRESET_COLORS = [
	'#3b82f6', // Blue
	'#ef4444', // Red
	'#10b981', // Green
	'#f59e0b', // Amber
	'#8b5cf6', // Purple
	'#ec4899', // Pink
	'#6366f1', // Indigo
	'#06b6d4', // Cyan
	'#64748b', // Slate
];

interface TagItem {
	id: number;
	name: string;
	color: string;
}

interface HandleTagProps {
	tagId?: number;
	tagData?: TagItem;
	onSuccess?: () => void;
}

export const HandleTag = ({tagId, tagData, onSuccess}: HandleTagProps) => {
	const [isOpen, setIsOpen] = useState(false);
	const [name, setName] = useState('');
	const [color, setColor] = useState('#3b82f6');
	const [isSubmitting, setIsSubmitting] = useState(false);
	const [errorMessage, setErrorMessage] = useState<string | null>(null);
	const colorInputRef = useRef<HTMLInputElement>(null);

	const createMutation = $api.useMutation('post', '/tags' as any);
	const patchMutation = $api.useMutation('patch', '/tags/{id}' as any);

	useEffect(() => {
		if (isOpen) {
			setErrorMessage(null);
			if (tagData) {
				setName(tagData.name || '');
				setColor(tagData.color || '#3b82f6');
			} else {
				setName('');
				setColor('#3b82f6');
			}
		}
	}, [tagData, isOpen]);

	const onSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		const trimmedName = name.trim();
		if (!trimmedName) {
			setErrorMessage('Tag name is required');
			return;
		}

		setIsSubmitting(true);
		setErrorMessage(null);
		try {
			if (tagId) {
				await patchMutation.mutateAsync({
					params: {path: {id: tagId}},
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
			setIsOpen(false);
			onSuccess?.();
		} catch (err: any) {
			const msg =
				formatApiError(err) ||
				(tagId ? 'Error updating tag' : 'Error creating tag');
			setErrorMessage(msg);
			toast.error(tagId ? 'Error updating tag' : 'Error creating tag');
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={setIsOpen}>
			<DialogTrigger
				render={
					tagId ? (
						<Button
							variant="ghost"
							size="icon"
							className="h-8 w-8 text-muted-foreground hover:text-foreground">
							<PenBoxIcon className="h-4 w-4" />
						</Button>
					) : (
						<Button
							size="sm"
							className="h-9 gap-1.5 bg-primary px-4 text-xs font-semibold text-primary-foreground shadow-xs hover:bg-primary/90">
							<PlusIcon className="h-3.5 w-3.5" />
							Create Tag
						</Button>
					)
				}
			/>
			<DialogContent className="rounded-2xl border border-border/80 bg-card p-6 shadow-2xl sm:max-w-md">
				<DialogHeader className="space-y-1">
					<DialogTitle className="text-base font-bold text-foreground">
						{tagId ? 'Update' : 'Create'} Tag
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						{tagId
							? 'Update the tag name and color'
							: 'Create a new tag to organize your projects'}
					</DialogDescription>
				</DialogHeader>
				{errorMessage && (
					<AlertBlock type="error">{errorMessage}</AlertBlock>
				)}

				<form
					id="hook-form-tag"
					onSubmit={onSubmit}
					className="flex flex-col gap-4 py-1">
					{/* Name Field */}
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">
							Name
						</label>
						<Input
							placeholder="e.g., Production, Client, Internal"
							value={name}
							onChange={e => setName(e.target.value)}
							required
							autoFocus
							className="h-9 text-xs"
						/>
					</div>

					{/* Color Field */}
					<div className="flex flex-col gap-2">
						<label className="text-xs font-semibold text-foreground">
							Color (Optional)
						</label>
						<div className="flex items-center gap-3">
							<label
								className="relative flex size-9 shrink-0 cursor-pointer items-center justify-center rounded-lg border border-border shadow-2xs transition-opacity hover:opacity-80"
								style={{
									backgroundColor: color || '#3b82f6',
								}}
								onClick={() => colorInputRef.current?.click()}>
								<div className="flex items-center justify-center">
									{!color && <Palette className="h-4 w-4 text-white" />}
								</div>
								<input
									ref={colorInputRef}
									type="color"
									className="pointer-events-none absolute top-0 left-0 size-9 opacity-0"
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
									className="h-9 font-mono text-xs"
								/>
							</div>
						</div>

						{/* Quick Preset Colors */}
						<div className="flex flex-wrap items-center gap-1.5 pt-1">
							{PRESET_COLORS.map(c => (
								<button
									key={c}
									type="button"
									onClick={() => setColor(c)}
									className={`flex size-6 items-center justify-center rounded-full border border-black/10 transition-transform ${
										color === c
											? 'scale-110 ring-2 ring-primary ring-offset-1'
											: 'opacity-80 hover:scale-105 hover:opacity-100'
									}`}
									style={{backgroundColor: c}}>
									{color === c && (
										<Check className="h-3 w-3 text-white drop-shadow-xs" />
									)}
								</button>
							))}
						</div>
					</div>

					{/* Live Preview */}
					<div className="mt-1 flex items-center justify-between rounded-lg border border-border/60 bg-muted/20 p-3">
						<span className="text-xs font-medium text-muted-foreground">
							Preview Badge:
						</span>
						<TagBadge name={name.trim() || 'Tag Name'} color={color} />
					</div>
				</form>

				<DialogFooter className="gap-2 border-t border-border/60 pt-2 sm:gap-0">
					<Button
						type="button"
						variant="outline"
						onClick={() => setIsOpen(false)}
						className="h-9 text-xs font-semibold">
						Cancel
					</Button>
					<Button
						disabled={isSubmitting}
						form="hook-form-tag"
						type="submit"
						className="h-9 bg-primary px-5 text-xs font-bold text-primary-foreground hover:bg-primary/90">
						{isSubmitting ? 'Saving...' : tagId ? 'Update' : 'Create'}
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
};
