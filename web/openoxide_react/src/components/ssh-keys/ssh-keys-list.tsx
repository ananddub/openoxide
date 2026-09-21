import {useState, useMemo} from 'react';
import {FileKey2, Search, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Skeleton} from '#/components/ui/skeleton';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {toast} from 'sonner';
import type {SshKeyResponse} from '#/types/api-helpers';
import {
	getKeyType,
	KEY_TYPES,
	type KeyTypeFilter,
} from '#/components/ssh-keys/ssh-keys-shared';
import {SshKeyRow} from '#/components/ssh-keys/ssh-key-row';

interface SshKeysListProps {
	keys: SshKeyResponse[];
	isLoading: boolean;
	onViewKey: (key: SshKeyResponse) => void;
	onDeleteKey: (key: SshKeyResponse) => void;
}

export function SshKeysList({
	keys,
	isLoading,
	onViewKey,
	onDeleteKey,
}: SshKeysListProps) {
	const [copiedId, setCopiedId] = useState<number | null>(null);
	const [search, setSearch] = useState('');
	const [typeFilter, setTypeFilter] = useState<KeyTypeFilter>('All');

	const keysList = Array.isArray(keys) ? keys : [];

	const filtered = useMemo(() => {
		return keysList.filter(k => {
			const matchName = k.name
				.toLowerCase()
				.includes(search.toLowerCase());
			const matchType =
				typeFilter === 'All' || getKeyType(k.public_key) === typeFilter;
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
					<div
						key={i}
						className="flex items-center gap-4 rounded-lg px-4 py-4">
						<Skeleton className="h-9 w-9 shrink-0 rounded-lg" />
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
					<Search className="pointer-events-none absolute top-1/2 left-2.5 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
					<Input
						placeholder="Search keys…"
						value={search}
						onChange={e => setSearch(e.target.value)}
						className="h-8 pl-8 text-xs"
					/>
					{search && (
						<button
							onClick={() => setSearch('')}
							className="absolute top-1/2 right-2 -translate-y-1/2 text-muted-foreground hover:text-foreground">
							<X className="h-3 w-3" />
						</button>
					)}
				</div>

				{/* Type filter dropdown */}
				<Select
					value={typeFilter}
					onValueChange={v => setTypeFilter(v as KeyTypeFilter)}>
					<SelectTrigger size="sm" className="h-8 w-36 text-xs">
						<SelectValue placeholder="Key type" />
					</SelectTrigger>
					<SelectContent>
						{KEY_TYPES.map(t => (
							<SelectItem key={t} value={t}>
								{t === 'All' ? 'All Types' : t}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>

			{/* ── Empty state ── */}
			{keysList.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 rounded-lg  py-20 text-center">
					<div className="flex h-12 w-12 items-center justify-center rounded-full bg-muted">
						<FileKey2 className="h-5 w-5 text-muted-foreground" />
					</div>
					<div>
						<p className="text-sm font-medium text-foreground">
							No SSH keys yet
						</p>
						<p className="mt-0.5 text-xs text-muted-foreground">
							Add a key pair to authenticate with remote servers.
						</p>
					</div>
				</div>
			) : filtered.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 rounded-lg  py-14 text-center">
					<Search className="h-5 w-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">
						No keys match your filter
					</p>
					<Button
						variant="ghost"
						size="sm"
						onClick={clearFilters}
						className="h-7 text-xs">
						Clear filters
					</Button>
				</div>
			) : (
				/* ── List ── */
				<div className="divide-y divide-border overflow-hidden rounded-lg ">
					{filtered.map((item: SshKeyResponse, idx: number) => (
						<SshKeyRow
							key={item.id}
							item={item}
							index={idx}
							isCopied={copiedId === item.id}
							onCopy={handleCopy}
							onView={onViewKey}
							onDelete={onDeleteKey}
						/>
					))}
				</div>
			)}

			{/* Result count */}
			{hasFilters && filtered.length > 0 && (
				<p className="px-1 text-xs text-muted-foreground">
					Showing {filtered.length} of {keysList.length} keys
				</p>
			)}
		</div>
	);
}
