export const KEY_COLORS = [
	'bg-blue-500/10 text-blue-500 border-blue-500/20',
	'bg-emerald-500/10 text-emerald-500 border-emerald-500/20',
	'bg-violet-500/10 text-violet-500 border-violet-500/20',
	'bg-amber-500/10 text-amber-500 border-amber-500/20',
	'bg-rose-500/10 text-rose-500 border-rose-500/20',
	'bg-cyan-500/10 text-cyan-500 border-cyan-500/20',
];

export function getKeyType(pubKey: string): string {
	if (!pubKey) return 'Unknown';
	if (pubKey.startsWith('ssh-rsa')) return 'RSA';
	if (pubKey.startsWith('ssh-ed25519')) return 'Ed25519';
	if (pubKey.startsWith('ecdsa-sha2')) return 'ECDSA';
	if (pubKey.startsWith('ssh-dss')) return 'DSA';
	return 'SSH';
}

export function getFingerprint(pubKey: string): string {
	if (!pubKey) return '—';
	const parts = pubKey.trim().split(' ');
	const keyData = parts[1] || parts[0] || '';
	return keyData.length > 20
		? keyData.slice(0, 10) + '…' + keyData.slice(-10)
		: keyData;
}

export const KEY_TYPES = ['All', 'RSA', 'Ed25519'] as const;
export type KeyTypeFilter = (typeof KEY_TYPES)[number];
