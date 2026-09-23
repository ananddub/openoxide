import {
	Key,
	Copy,
	Check,
	Eye,
	Trash2,
	MoreHorizontal,
	ShieldCheck,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import type {SshKeyResponse} from '#/types/api-helpers';
import {
	getKeyType,
	getFingerprint,
	KEY_COLORS,
} from '#/components/ssh-keys/ssh-keys-shared';

interface SshKeyCardProps {
	item: SshKeyResponse;
	index: number;
	isCopied: boolean;
	onCopy: (id: number, pubKey: string, name: string) => void;
	onView: (key: SshKeyResponse) => void;
	onDelete: (key: SshKeyResponse) => void;
}

/**
 * Vercel-style key card — the grid counterpart of SshKeyRow: flat hairline
 * card, icon tile + name up top, fingerprint meta line, actions kebab.
 */
export function SshKeyCard({
	item,
	index,
	isCopied,
	onCopy,
	onView,
	onDelete,
}: SshKeyCardProps) {
	const iconCls = KEY_COLORS[index % KEY_COLORS.length];
	const keyType = getKeyType(item.public_key);
	const fingerprint = getFingerprint(item.public_key);

	return (
		<div className="group flex min-h-[124px] flex-col justify-between gap-3.5 rounded-lg border bg-card p-4 transition-colors hover:bg-accent/60">
			{/* Identity row */}
			<div className="flex items-start justify-between gap-2.5">
				<div className="flex min-w-0 items-center gap-3">
					<div
						className={`flex size-8 shrink-0 items-center justify-center rounded-md border ${iconCls}`}>
						<Key className="size-4" />
					</div>
					<div className="flex min-w-0 flex-col">
						<h3 className="truncate text-sm font-semibold text-foreground">
							{item.name}
						</h3>
						<span className="mt-0.5 truncate text-xs text-muted-foreground">
							{item.public_key ? fingerprint : 'Private key only'}
						</span>
					</div>
				</div>

				<DropdownMenu>
					<DropdownMenuTrigger
						render={
							<Button
								variant="ghost"
								size="icon"
								className="size-7 rounded-md text-muted-foreground hover:bg-muted/80 hover:text-foreground focus-visible:ring-0"
								aria-label={`SSH key ${item.name} actions`}>
								<MoreHorizontal className="size-4" />
							</Button>
						}
					/>
					<DropdownMenuContent align="end" className="w-44">
						<DropdownMenuItem
							className="cursor-pointer gap-2"
							onClick={() => onView(item)}>
							<Eye className="h-3.5 w-3.5" /> View Details
						</DropdownMenuItem>
						<DropdownMenuItem
							className="cursor-pointer gap-2"
							onClick={() => onCopy(item.id, item.public_key, item.name)}>
							{isCopied ? (
								<Check className="h-3.5 w-3.5 text-emerald-500" />
							) : (
								<Copy className="h-3.5 w-3.5" />
							)}
							{isCopied ? 'Copied!' : 'Copy Public Key'}
						</DropdownMenuItem>
						<DropdownMenuSeparator />
						<DropdownMenuItem
							className="cursor-pointer gap-2 text-destructive focus:text-destructive"
							onClick={() => onDelete(item)}>
							<Trash2 className="h-3.5 w-3.5" /> Delete Key
						</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			</div>

			{/* Type row */}
			<div className="flex items-center gap-1.5">
				<Badge
					variant="secondary"
					className="gap-1 py-0 font-mono text-[10px]">
					<ShieldCheck className="h-2.5 w-2.5" />
					{keyType}
				</Badge>
			</div>
		</div>
	);
}
