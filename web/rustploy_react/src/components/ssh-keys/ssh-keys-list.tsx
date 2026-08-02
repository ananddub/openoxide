import {useState, useMemo} from 'react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Input} from '#/components/ui/input';
import {Separator} from '#/components/ui/separator';
import {Skeleton} from '#/components/ui/skeleton';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {Key, Copy, Check, FileKey2, Eye, Trash2, MoreVertical, ShieldCheck, Search, X} from 'lucide-react';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {toast} from 'sonner';
import type {SshKeyResponse} from '#/types/api-helpers';

interface SshKeysListProps {
	keys: SshKeyResponse[];
	isLoading: boolean;
	onViewKey: (key: SshKeyResponse) => void;
	onDeleteKey: (key: SshKeyResponse) => void;
}

const KEY_COLORS = [
	'bg-blue-500/10 text-blue-500 border-blue-500/20',
	'bg-emerald-500/10 text-emerald-500 border-emerald-500/20',
	'bg-violet-500/10 text-violet-500 border-violet-500/20',
	'bg-amber-500/10 text-amber-500 border-amber-500/20',
	'bg-rose-500/10 text-rose-500 border-rose-500/20',
	'bg-cyan-500/10 text-cyan-500 border-cyan-500/20',
];

function getKeyType(pubKey: string): string {
	if (!pubKey) return 'Unknown';
	if (pubKey.startsWith('ssh-rsa')) return 'RSA';
	if (pubKey.startsWith('ssh-ed25519')) return 'Ed25519';
	if (pubKey.startsWith('ecdsa-sha2')) return 'ECDSA';
	if (pubKey.startsWith('ssh-dss')) return 'DSA';
	return 'SSH';
}

function getFingerprint(pubKey: string): string {
	if (!pubKey) return '—';
	const parts = pubKey.trim().split(' ');
	const keyData = parts[1] || parts[0] || '';
	return keyData.length > 20 ? keyData.slice(0, 10) + '…' + keyData.slice(-10) : keyData;
}

const KEY_TYPES = ['All', 'RSA', 'Ed25519'] as const;
type KeyTypeFilter = (typeof KEY_TYPES)[number];

