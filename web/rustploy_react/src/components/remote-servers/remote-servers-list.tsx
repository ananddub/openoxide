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
	Copy,
	Check,
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
			toast.success(`Connection verified for ${server.name}`);
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
					<div key={i} className="h-48 bg-muted/40 animate-pulse rounded-xl border border-border/60" />
				))}
			</div>
		);
	}

	if (servers.length === 0) {
		return (
			<Card className="bg-card border-border shadow-sm p-12 text-center flex flex-col items-center justify-center rounded-xl">
				<div className="w-14 h-14 rounded-2xl bg-primary/10 flex items-center justify-center text-primary mb-3">
					<Server className="w-7 h-7" />
				</div>
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
					<Card key={item.id} className="bg-card/90 backdrop-blur-sm border-border/70 shadow-sm hover:border-primary/50 transition-all duration-200 flex flex-col justify-between group rounded-xl overflow-hidden">
						<CardContent className="p-4 flex flex-col gap-3.5">
							<div className="flex items-start justify-between gap-2">
								<div className="flex items-center gap-3 min-w-0">
									<div className={`p-2.5 rounded-xl border transition-colors ${isActive ? 'bg-primary/10 border-primary/20 text-primary' : 'bg-muted border-border text-muted-foreground'}`}>
										<Server className="w-4 h-4" />
									</div>
									<div className="min-w-0">
										<div className="flex items-center gap-2">
											<h3 className="text-sm font-bold text-foreground truncate group-hover:text-primary transition-colors">{item.name}</h3>
											<span className="flex items-center gap-1.5 text-[10px] font-semibold px-2 py-0.5 rounded-full border bg-muted/50 border-border">
												<span className={`w-1.5 h-1.5 rounded-full ${isActive ? 'bg-emerald-500 animate-pulse' : 'bg-rose-500'}`} />
												{isActive ? 'Active' : 'Disabled'}
											</span>
										</div>
										<p className="text-xs text-muted-foreground truncate min-w-0 mt-0.5 font-mono">
											{item.username || 'root'}@{item.ip_address}
										</p>
									</div>
								</div>

								<div className="flex items-center gap-1 shrink-0">
									<Button variant="ghost" size="icon" onClick={() => onEditServer(item)} title="Edit server" className="h-8 w-8 text-muted-foreground hover:text-foreground hover:bg-muted/80">
										<Edit className="w-3.5 h-3.5" />
									</Button>
									<Button variant="ghost" size="icon" onClick={() => onDeleteServer(item)} title="Delete server" className="h-8 w-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10">
										<Trash2 className="w-3.5 h-3.5" />
									</Button>
								</div>
							</div>

							<div className="grid grid-cols-2 gap-2 bg-muted/40 p-2.5 rounded-lg border border-border/50 text-xs font-mono text-muted-foreground min-w-0 w-full">
								<button type="button" onClick={() => handleCopyIp(item.id, item.ip_address)} className="flex items-center gap-1.5 truncate hover:text-foreground transition-colors text-left" title="Click to copy IP">
									<Globe className="w-3.5 h-3.5 text-primary shrink-0" />
									<span className="truncate">{item.ip_address}:{item.port || 22}</span>
									{copiedId === item.id ? <Check className="w-3 h-3 text-emerald-500 shrink-0 ml-auto" /> : <Copy className="w-3 h-3 text-muted-foreground/50 shrink-0 ml-auto opacity-0 group-hover:opacity-100 transition-opacity" />}
								</button>
								<div className="flex items-center gap-1.5 truncate">
									<Key className="w-3.5 h-3.5 text-primary shrink-0" />
									<span className="truncate">{attachedKey?.name || 'No SSH Key'}</span>
								</div>
							</div>

							<div className="flex items-center justify-between pt-2 border-t border-border/50 text-xs min-w-0 w-full gap-2">
								<div className="flex items-center gap-1.5">
									<Button variant="ghost" size="sm" onClick={() => onToggleStatus(item)} className="h-8 text-xs font-medium gap-1 text-muted-foreground hover:text-foreground hover:bg-muted/60">
										<Power className={`w-3.5 h-3.5 ${isActive ? 'text-rose-500' : 'text-emerald-500'}`} />
										{isActive ? 'Disable' : 'Enable'}
									</Button>
									<Button variant="outline" size="sm" onClick={() => handleTestConnection(item)} disabled={isTesting} className="h-8 text-xs font-semibold gap-1.5 border-border hover:bg-muted/80">
										{isTesting ? <RefreshCw className="w-3.5 h-3.5 animate-spin text-primary" /> : <ShieldCheck className="w-3.5 h-3.5 text-emerald-500" />}
										{isTesting ? 'Testing...' : 'Test'}
									</Button>
								</div>

								<div className="flex items-center gap-1.5">
									<Button variant="outline" size="sm" onClick={() => onOpenTerminal?.(item)} className="h-8 text-xs font-semibold gap-1.5 border-primary/30 text-primary hover:bg-primary/10 hover:border-primary/50 shadow-sm">
										<Terminal className="w-3.5 h-3.5" />
										Terminal
									</Button>
									<Button variant="secondary" size="sm" onClick={() => onSetupServer(item)} className="h-8 text-xs font-semibold px-2.5 hover:bg-secondary/80">
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
