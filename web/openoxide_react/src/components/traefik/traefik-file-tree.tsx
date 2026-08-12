import {useState, useMemo} from 'react';
import {RefreshCw, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import type {TraefikFileTreeNode} from './traefik-types';
import {TreeFolderNode, type TreeNodeItem} from './tree-node';

interface TraefikFileTreeProps {
	files: TraefikFileTreeNode[];
	selectedFile: string | null;
	onSelectFile: (path: string) => void;
	isLoading: boolean;
	onRefresh: () => void;
	onCreateFile: (name: string) => void;
}

function toTreeNodeItem(node: TraefikFileTreeNode): TreeNodeItem {
	const isDirectory = node.node_type === 'directory';
	return {
		id: node.relative_path,
		name: node.name,
		type: isDirectory ? 'directory' : 'file',
		size: isDirectory ? undefined : node.size,
		is_readonly: isDirectory ? false : node.is_readonly,
		children: isDirectory ? node.children.map(toTreeNodeItem) : undefined,
	};
}

export function TraefikFileTree({
	files,
	selectedFile,
	onSelectFile,
	isLoading,
	onRefresh,
	onCreateFile,
}: TraefikFileTreeProps) {
	const [searchFilter, setSearchFilter] = useState('');
	const [isAddingFile, setIsAddingFile] = useState(false);
	const [newFileName, setNewFileName] = useState('');

	const tree = useMemo(() => files.map(toTreeNodeItem), [files]);

	const filteredTree = useMemo(() => {
		if (!searchFilter.trim()) return tree;
		const query = searchFilter.toLowerCase();

		const filterNodes = (nodes: TreeNodeItem[]): TreeNodeItem[] => {
			const result: TreeNodeItem[] = [];
			nodes.forEach((node) => {
				if (node.type === 'file') {
					if (node.name.toLowerCase().includes(query) || node.id.toLowerCase().includes(query)) {
						result.push(node);
					}
				} else if (node.children) {
					const matchingChildren = filterNodes(node.children);
					if (matchingChildren.length > 0 || node.name.toLowerCase().includes(query)) {
						result.push({
							...node,
							children: matchingChildren,
						});
					}
				}
			});
			return result;
		};

		return filterNodes(tree);
	}, [tree, searchFilter]);

	const handleAddSubmit = (e: React.FormEvent) => {
		e.preventDefault();
		if (!newFileName.trim()) return;
		let finalName = newFileName.trim();
		if (!finalName.endsWith('.yml') && !finalName.endsWith('.yaml')) {
			finalName += '.yml';
		}
		onCreateFile(finalName);
		setNewFileName('');
		setIsAddingFile(false);
	};

	return (
		<div className="w-full lg:w-64 flex flex-col gap-2.5 shrink-0 pr-2.5 lg:border-r lg:border-border/40">
			<div className="flex items-center justify-between gap-2">
				<span className="text-xs font-bold text-foreground uppercase tracking-wider">Config Files</span>
				<div className="flex items-center gap-1">
					<Button
						variant="ghost"
						size="icon"
						onClick={() => setIsAddingFile(!isAddingFile)}
						title="New Dynamic Config File"
						className="size-7 hover:bg-muted cursor-pointer">
						<Plus className="size-3.5" />
					</Button>
					<Button
						variant="ghost"
						size="icon"
						onClick={onRefresh}
						disabled={isLoading}
						title="Refresh File List"
						className="size-7 hover:bg-muted cursor-pointer">
						<RefreshCw className={`size-3.5 ${isLoading ? 'animate-spin' : ''}`} />
					</Button>
				</div>
			</div>

			<Input
				value={searchFilter}
				onChange={(e) => setSearchFilter(e.target.value)}
				placeholder="Filter files..."
				className="h-8 text-xs font-mono bg-background border-border/50"
			/>

			{isAddingFile && (
				<form onSubmit={handleAddSubmit} className="flex items-center gap-1.5 bg-muted/20 p-2 rounded-lg border border-border/50">
					<Input
						value={newFileName}
						onChange={(e) => setNewFileName(e.target.value)}
						placeholder="custom-rules.yml"
						autoFocus
						className="h-7 text-xs font-mono bg-background border-border/50 flex-1"
					/>
					<Button type="submit" size="sm" className="h-7 px-2.5 text-xs font-semibold">
						Add
					</Button>
				</form>
			)}

			<div className="flex-1 overflow-auto min-h-[220px] max-h-[500px] lg:max-h-none flex flex-col gap-0.5 pr-1">
				{isLoading && files.length === 0 ? (
					<div className="py-8 text-center text-xs text-muted-foreground font-mono flex items-center justify-center gap-2">
						<RefreshCw className="size-3.5 animate-spin" /> Loading files...
					</div>
				) : filteredTree.length === 0 ? (
					<div className="py-8 text-center text-xs text-muted-foreground font-sans">No configuration files found.</div>
				) : (
					filteredTree.map((item) => (
						<TreeFolderNode
							key={item.id}
							item={item}
							selectedFile={selectedFile}
							onSelectFile={onSelectFile}
						/>
					))
				)}
			</div>
		</div>
	);
}
