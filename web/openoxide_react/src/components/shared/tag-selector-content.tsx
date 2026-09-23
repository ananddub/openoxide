import {useEffect, useRef} from 'react';
import {Check, Plus} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {cn} from '#/api/utils';
import type {TagItem} from './tag-selector';

interface TagSelectorPanelProps {
	searchQuery: string;
	setSearchQuery: (v: string) => void;
	availableTags: TagItem[];
	filteredTags: TagItem[];
	selectedTags: string[];
	onToggleTag: (name: string) => void;
	onClearAll: () => void;
}

/** Search + option list panel rendered inside the tag selector popover. */
export function TagSelectorPanel({
	searchQuery,
	setSearchQuery,
	availableTags,
	filteredTags,
	selectedTags,
	onToggleTag,
	onClearAll,
}: TagSelectorPanelProps) {
	const inputRef = useRef<HTMLInputElement>(null);
	useEffect(() => {
		// The popover mounts fresh on each open; land the caret in search.
		inputRef.current?.focus({preventScroll: true});
	}, []);

	const query = searchQuery.trim();
	const hasExactMatch = availableTags.some(
		t => t.name.toLowerCase() === query.toLowerCase(),
	);
	const showAddRow = query.length > 0 && !hasExactMatch;

	return (
		<div className="flex flex-col">
			{/* Search */}
			<div className="sticky top-0 z-10 flex items-center gap-2 border-b border-border bg-popover p-2">
				<Input
					ref={inputRef}
					placeholder="Search tags..."
					value={searchQuery}
					onChange={e => setSearchQuery(e.target.value)}
					className="h-8 flex-1 bg-muted/40 text-[13px]"
				/>
				{selectedTags.length > 0 && (
					<button
						type="button"
						onClick={onClearAll}
						className="h-8 shrink-0 cursor-pointer rounded-md px-2 text-[13px] font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-foreground">
						Clear
					</button>
				)}
			</div>

			{/* Options */}
			<div className="flex max-h-56 flex-col gap-px overflow-y-auto p-1.5">
				{filteredTags.length > 0 ? (
					filteredTags.map(tag => {
						const isSelected = selectedTags.includes(
							tag.name.toLowerCase(),
						);
						return (
							<button
								key={tag.id}
								type="button"
								onClick={() => onToggleTag(tag.name)}
								className={cn(
									'flex w-full cursor-pointer items-center justify-between gap-2 rounded-md px-2 py-1.5 text-left text-[13px] outline-none transition-colors hover:bg-accent focus-visible:bg-accent',
									isSelected && 'font-medium',
								)}>
								<span className="flex min-w-0 items-center gap-2">
									<span
										className="size-2 shrink-0 rounded-full ring-1 ring-black/10 ring-inset dark:ring-white/15"
										style={{backgroundColor: tag.color || '#3b82f6'}}
									/>
									<span className="truncate text-foreground">
										{tag.name}
									</span>
								</span>
								{isSelected && (
									<Check className="size-3.5 shrink-0 text-foreground" />
								)}
							</button>
						);
					})
				) : !showAddRow ? (
					<div className="px-2 py-5 text-center text-[13px] text-muted-foreground">
						{availableTags.length === 0
							? 'No tags yet — create some in Settings.'
							: 'No tags match your search.'}
					</div>
				) : null}
			</div>

			{/* Create-as-you-type */}
			{showAddRow && (
				<button
					type="button"
					onClick={() => {
						onToggleTag(query);
						setSearchQuery('');
					}}
					className="flex w-full cursor-pointer items-center gap-2 border-t border-border px-3 py-2.5 text-left text-[13px] font-medium text-foreground outline-none transition-colors hover:bg-accent focus-visible:bg-accent">
					<Plus className="size-3.5 shrink-0 text-muted-foreground" />
					<span className="truncate">
						Add “{query}” as a new tag
					</span>
				</button>
			)}
		</div>
	);
}
