import {TraefikHeader} from '#/components/traefik/traefik-header';
import {TraefikFileTree} from '#/components/traefik/traefik-file-tree';
import {TraefikEditor} from '#/components/traefik/traefik-editor';
import {TraefikDiffModal} from '#/components/traefik/traefik-diff-modal';
import {useTraefikPanel} from '#/components/traefik/use-traefik-panel';

// Rendered as the "Traefik" tab of pages/infra.tsx — no standalone route.

export function TraefikPanel() {
	const p = useTraefikPanel();

	return (
		<div className="flex h-full min-h-0 w-full flex-1 flex-col gap-3 overflow-hidden">
			<TraefikHeader
				selectedServerId={p.selectedServerId}
				onSelectServer={id => {
					p.setSelectedServerId(id);
					p.setSelectedFilePath(null);
				}}
				servers={p.servers}
				health={p.health}
				isCheckingHealth={p.isHealthLoading}
				onCheckHealth={() => p.refetchHealth()}
				onOpenDiff={() => p.setIsDiffOpen(true)}
				onSave={p.handleSave}
				isSaving={p.isSaving}
				isDirty={p.isDirty}
				isReadOnly={p.isReadOnly}
			/>

			<div className="flex min-h-0 flex-1 flex-col gap-3 overflow-hidden lg:flex-row">
				<TraefikFileTree
					files={p.fileTree}
					selectedFile={p.selectedFilePath}
					onSelectFile={path => {
						p.setSelectedFilePath(path);
						p.refetchContent();
					}}
					isLoading={p.isFilesLoading}
					onRefresh={() => p.refetchFiles()}
					onCreateFile={p.handleCreateFile}
				/>

				<TraefikEditor
					selectedFilePath={p.selectedFilePath}
					content={p.editedContent}
					onChangeContent={val => p.setEditedContent(val)}
					isReadOnly={p.isReadOnly}
					isLoading={p.isContentLoading}
				/>
			</div>

			<TraefikDiffModal
				isOpen={p.isDiffOpen}
				onClose={() => p.setIsDiffOpen(false)}
				filePath={p.selectedFilePath || ''}
				originalContent={p.originalContent}
				modifiedContent={p.editedContent}
			/>
		</div>
	);
}
