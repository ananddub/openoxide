import {Activity, Globe, Network, Shield} from 'lucide-react';
import {serverHostLabel} from '#/api/utils';
import {ListCard, SectionLabel} from '#/components/settings/list-card';
import type {RemoteServerResponse} from '#/types/api-helpers';
import type {ConnectionMode, PrivateNetworkConfig} from './types';
import {ConnectionRow} from '../server-detail-parts';
import {MODE_LABELS, statusDot} from './panel-parts';

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
		<div className="flex flex-col gap-5 lg:sticky lg:top-6">
			{/* Connection facts */}
			<div className="flex flex-col gap-2">
				<SectionLabel>Connection</SectionLabel>
				<ListCard>
					<ConnectionRow
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
					<ConnectionRow
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
					<ConnectionRow
						icon={Globe}
						label="Public IP"
						value={serverHostLabel(server.ip_address)}
					/>
					<ConnectionRow
						icon={Network}
						label="Private host"
						value={
							config?.private_host || (
								<span className="text-muted-foreground">—</span>
							)
						}
					/>
					{configured && config?.endpoint && (
						<ConnectionRow
							icon={Activity}
							label="Endpoint"
							value={config.endpoint}
						/>
					)}
					{configured && config?.dns_name && (
						<ConnectionRow
							icon={Globe}
							label="DNS name"
							value={config.dns_name}
						/>
					)}
					{configured && config && config.routes.length > 0 && (
						<ConnectionRow
							icon={Network}
							label="Routes"
							value={config.routes.join(', ')}
						/>
					)}
				</ListCard>
			</div>

			{/* Guide */}
			<div className="flex flex-col gap-2">
				<SectionLabel>How it works</SectionLabel>
				<div className="rounded-xl border bg-card p-5">
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
		</div>
	);
}
