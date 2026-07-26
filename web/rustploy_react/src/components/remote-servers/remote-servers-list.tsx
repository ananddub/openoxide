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
			<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
				{[1, 2, 3].map(i => (
					<div key={i} className="h-52 bg-muted/40 animate-pulse rounded-2xl border border-border/60" />
				))}
			</div>
		);
	}

	if (servers.length === 0) {
		return (
			<Card className="bg-card/80 border-border shadow-sm p-12 text-center flex flex-col items-center justify-center rounded-2xl">
				<div className="w-14 h-14 rounded-2xl bg-primary/10 flex items-center justify-center text-primary mb-3">
					<Server className="w-7 h-7" />
				</div>
				<h3 className="text-base font-bold text-foreground">No Remote Servers Registered</h3>
				<p className="text-xs text-muted-foreground max-w-sm mt-1 mb-4">
					Connect external Linux nodes via SSH for remote application deployment and orchestration.
				</p>
			</Card>
		);
	}

	return (
		<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
			{servers.map(item => {
				const attachedKey = sshKeys.find((k: any) => k.id === item.ssh_key_id);
				const isActive = (item.server_status || 'ACTIVE').toUpperCase() === 'ACTIVE';
				const isTesting = testingConnId === item.id;

				return (
					<Card
						key={item.id}
						className="bg-card/90 backdrop-blur-md border-border/80 shadow-md hover:border-primary/50 transition-all duration-200 flex flex-col justify-between group rounded-2xl overflow-hidden hover:shadow-xl"
					>
						<CardContent className="p-5 flex flex-col gap-4">
							{/* Card Header: Server Name, Active Dot & Actions */}
							<div className="flex items-start justify-between gap-3">
								<div className="flex items-center gap-3 min-w-0">
									<div className={`p-3 rounded-2xl border transition-colors ${isActive ? 'bg-primary/10 border-primary/20 text-primary' : 'bg-muted border-border text-muted-foreground'}`}>
										<Server className="w-5 h-5" />
									</div>
									<div className="min-w-0">
										<div className="flex items-center gap-2">
											<h3 className="text-base font-bold text-foreground truncate group-hover:text-primary transition-colors">
												{item.name}
											</h3>
											<button
												type="button"
												onClick={() => onToggleStatus(item)}
												className={`flex items-center gap-1.5 text-[10px] font-bold px-2 py-0.5 rounded-full border transition-all ${isActive ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-500 hover:bg-emerald-500/20' : 'bg-rose-500/10 border-rose-500/30 text-rose-500 hover:bg-rose-500/20'}`}
												title="Click to toggle status"
											>
												<span className={`w-1.5 h-1.5 rounded-full ${isActive ? 'bg-emerald-500 animate-pulse' : 'bg-rose-500'}`} />
												{isActive ? 'Active' : 'Disabled'}
											</button>
										</div>
										<p className="text-xs text-muted-foreground font-mono mt-0.5">
											{item.username || 'root'}@{item.ip_address}
										</p>
									</div>
								</div>

								<div className="flex items-center gap-1 shrink-0">
									<Button variant="ghost" size="icon" onClick={() => onEditServer(item)} title="Edit server" className="h-8 w-8 rounded-xl text-muted-foreground hover:text-foreground hover:bg-muted">
										<Edit className="w-4 h-4" />
									</Button>
									<Button variant="ghost" size="icon" onClick={() => onDeleteServer(item)} title="Delete server" className="h-8 w-8 rounded-xl text-muted-foreground hover:text-destructive hover:bg-destructive/10">
										<Trash2 className="w-4 h-4" />
									</Button>
								</div>
							</div>

							{/* Clean Info Grid: IP Address and SSH Key */}
							<div className="flex flex-col gap-2 bg-muted/40 p-3 rounded-xl border border-border/60 text-xs">
								<div className="flex items-center justify-between gap-2 min-w-0">
									<div className="flex items-center gap-2 font-mono min-w-0">
										<Globe className="w-4 h-4 text-primary shrink-0" />
										<span className="text-foreground font-semibold truncate">
											{item.ip_address}:{item.port || 22}
										</span>
									</div>
									<Button
										type="button"
										variant="ghost"
										size="sm"
										onClick={() => handleCopyIp(item.id, item.ip_address)}
										className="h-6 px-2 text-[11px] font-semibold gap-1 text-muted-foreground hover:text-foreground"
										title="Copy IP"
									>
										{copiedId === item.id ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
										{copiedId === item.id ? 'Copied' : 'Copy'}
									</Button>
								</div>

								<div className="flex items-center justify-between gap-2 pt-2 border-t border-border/40 min-w-0">
									<div className="flex items-center gap-2 min-w-0">
										<Key className="w-4 h-4 text-amber-500 shrink-0" />
										<span className="text-muted-foreground font-medium truncate">
											{attachedKey?.name || 'No SSH Key attached'}
										</span>
									</div>
									{attachedKey?.name && (
										<span className="text-[10px] font-bold px-2 py-0.5 rounded-md bg-amber-500/10 border border-amber-500/20 text-amber-500 shrink-0">
											SSH Key
										</span>
									)}
								</div>
							</div>

							{/* Bottom Actions Toolbar */}
							<div className="flex items-center justify-between pt-2 border-t border-border/50 gap-2">
								<div className="flex items-center gap-2">
									<Button
										variant="outline"
										size="sm"
										onClick={() => handleTestConnection(item)}
										disabled={isTesting}
										className="h-8.5 text-xs font-semibold gap-1.5 border-border/80 hover:bg-muted rounded-xl"
									>
										{isTesting ? <RefreshCw className="w-3.5 h-3.5 animate-spin text-primary" /> : <ShieldCheck className="w-3.5 h-3.5 text-emerald-500" />}
										{isTesting ? 'Testing...' : 'Test SSH'}
									</Button>
									<Button
										variant="ghost"
										size="sm"
										onClick={() => onSetupServer(item)}
										className="h-8.5 text-xs font-semibold gap-1.5 text-muted-foreground hover:text-foreground rounded-xl"
									>
										<Activity className="w-3.5 h-3.5 text-primary" />
										Audit
									</Button>
								</div>

								{onOpenTerminal && (
									<Button
										variant="default"
										size="sm"
										onClick={() => onOpenTerminal(item)}
										className="h-8.5 text-xs font-bold gap-2 px-4 shadow-md rounded-xl"
									>
										<Terminal className="w-3.5 h-3.5" />
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
