import { useState } from 'react';
import { Globe, ExternalLink, Edit2, Trash2, RefreshCw, ShieldCheck } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Badge } from '#/components/ui/badge';
import {
	Table,
	TableHeader,
	TableBody,
	TableHead,
	TableRow,
	TableCell,
} from '#/components/ui/table';

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
		<div className="w-full">
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
				<div className="rounded-xl border border-border/60 bg-card overflow-hidden shadow-xs">
					<Table>
						<TableHeader>
							<TableRow className="border-b border-border/60 bg-muted/30 hover:bg-muted/30">
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">Domain / Host</TableHead>
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">Service</TableHead>
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">Path & Port</TableHead>
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">SSL Status</TableHead>
								<TableHead className="py-3.5 px-4 text-right font-bold text-foreground text-xs uppercase tracking-wider">Actions</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{safeDomains.map((d: any) => {
								const domainHost = d.host || d.domain || '';
								const containerPort = d.port || d.container_port || 80;
								const serviceName = d.service_name || 'app';
								const url = `${d.https ? 'https' : 'http'}://${domainHost}${d.path && d.path !== '/' ? d.path : ''}`;
								return (
									<TableRow key={d.id} className="border-b border-border/40 hover:bg-muted/40 transition-colors">
										<TableCell className="py-3.5 px-4 font-bold text-xs text-foreground font-mono">
											<div className="flex items-center gap-2.5">
												<Globe className="size-4 text-primary shrink-0" />
												<a
													href={url}
													target="_blank"
													rel="noreferrer"
													className="hover:underline hover:text-primary transition-colors flex items-center gap-1.5"
												>
													<span>{domainHost}</span>
													<ExternalLink className="size-3 text-muted-foreground shrink-0" />
												</a>
											</div>
										</TableCell>
										<TableCell className="py-3.5 px-4 text-xs font-semibold text-foreground">
											<Badge variant="secondary" className="text-[10px] font-mono">
												{serviceName}
											</Badge>
										</TableCell>
										<TableCell className="py-3.5 px-4 text-xs text-muted-foreground font-mono">
											Path: {d.path || '/'} · Port: {containerPort}
										</TableCell>
										<TableCell className="py-3.5 px-4">
											{d.https ? (
												<Badge variant="outline" className="text-[10px] font-semibold text-emerald-500 border-emerald-500/30 bg-emerald-500/10 flex items-center gap-1 w-fit">
													<ShieldCheck className="size-3" /> HTTPS
												</Badge>
											) : (
												<Badge variant="secondary" className="text-[10px] font-semibold flex items-center gap-1 w-fit">
													HTTP
												</Badge>
											)}
										</TableCell>
										<TableCell className="py-3.5 px-4 text-right">
											<div className="flex items-center justify-end gap-1.5">
												<Button
													variant="outline"
													size="sm"
													onClick={() => onEdit(d)}
													className="h-7 text-xs font-semibold border-border hover:bg-muted flex items-center gap-1.5"
												>
													<Edit2 className="size-3" /> Edit
												</Button>
												<Button
													variant="ghost"
													size="icon"
													onClick={() => handleDelete(d.id)}
													disabled={activeDeletingId === d.id}
													className="size-7 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-lg"
												>
													<Trash2 className="size-3.5" />
												</Button>
											</div>
										</TableCell>
									</TableRow>
								);
							})}
						</TableBody>
					</Table>
				</div>
			)}
		</div>
	);
}
