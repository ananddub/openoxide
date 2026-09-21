import {Activity, Globe, Network, Shield} from 'lucide-react';
import {serverHostLabel} from '#/api/utils';
import type {RemoteServerResponse} from '#/types/api-helpers';
import type {ConnectionMode, PrivateNetworkConfig} from './types';
import {FactRow, MODE_LABELS, statusDot} from './panel-parts';

const GUIDE: Array<{icon: typeof Globe; term: string; text: string}> = [
	{
		icon: Globe,
		term: 'Direct SSH',
		text: 'uses the public IP — simplest, but every deploy travels over the open internet.',
	},
	{
		icon: Shield,
		term: 'Managed WireGuard',
		text: 'installs a point-to-point tunnel; panel and host share one /24. Traffic stays encrypted.',
	},
	{
		icon: Network,
		term: 'Existing VPN',
		text: 'keeps your Tailscale/NetBird mesh — give the private address and OpenOxide routes through it.',
	},
];

/** Sticky rail: live connection facts + the "How it works" guide. */
export function PanelRail({
	server,
	config,
	loading,
	effectiveMode,
}: {
	server: RemoteServerResponse;
	config: PrivateNetworkConfig | null;
	loading: boolean;
	effectiveMode: ConnectionMode;
}) {
	const configured = !!config && config.connection_mode !== 'DIRECT_SSH';
	return (
		<div className="flex flex-col gap-4 lg:sticky lg:top-6">
			{/* Connection facts */}
			<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
				<div className="mb-2 flex items-center gap-2">
					<Network
						className="size-4 text-muted-foreground"
						strokeWidth={2}
					/>
					<h2 className="text-sm font-semibold text-foreground">
						Connection
					</h2>
				</div>
				<div className="divide-y divide-border/40">
					<FactRow
						icon={Shield}
						label="Mode"
						value={
							<span className="flex items-center gap-2">
								<span
									className={`size-2 shrink-0 rounded-full ${config ? statusDot(config.status) : 'bg-muted-foreground/40'}`}
								/>
								{MODE_LABELS[effectiveMode]}
							</span>
						}
						mono={false}
					/>
					<FactRow
						icon={Activity}
						label="Status"
						value={
							config ? (
								<span>{config.status}</span>
							) : (
								<span className="text-muted-foreground">
									{loading ? 'Loading…' : 'Not configured'}
								</span>
							)
						}
						mono={false}
					/>
					<FactRow
						icon={Globe}
						label="Public IP"
						value={serverHostLabel(server.ip_address)}
					/>
					<FactRow
						icon={Network}
						label="Private host"
						value={
							config?.private_host || (
								<span className="text-muted-foreground">—</span>
							)
						}
					/>
					{configured && config?.endpoint && (
						<FactRow
							icon={Activity}
							label="Endpoint"
							value={config.endpoint}
						/>
					)}
					{configured && config?.dns_name && (
						<FactRow
							icon={Globe}
							label="DNS name"
							value={config.dns_name}
						/>
					)}
					{configured && config && config.routes.length > 0 && (
						<FactRow
							icon={Network}
							label="Routes"
							value={config.routes.join(', ')}
						/>
					)}
				</div>
			</div>

			{/* Guide */}
			<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
				<div className="mb-3 flex items-center gap-2">
					<Shield
						className="size-4 text-muted-foreground"
						strokeWidth={2}
					/>
					<h2 className="text-sm font-semibold text-foreground">
						How it works
					</h2>
				</div>
				<ul className="space-y-2.5 text-xs leading-relaxed text-muted-foreground">
					{GUIDE.map(item => (
						<li key={item.term} className="flex gap-2">
							<item.icon className="mt-0.5 size-3.5 shrink-0" />
							<span>
								<strong className="font-medium text-foreground">
									{item.term}
								</strong>{' '}
								{item.text}
							</span>
						</li>
					))}
				</ul>
			</div>
		</div>
	);
}
