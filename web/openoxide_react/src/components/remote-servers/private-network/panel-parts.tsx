import {Check, Globe, Network, Shield} from 'lucide-react';

import {cn} from '#/api/utils';
import type {
	ConnectionMode,
	PrivateNetworkConfig,
	PrivateNetworkProvider,
} from './types';

export const EXTERNAL_PROVIDERS: Array<{
	value: PrivateNetworkProvider;
	label: string;
}> = [
	{value: 'TAILSCALE', label: 'Tailscale'},
	{value: 'NETBIRD', label: 'NetBird'},
	{value: 'ZEROTIER', label: 'ZeroTier'},
	{value: 'CUSTOM', label: 'Other / Custom VPN'},
];

/** The three connection modes as selectable tiles — never a bare dropdown. */
export const MODE_TILES: Array<{
	value: ConnectionMode;
	label: string;
	desc: string;
	icon: typeof Globe;
}> = [
	{
		value: 'DIRECT_SSH',
		label: 'Direct SSH',
		desc: 'Reach the host over its public IP — no tunnel.',
		icon: Globe,
	},
	{
		value: 'MANAGED_WIREGUARD',
		label: 'Managed WireGuard',
		desc: 'OpenOxide installs and owns the tunnel.',
		icon: Shield,
	},
	{
		value: 'EXTERNAL_PRIVATE_NETWORK',
		label: 'Existing VPN',
		desc: 'Route via Tailscale, NetBird, or similar.',
		icon: Network,
	},
];

export const MODE_LABELS: Record<ConnectionMode, string> = {
	DIRECT_SSH: 'Direct SSH',
	MANAGED_WIREGUARD: 'Managed WireGuard',
	EXTERNAL_PRIVATE_NETWORK: 'Existing VPN',
};

export function statusDot(status: PrivateNetworkConfig['status']): string {
	switch (status) {
		case 'ACTIVE':
			return 'bg-emerald-500';
		case 'FAILED':
			return 'bg-rose-500';
		case 'CONFIGURING':
			return 'bg-amber-500 animate-pulse';
		default:
			return 'bg-muted-foreground/40';
	}
}

export function remoteHostFromTunnel(cidr: string): string | null {
	const [address, prefix] = cidr.trim().split('/');
	const octets = address?.split('.').map(Number);
	if (
		prefix !== '24' ||
		octets?.length !== 4 ||
		octets.some(
			value => !Number.isInteger(value) || value < 0 || value > 255,
		)
	)
		return null;
	return `${octets[0]}.${octets[1]}.${octets[2]}.2`;
}

/** Label/value row in the right-rail cards (OpenShip Connection-card language). */
export function FactRow({
	icon: Icon,
	label,
	value,
	mono = true,
}: {
	icon: typeof Globe;
	label: string;
	value: React.ReactNode;
	mono?: boolean;
}) {
	return (
		<div className="flex items-center justify-between gap-3 py-2.5">
			<div className="flex min-w-0 items-center gap-2 text-sm text-muted-foreground">
				<Icon className="size-4 shrink-0" strokeWidth={2} />
				{label}
			</div>
			<div
				className={cn(
					'min-w-0 truncate text-sm text-foreground',
					mono && 'font-mono text-[13px]',
				)}>
				{value}
			</div>
		</div>
	);
}

export function Field({
	label,
	children,
}: {
	label: string;
	children: React.ReactNode;
}) {
	return (
		<div className="grid gap-1.5">
			<label className="text-xs font-medium">{label}</label>
			{children}
		</div>
	);
}

/** Mode picker row rendered from MODE_TILES. */
export function ModeTiles({
	mode,
	disabled,
	onSelect,
}: {
	mode: ConnectionMode;
	disabled: boolean;
	onSelect: (mode: ConnectionMode) => void;
}) {
	return (
		<div className="grid gap-2.5 p-5 sm:grid-cols-3">
			{MODE_TILES.map(tile => {
				const selected = mode === tile.value;
				return (
					<button
						key={tile.value}
						type="button"
						onClick={() => onSelect(tile.value)}
						disabled={disabled}
						className={cn(
							'flex cursor-pointer flex-col gap-1.5 rounded-xl border p-3.5 text-start transition-colors disabled:cursor-not-allowed disabled:opacity-60',
							selected
								? 'border-primary/60 bg-primary/5'
								: 'border-border/60 bg-muted/20 hover:bg-muted/40',
						)}>
						<span className="flex items-center gap-2 text-sm font-medium text-foreground">
							<tile.icon className="size-4 shrink-0" strokeWidth={2} />
							<span className="flex-1 truncate">{tile.label}</span>
							{selected && (
								<Check className="size-3.5 shrink-0 text-primary" />
							)}
						</span>
						<span className="text-xs leading-snug text-muted-foreground">
							{tile.desc}
						</span>
					</button>
				);
			})}
		</div>
	);
}
