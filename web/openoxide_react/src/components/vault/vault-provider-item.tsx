import {
	Check,
	Copy,
	KeyRound,
	MoreHorizontal,
	Pencil,
	Plug,
	RefreshCw,
	Trash2,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {ListRow, RowTile} from '#/components/settings/list-card';
import {
	getVaultProviderLabel,
	vaultReferenceSyntax,
	vaultScopeLabel,
	VaultProviderIcon,
	type VaultProviderItem,
} from './vault-provider-catalog';

function ScopeChip({provider}: {provider: VaultProviderItem}) {
	return (
		<span className="rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
			{vaultScopeLabel(provider)}
		</span>
	);
}

function TypeChip({type}: {type: string}) {
	return (
		<span className="rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium tracking-wide text-muted-foreground uppercase">
			{getVaultProviderLabel(type)}
		</span>
	);
}

export interface VaultProviderItemProps {
	provider: VaultProviderItem;
	copied: boolean;
	testing: boolean;
	onSecrets: () => void;
	onTest: () => void;
	onCopy: () => void;
	onEdit: () => void;
	onDelete: () => void;
}

/** List row — tile, name + scope, mono endpoint, quiet hover actions. */
export function VaultProviderRow({
	provider,
	copied,
	testing,
	onSecrets,
	onTest,
	onCopy,
	onEdit,
	onDelete,
}: VaultProviderItemProps) {
	const iconButton =
		'size-7 rounded-lg text-muted-foreground hover:text-foreground';
	return (
		<ListRow
			icon={
				<RowTile>
					<VaultProviderIcon
						type={provider.provider_type}
						className="size-5"
					/>
				</RowTile>
			}
			title={provider.name}
			badges={
				<>
					<TypeChip type={provider.provider_type} />
					<ScopeChip provider={provider} />
				</>
			}
			subtitle={
				<span className="font-mono text-[11px]">
					{provider.api_url || 'Provider default endpoint'}
				</span>
			}>
			<Button
				variant="ghost"
				size="icon"
				title="Browse secrets"
				onClick={onSecrets}
				className={iconButton}>
				<KeyRound className="size-3.5" />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				title="Test connection"
				disabled={testing}
				onClick={onTest}
				className={iconButton}>
				{testing ? (
					<RefreshCw className="size-3.5 animate-spin" />
				) : (
					<Plug className="size-3.5" />
				)}
			</Button>
			<Button
				variant="ghost"
				size="icon"
				title="Copy reference syntax"
				onClick={onCopy}
				className={iconButton}>
				{copied ? (
					<Check className="size-3.5 text-emerald-500" />
				) : (
					<Copy className="size-3.5" />
				)}
			</Button>
			<Button
				variant="ghost"
				size="icon"
				title="Edit provider"
				onClick={onEdit}
				className={iconButton}>
				<Pencil className="size-3.5" />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				title="Delete provider"
				onClick={onDelete}
				className="size-7 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
				<Trash2 className="size-3.5" />
			</Button>
		</ListRow>
	);
}

/** Grid card — the row's counterpart: hairline card, provider meta, kebab. */
export function VaultProviderCard({
	provider,
	copied,
	testing,
	onSecrets,
	onTest,
	onCopy,
	onEdit,
	onDelete,
}: VaultProviderItemProps) {
	return (
		<div className="group flex min-h-[124px] flex-col justify-between gap-3.5 rounded-lg border bg-card p-4 transition-colors hover:bg-accent/60">
			<div className="flex items-start justify-between gap-2.5">
				<div className="flex min-w-0 items-center gap-3">
					<span className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05]">
						<VaultProviderIcon
							type={provider.provider_type}
							className="size-5"
						/>
					</span>
					<div className="flex min-w-0 flex-col">
						<h3 className="truncate text-sm font-semibold text-foreground">
							{provider.name}
						</h3>
						<span className="mt-0.5 truncate font-mono text-xs text-muted-foreground">
							{provider.api_url || 'Provider default endpoint'}
						</span>
					</div>
				</div>

				<DropdownMenu>
					<DropdownMenuTrigger
						render={
							<Button
								variant="ghost"
								size="icon"
								aria-label={`Vault provider ${provider.name} actions`}
								className="size-7 rounded-md text-muted-foreground hover:bg-muted/80 hover:text-foreground focus-visible:ring-0"
							/>
						}>
						<MoreHorizontal className="size-4" />
					</DropdownMenuTrigger>
					<DropdownMenuContent align="end" className="w-52">
						<DropdownMenuItem
							className="cursor-pointer gap-2"
							onClick={onSecrets}>
							<KeyRound className="size-3.5" /> Browse secrets
						</DropdownMenuItem>
						<DropdownMenuItem
							className="cursor-pointer gap-2"
							disabled={testing}
							onClick={onTest}>
							{testing ? (
								<RefreshCw className="size-3.5 animate-spin" />
							) : (
								<Plug className="size-3.5" />
							)}
							Test connection
						</DropdownMenuItem>
						<DropdownMenuItem
							className="cursor-pointer gap-2"
							onClick={onCopy}>
							{copied ? (
								<Check className="size-3.5 text-emerald-500" />
							) : (
								<Copy className="size-3.5" />
							)}
							{copied ? 'Copied!' : 'Copy reference syntax'}
						</DropdownMenuItem>
						<DropdownMenuSeparator />
						<DropdownMenuItem
							className="cursor-pointer gap-2"
							onClick={onEdit}>
							<Pencil className="size-3.5" /> Edit provider
						</DropdownMenuItem>
						<DropdownMenuItem
							className="cursor-pointer gap-2 text-destructive focus:text-destructive"
							onClick={onDelete}>
							<Trash2 className="size-3.5" />
							Delete provider
						</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			</div>

			<div className="flex flex-col gap-2">
				<code className="truncate rounded-md bg-foreground/[0.04] px-2 py-1 font-mono text-[11px] text-primary">
					{vaultReferenceSyntax(provider.name)}
				</code>
				<div className="flex flex-wrap items-center gap-1.5">
					<TypeChip type={provider.provider_type} />
					<ScopeChip provider={provider} />
				</div>
			</div>
		</div>
	);
}
