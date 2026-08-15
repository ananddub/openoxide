import {createFileRoute} from '@tanstack/react-router';
import {ShieldCheck, Lock, CheckCircle2} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';

export const Route = createFileRoute('/_app/settings/certificates')({
	component: CertificatesPage,
});

function CertificatesPage() {
	return (
		<div className="p-6 flex flex-col gap-6 max-w-7xl mx-auto w-full animate-in fade-in duration-200">
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold text-foreground tracking-tight">SSL / TLS Certificates</h1>
				<p className="text-xs text-muted-foreground">
					Manage Let's Encrypt automatic SSL certificates and custom TLS keys
				</p>
			</div>

			<div className="p-5 border rounded-xl bg-card flex flex-col gap-4">
				<div className="flex items-center justify-between">
					<div className="flex items-center gap-3">
						<div className="size-10 rounded-xl bg-emerald-500/10 flex items-center justify-center text-emerald-500">
							<ShieldCheck className="size-6" />
						</div>
						<div>
							<h3 className="text-sm font-bold text-foreground">Let's Encrypt Auto-Renewal</h3>
							<p className="text-[10px] text-muted-foreground">Automatic ACME HTTP-01 challenge solver</p>
						</div>
					</div>
					<Badge variant="default" className="bg-emerald-500 text-[10px]">ACTIVE</Badge>
				</div>
				<p className="text-xs text-muted-foreground border-t border-border/40 pt-3">
					All domain routes configured in applications automatically request and renew SSL certificates via Traefik.
				</p>
			</div>
		</div>
	);
}
