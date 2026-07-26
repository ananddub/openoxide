import {useState} from 'react';
import {Button} from '#/components/ui/button';
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
	RefreshCw,
	Copy,
	Check,
	Activity,
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
	const [copiedId, setCopiedId] = useState<number | null>(null);
	const testConnMutation = $api.useMutation('post', '/servers/{id}/test-connection');

	const handleTestConnection = async (server: any) => {
		setTestingConnId(server.id);
		try {
			await testConnMutation.mutateAsync({
				params: {path: {id: server.id}},
				body: {host_key_fingerprint: ''} as any,
			});
			toast.success(`SSH Connection verified for ${server.name}`);
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setTestingConnId(null);
		}
	};

	const handleCopyIp = (id: number, ip: string) => {
		navigator.clipboard.writeText(ip);
		setCopiedId(id);
		toast.success(`Copied IP ${ip} to clipboard`);
		setTimeout(() => setCopiedId(null), 2000);
	};

	if (isLoading) {
		return (
			<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
				{[1, 2, 3].map(i => (
					<div key={i} className="h-44 bg-muted/40 animate-pulse rounded-xl border border-border/60" />
				))}
			</div>
		);
	}

	if (servers.length === 0) {
		return (
			<Card className="bg-card border-border shadow-sm p-12 text-center flex flex-col items-center justify-center rounded-xl">
				<div className="w-12 h-12 rounded-xl bg-primary/10 flex items-center justify-center text-primary mb-3">
					<Server className="w-6 h-6" />
				</div>
				<h3 className="text-sm font-bold text-foreground">No Remote Servers Registered</h3>
				<p className="text-xs text-muted-foreground max-w-sm mt-1 mb-4">
					Connect external Linux nodes via SSH for remote application deployment.
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
					<Card key={item.id} className="bg-card border-border shadow-sm hover:border-border/80 transition-all rounded-xl">
						<CardContent className="p-4 flex flex-col gap-3">
							{/* Header: Server Name, Status & Controls */}
							<div className="flex items-center justify-between">
								<div className="flex items-center gap-2.5 min-w-0">
									<Server className="w-4 h-4 text-primary shrink-0" />
									<h3 className="text-sm font-bold text-foreground truncate">{item.name}</h3>
									<button
										type="button"
										onClick={() => onToggleStatus(item)}
										className="flex items-center gap-1 text-[11px] font-medium text-muted-foreground hover:text-foreground"
										title="Toggle server status"
									>
										<span className={`w-2 h-2 rounded-full ${isActive ? 'bg-emerald-500' : 'bg-rose-500'}`} />
										{isActive ? 'Active' : 'Disabled'}
									</button>
								</div>

								<div className="flex items-center gap-1 shrink-0">
									<Button variant="ghost" size="icon" onClick={() => onEditServer(item)} title="Edit server" className="h-7 w-7 text-muted-foreground hover:text-foreground">
										<Edit className="w-3.5 h-3.5" />
									</Button>
									<Button variant="ghost" size="icon" onClick={() => onDeleteServer(item)} title="Delete server" className="h-7 w-7 text-muted-foreground hover:text-destructive">
										<Trash2 className="w-3.5 h-3.5" />
									</Button>
								</div>
							</div>

							{/* Connection Details */}
							<div className="flex flex-col gap-1 text-xs text-muted-foreground font-mono">
								<div className="flex items-center gap-2">
									<Globe className="w-3.5 h-3.5 shrink-0 text-muted-foreground/70" />
									<span className="text-foreground">{item.username || 'root'}@{item.ip_address}:{item.port || 22}</span>
									<button
										type="button"
										onClick={() => handleCopyIp(item.id, item.ip_address)}
										className="text-muted-foreground/60 hover:text-foreground ml-1"
										title="Copy IP"
									>
										{copiedId === item.id ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
									</button>
								</div>

								<div className="flex items-center gap-2 text-muted-foreground">
									<Key className="w-3.5 h-3.5 shrink-0 text-muted-foreground/70" />
									<span className="truncate">{attachedKey?.name || 'No Key'}</span>
								</div>
							</div>

							{/* Actions Row */}
							<div className="flex items-center justify-between pt-2 border-t border-border/50 gap-2">
								<div className="flex items-center gap-1.5">
									<Button
										variant="outline"
										size="sm"
										onClick={() => handleTestConnection(item)}
										disabled={isTesting}
										className="h-8 text-xs font-medium gap-1.5 px-3"
									>
										{isTesting ? <RefreshCw className="w-3 h-3 animate-spin" /> : <ShieldCheck className="w-3 h-3 text-emerald-500" />}
										{isTesting ? 'Testing...' : 'Test Connection'}
									</Button>
									<Button
										variant="ghost"
										size="sm"
										onClick={() => onSetupServer(item)}
										className="h-8 text-xs font-medium gap-1 px-2.5"
									>
										<Activity className="w-3 h-3" />
										Audit
									</Button>
								</div>

								{onOpenTerminal && (
									<Button
										variant="secondary"
										size="sm"
										onClick={() => onOpenTerminal(item)}
										className="h-8 text-xs font-medium gap-1.5 px-3"
									>
										<Terminal className="w-3 h-3" />
										Terminal
									</Button>
								)}
							</div>
						</CardContent>
					</Card>
				);
			})}
		</div>
	);
}
