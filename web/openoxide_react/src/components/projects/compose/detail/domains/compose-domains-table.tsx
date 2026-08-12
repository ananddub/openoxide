import {useState} from 'react';
import {Globe, ExternalLink, Edit2, Trash2, RefreshCw, Box, ShieldCheck} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';

interface ComposeDomainsTableProps {
	domains: any[];
	isLoading: boolean;
	onEdit: (domain: any) => void;
	onDelete: (id: number) => Promise<void>;
}

export function ComposeDomainsTable({domains, isLoading, onEdit, onDelete}: ComposeDomainsTableProps) {
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
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4 shadow-sm">
			{isLoading && safeDomains.length === 0 ? (
				<div className="flex items-center justify-center h-48 text-xs text-muted-foreground gap-2">
					<RefreshCw className="w-4 h-4 animate-spin text-primary" /> Loading domains...
				</div>
			) : safeDomains.length === 0 ? (
				<div className="flex flex-col items-center justify-center h-48 text-muted-foreground gap-2 text-xs">
					<Globe className="w-8 h-8 opacity-40" />
					<p>No compose domain routes configured.</p>
				</div>
			) : (
				<div className="flex flex-col gap-3">
					{safeDomains.map((d: any) => {
						const domainHost = d.host || d.domain || '';
						const containerPort = d.port || d.container_port || 80;
						const serviceName = d.service_name || 'app';
						const url = `${d.https ? 'https' : 'http'}://${domainHost}${d.path && d.path !== '/' ? d.path : ''}`;
						return (
							<div
								key={d.id}
								className="border border-border/80 rounded-lg p-4 bg-muted/20 hover:bg-muted/40 transition-colors flex items-center justify-between gap-4 flex-wrap"
							>
								<div className="flex items-start gap-3">
									<div className="w-8 h-8 rounded-lg bg-secondary flex items-center justify-center text-primary shrink-0 border border-border/40">
										<Globe className="w-4 h-4" />
									</div>
									<div className="flex flex-col gap-1">
										<div className="flex items-center gap-2 flex-wrap">
											<a
												href={url}
												target="_blank"
												rel="noreferrer"
												className="text-xs font-bold text-foreground hover:underline flex items-center gap-1"
											>
												{domainHost} <ExternalLink className="w-3 h-3 text-muted-foreground" />
											</a>
											{d.https && (
												<Badge variant="outline" className="text-[10px] text-emerald-400 border-emerald-500/20 bg-emerald-500/10">
													<ShieldCheck className="w-3 h-3 mr-1" /> HTTPS / SSL
												</Badge>
											)}
											<Badge variant="secondary" className="text-[10px] font-mono">
												<Box className="w-3 h-3 mr-1 text-primary" /> Service: {serviceName}
											</Badge>
										</div>
										<span className="text-xs text-muted-foreground font-mono">
											Port: {containerPort} ➔ Path: {d.path || '/'}
										</span>
									</div>
								</div>

								<div className="flex items-center gap-2">
									<Button
										variant="outline"
										size="sm"
										onClick={() => onEdit(d)}
										className="h-8 text-xs font-semibold border-border hover:bg-muted flex items-center gap-1.5"
									>
										<Edit2 className="w-3.5 h-3.5" /> Edit
									</Button>

									<Button
										variant="ghost"
										size="icon"
										onClick={() => handleDelete(d.id)}
										disabled={activeDeletingId === d.id}
										className="h-8 w-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10"
									>
										<Trash2 className="w-4 h-4" />
									</Button>
								</div>
							</div>
						);
					})}
				</div>
			)}
		</section>
	);
}
