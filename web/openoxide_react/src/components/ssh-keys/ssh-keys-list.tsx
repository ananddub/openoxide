import {useState} from 'react';
import {Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Skeleton} from '#/components/ui/skeleton';
import {toast} from 'sonner';
import type {SshKeyResponse} from '#/types/api-helpers';
import {SshKeyRow} from '#/components/ssh-keys/ssh-key-row';
import {SshKeyCard} from '#/components/ssh-keys/ssh-key-card';

export type SshKeysView = 'grid' | 'list';

interface SshKeysListProps {
	/** Already filtered by the page toolbar. */
	keys: SshKeyResponse[];
	isLoading: boolean;
	hasFilters: boolean;
	view: SshKeysView;
	onClearFilters: () => void;
	onViewKey: (key: SshKeyResponse) => void;
	onDeleteKey: (key: SshKeyResponse) => void;
}

/** Keys surface — Projects grammar: grid cards or one bordered list card. */
export function SshKeysList({
	keys,
	isLoading,
	hasFilters,
	view,
	onClearFilters,
	onViewKey,
	onDeleteKey,
}: SshKeysListProps) {
	const [copiedId, setCopiedId] = useState<number | null>(null);

	const handleCopy = (id: number, pubKey: string, name: string) => {
		if (!pubKey) return void toast.error('No public key to copy');
		navigator.clipboard.writeText(pubKey);
		setCopiedId(id);
		toast.success(`"${name}" copied!`);
		setTimeout(() => setCopiedId(null), 2000);
	};

	if (isLoading) {
		if (view === 'grid') {
			return (
				<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
					{[1, 2, 3, 4].map(i => (
						<div
							key={i}
							className="min-h-[124px] animate-pulse rounded-lg border bg-card"
						/>
					))}
				</div>
			);
		}
		return (
			<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
				{[1, 2, 3].map(i => (
					<div key={i} className="flex items-center gap-3.5 px-4 py-3">
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

	if (keys.length === 0) {
		return (
			<div className="flex flex-col items-center justify-center gap-3 rounded-xl border bg-card p-12 text-center">
				<Search className="size-10 text-muted-foreground/40" />
				<h3 className="text-xl font-semibold tracking-[-0.01em] text-foreground">
					No matching keys found
				</h3>
				<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
					Try adjusting your search keywords or clearing the type filter.
				</p>
				{hasFilters && (
					<Button
						variant="ghost"
						onClick={onClearFilters}
						className="mt-4 text-xs font-semibold text-primary">
						Clear All Filters
					</Button>
				)}
			</div>
		);
	}

	if (view === 'grid') {
		return (
			<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
				{keys.map((item, idx) => (
					<SshKeyCard
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
		);
	}

	return (
		<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
			{keys.map((item, idx) => (
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
	);
}
