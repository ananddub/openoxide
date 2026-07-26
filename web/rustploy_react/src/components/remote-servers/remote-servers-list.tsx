import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Card, CardContent} from '#/components/ui/card';
import {Badge} from '#/components/ui/badge';
import {
	Server,
	Globe,
	Key,
	Terminal,
	Copy,
	Check,
	Trash2,
	Edit,
	ShieldCheck,
	RefreshCw,
	Activity,
	CheckCircle2,
	XCircle,
} from 'lucide-react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';

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
	const [testStatusMap, setTestStatusMap] = useState<Record<number, 'success' | 'failed'>>({});
	const [copiedId, setCopiedId] = useState<number | null>(null);
	const testConnMutation = $api.useMutation('post', '/servers/{id}/test-connection');

	const handleTestConnection = async (server: any) => {
		setTestingConnId(server.id);
		try {
			await testConnMutation.mutateAsync({
				params: {path: {id: server.id}},
				body: {host_key_fingerprint: ''} as any,
			});
			setTestStatusMap(prev => ({...prev, [server.id]: 'success'}));
			toast.success(`SSH Connection verified for ${server.name}`);
		} catch (err: any) {
			setTestStatusMap(prev => ({...prev, [server.id]: 'failed'}));
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
				<h3 className="text-sm font-bold text-foreground">No Remote Servers Found</h3>
				<p className="text-xs text-muted-foreground max-w-sm mt-1 mb-4">
					Add your remote Linux VPS or bare metal server to start deploying Docker containers.
				</p>
			</Card>
		);
	}

	return (
		<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 w-full">
			{servers.map((item: any) => {
				const isTesting = testingConnId === item.id;
				const status = testStatusMap[item.id];
				const attachedKey = sshKeys.find(k => k.id === item.ssh_key_id);
				const isOnline = item.status === 1 || item.status === 'ONLINE';

				return (
					<Card key={item.id} className="bg-card border-border hover:border-border/80 transition-all rounded-xl shadow-sm">
						<CardContent className="p-4 flex flex-col gap-3">
							{/* Header: Status, Name, Edit, Delete */}
							<div className="flex items-start justify-between gap-3 min-w-0">
								<div className="flex items-center gap-2.5 min-w-0 flex-1">
									<div className="p-2 rounded-lg bg-muted/60 text-foreground shrink-0">
										<Server className="w-4 h-4" />
									</div>
									<div className="min-w-0 flex-1">
										<div className="flex items-center gap-2">
											<h3 className="text-sm font-bold text-foreground truncate">{item.name}</h3>
											<Badge
												variant={isOnline ? 'default' : 'secondary'}
												onClick={() => onToggleStatus(item)}
												className={`cursor-pointer text-[10px] px-1.5 py-0 h-4 font-semibold shrink-0 ${isOnline ? 'bg-emerald-500/15 text-emerald-500 border-emerald-500/30 hover:bg-emerald-500/25' : 'bg-muted text-muted-foreground'}`}
											>
												{isOnline ? 'ONLINE' : 'OFFLINE'}
											</Badge>
										</div>
										{item.description && (
											<p className="text-xs text-muted-foreground truncate mt-0.5 font-normal">
												{item.description}
											</p>
										)}
									</div>
								</div>

								<div className="flex items-center gap-1 shrink-0">
									<Button
										variant="ghost"
										size="icon"
										onClick={() => onEditServer(item)}
										title="Edit server"
										className="h-7 w-7 text-muted-foreground hover:text-foreground"
									>
										<Edit className="w-3.5 h-3.5" />
									</Button>
									<Button
										variant="ghost"
										size="icon"
										onClick={() => onDeleteServer(item)}
										title="Delete server"
										className="h-7 w-7 text-muted-foreground hover:text-destructive"
									>
										<Trash2 className="w-3.5 h-3.5" />
									</Button>
								</div>
							</div>

							{/* IP & SSH Key Info */}
							<div className="grid grid-cols-2 gap-2 p-2.5 bg-muted/30 border border-border/50 rounded-lg text-xs">
								<div className="flex items-center gap-1.5 min-w-0">
									<Globe className="w-3.5 h-3.5 shrink-0 text-muted-foreground/70" />
									<span className="text-foreground truncate">{item.username || 'root'}@{item.ip_address}</span>
									<button
										type="button"
										onClick={() => handleCopyIp(item.id, item.ip_address)}
										className="text-muted-foreground/60 hover:text-foreground shrink-0 ml-0.5"
										title="Copy IP"
									>
										{copiedId === item.id ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
									</button>
								</div>

								<div className="flex items-center gap-1.5 text-muted-foreground min-w-0">
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
										{isTesting ? (
											<RefreshCw className="w-3 h-3 animate-spin text-primary" />
										) : status === 'success' ? (
											<CheckCircle2 className="w-3.5 h-3.5 text-emerald-500" />
										) : status === 'failed' ? (
											<XCircle className="w-3.5 h-3.5 text-rose-500" />
										) : (
											<ShieldCheck className="w-3.5 h-3.5 text-muted-foreground" />
										)}
										{isTesting ? 'Testing...' : status === 'success' ? 'Connected' : status === 'failed' ? 'Failed' : 'Test Connection'}
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
