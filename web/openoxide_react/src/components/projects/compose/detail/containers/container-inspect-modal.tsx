import {useState} from 'react';
import {
	Terminal,
	Settings,
	Folder,
	Globe,
	X,
	Check,
	Copy,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';

export interface ContainerItem {
	id: string;
	name: string;
	service: string;
	image: string;
	status: 'running' | 'stopped' | 'restarting';
	statusText: string;
	ports: string;
	networks: string[];
	mounts: {source: string; destination: string; mode: string}[];
	env: Record<string, string>;
}

interface ContainerInspectModalProps {
	activeModal: {
		type: 'logs' | 'config' | 'mount' | 'network';
		container: ContainerItem;
	} | null;
	onClose: () => void;
	logsStream: string[];
}

export function ContainerInspectModal({
	activeModal,
	onClose,
	logsStream,
}: ContainerInspectModalProps) {
	const [copied, setCopied] = useState(false);

	if (!activeModal) return null;

	const handleCopy = (text: string) => {
		navigator.clipboard.writeText(text);
		setCopied(true);
		toast.success('Copied to clipboard');
		setTimeout(() => setCopied(false), 2000);
	};

	return (
		<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm">
			<div className="flex max-h-[85vh] w-full max-w-3xl animate-in flex-col overflow-hidden rounded-xl border border-border bg-card shadow-2xl duration-150 fade-in">
				{/* Modal Header */}
				<div className="flex items-center justify-between border-b border-border bg-muted/30 p-4">
					<div className="flex items-center gap-2">
						{activeModal.type === 'logs' && (
							<Terminal className="h-4 w-4 text-sky-400" />
						)}
						{activeModal.type === 'config' && (
							<Settings className="h-4 w-4 text-purple-400" />
						)}
						{activeModal.type === 'mount' && (
							<Folder className="h-4 w-4 text-amber-400" />
						)}
						{activeModal.type === 'network' && (
							<Globe className="h-4 w-4 text-emerald-400" />
						)}
						<h3 className="text-xs font-bold text-foreground capitalize">
							Container {activeModal.type} — {activeModal.container.name}
						</h3>
					</div>
					<div className="flex items-center gap-2">
						<Button
							variant="outline"
							size="sm"
							onClick={() =>
								handleCopy(JSON.stringify(activeModal.container, null, 2))
							}
							className="flex h-7 items-center gap-1 border-border px-2 text-xs font-semibold">
							{copied ? (
								<Check className="h-3 w-3 text-emerald-500" />
							) : (
								<Copy className="h-3 w-3" />
							)}
							{copied ? 'Copied' : 'Copy Data'}
						</Button>
						<Button
							variant="ghost"
							size="icon"
							onClick={onClose}
							className="h-7 w-7 text-muted-foreground">
							<X className="h-4 w-4" />
						</Button>
					</div>
				</div>

				{/* Modal Body Content */}
				<div className="overflow-y-auto p-4">
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
							<div className="flex flex-col gap-2 rounded-lg border border-border bg-muted/30 p-3 font-mono text-xs">
								<span className="text-[10px] font-bold text-muted-foreground uppercase">
									Environment Variables
								</span>
								<pre className="break-all whitespace-pre-wrap text-zinc-200">
									{JSON.stringify(activeModal.container.env, null, 2)}
								</pre>
							</div>

							<div className="flex flex-col gap-2 rounded-lg border border-border bg-muted/30 p-3 font-mono text-xs">
								<span className="text-[10px] font-bold text-muted-foreground uppercase">
									Container Specifications
								</span>
								<div className="grid grid-cols-2 gap-2 text-foreground">
									<div>
										<b>ID:</b> {activeModal.container.id}
									</div>
									<div>
										<b>Image:</b> {activeModal.container.image}
									</div>
									<div>
										<b>Service:</b> {activeModal.container.service}
									</div>
									<div>
										<b>Status:</b> {activeModal.container.statusText}
									</div>
								</div>
							</div>
						</div>
					)}

					{/* 3. Mount View */}
					{activeModal.type === 'mount' && (
						<div className="flex flex-col gap-3">
							<p className="text-xs text-muted-foreground">
								Volume mounts and directory bind paths for this container
							</p>
							{activeModal.container.mounts.map((m, idx) => (
								<div
									key={idx}
									className="flex flex-col gap-1.5 rounded-lg border border-border bg-muted/30 p-3 font-mono text-xs">
									<div className="flex items-center justify-between">
										<span className="font-bold text-primary">
											Host Source:
										</span>
										<Badge
											variant="outline"
											className="text-[10px] font-bold uppercase">
											{m.mode}
										</Badge>
									</div>
									<span className="truncate rounded border border-border/40 bg-background p-1.5 text-foreground">
										{m.source}
									</span>
									<span className="mt-1 font-bold text-primary">
										Container Destination:
									</span>
									<span className="truncate rounded border border-border/40 bg-background p-1.5 text-foreground">
										{m.destination}
									</span>
								</div>
							))}
						</div>
					)}

					{/* 4. Network View */}
					{activeModal.type === 'network' && (
						<div className="flex flex-col gap-4">
							<div className="flex flex-col gap-2 rounded-lg border border-border bg-muted/30 p-3">
								<span className="text-xs font-bold text-foreground">
									Connected Docker Networks
								</span>
								<div className="flex flex-wrap gap-2">
									{activeModal.container.networks.map((net, i) => (
										<Badge
											key={i}
											variant="secondary"
											className="font-mono text-xs">
											<Globe className="mr-1 h-3 w-3 text-emerald-400" />{' '}
											{net}
										</Badge>
									))}
								</div>
							</div>

							<div className="flex flex-col gap-2 rounded-lg border border-border bg-muted/30 p-3 font-mono text-xs">
								<span className="text-xs font-bold text-foreground">
									Exposed Container Ports
								</span>
								<span className="rounded border border-border/40 bg-background p-2 text-foreground">
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
