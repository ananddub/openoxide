import {Globe, Network, Shield} from 'lucide-react';

import {OptionRow} from '#/components/shared/sheet-wizard';
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

/** Mode picker — the wizard's option grammar, never a bare dropdown. */
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
		<div className="flex flex-col gap-2.5 border-b p-5">
			{MODE_TILES.map(tile => (
				<OptionRow
					key={tile.value}
					selected={mode === tile.value}
					disabled={disabled}
					onClick={() => onSelect(tile.value)}
					icon={<tile.icon className="size-4" strokeWidth={2} />}
					title={tile.label}
					subtitle={tile.desc}
				/>
			))}
		</div>
	);
}
