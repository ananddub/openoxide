import {Check, Plus} from 'lucide-react';
import {TagBadge} from '#/components/shared/tag-badge';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {DropdownMenuContent} from '#/components/ui/dropdown';
import {cn} from '#/api/utils';
import type {TagItem} from './tag-selector';

interface TagSelectorContentProps {
	searchQuery: string;
	setSearchQuery: (v: string) => void;
	availableTags: TagItem[];
	filteredTags: TagItem[];
	selectedTags: string[];
	onToggleTag: (name: string) => void;
	onClearAll: (e: React.MouseEvent) => void;
}

/** Search + option list panel rendered inside the tag selector dropdown. */
export function TagSelectorContent({
	searchQuery,
	setSearchQuery,
	availableTags,
	filteredTags,
	selectedTags,
	onToggleTag,
	onClearAll,
}: TagSelectorContentProps) {
	return (
		<DropdownMenuContent
			className="w-[280px] rounded-lg bg-popover p-1.5"
			align="start">
			<div
				className="flex flex-col gap-2"
				onClick={e => e.stopPropagation()}>
				<div className="flex items-center justify-between gap-2 border-b px-1 pb-2">
					<Input
						placeholder="Search tags..."
						value={searchQuery}
						onChange={e => setSearchQuery(e.target.value)}
						className="h-8 bg-muted/40 text-xs"
					/>
					{selectedTags.length > 0 && (
						<Button
							variant="ghost"
							size="sm"
							onClick={onClearAll}
							className="h-8 px-2 text-xs font-semibold text-muted-foreground hover:text-foreground">
							Clear
						</Button>
					)}
				</div>

				<div className="flex max-h-52 flex-col gap-1 overflow-y-auto pr-1">
					{filteredTags.length > 0 ? (
						filteredTags.map(tag => {
							const isSelected = selectedTags.includes(
								tag.name.toLowerCase(),
							);
							return (
								<div
									key={tag.id}
									onClick={() => onToggleTag(tag.name)}
									className={cn(
										'flex cursor-pointer items-center justify-between rounded-md px-2 py-1.5 text-sm transition-colors hover:bg-muted/60',
										isSelected && 'bg-muted/40 font-medium',
									)}>
									<TagBadge
										name={tag.name}
										color={tag.color}
										className="text-[11px]"
									/>
									{isSelected && (
										<Check className="size-4 shrink-0 text-primary" />
									)}
								</div>
							);
						})
					) : (
						<div className="py-4 text-center text-xs text-muted-foreground">
							{availableTags.length === 0
								? 'No tags created in settings.'
								: 'No tags found.'}
						</div>
					)}
				</div>

				{searchQuery.trim() &&
					!availableTags.some(
						t => t.name.toLowerCase() === searchQuery.trim().toLowerCase(),
					) && (
						<Button
							variant="ghost"
							size="sm"
							onClick={() => {
								onToggleTag(searchQuery.trim());
								setSearchQuery('');
							}}
							className="mt-1 h-8 justify-start gap-1.5 rounded-none border-t pt-2 text-xs font-semibold text-primary">
							<Plus className="size-3.5" />
							<span>Add "#{searchQuery.trim()}"</span>
						</Button>
					)}
			</div>
		</DropdownMenuContent>
	);
}
