import {Globe, MoreHorizontal, Pencil, Search, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {ListEmpty} from '#/components/settings/list-card';
import type {SshKeysView} from '#/components/ssh-keys/ssh-keys-list';
import {
	DnsProviderIcon,
	getDnsProviderLabel,
	type DnsProviderItem,
} from './dns-provider-catalog';

interface DnsProvidersListProps {
	/** Already filtered by the page toolbar. */
	providers: DnsProviderItem[];
	isLoading: boolean;
	hasFilters: boolean;
	view: SshKeysView;
	onClearFilters: () => void;
	onDomains: (item: DnsProviderItem) => void;
	onEdit: (item: DnsProviderItem) => void;
	onDelete: (item: DnsProviderItem) => void;
}

function ProviderTile({provider}: {provider: DnsProviderItem}) {
	return (
		<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05] text-foreground">
			<DnsProviderIcon type={provider.provider_type} className="size-5" />
		</div>
	);
}

function TypeChip({type}: {type: string}) {
	return (
		<span className="shrink-0 rounded bg-muted/50 px-1.5 py-0.5 font-mono text-[10px] font-medium text-muted-foreground">
			{getDnsProviderLabel(type)}
		</span>
	);
}

function WildcardChip() {
	return (
		<span className="shrink-0 rounded bg-emerald-500/10 px-1.5 py-0.5 font-mono text-[10px] font-medium text-emerald-500">
			Wildcard Ready
		</span>
	);
}

function providerLine(provider: DnsProviderItem) {
	return `Zones and records managed from View Domains · Added ${new Date(
		Number(provider.created_at) * 1000,
	).toLocaleDateString()}`;
}

function DomainsButton({onDomains}: {onDomains: () => void}) {
	return (
		<Button
			variant="ghost"
			size="icon"
			title="View Domains"
			onClick={onDomains}
			className="size-7 shrink-0 rounded-md text-muted-foreground hover:text-foreground">
			<Globe className="size-3.5" />
		</Button>
	);
}

function KebabMenu({
	onDomains,
	onEdit,
	onDelete,
}: {
	onDomains: () => void;
	onEdit: () => void;
	onDelete: () => void;
}) {
	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<Button
						variant="ghost"
						size="icon"
						className="size-7 shrink-0 rounded-md text-muted-foreground hover:bg-muted/80 hover:text-foreground focus-visible:ring-0"
						aria-label="DNS provider actions">
						<MoreHorizontal className="size-4" />
					</Button>
				}
			/>
			<DropdownMenuContent align="end" className="w-44">
				<DropdownMenuItem
					className="cursor-pointer gap-2"
					onClick={onDomains}>
					<Globe className="size-3.5" /> View Domains
				</DropdownMenuItem>
				<DropdownMenuItem
					className="cursor-pointer gap-2"
					onClick={onEdit}>
					<Pencil className="size-3.5" /> Edit Provider
				</DropdownMenuItem>
				<DropdownMenuSeparator />
				<DropdownMenuItem
					className="cursor-pointer gap-2 text-destructive focus:text-destructive"
					onClick={onDelete}>
					<Trash2 className="size-3.5" /> Delete Provider
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}

