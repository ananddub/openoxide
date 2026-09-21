import {useState, useEffect, useMemo} from 'react';
import {toast} from 'sonner';
import {
	useTraefikFileContent,
	useTraefikFileContentUpdate,
	useTraefikFileTree,
	useTraefikHealth,
	useRemoteServerList,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import type {
	TraefikFileNode,
	TraefikFileTreeNode,
	TraefikFileContent,
	TraefikHealthResponse,
	RemoteServerItem,
} from './traefik-types';

/** Owns all Traefik file-editor state, queries and mutations. */
export function useTraefikPanel() {
	const [selectedServerId, setSelectedServerId] =
		useState<string>('local');
	const [selectedFilePath, setSelectedFilePath] = useState<string | null>(
		null,
	);
	const [originalContent, setOriginalContent] = useState<string>('');
	const [editedContent, setEditedContent] = useState<string>('');
	const [isDiffOpen, setIsDiffOpen] = useState<boolean>(false);
	const [isSaving, setIsSaving] = useState<boolean>(false);

	const parsedServerId =
		selectedServerId !== 'local' ? Number(selectedServerId) : undefined;

	// Live servers query; the table-watch stream keeps it fresh.
	const {data: rawServers} = useRemoteServerList();
	const servers: RemoteServerItem[] = useMemo(() => {
		const list = Array.isArray(rawServers) ? rawServers : [];
		return list.map((item: unknown) => {
			const s = item as Record<string, unknown>;
			return {
				id: Number(s.id || 0),
				name: String(s.name || `Server ${s.id}`),
				ip_address: s.ip_address ? String(s.ip_address) : undefined,
			};
		});
	}, [rawServers]);

	// Query Traefik File Tree
	const {
		data: rawFileTree = [],
		isLoading: isFilesLoading,
		refetch: refetchFiles,
	} = useTraefikFileTree(parsedServerId ? BigInt(parsedServerId) : 0n);

	const fileTree: TraefikFileTreeNode[] = useMemo(() => {
		const parseNode = (item: unknown): TraefikFileTreeNode => {
			const f = item as Record<string, unknown>;
			const nodeType = f.node_type === 'directory' ? 'directory' : 'file';
			return {
				name: String(f.name || ''),
				relative_path: String(f.relative_path || ''),
				node_type: nodeType,
				size: Number(f.size || 0),
				is_readonly: Boolean(f.is_readonly),
				modified_at: Number(f.modified_at || 0),
				children: Array.isArray(f.children)
					? f.children.map(parseNode)
					: [],
			};
		};
		const list = Array.isArray(rawFileTree) ? rawFileTree : [];
		return list.map(parseNode);
	}, [rawFileTree]);

	const files: TraefikFileNode[] = useMemo(() => {
		const flat: TraefikFileNode[] = [];
		const visit = (node: TraefikFileTreeNode) => {
			if (node.node_type === 'file') {
				flat.push({
					name: node.name,
					relative_path: node.relative_path,
					size: node.size,
					is_readonly: node.is_readonly,
					modified_at: node.modified_at,
				});
				return;
			}
			node.children.forEach(visit);
		};
		fileTree.forEach(visit);
		return flat;
	}, [fileTree]);

	// Automatically select first file when list loads
	useEffect(() => {
		if (files.length > 0 && !selectedFilePath) {
			setSelectedFilePath(files[0].relative_path);
		}
	}, [files, selectedFilePath]);

	// Query File Content when active file changes
	const {
		data: rawFileContent,
		isLoading: isContentLoading,
		refetch: refetchContent,
	} = useTraefikFileContent(
		parsedServerId ? BigInt(parsedServerId) : 0n,
		selectedFilePath || '',
	);

	useEffect(() => {
		if (rawFileContent) {
			const fc = rawFileContent as unknown as TraefikFileContent;
			const text = String(fc.content || '');
			setOriginalContent(text);
			setEditedContent(text);
		}
	}, [rawFileContent]);

	// Query Traefik Health Status
	const {
		data: rawHealth,
		isLoading: isHealthLoading,
		refetch: refetchHealth,
	} = useTraefikHealth();

	const health: TraefikHealthResponse | null = useMemo(() => {
		if (!rawHealth) return null;
		const h = rawHealth as unknown as Record<string, unknown>;
		return {
			is_healthy: Boolean(h.is_healthy),
			rawdata_status: String(h.rawdata_status || ''),
			configuration_errors: Array.isArray(h.configuration_errors)
				? (h.configuration_errors as string[])
				: [],
		};
	}, [rawHealth]);

	const activeFileObj = useMemo(() => {
		return files.find(f => f.relative_path === selectedFilePath);
	}, [files, selectedFilePath]);

	const isReadOnly = Boolean(activeFileObj?.is_readonly);
	const isDirty = originalContent !== editedContent;

	const writeFileMutation = useTraefikFileContentUpdate();

	const handleSave = async () => {
		if (!selectedFilePath || isReadOnly) return;
		setIsSaving(true);
		try {
			await writeFileMutation.mutateAsync({
				server_id: parsedServerId,
				path: selectedFilePath,
				content: editedContent,
			});
			toast.success(`Successfully saved ${selectedFilePath}`);
			setOriginalContent(editedContent);
			refetchContent();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsSaving(false);
		}
	};

	const handleCreateFile = (newFileName: string) => {
		const initialText = `# Dynamic configuration for ${newFileName}\nhttp:\n  routers:\n    # Add router configuration here\n`;
		setSelectedFilePath(newFileName);
		setOriginalContent('');
		setEditedContent(initialText);
	};

	return {
		selectedServerId,
		setSelectedServerId,
		selectedFilePath,
		setSelectedFilePath,
		originalContent,
		editedContent,
		setEditedContent,
		isDiffOpen,
		setIsDiffOpen,
		isSaving,
		servers,
		fileTree,
		isFilesLoading,
		refetchFiles,
		isContentLoading,
		refetchContent,
		health,
		isHealthLoading,
		refetchHealth,
		isReadOnly,
		isDirty,
		handleSave,
		handleCreateFile,
	};
}
