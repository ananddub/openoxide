import {useState} from 'react';
import {Terminal, Settings, Folder, Globe, X, Check, Copy} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';

export interface GlobalContainerItem {
	id: string;
	name: string;
	image: string;
	status: 'running' | 'stopped' | 'restarting';
	statusText: string;
	created: string;
	ports: string;
	networks: string[];
	mounts: {source: string; destination: string; mode: string}[];
	env: Record<string, string>;
}

interface DockerInspectModalProps {
	activeModal: {
		type: 'logs' | 'config' | 'mount' | 'network';
		container: GlobalContainerItem;
	} | null;
	onClose: () => void;
	logsStream: string[];
}

export function DockerInspectModal({activeModal, onClose, logsStream}: DockerInspectModalProps) {
	const [copied, setCopied] = useState(false);

	if (!activeModal) return null;

	const handleCopy = (text: string) => {
		navigator.clipboard.writeText(text);
		setCopied(true);
		toast.success('Copied to clipboard');
		setTimeout(() => setCopied(false), 2000);
	};

	return (
		<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
			<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-3xl flex flex-col overflow-hidden max-h-[85vh] animate-in fade-in duration-150">
				{/* Modal Header */}
				<div className="p-4 border-b border-border flex items-center justify-between bg-muted/30">
					<div className="flex items-center gap-2">
						{activeModal.type === 'logs' && <Terminal className="w-4 h-4 text-sky-400" />}
						{activeModal.type === 'config' && <Settings className="w-4 h-4 text-purple-400" />}
						{activeModal.type === 'mount' && <Folder className="w-4 h-4 text-amber-400" />}
						{activeModal.type === 'network' && <Globe className="w-4 h-4 text-emerald-400" />}
						<h3 className="text-xs font-bold text-foreground capitalize">
							Container {activeModal.type} — {activeModal.container.name}
						</h3>
					</div>
					<div className="flex items-center gap-2">
						<Button
							variant="outline"
							size="sm"
							onClick={() => handleCopy(JSON.stringify(activeModal.container, null, 2))}
							className="h-7 text-xs font-semibold px-2 flex items-center gap-1 border-border"
						>
							{copied ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
							{copied ? 'Copied' : 'Copy Data'}
						</Button>
						<Button variant="ghost" size="icon" onClick={onClose} className="h-7 w-7 text-muted-foreground">
							<X className="w-4 h-4" />
						</Button>
					</div>
				</div>

				{/* Modal Body Content */}
				<div className="p-4 overflow-y-auto">
					{/* 1. Logs View */}
					{activeModal.type === 'logs' && (
						<DeploymentViewer
							logs={logsStream}
							isLoading={logsStream.length === 0}
							isLive={true}
							isDeployment={false}
							heightClass="h-[400px]"
							loadingText={`Connecting to '${activeModal.container.name}' container logs...`}
							emptyText="No container log output received."
						/>
					)}

					{/* 2. Config View */}
					{activeModal.type === 'config' && (
						<div className="flex flex-col gap-4">
							<div className="bg-muted/30 border border-border rounded-lg p-3 font-mono text-xs flex flex-col gap-2">
								<span className="text-muted-foreground font-bold uppercase text-[10px]">Environment Variables</span>
								<pre className="text-zinc-200 whitespace-pre-wrap break-all">
									{JSON.stringify(activeModal.container.env, null, 2)}
								</pre>
							</div>

							<div className="bg-muted/30 border border-border rounded-lg p-3 font-mono text-xs flex flex-col gap-2">
								<span className="text-muted-foreground font-bold uppercase text-[10px]">Container Specifications</span>
								<div className="grid grid-cols-2 gap-2 text-foreground">
									<div><b>ID:</b> {activeModal.container.id}</div>
									<div><b>Name:</b> {activeModal.container.name}</div>
									<div><b>Image:</b> {activeModal.container.image}</div>
									<div><b>Status:</b> {activeModal.container.statusText}</div>
								</div>
							</div>
						</div>
					)}

					{/* 3. Mount View */}
					{activeModal.type === 'mount' && (
						<div className="flex flex-col gap-3">
							<p className="text-xs text-muted-foreground">Volume mounts and directory bind paths for this container</p>
							{activeModal.container.mounts.map((m, idx) => (
								<div key={idx} className="bg-muted/30 border border-border rounded-lg p-3 flex flex-col gap-1.5 font-mono text-xs">
									<div className="flex items-center justify-between">
										<span className="text-primary font-bold">Host Source:</span>
										<Badge variant="outline" className="text-[10px] uppercase font-bold">{m.mode}</Badge>
									</div>
									<span className="text-foreground bg-background p-1.5 rounded border border-border/40 truncate">{m.source}</span>
									<span className="text-primary font-bold mt-1">Container Destination:</span>
									<span className="text-foreground bg-background p-1.5 rounded border border-border/40 truncate">{m.destination}</span>
								</div>
							))}
						</div>
					)}

					{/* 4. Network View */}
					{activeModal.type === 'network' && (
						<div className="flex flex-col gap-4">
							<div className="bg-muted/30 border border-border rounded-lg p-3 flex flex-col gap-2">
								<span className="text-xs font-bold text-foreground">Connected Docker Networks</span>
								<div className="flex flex-wrap gap-2">
									{activeModal.container.networks.map((net, i) => (
										<Badge key={i} variant="secondary" className="font-mono text-xs">
											<Globe className="w-3 h-3 mr-1 text-emerald-400" /> {net}
										</Badge>
									))}
								</div>
							</div>

							<div className="bg-muted/30 border border-border rounded-lg p-3 flex flex-col gap-2 font-mono text-xs">
								<span className="text-xs font-bold text-foreground">Exposed Container Ports</span>
								<span className="text-foreground bg-background p-2 rounded border border-border/40">
									{activeModal.container.ports}
								</span>
							</div>
						</div>
					)}
				</div>
			</div>
		</div>
	);
}
