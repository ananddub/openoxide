import {HardDrive, Plus, Download, Trash2, Clock} from 'lucide-react';
import {Button} from '#/components/ui/button';

export function VolumeBackupsTab() {
	const mockBackups = [
		{id: 1, name: 'auto-volume-snapshot-2026-07-24', size: '142.5 MB', path: '/data/persistent/db', date: '3 hours ago'},
		{id: 2, name: 'manual-pre-rebuild-snapshot', size: '138.1 MB', path: '/data/persistent/db', date: 'Yesterday'},
		{id: 3, name: 'weekly-system-backup-v2', size: '124.9 MB', path: '/data/persistent/db', date: '6 days ago'},
	];

	return (
		<div className="flex flex-col gap-6">
			{/* Action Header */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between">
				<div>
					<h3 className="text-sm font-bold text-foreground">Storage snapshots</h3>
					<p className="text-xs text-muted-foreground mt-1">Manage state directories, volume backups and persistent storage snapshots</p>
				</div>
				<Button size="sm" className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-1.5 h-8 text-xs">
					<Plus className="w-3.5 h-3.5" /> Trigger Backup
				</Button>
			</section>

			{/* Backups List */}
			<section className="bg-card border border-border rounded-xl overflow-hidden">
				<div className="divide-y divide-border/60">
					{mockBackups.map(b => (
						<div key={b.id} className="flex flex-col sm:flex-row sm:items-center justify-between p-4 gap-4 hover:bg-muted/10 transition-colors">
							<div className="flex items-start gap-3 min-w-0">
								<div className="w-8 h-8 rounded-lg bg-secondary flex items-center justify-center text-muted-foreground border border-border/40 shrink-0 mt-0.5">
									<HardDrive className="w-4 h-4" />
								</div>
								<div className="min-w-0">
									<span className="text-xs font-semibold text-foreground truncate block">{b.name}</span>
									<div className="flex flex-wrap items-center gap-x-3 gap-y-1 mt-1 text-[10px] text-muted-foreground font-medium">
										<span className="font-mono bg-muted/30 px-1 py-0.25 rounded">{b.path}</span>
										<span className="flex items-center gap-1"><Clock className="w-3 h-3" /> Created: {b.date}</span>
										<span className="font-bold text-foreground/80">{b.size}</span>
									</div>
								</div>
							</div>

							<div className="flex items-center gap-2 self-end sm:self-auto shrink-0">
								<Button variant="outline" size="sm" className="border-border text-foreground hover:bg-muted font-semibold h-7 text-[10px] flex items-center gap-1 rounded-md">
									<Download className="w-3 h-3" /> Download
								</Button>
								<Button variant="ghost" size="icon" className="w-7 h-7 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-md">
									<Trash2 className="w-3.5 h-3.5" />
								</Button>
							</div>
						</div>
					))}
				</div>
			</section>
		</div>
	);
}
