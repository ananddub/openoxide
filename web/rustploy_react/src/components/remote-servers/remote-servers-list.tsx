import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Card, CardContent} from '#/components/ui/card';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
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
	CheckCircle2,
	XCircle,
	MoreVertical,
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
	const [testStateMap, setTestStateMap] = useState<Record<number, 'success' | 'failed'>>({});
	const [copiedId, setCopiedId] = useState<number | null>(null);
	const testConnMutation = $api.useMutation('post', '/servers/{id}/test-connection');

	const handleTestConnection = async (server: any) => {
		setTestingConnId(server.id);
		try {
			await testConnMutation.mutateAsync({
				params: {path: {id: server.id}},
				body: {host_key_fingerprint: ''} as any,
			});
			setTestStateMap(prev => ({...prev, [server.id]: 'success'}));
			toast.success(`SSH Connection verified for ${server.name}`);
		} catch (err: any) {
			setTestStateMap(prev => ({...prev, [server.id]: 'failed'}));
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
				const status = testStateMap[item.id];

				return (
					<Card key={item.id} className="bg-card border-border shadow-sm hover:border-border/80 transition-all rounded-xl">
						<CardContent className="p-4 flex flex-col gap-3">
							{/* Header: Server Name, Status & 3-Dots Dropdown */}
							<div className="flex items-center justify-between gap-2">
								<div className="flex items-center gap-2.5 min-w-0 flex-1">
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

								{/* Top Right 3-Dots Actions Dropdown */}
								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<Button variant="ghost" size="icon" className="h-7 w-7 text-muted-foreground hover:text-foreground shrink-0" />
										}
									>
										<MoreVertical className="w-4 h-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-48 bg-card border-border shadow-xl rounded-xl p-1 text-xs z-50">
										<DropdownMenuItem
											className="flex cursor-pointer items-center gap-2.5 px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => handleTestConnection(item)}
										>
											{isTesting ? (
												<RefreshCw className="w-3.5 h-3.5 animate-spin text-primary shrink-0" />
											) : status === 'success' ? (
												<CheckCircle2 className="w-3.5 h-3.5 text-emerald-500 shrink-0" />
											) : status === 'failed' ? (
												<XCircle className="w-3.5 h-3.5 text-rose-500 shrink-0" />
											) : (
												<ShieldCheck className="w-3.5 h-3.5 text-emerald-500 shrink-0" />
											)}
											<span>{isTesting ? 'Testing SSH...' : 'Test Connection'}</span>
										</DropdownMenuItem>

										<DropdownMenuItem
											className="flex cursor-pointer items-center gap-2.5 px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => onSetupServer(item)}
										>
											<Activity className="w-3.5 h-3.5 text-sky-500 shrink-0" />
											<span>Audit Server</span>
										</DropdownMenuItem>

										{onOpenTerminal && (
											<DropdownMenuItem
												className="flex cursor-pointer items-center gap-2.5 px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
												onClick={() => onOpenTerminal(item)}
											>
												<Terminal className="w-3.5 h-3.5 text-amber-500 shrink-0" />
												<span>Open Terminal</span>
											</DropdownMenuItem>
										)}

										<DropdownMenuSeparator className="my-1 border-border/50" />

										<DropdownMenuItem
											className="flex cursor-pointer items-center gap-2.5 px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => onEditServer(item)}
										>
											<Edit className="w-3.5 h-3.5 text-muted-foreground shrink-0" />
											<span>Edit Server</span>
										</DropdownMenuItem>

										<DropdownMenuItem
											className="flex cursor-pointer items-center gap-2.5 px-2.5 py-1.5 rounded-lg hover:bg-muted/80 text-rose-500 text-xs font-medium"
											onClick={() => onDeleteServer(item)}
										>
											<Trash2 className="w-3.5 h-3.5 text-rose-500 shrink-0" />
											<span>Delete Server</span>
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</div>

							{/* Connection Details */}
							<div className="flex flex-col gap-1 text-xs text-muted-foreground font-mono bg-muted/30 p-2.5 rounded-lg border border-border/50">
								<div className="flex items-center justify-between gap-2">
									<div className="flex items-center gap-2 min-w-0">
										<Globe className="w-3.5 h-3.5 shrink-0 text-muted-foreground/70" />
										<span className="text-foreground truncate">{item.username || 'root'}@{item.ip_address}:{item.port || 22}</span>
									</div>
									<button
										type="button"
										onClick={() => handleCopyIp(item.id, item.ip_address)}
										className="text-muted-foreground/60 hover:text-foreground shrink-0"
										title="Copy IP"
									>
										{copiedId === item.id ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
									</button>
								</div>

								<div className="flex items-center gap-2 text-muted-foreground pt-0.5">
									<Key className="w-3.5 h-3.5 shrink-0 text-muted-foreground/70" />
									<span className="truncate">{attachedKey?.name || 'No Key'}</span>
								</div>
							</div>
						</CardContent>
					</Card>
				);
			})}
		</div>
	);
}
