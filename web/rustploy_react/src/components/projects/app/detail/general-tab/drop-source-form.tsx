import {useState} from 'react';
import {UploadCloud, Trash2, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface DropSourceFormProps {
	app: any;
	onUpdated: () => void;
}

export function DropSourceForm({app, onUpdated}: DropSourceFormProps) {
	const [dropBuildPath, setDropBuildPath] = useState(app.drop_build_path || '/');
	const [selectedZip, setSelectedZip] = useState<File | null>(null);
	const [isDragging, setIsDragging] = useState(false);
	const [saving, setSaving] = useState(false);

	const patchDrop = $api.useMutation('patch', '/applications/{id}/source/drop');

	const handleFileSelect = (files: FileList | null) => {
		if (files && files[0]) {
			const file = files[0];
			if (!file.name.endsWith('.zip') && !file.name.endsWith('.tar.gz') && !file.name.endsWith('.tgz')) {
				toast.error('Please select a .zip or .tar.gz archive file');
				return;
			}
			setSelectedZip(file);
		}
	};

	const handleSave = async () => {
		setSaving(true);
		try {
			await patchDrop.mutateAsync({
				params: {path: {id: app.id}},
				body: {
					drop_build_path: dropBuildPath || '/',
				},
			});
			toast.success('Drop source configuration saved');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<div className="flex flex-col gap-4">
			<div className="flex flex-col gap-1.5">
				<span className="text-xs font-bold text-foreground">Build Path</span>
				<Input
					placeholder="e.g. /"
					value={dropBuildPath}
					onChange={e => setDropBuildPath(e.target.value)}
					className="bg-card border-border text-xs h-9"
				/>
			</div>

			<div className="flex flex-col gap-1.5">
				<span className="text-xs font-bold text-foreground font-semibold">Upload Source Code Archive</span>
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
					className={`border-2 border-dashed rounded-xl p-8 flex flex-col items-center justify-center text-center cursor-pointer transition-colors ${
						isDragging ? 'border-primary bg-primary/5' : 'border-border/80 hover:bg-muted/20'
					}`}
					onClick={() => {
						const input = document.createElement('input');
						input.type = 'file';
						input.accept = '.zip,.tar.gz,.tgz';
						input.onchange = e => {
							const target = e.target as HTMLInputElement;
							handleFileSelect(target.files);
						};
						input.click();
					}}
				>
					<UploadCloud className="w-8 h-8 text-muted-foreground mb-2" />
					<p className="text-xs font-semibold text-foreground">Drop files or click here to upload .zip archive</p>
					<p className="text-[10px] text-muted-foreground mt-0.5">Supports .zip, .tar.gz archives up to 100MB</p>
				</div>

				{selectedZip && (
					<div className="flex items-center justify-between p-3 rounded-lg border border-border bg-muted/20 mt-1">
						<div className="flex items-center gap-2 min-w-0">
							<span className="text-xs font-mono font-medium text-foreground truncate">{selectedZip.name}</span>
							<span className="text-[10px] text-muted-foreground">({(selectedZip.size / (1024 * 1024)).toFixed(2)} MB)</span>
						</div>
						<Button
							type="button"
							variant="ghost"
							size="sm"
							onClick={() => setSelectedZip(null)}
							className="h-7 w-7 p-0 text-muted-foreground hover:text-destructive"
						>
							<Trash2 className="w-4 h-4" />
						</Button>
					</div>
				)}
			</div>

			<div className="flex justify-end mt-2">
				<Button onClick={handleSave} disabled={saving} className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg text-xs">
					<Save className="w-3.5 h-3.5" /> {saving ? 'Saving...' : 'Save Source'}
				</Button>
			</div>
		</div>
	);
}
