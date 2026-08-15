import React, { useState, useMemo } from 'react';
import { Check, ChevronsUpDown, X, Tags as TagsIcon, Plus } from 'lucide-react';
import { useAppStore } from '#/stores/app-store';
import { TagBadge } from '#/components/shared/tag-badge';
import { Button } from '#/components/ui/button';
import { Input } from '#/components/ui/input';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
} from '#/components/ui/dropdown';
import { cn } from '#/api/utils';

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
	const rawTags = useAppStore((state) => state.tags || []);

	const availableTags: TagItem[] = useMemo(() => {
		return Array.isArray(rawTags) ? (rawTags as unknown as TagItem[]) : [];
	}, [rawTags]);

	const handleToggleTag = (tagName: string) => {
		const clean = tagName.toLowerCase();
		if (selectedTags.includes(clean)) {
			onTagsChange(selectedTags.filter((t) => t !== clean));
		} else {
			onTagsChange([...selectedTags, clean]);
		}
	};

	const handleRemoveTag = (tagName: string, e?: React.MouseEvent) => {
		e?.stopPropagation();
		onTagsChange(selectedTags.filter((t) => t.toLowerCase() !== tagName.toLowerCase()));
	};

	const handleClearAll = (e: React.MouseEvent) => {
		e.stopPropagation();
		onTagsChange([]);
	};

	const filteredTags = useMemo(() => {
		if (!searchQuery.trim()) return availableTags;
		const q = searchQuery.toLowerCase();
		return availableTags.filter((t) => t.name.toLowerCase().includes(q));
	}, [availableTags, searchQuery]);

	// Match selected tags with available tag objects to get exact custom colors
	const selectedTagObjects = useMemo(() => {
		return selectedTags.map((st) => {
			const found = availableTags.find((t) => t.name.toLowerCase() === st.toLowerCase());
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
							'w-full justify-between min-h-10 h-auto bg-card border-border/80 px-3 py-2 text-xs font-semibold cursor-pointer shadow-2xs rounded-lg',
							selectedTags.length > 0 && 'border-primary text-primary',
							disabled && 'cursor-not-allowed opacity-50'
						)}
					>
						{variant === 'filter' ? (
							<div className="flex items-center gap-2 flex-1 min-w-0">
								<TagsIcon className="size-4 text-muted-foreground shrink-0" />
								<span>
									{placeholder}
									{selectedTags.length > 0 ? ` (${selectedTags.length})` : ''}
								</span>
							</div>
						) : (
							<div className="flex flex-wrap gap-1.5 items-center flex-1 min-w-0">
								{selectedTagObjects.length > 0 ? (
									selectedTagObjects.map((t) => (
										<TagBadge
											key={t.name}
											name={t.name}
											color={t.color}
											className="px-2 py-0.5 text-[11px] font-semibold"
										>
											<button
												type="button"
												onClick={(e) => handleRemoveTag(t.name, e)}
												className="ml-1 rounded-full text-muted-foreground hover:text-foreground cursor-pointer"
											>
												<X className="size-3" />
											</button>
										</TagBadge>
									))
								) : (
									<span className="text-muted-foreground flex items-center gap-1.5 text-xs font-normal">
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

			<DropdownMenuContent className="w-[280px] p-2 bg-popover border-border shadow-xl rounded-xl" align="start">
				<div className="flex flex-col gap-2" onClick={(e) => e.stopPropagation()}>
					<div className="flex items-center justify-between gap-2 border-b border-border/40 pb-2 px-1">
						<Input
							placeholder="Search tags..."
							value={searchQuery}
							onChange={(e) => setSearchQuery(e.target.value)}
							className="h-8 text-xs bg-muted/40 border-border/60"
						/>
						{selectedTags.length > 0 && (
							<Button
								variant="ghost"
								size="sm"
								onClick={handleClearAll}
								className="h-8 px-2 text-xs font-semibold text-muted-foreground hover:text-foreground"
							>
								Clear
							</Button>
						)}
					</div>

					<div className="max-h-52 overflow-y-auto flex flex-col gap-1 pr-1">
						{filteredTags.length > 0 ? (
							filteredTags.map((tag) => {
								const isSelected = selectedTags.includes(tag.name.toLowerCase());
								return (
									<div
										key={tag.id}
										onClick={() => handleToggleTag(tag.name)}
										className={cn(
											'flex items-center justify-between p-2 rounded-lg cursor-pointer text-xs transition-colors hover:bg-muted/60',
											isSelected && 'bg-muted/40 font-semibold'
										)}
									>
										<TagBadge name={tag.name} color={tag.color} className="text-[11px]" />
										{isSelected && <Check className="size-4 text-primary shrink-0" />}
									</div>
								);
							})
						) : (
							<div className="py-4 text-center text-xs text-muted-foreground">
								{availableTags.length === 0 ? 'No tags created in settings.' : 'No tags found.'}
							</div>
						)}
					</div>

					{searchQuery.trim() &&
						!availableTags.some((t) => t.name.toLowerCase() === searchQuery.trim().toLowerCase()) && (
							<Button
								variant="ghost"
								size="sm"
								onClick={() => {
									handleToggleTag(searchQuery.trim());
									setSearchQuery('');
								}}
								className="h-8 text-xs font-semibold gap-1.5 justify-start text-primary border-t border-border/40 pt-2 rounded-none mt-1"
							>
								<Plus className="size-3.5" />
								<span>Add "#{searchQuery.trim()}"</span>
							</Button>
						)}
				</div>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}
