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
	Key,
	ShieldCheck,
	Trash2,
	Edit,
	Terminal,
	RefreshCw,
	Copy,
	Check,
	Activity,
	AlertCircle,
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
			<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 py-4">
				{[1, 2, 3].map(i => (
					<div key={i} className="h-16 bg-muted/40 animate-pulse rounded-xl border border-border/60" />
				))}
			</div>
		);
	}

	if (servers.length === 0) {
		return (
			<Card className="bg-card border-border shadow-sm p-12 text-center flex flex-col items-center justify-center rounded-xl my-4">
				<div className="w-12 h-12 rounded-xl bg-primary/10 flex items-center justify-center text-primary mb-3">
					<Server className="w-6 h-6" />
				</div>
				<h3 className="text-sm font-bold text-foreground">No Remote Servers Found</h3>
				<p className="text-xs text-muted-foreground max-w-sm mt-1 mb-4">
					Connect external Linux nodes via SSH for remote application deployment.
				</p>
			</Card>
		);
	}

	return (
		<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 py-4 w-full">
			{servers.map(item => {
				const attachedKey = sshKeys.find((k: any) => k.id === item.ssh_key_id);
				const isActive = (item.server_status || 'ACTIVE').toUpperCase() === 'ACTIVE';
				const isTesting = testingConnId === item.id;
				const status = testStateMap[item.id];

				return (
					<Card key={item.id} className="bg-card border-border hover:border-border/80 transition-all rounded-xl shadow-sm">
						<CardContent className="p-3.5 flex items-center justify-between gap-3">
							{/* Left: Status Dot, Server Name & IP Info */}
							<div className="flex items-center gap-3 min-w-0 flex-1">
								<button
									type="button"
									onClick={() => onToggleStatus(item)}
									title={`Status: ${isActive ? 'Active' : 'Disabled'} (Click to toggle)`}
									className="shrink-0"
								>
									<span className={`block w-2.5 h-2.5 rounded-full transition-transform hover:scale-125 ${isActive ? 'bg-emerald-500 shadow-sm shadow-emerald-500/50' : 'bg-rose-500'}`} />
								</button>

								<div className="flex flex-col min-w-0 flex-1">
									<div className="flex items-center gap-2">
										<h3 className="text-xs font-bold text-foreground truncate">{item.name}</h3>
										{attachedKey && (
											<span className="text-[10px] text-muted-foreground/80 flex items-center gap-1 shrink-0">
												<Key className="w-2.5 h-2.5" />
												<span className="truncate max-w-[80px]">{attachedKey.name}</span>
											</span>
										)}
									</div>
									<div className="flex items-center gap-1.5 text-[11px] text-muted-foreground font-mono truncate mt-0.5">
										<span className="truncate">{item.username || 'root'}@{item.ip_address}:{item.port || 22}</span>
										<button
											type="button"
											onClick={() => handleCopyIp(item.id, item.ip_address)}
											className="text-muted-foreground/60 hover:text-foreground shrink-0"
											title="Copy IP"
										>
											{copiedId === item.id ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
										</button>
									</div>
								</div>
							</div>

							{/* Right: Test Connection Icon Button & 3-Dots Dropdown */}
							<div className="flex items-center gap-1 shrink-0">
								<Button
									variant="outline"
									size="icon"
									onClick={() => handleTestConnection(item)}
									disabled={isTesting}
									title={
										isTesting
											? 'Testing SSH Connection...'
											: status === 'success'
												? 'SSH Connection Verified'
												: status === 'failed'
													? 'SSH Connection Failed'
													: 'Test SSH Connection'
									}
									className={`h-7 w-7 rounded-lg transition-all ${
										status === 'success'
											? 'border-emerald-500/40 bg-emerald-500/10'
											: status === 'failed'
												? 'border-rose-500/40 bg-rose-500/10'
												: 'border-border/60 bg-muted/20 hover:bg-muted/60'
									}`}
								>
									{isTesting ? (
										<RefreshCw className="w-3 h-3 animate-spin text-primary" />
									) : status === 'success' ? (
										<Check className="w-3 h-3 text-emerald-500 stroke-[2.5]" />
									) : status === 'failed' ? (
										<AlertCircle className="w-3 h-3 text-rose-500 stroke-[2.5]" />
									) : (
										<ShieldCheck className="w-3 h-3 text-emerald-500" />
									)}
								</Button>

								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<Button variant="ghost" size="icon" className="h-7 w-7 text-muted-foreground hover:text-foreground shrink-0" />
										}
									>
										<MoreVertical className="w-4 h-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-44 bg-card border-border shadow-xl rounded-xl p-1 text-xs z-50">
										<DropdownMenuItem
											className="flex cursor-pointer items-center gap-2 px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => handleTestConnection(item)}
										>
											<ShieldCheck className="w-3.5 h-3.5 text-emerald-500 shrink-0" />
											<span>Test Connection</span>
										</DropdownMenuItem>

										<DropdownMenuItem
											className="flex cursor-pointer items-center gap-2 px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => onSetupServer(item)}
										>
											<Activity className="w-3.5 h-3.5 text-sky-500 shrink-0" />
											<span>Audit Server</span>
										</DropdownMenuItem>

										{onOpenTerminal && (
											<DropdownMenuItem
												className="flex cursor-pointer items-center gap-2 px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
												onClick={() => onOpenTerminal(item)}
											>
												<Terminal className="w-3.5 h-3.5 text-amber-500 shrink-0" />
												<span>Open Terminal</span>
											</DropdownMenuItem>
										)}

										<DropdownMenuSeparator className="my-1 border-border/50" />

										<DropdownMenuItem
											className="flex cursor-pointer items-center gap-2 px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => onEditServer(item)}
										>
											<Edit className="w-3.5 h-3.5 text-muted-foreground shrink-0" />
											<span>Edit Server</span>
										</DropdownMenuItem>

										<DropdownMenuItem
											className="flex cursor-pointer items-center gap-2 px-2.5 py-1.5 rounded-lg hover:bg-muted/80 text-rose-500 text-xs font-medium"
											onClick={() => onDeleteServer(item)}
										>
											<Trash2 className="w-3.5 h-3.5 text-rose-500 shrink-0" />
											<span>Delete Server</span>
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</div>
						</CardContent>
					</Card>
				);
			})}
		</div>
	);
}
