import {useState, useEffect, useMemo} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {useRemoteServerList} from 'virtual:openoxide-live';
import {formatApiError} from '#/api/utils';

import {TraefikHeader} from '#/components/traefik/traefik-header';
import {TraefikFileTree} from '#/components/traefik/traefik-file-tree';
import {TraefikEditor} from '#/components/traefik/traefik-editor';
import {TraefikDiffModal} from '#/components/traefik/traefik-diff-modal';
import type {
	TraefikFileNode,
	TraefikFileTreeNode,
	TraefikFileContent,
	TraefikHealthResponse,
	RemoteServerItem,
} from '#/components/traefik/traefik-types';

export const Route = createFileRoute('/_app/traefik')({
	component: TraefikPage,
});

function TraefikPage() {
	const [selectedServerId, setSelectedServerId] = useState<string>('local');
	const [selectedFilePath, setSelectedFilePath] = useState<string | null>(null);
	const [originalContent, setOriginalContent] = useState<string>('');
	const [editedContent, setEditedContent] = useState<string>('');
	const [isDiffOpen, setIsDiffOpen] = useState<boolean>(false);
	const [isSaving, setIsSaving] = useState<boolean>(false);

	const parsedServerId = selectedServerId !== 'local' ? Number(selectedServerId) : undefined;

	// Read Remote Servers list from Zustand RAM store
	const rawServers = useAppStore((state) => state.servers);
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
	} = $api.useQuery('get', '/traefik/files/tree', {
		params: {
			query: {
				server_id: parsedServerId,
			} as any,
		},
	});

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
				children: Array.isArray(f.children) ? f.children.map(parseNode) : [],
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
	} = $api.useQuery(
		'get',
		'/traefik/files/content',
		{
			params: {
				query: {
					server_id: parsedServerId,
					path: selectedFilePath || '',
				} as any,
			},
		},
		{
			enabled: Boolean(selectedFilePath),
		}
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
	} = $api.useQuery('get', '/traefik/health', {
		params: {
			query: {
				server_id: parsedServerId,
			} as any,
		},
	});

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
		return files.find((f) => f.relative_path === selectedFilePath);
	}, [files, selectedFilePath]);

	const isReadOnly = Boolean(activeFileObj?.is_readonly);
	const isDirty = originalContent !== editedContent;

	// Mutation for Writing File Content
	const writeFileMutation = $api.useMutation('put', '/traefik/files/content');

	const handleSave = async () => {
		if (!selectedFilePath || isReadOnly) return;
		setIsSaving(true);
		try {
			await writeFileMutation.mutateAsync({
				body: {
					server_id: parsedServerId,
					path: selectedFilePath,
					content: editedContent,
				},
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

	return (
		<div className="flex flex-col gap-3 w-full h-[calc(100vh-7rem)] p-4 animate-in fade-in duration-200">
			{/* Minimal Header */}
			<TraefikHeader
				selectedServerId={selectedServerId}
				onSelectServer={(id) => {
					setSelectedServerId(id);
					setSelectedFilePath(null);
				}}
				servers={servers}
				health={health}
				isCheckingHealth={isHealthLoading}
				onCheckHealth={() => refetchHealth()}
				onOpenDiff={() => setIsDiffOpen(true)}
				onSave={handleSave}
				isSaving={isSaving}
				isDirty={isDirty}
				isReadOnly={isReadOnly}
			/>

			{/* Main Body: File Tree + Editor */}
			<div className="flex flex-col lg:flex-row gap-3 flex-1 overflow-hidden min-h-0">
				<TraefikFileTree
					files={fileTree}
					selectedFile={selectedFilePath}
					onSelectFile={(path) => {
						setSelectedFilePath(path);
						refetchContent();
					}}
					isLoading={isFilesLoading}
					onRefresh={() => refetchFiles()}
					onCreateFile={handleCreateFile}
				/>

				<TraefikEditor
					selectedFilePath={selectedFilePath}
					content={editedContent}
					onChangeContent={(val) => setEditedContent(val)}
					isReadOnly={isReadOnly}
					isLoading={isContentLoading}
				/>
			</div>

			{/* Configuration Diff View Modal */}
			<TraefikDiffModal
				isOpen={isDiffOpen}
				onClose={() => setIsDiffOpen(false)}
				filePath={selectedFilePath || ''}
				originalContent={originalContent}
				modifiedContent={editedContent}
			/>
		</div>
	);
}
