import React, {useState, useMemo} from 'react';
import {ChevronsUpDown, X, Tags as TagsIcon} from 'lucide-react';
import {useTagListAll} from '#/api/live';
import {TagBadge} from '#/components/shared/tag-badge';
import {Button} from '#/components/ui/button';
import {DropdownMenu, DropdownMenuTrigger} from '#/components/ui/dropdown';
import {cn} from '#/api/utils';
import {TagSelectorContent} from './tag-selector-content';

export interface TagItem {
	id: number;
	name: string;
	color: string;
}

interface TagSelectorProps {
	selectedTags: string[]; // Tag names
	onTagsChange: (tags: string[]) => void;
	placeholder?: string;
	disabled?: boolean;
	variant?: 'form' | 'filter';
}

export function TagSelector({
	selectedTags,
	onTagsChange,
	placeholder = 'Tags',
	disabled = false,
	variant = 'form',
}: TagSelectorProps) {
	const [searchQuery, setSearchQuery] = useState('');
	// Live tags query keyed ['tags','list'] — table-watch invalidates it
	// while this component is mounted.
	const rawTags = useTagListAll().data;

	const availableTags: TagItem[] = useMemo(() => {
		return Array.isArray(rawTags) ? (rawTags as unknown as TagItem[]) : [];
	}, [rawTags]);

	const handleToggleTag = (tagName: string) => {
		const clean = tagName.toLowerCase();
		if (selectedTags.includes(clean)) {
			onTagsChange(selectedTags.filter(t => t !== clean));
		} else {
			onTagsChange([...selectedTags, clean]);
		}
	};

	const handleRemoveTag = (tagName: string, e?: React.MouseEvent) => {
		e?.stopPropagation();
		onTagsChange(
			selectedTags.filter(t => t.toLowerCase() !== tagName.toLowerCase()),
		);
	};

	const handleClearAll = (e: React.MouseEvent) => {
		e.stopPropagation();
		onTagsChange([]);
	};

	const filteredTags = useMemo(() => {
		if (!searchQuery.trim()) return availableTags;
		const q = searchQuery.toLowerCase();
		return availableTags.filter(t => t.name.toLowerCase().includes(q));
	}, [availableTags, searchQuery]);

	// Match selected tags with available tag objects to get exact custom colors
	const selectedTagObjects = useMemo(() => {
		return selectedTags.map(st => {
			const found = availableTags.find(
				t => t.name.toLowerCase() === st.toLowerCase(),
			);
			return {
				name: st,
				color: found?.color || '#3b82f6',
			};
		});
	}, [selectedTags, availableTags]);

	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<Button
						variant="outline"
						disabled={disabled}
						className={cn(
							'h-auto min-h-11 w-full cursor-pointer justify-between rounded-xl bg-card px-3.5 py-2 text-sm',
							selectedTags.length > 0 && 'text-primary',
							disabled && 'cursor-not-allowed opacity-50',
						)}>
						{variant === 'filter' ? (
							<div className="flex min-w-0 flex-1 items-center gap-2">
								<TagsIcon className="size-4 shrink-0 text-muted-foreground" />
								<span>
									{placeholder}
									{selectedTags.length > 0
										? ` (${selectedTags.length})`
										: ''}
								</span>
							</div>
						) : (
							<div className="flex min-w-0 flex-1 flex-wrap items-center gap-1.5">
								{selectedTagObjects.length > 0 ? (
									selectedTagObjects.map(t => (
										<TagBadge
											key={t.name}
											name={t.name}
											color={t.color}
											className="px-2 py-0.5 text-[11px] font-semibold">
											<button
												type="button"
												onClick={e => handleRemoveTag(t.name, e)}
												className="ml-1 cursor-pointer rounded-full text-muted-foreground hover:text-foreground">
												<X className="size-3" />
											</button>
										</TagBadge>
									))
								) : (
									<span className="flex items-center gap-1.5 text-xs font-normal text-muted-foreground">
										<TagsIcon className="size-3.5" />
										{placeholder}
									</span>
								)}
							</div>
						)}
						<ChevronsUpDown className="ml-2 size-4 shrink-0 opacity-50" />
					</Button>
				}
			/>

			<TagSelectorContent
				searchQuery={searchQuery}
				setSearchQuery={setSearchQuery}
				availableTags={availableTags}
				filteredTags={filteredTags}
				selectedTags={selectedTags}
				onToggleTag={handleToggleTag}
				onClearAll={handleClearAll}
			/>
		</DropdownMenu>
	);
}
