import {
	Key,
	Copy,
	Check,
	Eye,
	Trash2,
	MoreVertical,
	ShieldCheck,
} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Separator} from '#/components/ui/separator';
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

interface SshKeyRowProps {
	item: SshKeyResponse;
	index: number;
	isCopied: boolean;
	onCopy: (id: number, pubKey: string, name: string) => void;
	onView: (key: SshKeyResponse) => void;
	onDelete: (key: SshKeyResponse) => void;
}

/** A single SSH key row with copy / view actions and a dropdown menu. */
export function SshKeyRow({
	item,
	index,
	isCopied,
	onCopy,
	onView,
	onDelete,
}: SshKeyRowProps) {
	const iconCls = KEY_COLORS[index % KEY_COLORS.length];
	const keyType = getKeyType(item.public_key);
	const fingerprint = getFingerprint(item.public_key);

	return (
		<div className="group flex items-center gap-4 bg-card px-4 py-3.5 transition-colors hover:bg-accent/30">
			<div
				className={`flex h-9 w-9 shrink-0 items-center justify-center rounded-lg  ${iconCls}`}>
				<Key className="h-4 w-4" />
			</div>

			<div className="min-w-0 flex-1">
				<div className="flex items-center gap-2">
					<span className="truncate text-sm font-medium text-foreground">
						{item.name}
					</span>
					<Badge
						variant="secondary"
						className="shrink-0 gap-1 py-0 font-mono text-[10px]">
						<ShieldCheck className="h-2.5 w-2.5" />
						{keyType}
					</Badge>
				</div>
				<p className="mt-0.5 truncate font-mono text-[11px] text-muted-foreground">
					{item.public_key ? fingerprint : 'Private key only'}
				</p>
			</div>

			<div className="flex items-center gap-3 opacity-0 transition-opacity group-hover:opacity-100">
				<button
					onClick={() => onCopy(item.id, item.public_key, item.name)}
					title="Copy public key"
					className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground">
					{isCopied ? (
						<Check className="h-4 w-4 text-emerald-500" />
					) : (
						<Copy className="h-4 w-4" />
					)}
				</button>
				<button
					onClick={() => onView(item)}
					title="View details"
					className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground">
					<Eye className="h-4 w-4" />
				</button>
			</div>

			<Separator
				orientation="vertical"
				className="h-5 opacity-0 transition-opacity group-hover:opacity-100"
			/>

			<DropdownMenu>
				<DropdownMenuTrigger
					render={
						<button className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground" />
					}>
					<MoreVertical className="h-4 w-4" />
				</DropdownMenuTrigger>
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
	);
}
