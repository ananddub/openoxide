import {useState} from 'react';
import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {ArrowLeft, Tag as TagIcon} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useTagCreate} from '#/api/live';
import {TagBadge} from '#/components/shared/tag-badge';

export const Route = createFileRoute('/_app/tags/new')({
	component: TagCreatePage,
});

const PRESET_COLORS = [
	'#3b82f6', // Blue
	'#10b981', // Green
	'#f59e0b', // Amber
	'#ef4444', // Red
	'#8b5cf6', // Purple
	'#ec4899', // Pink
	'#06b6d4', // Cyan
	'#64748b', // Slate
];

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
			{/* ── Page header ── */}
			<div className="flex items-center gap-3">
				<button
					type="button"
					onClick={() => void navigate({to: '/tags'})}
					title="Back to Tags"
					className="flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-foreground/[0.06] hover:text-foreground">
					<ArrowLeft className="size-4" />
				</button>
				<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted">
					<TagIcon className="size-4 text-foreground/70" />
				</div>
				<div className="min-w-0">
					<h1 className="truncate text-2xl font-medium tracking-tight text-foreground/80">
						New Tag
					</h1>
					<p className="text-sm text-muted-foreground/70">
						Color-coded label for organizing projects and resources.
					</p>
				</div>
			</div>

			{/* ── Form card ── */}
			<div className="mx-auto w-full min-w-0 max-w-xl rounded-xl bg-card p-6 ring-1 ring-border/50">
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
						<div className="flex flex-wrap items-center gap-2">
							{PRESET_COLORS.map(c => (
								<button
									key={c}
									type="button"
									onClick={() => setTagColor(c)}
									aria-label={`Use ${c}`}
									className={`size-6 cursor-pointer rounded-full transition-transform ${
										tagColor === c
											? 'scale-125 ring-2 ring-primary/60'
											: 'opacity-75 hover:opacity-100'
									}`}
									style={{backgroundColor: c}}
								/>
							))}
						</div>
						<div className="mt-1 flex items-center gap-2">
							<span className="font-mono text-[11px] text-muted-foreground">
								Hex:
							</span>
							<Input
								type="text"
								value={tagColor}
								onChange={e => setTagColor(e.target.value)}
								className="h-8 w-28 bg-muted/20 font-mono text-[11px]"
							/>
						</div>
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
