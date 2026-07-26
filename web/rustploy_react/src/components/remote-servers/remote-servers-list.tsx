import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Card, CardContent} from '#/components/ui/card';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	Server,
	Globe,
	Key,
	ShieldCheck,
	Trash2,
	Edit,
	Terminal,
	Power,
	RefreshCw,
} from 'lucide-react';

interface RemoteServersListProps {
	servers: any[];
	sshKeys: any[];
	isLoading: boolean;
	onEditServer: (server: any) => void;
	onDeleteServer: (server: any) => void;
	onSetupServer: (server: any) => void;
	onToggleStatus: (server: any) => void;
	onOpenTerminal?: (server: any) => void;
}

export function RemoteServersList({
	servers,
	sshKeys,
	isLoading,
	onEditServer,
	onDeleteServer,
	onSetupServer,
	onToggleStatus,
	onOpenTerminal,
}: RemoteServersListProps) {
	const [testingConnId, setTestingConnId] = useState<number | null>(null);
	const testConnMutation = $api.useMutation('post', '/servers/{id}/test-connection');

	const handleTestConnection = async (server: any) => {
		setTestingConnId(server.id);
		try {
			await testConnMutation.mutateAsync({
				params: {path: {id: server.id}},
				body: {host_key_fingerprint: ''} as any,
			});
			toast.success(`Connection test succeeded for ${server.name}`);
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setTestingConnId(null);
		}
	};

	if (isLoading) {
		return (
			<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
				{[1, 2, 3].map(i => (
					<div key={i} className="h-44 bg-muted/40 animate-pulse rounded-xl border border-border" />
				))}
			</div>
		);
	}

	if (servers.length === 0) {
		return (
			<Card className="bg-card border-border shadow-sm p-12 text-center flex flex-col items-center justify-center">
				<Server className="w-12 h-12 text-muted-foreground/40 mb-3" />
				<h3 className="text-base font-bold text-foreground">No Remote Servers Registered</h3>
				<p className="text-xs text-muted-foreground max-w-sm mt-1 mb-4">
					Connect external Linux servers via SSH for remote application deployment and Docker compose orchestration.
				</p>
			</Card>
		);
	}

	return (
		<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
			{servers.map(item => {
				const attachedKey = sshKeys.find((k: any) => k.id === item.ssh_key_id);
				const isActive = (item.server_status || 'ACTIVE').toUpperCase() === 'ACTIVE';
				const isTesting = testingConnId === item.id;

				return (
					<Card key={item.id} className="bg-card border-border shadow-sm hover:border-border/80 transition-all flex flex-col justify-between">
						<CardContent className="p-4 flex flex-col gap-3">
							<div className="flex items-start justify-between gap-2">
								<div className="flex items-center gap-2.5 min-w-0">
									<div className="p-2 rounded-lg bg-primary/10 text-primary shrink-0">
										<Server className="w-4 h-4" />
									</div>
									<div className="min-w-0">
										<div className="flex items-center gap-2">
											<h3 className="text-sm font-bold text-foreground truncate">{item.name}</h3>
											<Badge variant={isActive ? 'default' : 'secondary'} className="text-[10px] px-1.5 py-0 font-semibold">
												{isActive ? 'Active' : 'Disabled'}
											</Badge>
										</div>
										<p className="text-xs text-muted-foreground truncate min-w-0 mt-0.5">
											{item.description || `${item.username || 'root'}@${item.ip_address}`}
										</p>
									</div>
								</div>

								<div className="flex items-center gap-1 shrink-0">
									<Button variant="ghost" size="icon" onClick={() => onEditServer(item)} className="h-8 w-8 text-muted-foreground hover:text-foreground">
										<Edit className="w-4 h-4" />
									</Button>
									<Button variant="ghost" size="icon" onClick={() => onDeleteServer(item)} className="h-8 w-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10">
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

							<div className="flex items-center justify-between pt-2 border-t border-border/40 text-xs min-w-0 w-full gap-1 flex-wrap">
								<div className="flex items-center gap-1">
									<Button variant="ghost" size="sm" onClick={() => onToggleStatus(item)} className="h-8 text-xs font-medium gap-1 text-muted-foreground hover:text-foreground">
										<Power className="w-3.5 h-3.5" />
										{isActive ? 'Disable' : 'Enable'}
									</Button>
									<Button variant="outline" size="sm" onClick={() => handleTestConnection(item)} disabled={isTesting} className="h-8 text-xs font-semibold gap-1">
										{isTesting ? <RefreshCw className="w-3.5 h-3.5 animate-spin" /> : <ShieldCheck className="w-3.5 h-3.5 text-primary" />}
										{isTesting ? 'Testing...' : 'Test Connection'}
									</Button>
								</div>

								<div className="flex items-center gap-1">
									<Button variant="outline" size="sm" onClick={() => onOpenTerminal?.(item)} className="h-8 text-xs font-semibold gap-1 border-primary/40 text-primary hover:bg-primary/10">
										<Terminal className="w-3.5 h-3.5" />
										Terminal
									</Button>
									<Button variant="secondary" size="sm" onClick={() => onSetupServer(item)} className="h-8 text-xs font-semibold gap-1 px-2.5">
										Audit
									</Button>
								</div>
							</div>
						</CardContent>
					</Card>
				);
			})}
		</div>
	);
}
