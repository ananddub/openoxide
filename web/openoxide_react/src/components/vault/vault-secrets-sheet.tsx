import {useState} from 'react';
import {
	AlertTriangle,
	Check,
	Copy,
	KeyRound,
	RefreshCw,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Skeleton} from '#/components/ui/skeleton';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useVaultSecrets} from '#/api/live';
import {
	getVaultProviderLabel,
	vaultReferenceSyntax,
} from './vault-provider-catalog';
import type {VaultProviderItem} from './vault-provider-catalog';

/**
 * Secret browser for one vault provider — the backend only ever returns
 * secret *names*, so this sheet is read-only by design: it shows what can be
 * referenced, and copies the `${{vault.<name>.<secret>}}` snippet.
 */
export function VaultSecretsSheet({
	provider,
	onClose,
}: {
	provider: VaultProviderItem | null;
	onClose: () => void;
}) {
	return (
		<Sheet open={!!provider} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				{provider && (
					<VaultSecretsBody provider={provider} onClose={onClose} />
				)}
			</SheetContent>
		</Sheet>
	);
}

function VaultSecretsBody({
	provider,
	onClose,
}: {
	provider: VaultProviderItem;
	onClose: () => void;
}) {
	const secretsQuery = useVaultSecrets(provider.id);
	const secrets = secretsQuery.data ?? [];
	const [copied, setCopied] = useState<string | null>(null);

	const copy = (text: string, key: string) => {
		navigator.clipboard.writeText(text);
		setCopied(key);
		toast.success('Copied to clipboard');
		setTimeout(() => setCopied(null), 2000);
	};

	const reference = vaultReferenceSyntax(provider.name);

	return (
		<>
			<SheetHeader>
				<SheetTitle className="flex items-center gap-2 pr-8">
					<KeyRound className="size-4 text-primary" />
					<span className="truncate">{provider.name}</span>
				</SheetTitle>
				<SheetDescription>
					{getVaultProviderLabel(provider.provider_type)} — secret names
					only. Values are resolved by the backend when a deployment is
					built.
				</SheetDescription>
			</SheetHeader>

			<div className="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-6 py-5">
				<button
					type="button"
					onClick={() => copy(reference, '*')}
					className="flex w-full items-center justify-between gap-3 rounded-lg border bg-muted/20 px-3 py-2.5 text-start transition-colors hover:bg-muted/40">
					<span className="min-w-0">
						<span className="block text-[11px] font-medium tracking-wide text-muted-foreground uppercase">
							Reference syntax
						</span>
						<code className="block truncate font-mono text-[12px] text-primary">
							{reference}
						</code>
					</span>
					{copied === '*' ? (
						<Check className="size-4 shrink-0 text-emerald-500" />
					) : (
						<Copy className="size-4 shrink-0 text-muted-foreground" />
					)}
				</button>

				{secretsQuery.isPending ? (
					<div className="flex flex-col gap-2">
						{[1, 2, 3, 4].map(i => (
							<Skeleton key={i} className="h-10 w-full rounded-lg" />
						))}
					</div>
				) : secretsQuery.isError ? (
					<div className="flex flex-col items-center gap-3 rounded-xl border bg-card p-8 text-center">
						<AlertTriangle className="size-8 text-rose-500/70" />
						<p className="text-sm text-muted-foreground">
							{formatApiError(secretsQuery.error)}
						</p>
						<Button
							variant="outline"
							size="sm"
							onClick={() => void secretsQuery.refetch()}
							className="h-9 gap-1.5 text-xs font-semibold">
							<RefreshCw className="size-3.5" /> Try again
						</Button>
					</div>
				) : secrets.length === 0 ? (
					<div className="flex flex-col items-center gap-2 rounded-xl border bg-card p-8 text-center">
						<KeyRound className="size-8 text-muted-foreground/40" />
						<h3 className="text-sm font-semibold text-foreground">
							No secrets returned
						</h3>
						<p className="mx-auto max-w-xs text-xs leading-relaxed text-muted-foreground/70">
							This provider is reachable but listed nothing for the
							configured path, mount or project scope.
						</p>
					</div>
				) : (
					<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
						{secrets.map(name => (
							<div
								key={name}
								className="group flex items-center gap-3 px-3.5 py-2.5 transition-colors hover:bg-accent/30">
								<span className="min-w-0 flex-1 truncate font-mono text-xs text-foreground">
									{name}
								</span>
								<Button
									variant="ghost"
									size="icon"
									title="Copy reference for this secret"
									onClick={() =>
										copy(vaultReferenceSyntax(provider.name, name), name)
									}
									className="size-7 shrink-0 rounded-lg text-muted-foreground opacity-0 transition-opacity group-hover:opacity-100 hover:text-foreground">
									{copied === name ? (
										<Check className="size-3.5 text-emerald-500" />
									) : (
										<Copy className="size-3.5" />
									)}
								</Button>
							</div>
						))}
					</div>
				)}
			</div>

			<SheetFooter>
				<Button type="button" variant="outline" onClick={onClose}>
					Close
				</Button>
			</SheetFooter>
		</>
	);
}