/** DNS providers surface — Projects grammar: grid cards or one bordered list card, matching the SSH Keys page. */
export function DnsProvidersList({
	providers,
	isLoading,
	hasFilters,
	view,
	onClearFilters,
	onDomains,
	onEdit,
	onDelete,
}: DnsProvidersListProps) {
	if (isLoading) {
		if (view === 'grid') {
			return (
				<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
					{[1, 2, 3, 4].map(i => (
						<div
							key={i}
							className="min-h-[88px] animate-pulse rounded-lg border bg-card"
						/>
					))}
				</div>
			);
		}
		return (
			<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
				{[1, 2, 3].map(i => (
					<div key={i} className="flex items-center gap-3.5 px-4 py-3">
						<div className="size-9 shrink-0 animate-pulse rounded-lg bg-muted/60" />
						<div className="flex-1 space-y-2">
							<div className="h-3.5 w-40 animate-pulse rounded bg-muted/60" />
							<div className="h-3 w-64 animate-pulse rounded bg-muted/60" />
						</div>
					</div>
				))}
			</div>
		);
	}

	if (providers.length === 0) {
		return (
			<ListEmpty
				icon={Search}
				title="No matching providers found"
				description="Try adjusting your search keywords or clearing the provider filter."
				action={
					hasFilters && (
						<Button
							variant="ghost"
							onClick={onClearFilters}
							className="text-xs font-semibold text-primary">
							Clear All Filters
						</Button>
					)
				}
			/>
		);
	}

	if (view === 'grid') {
		return (
			<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
				{providers.map(provider => (
					<DnsProviderGridCard
						key={provider.id}
						provider={provider}
						onDomains={onDomains}
						onEdit={onEdit}
						onDelete={onDelete}
					/>
				))}
			</div>
		);
	}

	return (
		<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
			{providers.map(provider => (
				<DnsProviderListRow
					key={provider.id}
					provider={provider}
					onDomains={onDomains}
					onEdit={onEdit}
					onDelete={onDelete}
				/>
			))}
		</div>
	);
}

function DnsProviderGridCard({
	provider,
	onDomains,
	onEdit,
	onDelete,
}: {
	provider: DnsProviderItem;
	onDomains: (item: DnsProviderItem) => void;
	onEdit: (item: DnsProviderItem) => void;
	onDelete: (item: DnsProviderItem) => void;
}) {
	return (
		<div className="group flex min-h-[88px] flex-col justify-between gap-3 rounded-lg border bg-card p-4 transition-colors hover:bg-accent/30">
			<div className="flex min-w-0 items-start gap-3">
				<ProviderTile provider={provider} />
				<div className="min-w-0 flex-1">
					<span className="truncate text-sm font-medium text-foreground">
						{provider.name}
					</span>
					<p className="mt-0.5 truncate text-[11px] text-muted-foreground">
						{providerLine(provider)}
					</p>
				</div>
			</div>
			<div className="flex items-center justify-between gap-2">
				<div className="flex min-w-0 items-center gap-1.5">
					<TypeChip type={provider.provider_type} />
					<WildcardChip />
				</div>
				<div className="flex shrink-0 items-center gap-1">
					<DomainsButton onDomains={() => onDomains(provider)} />
					<KebabMenu
						onDomains={() => onDomains(provider)}
						onEdit={() => onEdit(provider)}
						onDelete={() => onDelete(provider)}
					/>
				</div>
			</div>
		</div>
	);
}

function DnsProviderListRow({
	provider,
	onDomains,
	onEdit,
	onDelete,
}: {
	provider: DnsProviderItem;
	onDomains: (item: DnsProviderItem) => void;
	onEdit: (item: DnsProviderItem) => void;
	onDelete: (item: DnsProviderItem) => void;
}) {
	return (
		<div className="group flex items-center gap-3.5 px-4 py-3 transition-colors hover:bg-accent/30">
			<ProviderTile provider={provider} />
			<div className="min-w-0 flex-1">
				<div className="flex items-center gap-2">
					<span className="truncate text-sm font-medium text-foreground">
						{provider.name}
					</span>
					<TypeChip type={provider.provider_type} />
					<WildcardChip />
				</div>
				<p className="mt-0.5 truncate text-[11px] text-muted-foreground">
					{providerLine(provider)}
				</p>
			</div>
			<div className="flex shrink-0 items-center gap-1">
				<DomainsButton onDomains={() => onDomains(provider)} />
				<KebabMenu
					onDomains={() => onDomains(provider)}
					onEdit={() => onEdit(provider)}
					onDelete={() => onDelete(provider)}
				/>
			</div>
		</div>
	);
}
