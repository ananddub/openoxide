import {useState} from 'react';
import {
	Globe,
	ExternalLink,
	Edit2,
	Trash2,
	RefreshCw,
	ShieldCheck,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
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

export function ComposeDomainsTable({
	domains,
	isLoading,
	onEdit,
	onDelete,
}: ComposeDomainsTableProps) {
	const [activeDeletingId, setActiveDeletingId] = useState<number | null>(
		null,
	);
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
				<div className="flex h-40 items-center justify-center gap-2 rounded-xl border border-dashed border-border/60 bg-card/20 text-xs text-muted-foreground">
					<RefreshCw className="size-4 animate-spin text-primary" />{' '}
					Loading domains...
				</div>
			) : safeDomains.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 rounded-xl border border-dashed border-border/60 bg-card/10 py-16 text-xs text-muted-foreground">
					<Globe className="size-8 opacity-40" />
					<p className="font-semibold text-foreground">
						No compose domain routes configured
					</p>
					<p className="text-[11px] text-muted-foreground">
						Add domain routes to map traffic to compose services
					</p>
				</div>
			) : (
				<div className="overflow-hidden rounded-xl border border-border/60 bg-card shadow-xs">
					<Table>
						<TableHeader>
							<TableRow className="border-b border-border/60 bg-muted/30 hover:bg-muted/30">
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									Domain / Host
								</TableHead>
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									Service
								</TableHead>
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									Path & Port
								</TableHead>
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									SSL Status
								</TableHead>
								<TableHead className="px-4 py-3.5 text-right text-xs font-bold tracking-wider text-foreground uppercase">
									Actions
								</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{safeDomains.map((d: any) => {
								const domainHost = d.host || d.domain || '';
								const containerPort = d.port || d.container_port || 80;
								const serviceName = d.service_name || 'app';
								const url = `${d.https ? 'https' : 'http'}://${domainHost}${d.path && d.path !== '/' ? d.path : ''}`;
								return (
									<TableRow
										key={d.id}
										className="border-b border-border/40 transition-colors hover:bg-muted/40">
										<TableCell className="px-4 py-3.5 font-mono text-xs font-bold text-foreground">
											<div className="flex items-center gap-2.5">
												<Globe className="size-4 shrink-0 text-primary" />
												<a
													href={url}
													target="_blank"
													rel="noreferrer"
													className="flex items-center gap-1.5 transition-colors hover:text-primary hover:underline">
													<span>{domainHost}</span>
													<ExternalLink className="size-3 shrink-0 text-muted-foreground" />
												</a>
											</div>
										</TableCell>
										<TableCell className="px-4 py-3.5 text-xs font-semibold text-foreground">
											<Badge
												variant="secondary"
												className="font-mono text-[10px]">
												{serviceName}
											</Badge>
										</TableCell>
										<TableCell className="px-4 py-3.5 font-mono text-xs text-muted-foreground">
											Path: {d.path || '/'} · Port: {containerPort}
										</TableCell>
										<TableCell className="px-4 py-3.5">
											{d.https ? (
												<Badge
													variant="outline"
													className="flex w-fit items-center gap-1 border-emerald-500/30 bg-emerald-500/10 text-[10px] font-semibold text-emerald-500">
													<ShieldCheck className="size-3" /> HTTPS
												</Badge>
											) : (
												<Badge
													variant="secondary"
													className="flex w-fit items-center gap-1 text-[10px] font-semibold">
													HTTP
												</Badge>
											)}
										</TableCell>
										<TableCell className="px-4 py-3.5 text-right">
											<div className="flex items-center justify-end gap-1.5">
												<Button
													variant="outline"
													size="sm"
													onClick={() => onEdit(d)}
													className="flex h-7 items-center gap-1.5 border-border text-xs font-semibold hover:bg-muted">
													<Edit2 className="size-3" /> Edit
												</Button>
												<Button
													variant="ghost"
													size="icon"
													onClick={() => handleDelete(d.id)}
													disabled={activeDeletingId === d.id}
													className="size-7 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
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
