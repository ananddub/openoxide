import {useState} from 'react';
import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {Tag as TagIcon} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Titlebar} from '#/components/shared/titlebar';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useTagCreate} from '#/api/live';
import {TagBadge} from '#/components/shared/tag-badge';
import {TagColorPicker} from '#/components/tags/tag-color-picker';

export const Route = createFileRoute('/_app/tags/new')({
	component: TagCreatePage,
});

function TagCreatePage() {
	const navigate = useNavigate();
	const [tagName, setTagName] = useState('');
	const [tagColor, setTagColor] = useState('#3b82f6');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const createMutation = useTagCreate();

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!tagName.trim()) return;
		setIsSubmitting(true);
		try {
			await createMutation.mutateAsync({
				name: tagName.trim(),
				color: tagColor,
			});
			toast.success('Tag created');
			void navigate({to: '/tags'});
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* ── Vercel-style titlebar ── */}
			<Titlebar
				breadcrumbs={[
					{label: 'Tags', to: '/tags', icon: TagIcon},
					{label: 'New Tag'},
				]}
				icon={<TagIcon className="text-foreground/70" />}
				title="New Tag"
				description="Color-coded label for organizing projects and resources."
			/>

			{/* ── Form card ── */}
			<div className="mx-auto w-full max-w-xl min-w-0 rounded-xl bg-card p-6 ring-1 ring-border/50">
				<form onSubmit={handleSubmit} className="flex flex-col gap-4">
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold">Name *</label>
						<Input
							placeholder="Tag name"
							value={tagName}
							onChange={e => setTagName(e.target.value)}
							required
							autoFocus
							className="h-9 bg-muted/20 text-xs"
						/>
					</div>

					<div className="flex flex-col gap-2">
						<label className="text-xs font-semibold">Color</label>
						<TagColorPicker color={tagColor} onChange={setTagColor} />
					</div>

					<div className="flex items-center justify-between rounded-lg bg-muted/20 px-3 py-2.5">
						<span className="text-[11px] text-muted-foreground">
							Preview:
						</span>
						<TagBadge
							name={tagName.trim() || 'Preview'}
							color={tagColor}
						/>
					</div>

					<div className="flex items-center justify-end gap-2 border-t border-border/50 pt-4">
						<Button
							type="button"
							variant="ghost"
							onClick={() => void navigate({to: '/tags'})}
							className="h-9 text-xs">
							Cancel
						</Button>
						<Button
							type="submit"
							disabled={isSubmitting}
							className="h-9 px-5 text-xs font-semibold">
							{isSubmitting ? 'Saving...' : 'Create Tag'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);
}
