import {useState} from 'react';
import {
	HardDrive,
	Trash2,
	Edit2,
	RefreshCw,
	CheckCircle2,
	Plug,
	Check,
	X,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';

import type {DestinationResponse} from '#/types/api-helpers';

interface DestinationsListProps {
	destinations: DestinationResponse[];
	isLoading: boolean;
	onEdit: (item: DestinationResponse) => void;
	onDelete: (id: string | number) => void;
	onTest: (id: string | number) => Promise<void>;
}

export function DestinationsList({
	destinations,
	isLoading,
	onEdit,
	onDelete,
	onTest,
}: DestinationsListProps) {
	const [testStatusMap, setTestStatusMap] = useState<Record<string, 'testing' | 'success' | 'failed' | undefined>>({});
	const [deletingId, setDeletingId] = useState<string | number | null>(null);

	const handleTest = async (id: string | number) => {
		const key = String(id);
		setTestStatusMap(prev => ({...prev, [key]: 'testing'}));
		try {
			await onTest(id);
			setTestStatusMap(prev => ({...prev, [key]: 'success'}));
			setTimeout(() => {
				setTestStatusMap(prev => ({...prev, [key]: undefined}));
			}, 3000);
		} catch {
			setTestStatusMap(prev => ({...prev, [key]: 'failed'}));
			setTimeout(() => {
				setTestStatusMap(prev => ({...prev, [key]: undefined}));
			}, 3000);
		}
	};

	const handleDelete = async (id: string | number) => {
		setDeletingId(id);
		try {
			await onDelete(id);
		} finally {
			setDeletingId(null);
		}
	};

	if (isLoading && destinations.length === 0) {
		return (
			<div className="flex flex-col items-center justify-center h-64 text-xs text-muted-foreground gap-2">
				<RefreshCw className="w-5 h-5 animate-spin text-primary" />
				<p>Loading S3 Storage Destinations...</p>
			</div>
		);
	}

	if (destinations.length === 0) {
		return (
			<div className="flex flex-col items-center justify-center h-64 text-muted-foreground gap-3 text-xs bg-card border border-border rounded-xl p-8 text-center shadow-sm">
				<HardDrive className="w-10 h-10 opacity-40 text-primary" />
				<div>
					<p className="text-sm font-bold text-foreground">No S3 Storage Destinations configured</p>
					<p className="text-xs text-muted-foreground mt-1">Add an AWS S3, Cloudflare R2, or S3 compatible bucket to enable database & volume backups</p>
				</div>
			</div>
		);
	}

	return (
		<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
			{destinations.map((d: DestinationResponse) => {
				const status = testStatusMap[String(d.id)];
				const isTesting = status === 'testing';

				return (
					<div
						key={d.id}
						className="bg-card border border-border/80 rounded-xl p-5 flex flex-col justify-between gap-4 shadow-sm hover:border-border transition-all"
					>
						<div className="flex flex-col gap-3">
							<div className="flex items-start justify-between gap-3">
								<div className="flex items-center gap-2.5">
									<div className="w-9 h-9 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary">
										<HardDrive className="w-4.5 h-4.5" />
									</div>
									<div>
										<h3 className="text-sm font-bold text-foreground leading-snug">{d.name}</h3>
										<span className="text-[11px] font-mono text-muted-foreground uppercase">{d.provider || 'S3 COMPATIBLE'}</span>
									</div>
								</div>
								<Badge variant="outline" className="text-[10px] font-mono border-emerald-500/30 text-emerald-400 bg-emerald-500/10 px-2 py-0.5">
									<CheckCircle2 className="w-3 h-3 mr-1 inline" /> Ready
								</Badge>
							</div>

							<div className="bg-muted/30 border border-border/40 rounded-lg p-3 flex flex-col gap-1.5 text-xs font-mono">
								<div className="flex items-center justify-between">
									<span className="text-muted-foreground font-sans">Bucket:</span>
									<span className="text-foreground font-bold">{d.bucket}</span>
								</div>
								<div className="flex items-center justify-between">
									<span className="text-muted-foreground font-sans">Region:</span>
									<span className="text-foreground">{d.region || 'us-east-1'}</span>
								</div>
								{d.endpoint && (
									<div className="flex items-center justify-between truncate">
										<span className="text-muted-foreground font-sans shrink-0 mr-2">Endpoint:</span>
										<span className="text-foreground truncate">{d.endpoint}</span>
									</div>
								)}
							</div>
						</div>

						<div className="flex items-center justify-between gap-2 border-t border-border/40 pt-3 mt-1">
							<Button
								variant="outline"
								size="icon"
								onClick={() => handleTest(d.id)}
								disabled={isTesting}
								title={
									isTesting
										? 'Testing S3 Connection...'
										: status === 'success'
											? 'S3 Connection Passed!'
											: status === 'failed'
												? 'S3 Connection Failed!'
												: 'Test Connection'
								}
								className={`h-8 w-8 shrink-0 transition-all ${
									status === 'success'
										? 'border-emerald-500/50 text-emerald-500 bg-emerald-500/10'
										: status === 'failed'
											? 'border-rose-500/50 text-rose-500 bg-rose-500/10'
											: ''
								}`}
							>
								{isTesting ? (
									<RefreshCw className="w-3.5 h-3.5 animate-spin text-amber-500" />
								) : status === 'success' ? (
									<Check className="w-3.5 h-3.5 text-emerald-500" />
								) : status === 'failed' ? (
									<X className="w-3.5 h-3.5 text-rose-500" />
								) : (
									<Plug className="w-3.5 h-3.5 text-muted-foreground hover:text-foreground" />
								)}
							</Button>

							<div className="flex items-center gap-1">
								<Button
									variant="ghost"
									size="sm"
									onClick={() => onEdit(d)}
									className="h-8 w-8 p-0 text-muted-foreground hover:text-foreground"
								>
									<Edit2 className="w-3.5 h-3.5" />
								</Button>
								<Button
									variant="ghost"
									size="sm"
									onClick={() => handleDelete(d.id)}
									disabled={deletingId === d.id}
									className="h-8 w-8 p-0 text-muted-foreground hover:text-rose-400"
								>
									<Trash2 className="w-3.5 h-3.5" />
								</Button>
							</div>
						</div>
					</div>
				);
			})}
		</div>
	);
}
