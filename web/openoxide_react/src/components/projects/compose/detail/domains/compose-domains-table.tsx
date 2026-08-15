import { useState } from 'react';
import { Globe, ExternalLink, Edit2, Trash2, RefreshCw, Box, ShieldCheck } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Badge } from '#/components/ui/badge';

interface ComposeDomainsTableProps {
	domains: any[];
	isLoading: boolean;
	onEdit: (domain: any) => void;
	onDelete: (id: number) => Promise<void>;
}

export function ComposeDomainsTable({ domains, isLoading, onEdit, onDelete }: ComposeDomainsTableProps) {
	const [activeDeletingId, setActiveDeletingId] = useState<number | null>(null);
	const safeDomains = Array.isArray(domains) ? domains : [];

	const handleDelete = async (id: number) => {
		setActiveDeletingId(id);
		try {
			await onDelete(id);
		} finally {
			setActiveDeletingId(null);
		}
	};

	return (
		<div className="flex flex-col gap-4 w-full">
			{isLoading && safeDomains.length === 0 ? (
				<div className="flex items-center justify-center h-40 text-xs text-muted-foreground gap-2 border border-dashed border-border/60 rounded-xl bg-card/20">
					<RefreshCw className="size-4 animate-spin text-primary" /> Loading domains...
				</div>
			) : safeDomains.length === 0 ? (
				<div className="flex flex-col items-center justify-center py-16 text-muted-foreground gap-2 text-xs border border-dashed border-border/60 rounded-xl bg-card/10">
					<Globe className="size-8 opacity-40" />
					<p className="font-semibold text-foreground">No compose domain routes configured</p>
					<p className="text-[11px] text-muted-foreground">Add domain routes to map traffic to compose services</p>
				</div>
			) : (
				<div className="grid grid-cols-1 gap-3.5 lg:grid-cols-2 w-full">
					{safeDomains.map((d: any) => {
						const domainHost = d.host || d.domain || '';
						const containerPort = d.port || d.container_port || 80;
						const serviceName = d.service_name || 'app';
						const url = `${d.https ? 'https' : 'http'}://${domainHost}${d.path && d.path !== '/' ? d.path : ''}`;
						return (
							<div
								key={d.id}
								className="border border-border/60 rounded-xl p-4 bg-card hover:bg-muted/30 transition-all flex items-center justify-between gap-4 flex-wrap shadow-xs"
							>
								<div className="flex items-start gap-3 min-w-0">
									<div className="size-8 rounded-lg bg-primary/10 text-primary flex items-center justify-center shrink-0 border border-primary/20">
										<Globe className="size-4" />
									</div>
									<div className="flex flex-col gap-1 min-w-0">
										<div className="flex items-center gap-2 flex-wrap min-w-0">
											<a
												href={url}
												target="_blank"
												rel="noreferrer"
												className="text-sm font-bold text-foreground hover:underline hover:text-primary transition-colors flex items-center gap-1.5 truncate"
											>
												<span className="truncate">{domainHost}</span>
												<ExternalLink className="size-3 text-muted-foreground shrink-0" />
											</a>
											{d.https && (
												<Badge variant="outline" className="text-[10px] font-semibold text-emerald-500 border-emerald-500/30 bg-emerald-500/10">
													<ShieldCheck className="size-3 mr-1" /> HTTPS
												</Badge>
											)}
											<Badge variant="secondary" className="text-[10px] font-mono">
												<Box className="size-3 mr-1 text-primary" /> Service: {serviceName}
											</Badge>
										</div>
										<span className="text-xs text-muted-foreground font-mono">
											Port: {containerPort} · Path: {d.path || '/'}
										</span>
									</div>
								</div>

								<div className="flex items-center gap-2 shrink-0">
									<Button
										variant="outline"
										size="sm"
										onClick={() => onEdit(d)}
										className="h-8 text-xs font-semibold border-border hover:bg-muted flex items-center gap-1.5"
									>
										<Edit2 className="size-3.5" /> Edit
									</Button>

									<Button
										variant="ghost"
										size="icon"
										onClick={() => handleDelete(d.id)}
										disabled={activeDeletingId === d.id}
										className="size-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-lg"
									>
										<Trash2 className="size-4" />
									</Button>
								</div>
							</div>
						);
					})}
				</div>
			)}
		</div>
	);
}