export function SshKeysList({keys, isLoading, onViewKey, onDeleteKey}: SshKeysListProps) {
	const [copiedId, setCopiedId] = useState<number | null>(null);
	const [search, setSearch] = useState('');
	const [typeFilter, setTypeFilter] = useState<KeyTypeFilter>('All');

	const keysList = Array.isArray(keys) ? keys : [];

	const filtered = useMemo(() => {
		return keysList.filter(k => {
			const matchName = k.name.toLowerCase().includes(search.toLowerCase());
			const matchType = typeFilter === 'All' || getKeyType(k.public_key) === typeFilter;
			return matchName && matchType;
		});
	}, [keysList, search, typeFilter]);

	const handleCopy = (id: number, pubKey: string, name: string) => {
		if (!pubKey) return void toast.error('No public key to copy');
		navigator.clipboard.writeText(pubKey);
		setCopiedId(id);
		toast.success(`"${name}" copied!`);
		setTimeout(() => setCopiedId(null), 2000);
	};

	const clearFilters = () => {
		setSearch('');
		setTypeFilter('All');
	};
	const hasFilters = search !== '' || typeFilter !== 'All';

	/* ── Loading ── */
	if (isLoading) {
		return (
			<div className="flex flex-col gap-2">
				{[1, 2, 3].map(i => (
					<div key={i} className="flex items-center gap-4 px-4 py-4 border border-border rounded-lg">
						<Skeleton className="w-9 h-9 rounded-lg shrink-0" />
						<div className="flex-1 space-y-2">
							<Skeleton className="h-3.5 w-40" />
							<Skeleton className="h-3 w-64" />
						</div>
						<Skeleton className="h-5 w-16 rounded-full" />
					</div>
				))}
			</div>
		);
	}

	return (
		<div className="flex flex-col gap-3">
			{/* ── Search + Filter bar ── */}
			<div className="flex items-center gap-2">
				{/* Search */}
				<div className="relative flex-1">
					<Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground pointer-events-none" />
					<Input
						placeholder="Search keys…"
						value={search}
						onChange={e => setSearch(e.target.value)}
						className="pl-8 h-8 text-xs"
					/>
					{search && (
						<button
							onClick={() => setSearch('')}
							className="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
						>
							<X className="w-3 h-3" />
						</button>
					)}
				</div>

				{/* Type filter dropdown */}
				<Select value={typeFilter} onValueChange={(v) => setTypeFilter(v as KeyTypeFilter)}>
					<SelectTrigger size="sm" className="h-8 w-36 text-xs">
						<SelectValue placeholder="Key type" />
					</SelectTrigger>
					<SelectContent>
						{KEY_TYPES.map(t => (
							<SelectItem key={t} value={t}>{t === 'All' ? 'All Types' : t}</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>

			{/* ── Empty state ── */}
			{keysList.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 py-20 text-center border border-dashed border-border rounded-lg">
					<div className="w-12 h-12 rounded-full bg-muted flex items-center justify-center">
						<FileKey2 className="w-5 h-5 text-muted-foreground" />
					</div>
					<div>
						<p className="text-sm font-medium text-foreground">No SSH keys yet</p>
						<p className="text-xs text-muted-foreground mt-0.5">Add a key pair to authenticate with remote servers.</p>
					</div>
				</div>
			) : filtered.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 py-14 text-center border border-dashed border-border rounded-lg">
					<Search className="w-5 h-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">No keys match your filter</p>
					<Button variant="ghost" size="sm" onClick={clearFilters} className="text-xs h-7">Clear filters</Button>
				</div>
			) : (
				/* ── List ── */
				<div className="border border-border rounded-lg overflow-hidden divide-y divide-border">
					{filtered.map((item: SshKeyResponse, idx: number) => {
						const isCopied = copiedId === item.id;
						const iconCls = KEY_COLORS[idx % KEY_COLORS.length];
						const keyType = getKeyType(item.public_key);
						const fingerprint = getFingerprint(item.public_key);

						return (
							<div
								key={item.id}
								className="group flex items-center gap-4 px-4 py-3.5 bg-card hover:bg-accent/30 transition-colors"
							>
								<div className={`shrink-0 w-9 h-9 rounded-lg border flex items-center justify-center ${iconCls}`}>
									<Key className="w-4 h-4" />
								</div>

								<div className="flex-1 min-w-0">
									<div className="flex items-center gap-2">
										<span className="text-sm font-medium text-foreground truncate">{item.name}</span>
										<Badge variant="secondary" className="shrink-0 text-[10px] font-mono gap-1 py-0">
											<ShieldCheck className="w-2.5 h-2.5" />
											{keyType}
										</Badge>
									</div>
									<p className="text-[11px] font-mono text-muted-foreground mt-0.5 truncate">
										{item.public_key ? fingerprint : 'Private key only'}
									</p>
								</div>

								<div className="flex items-center gap-3 opacity-0 group-hover:opacity-100 transition-opacity">
									<button
										onClick={() => handleCopy(item.id, item.public_key, item.name)}
										title="Copy public key"
										className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
									>
										{isCopied ? <Check className="w-4 h-4 text-emerald-500" /> : <Copy className="w-4 h-4" />}
									</button>
									<button
										onClick={() => onViewKey(item)}
										title="View details"
										className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
									>
										<Eye className="w-4 h-4" />
									</button>
								</div>

								<Separator orientation="vertical" className="h-5 opacity-0 group-hover:opacity-100 transition-opacity" />

								<DropdownMenu>
									<DropdownMenuTrigger render={<button className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer" />}>
										<MoreVertical className="w-4 h-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-44">
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onViewKey(item)}>
											<Eye className="w-3.5 h-3.5" /> View Details
										</DropdownMenuItem>
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => handleCopy(item.id, item.public_key, item.name)}>
											{isCopied ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
											{isCopied ? 'Copied!' : 'Copy Public Key'}
										</DropdownMenuItem>
										<DropdownMenuSeparator />
										<DropdownMenuItem className="gap-2 cursor-pointer text-destructive focus:text-destructive" onClick={() => onDeleteKey(item)}>
											<Trash2 className="w-3.5 h-3.5" /> Delete Key
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</div>
						);
					})}
				</div>
			)}

			{/* Result count */}
			{hasFilters && filtered.length > 0 && (
				<p className="text-xs text-muted-foreground px-1">
					Showing {filtered.length} of {keysList.length} keys
				</p>
			)}
		</div>
	);
}
