import {useState} from 'react';
import {UploadCloud, Trash2, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {useApplicationSetDropSource} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {createServiceClient} from '#/api/live/query';
import {ApplicationService} from '#/gen/connect/openoxide/api/v1/application_pb';
import {Progress} from '#/components/ui/progress';

const applicationClient = createServiceClient(ApplicationService);

// Matches the proto's max chunk size for UploadSourceRequest.
const UPLOAD_CHUNK_BYTES = 4 * 1024 * 1024;

interface DropSourceFormProps {
	app: any;
	onUpdated: () => void;
}

export function DropSourceForm({app, onUpdated}: DropSourceFormProps) {
	const [dropBuildPath, setDropBuildPath] = useState(
		app.drop_build_path || '/',
	);
	const [selectedZip, setSelectedZip] = useState<File | null>(null);
	const [isDragging, setIsDragging] = useState(false);
	const [saving, setSaving] = useState(false);
	const [uploadProgress, setUploadProgress] = useState(0);

	const patchDrop = useApplicationSetDropSource();

	const handleFileSelect = (files: FileList | null) => {
		if (files && files[0]) {
			const file = files[0];
			if (!file.name.toLowerCase().endsWith('.zip')) {
				toast.error('Please select a .zip archive file');
				return;
			}
			setSelectedZip(file);
		}
	};

	const handleSave = async () => {
		setSaving(true);
		try {
			if (selectedZip) {
				const total = selectedZip.size;
				let sent = 0;
				const chunks = (async function* () {
					for (let offset = 0; offset < total; offset += UPLOAD_CHUNK_BYTES) {
						const slice = new Uint8Array(
							await selectedZip
								.slice(offset, Math.min(offset + UPLOAD_CHUNK_BYTES, total))
								.arrayBuffer(),
						);
						sent += slice.length;
						setUploadProgress(Math.min(99, Math.round((sent / total) * 100)));
						yield {payload: {case: 'chunk' as const, value: slice}};
					}
				})();
				await applicationClient.uploadSource(
					(async function* () {
						yield {
							payload: {
								case: 'header',
								value: {
									applicationId: BigInt(app.id),
									buildPath: dropBuildPath || '/',
								},
							},
						};
						yield* chunks;
					})(),
				);
				setUploadProgress(100);
			} else {
				await patchDrop.mutateAsync({
					id: app.id,
					build_path: dropBuildPath || '/',
				});
			}
			toast.success('Drop source configuration saved');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
			setUploadProgress(0);
		}
	};

	return (
		<div className="flex flex-col gap-4">
			<div className="flex flex-col gap-1.5">
				<span className="text-xs font-bold text-foreground">
					Build Path
				</span>
				<Input
					placeholder="e.g. /"
					value={dropBuildPath}
					onChange={e => setDropBuildPath(e.target.value)}
					className="h-9 border- bg-card text-xs"
				/>
			</div>

			<div className="flex flex-col gap-1.5">
				<span className="text-xs font-bold font-semibold text-foreground">
					Upload Source Code Archive
				</span>
				<div
					onDragOver={e => {
						e.preventDefault();
						setIsDragging(true);
					}}
					onDragLeave={e => {
						e.preventDefault();
						setIsDragging(false);
					}}
					onDrop={e => {
						e.preventDefault();
						setIsDragging(false);
						handleFileSelect(e.dataTransfer.files);
					}}
					className={`flex cursor-pointer flex-col items-center justify-center rounded-xl border-2 border-dashed p-8 text-center transition-colors ${
						isDragging
							? ' bg-primary/5'
							: ' hover:bg-muted/20'
					}`}
					onClick={() => {
						const input = document.createElement('input');
						input.type = 'file';
						input.accept = '.zip';
						input.onchange = e => {
							const target = e.target as HTMLInputElement;
							handleFileSelect(target.files);
						};
						input.click();
					}}>
					<UploadCloud className="mb-2 h-8 w-8 text-muted-foreground" />
					<p className="text-xs font-semibold text-foreground">
						Drop files or click here to upload .zip archive
					</p>
					<p className="mt-0.5 text-[10px] text-muted-foreground">
						Supports sanitized .zip archives up to 100 MiB
					</p>
				</div>

				{selectedZip && (
					<div className="mt-1 flex items-center justify-between rounded-lg bg-muted/20 p-3">
						<div className="flex min-w-0 items-center gap-2">
							<span className="truncate font-mono text-xs font-medium text-foreground">
								{selectedZip.name}
							</span>
							<span className="text-[10px] text-muted-foreground">
								({(selectedZip.size / (1024 * 1024)).toFixed(2)} MB)
							</span>
						</div>
						<Button
							type="button"
							variant="ghost"
							size="sm"
							disabled={saving}
							onClick={() => setSelectedZip(null)}
							className="h-7 w-7 p-0 text-muted-foreground hover:text-destructive">
							<Trash2 className="h-4 w-4" />
						</Button>
					</div>
				)}
				{saving && selectedZip && (
					<div className="mt-2 flex flex-col gap-1.5">
						<div className="flex justify-between text-[10px] text-muted-foreground">
							<span>Uploading source archive</span><span>{uploadProgress}%</span>
						</div>
						<Progress value={uploadProgress} />
					</div>
				)}
			</div>

			<div className="mt-2 flex justify-end">
				<Button
					onClick={handleSave}
					disabled={saving}
					className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
					<Save className="h-3.5 w-3.5" />{' '}
					{saving ? 'Saving...' : 'Save Source'}
				</Button>
			</div>
		</div>
	);
}
