import {useState} from 'react';
import {FileCode, Lock, Folder, FolderOpen, ChevronRight, ChevronDown} from 'lucide-react';

export interface TreeNodeItem {
	id: string; // full relative path
	name: string;
	type: 'file' | 'directory';
	size?: number;
	is_readonly?: boolean;
	children?: TreeNodeItem[];
}

interface TreeFolderNodeProps {
	item: TreeNodeItem;
	selectedFile: string | null;
	onSelectFile: (path: string) => void;
	level?: number;
}

export function TreeFolderNode({
	item,
	selectedFile,
	onSelectFile,
	level = 0,
}: TreeFolderNodeProps) {
	const [isOpen, setIsOpen] = useState(true);

	if (item.type === 'file') {
		const isSelected = selectedFile === item.id;
		return (
			<button
				type="button"
				onClick={() => onSelectFile(item.id)}
				style={{paddingLeft: `${level * 12 + 10}px`}}
				className={`w-full flex items-center justify-between gap-2 py-1.5 px-2.5 rounded-md text-xs font-mono transition-colors text-left cursor-pointer outline-none focus:outline-none focus-visible:ring-0 ${
					isSelected
						? 'bg-primary/15 text-primary font-semibold'
						: 'text-muted-foreground hover:bg-muted/30 hover:text-foreground'
				}`}>
				<div className="flex items-center gap-2 truncate">
					<FileCode className={`size-3.5 shrink-0 ${isSelected ? 'text-primary' : 'text-muted-foreground/70'}`} />
					<span className="truncate">{item.name}</span>
				</div>
				{item.is_readonly ? (
					<span title="Read-Only" className="shrink-0 text-muted-foreground">
						<Lock className="size-3" />
					</span>
				) : (
					item.size !== undefined && (
						<span className="text-[10px] text-muted-foreground/60 font-sans shrink-0">
							{(item.size / 1024).toFixed(1)}K
						</span>
					)
				)}
			</button>
		);
	}

	return (
		<div className="w-full">
			<button
				type="button"
				onClick={() => setIsOpen(!isOpen)}
				style={{paddingLeft: `${level * 12 + 6}px`}}
				className="w-full flex items-center gap-1.5 py-1.5 pr-2 rounded-md text-xs font-medium text-foreground/90 hover:bg-muted/40 transition-colors text-left cursor-pointer">
				{isOpen ? (
					<ChevronDown className="size-3.5 text-muted-foreground shrink-0" />
				) : (
					<ChevronRight className="size-3.5 text-muted-foreground shrink-0" />
				)}
				{isOpen ? (
					<FolderOpen className="size-3.5 text-primary/80 shrink-0" />
				) : (
					<Folder className="size-3.5 text-muted-foreground shrink-0" />
				)}
				<span className="truncate">{item.name}</span>
			</button>
			{isOpen && item.children && (
				<div className="flex flex-col gap-0.5 mt-0.5">
					{item.children.map((child) => (
						<TreeFolderNode
							key={child.id}
							item={child}
							selectedFile={selectedFile}
							onSelectFile={onSelectFile}
							level={level + 1}
						/>
					))}
				</div>
			)}
		</div>
	);
}
