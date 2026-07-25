import {useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {Terminal, ShieldCheck, Cpu, RefreshCw, CheckCircle2} from 'lucide-react';

interface SetupServerModalProps {
	isOpen: boolean;
	server: any | null;
	onClose: () => void;
}

export function SetupServerModal({
	isOpen,
	server,
	onClose,
}: SetupServerModalProps) {
	const [testingConn, setTestingConn] = useState(false);
	const [auditing, setAuditing] = useState(false);
	const [auditResult, setAuditResult] = useState<any | null>(null);

	const testConnMutation = $api.useMutation('post', '/servers/{id}/test-connection');
	const auditMutation = $api.useMutation('post', '/servers/{id}/audit');

	const handleTestConnection = async () => {
		if (!server?.id) return;
		setTestingConn(true);
		try {
			await testConnMutation.mutateAsync({
				params: {path: {id: server.id}},
				body: {host_key_fingerprint: ''} as any,
			});
			toast.success(`SSH Connection to ${server.name} verified successfully!`);
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setTestingConn(false);
		}
	};

	const handleAudit = async () => {
		if (!server?.id) return;
		setAuditing(true);
		try {
			const res = await auditMutation.mutateAsync({
				params: {path: {id: server.id}},
				body: {host_key_fingerprint: ''} as any,
			});
			setAuditResult(res);
			toast.success('Server audit completed!');
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setAuditing(false);
		}
	};

	if (!server) return null;

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-xl md:max-w-2xl w-full bg-card border-border p-6 shadow-xl rounded-xl">
				<DialogHeader className="pb-3 border-b border-border/40">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
						<Terminal className="w-5 h-5 text-primary" />
						Setup & Audit: {server.name}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Test SSH connectivity and audit host specs ({server.username || 'root'}@{server.ip_address})
					</DialogDescription>
				</DialogHeader>

				<div className="flex flex-col gap-4 py-3">
					<div className="flex items-center justify-between p-3.5 bg-muted/20 border border-border/50 rounded-xl">
						<div className="flex items-center gap-2.5">
							<ShieldCheck className="w-5 h-5 text-primary" />
							<div>
								<h4 className="text-xs font-bold text-foreground">SSH Connection Test</h4>
								<p className="text-[11px] text-muted-foreground">Verify port {server.port || 22} reachability</p>
							</div>
						</div>
						<Button
							variant="outline"
							size="sm"
							onClick={handleTestConnection}
							disabled={testingConn}
							className="h-8 text-xs font-semibold gap-1.5"
						>
							{testingConn ? <RefreshCw className="w-3.5 h-3.5 animate-spin" /> : <ShieldCheck className="w-3.5 h-3.5" />}
							{testingConn ? 'Testing...' : 'Test Connection'}
						</Button>
					</div>

					<div className="flex items-center justify-between p-3.5 bg-muted/20 border border-border/50 rounded-xl">
						<div className="flex items-center gap-2.5">
							<Cpu className="w-5 h-5 text-primary" />
							<div>
								<h4 className="text-xs font-bold text-foreground">System Specs Audit</h4>
								<p className="text-[11px] text-muted-foreground">Check Docker engine, OS, CPU, and RAM</p>
							</div>
						</div>
						<Button
							variant="outline"
							size="sm"
							onClick={handleAudit}
							disabled={auditing}
							className="h-8 text-xs font-semibold gap-1.5"
						>
							{auditing ? <RefreshCw className="w-3.5 h-3.5 animate-spin" /> : <Cpu className="w-3.5 h-3.5" />}
							{auditing ? 'Auditing...' : 'Run Audit'}
						</Button>
					</div>

					{auditResult && (
						<div className="p-3.5 bg-emerald-500/10 border border-emerald-500/30 rounded-xl flex flex-col gap-2 font-mono text-xs text-foreground">
							<div className="flex items-center gap-1.5 text-emerald-500 font-bold font-sans">
								<CheckCircle2 className="w-4 h-4" /> Audit Outcome
							</div>
							<div className="grid grid-cols-2 gap-2 text-[11px] text-muted-foreground">
								<span>Docker: {auditResult.docker_installed ? 'Installed' : 'Missing'}</span>
								<span>Arch: {auditResult.arch || 'x86_64'}</span>
								<span>OS: {auditResult.os || 'Linux'}</span>
								<span>CPU Cores: {auditResult.cpus || 1}</span>
							</div>
						</div>
					)}
				</div>
			</DialogContent>
		</Dialog>
	);
}
