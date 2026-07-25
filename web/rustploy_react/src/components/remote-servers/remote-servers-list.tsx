import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Card, CardContent} from '#/components/ui/card';
import {
	Server,
	Globe,
	Key,
	ShieldCheck,
	Activity,
	Trash2,
	Edit,
	Terminal,
	Cpu,
	Power,
} from 'lucide-react';

interface RemoteServersListProps {
	servers: any[];
	sshKeys: any[];
	isLoading: boolean;
	onEditServer: (server: any) => void;
	onDeleteServer: (server: any) => void;
	onSetupServer: (server: any) => void;
	onToggleStatus: (server: any) => void;
}

export function RemoteServersList({
	servers,
	sshKeys,
	isLoading,
	onEditServer,
	onDeleteServer,
	onSetupServer,
	onToggleStatus,
}: RemoteServersListProps) {
	if (isLoading) {
		return (
			<div className="grid grid-cols-1 md:grid-cols-2 gap-4 py-6">
				{[1, 2].map(i => (
					<div key={i} className="h-44 rounded-xl bg-card/60 animate-pulse border border-border/40" />
				))}
			</div>
		);
	}

	if (!servers || servers.length === 0) {
		return (
			<div className="flex flex-col items-center justify-center p-12 border border-dashed border-border/60 rounded-2xl bg-card/40 my-6 text-center">
				<div className="p-4 rounded-full bg-primary/10 mb-4 text-primary">
					<Server className="w-8 h-8" />
				</div>
				<h3 className="text-base font-bold text-foreground">No Remote Servers Connected</h3>
				<p className="text-xs text-muted-foreground max-w-sm mt-1 mb-4">
					You have not connected any external Linux server nodes. Add a remote server with SSH credentials to deploy distributed apps.
				</p>
			</div>
		);
	}

	return (
		<div className="grid grid-cols-1 md:grid-cols-2 gap-4 py-4 w-full">
			{servers.map((item: any) => {
				const attachedKey = sshKeys?.find((k: any) => Number(k.id) === Number(item.ssh_key_id));
				const isActive = (item.server_status || 'ACTIVE').toUpperCase() === 'ACTIVE';

				return (
					<Card key={item.id} className="bg-card border-border/70 hover:border-primary/40 transition-colors shadow-sm min-w-0 w-full overflow-hidden">
						<CardContent className="p-5 flex flex-col justify-between gap-4 h-full min-w-0 w-full">
							<div className="flex items-start justify-between gap-3 min-w-0 w-full">
								<div className="flex items-start gap-3 min-w-0 flex-1">
									<div className="p-2.5 rounded-xl bg-muted/40 border border-border/50 text-primary shrink-0 mt-0.5">
										<Server className="w-5 h-5" />
									</div>
									<div className="flex flex-col min-w-0 flex-1">
										<div className="flex items-center gap-2 min-w-0">
											<span className="text-sm font-bold text-foreground truncate min-w-0">{item.name}</span>
											<Badge
												variant="outline"
												className={`text-[10px] px-1.5 py-0 shrink-0 ${
													isActive
														? 'bg-emerald-500/10 text-emerald-500 border-emerald-500/30'
														: 'bg-muted text-muted-foreground border-border'
												}`}
											>
												{isActive ? 'ACTIVE' : 'DISABLED'}
											</Badge>
										</div>
										<p className="text-xs text-muted-foreground truncate min-w-0 mt-0.5">
											{item.description || `${item.username || 'root'}@${item.ip_address}`}
										</p>
									</div>
								</div>

								<div className="flex items-center gap-1 shrink-0">
									<Button
										variant="ghost"
										size="icon"
										onClick={() => onEditServer(item)}
										className="h-8 w-8 text-muted-foreground hover:text-foreground"
									>
										<Edit className="w-4 h-4" />
									</Button>
									<Button
										variant="ghost"
										size="icon"
										onClick={() => onDeleteServer(item)}
										className="h-8 w-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10"
									>
										<Trash2 className="w-4 h-4" />
									</Button>
								</div>
							</div>

							<div className="grid grid-cols-2 gap-2 bg-muted/30 p-2.5 rounded-lg border border-border/40 text-xs font-mono text-muted-foreground min-w-0 w-full">
								<div className="flex items-center gap-1.5 truncate">
									<Globe className="w-3.5 h-3.5 text-primary shrink-0" />
									<span className="truncate">{item.ip_address}:{item.port || 22}</span>
								</div>
								<div className="flex items-center gap-1.5 truncate">
									<Key className="w-3.5 h-3.5 text-primary shrink-0" />
									<span className="truncate">{attachedKey?.name || 'No SSH Key'}</span>
								</div>
							</div>

							<div className="flex items-center justify-between pt-2 border-t border-border/40 text-xs min-w-0 w-full">
								<Button
									variant="ghost"
									size="sm"
									onClick={() => onToggleStatus(item)}
									className="h-8 text-xs font-medium gap-1 text-muted-foreground hover:text-foreground"
								>
									<Power className="w-3.5 h-3.5" />
									{isActive ? 'Disable' : 'Enable'}
								</Button>

								<Button
									variant="secondary"
									size="sm"
									onClick={() => onSetupServer(item)}
									className="h-8 text-xs font-semibold gap-1.5 px-3"
								>
									<Terminal className="w-3.5 h-3.5 text-primary" />
									Setup & Audit
								</Button>
							</div>
						</CardContent>
					</Card>
				);
			})}
		</div>
	);
}
