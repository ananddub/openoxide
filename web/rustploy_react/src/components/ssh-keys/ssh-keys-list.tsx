import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Card, CardContent} from '#/components/ui/card';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {Key, Copy, Check, FileKey2, MoreVertical} from 'lucide-react';
import {toast} from 'sonner';

interface SshKeysListProps {
	keys: any[];
	isLoading: boolean;
	onViewKey: (key: any) => void;
	onDeleteKey: (key: any) => void;
}

export function SshKeysList({
	keys,
	isLoading,
	onViewKey,
	onDeleteKey,
}: SshKeysListProps) {
	const [copiedId, setCopiedId] = useState<number | null>(null);

	const handleCopyPublicKey = (id: number, pubKey: string, keyName: string) => {
		if (!pubKey) {
			toast.error('No public key available to copy');
			return;
		}
		navigator.clipboard.writeText(pubKey);
		setCopiedId(id);
		toast.success(`SSH Key "${keyName}" copied to clipboard!`);
		setTimeout(() => setCopiedId(null), 2000);
	};

	const keysList = Array.isArray(keys) ? keys : [];

	if (isLoading) {
		return (
			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 py-4">
				{[1, 2, 3, 4].map(i => (
					<div key={i} className="h-16 rounded-xl bg-muted/40 animate-pulse border border-border/60" />
				))}
			</div>
		);
	}

	if (!keysList || keysList.length === 0) {
		return (
			<Card className="bg-card border-border p-12 text-center flex flex-col items-center justify-center rounded-xl my-4">
				<div className="w-12 h-12 rounded-xl bg-primary/10 flex items-center justify-center text-primary mb-3">
					<FileKey2 className="w-6 h-6" />
				</div>
				<h3 className="text-sm font-bold text-foreground">No SSH Keys Found</h3>
				<p className="text-xs text-muted-foreground max-w-sm mt-1 mb-4">
					Add an existing SSH key pair or generate a new key pair to authenticate remote servers.
				</p>
			</Card>
		);
	}

	return (
		<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 py-3 w-full">
			{keysList.map((item: any) => {
				const isCopied = copiedId === item.id;

				return (
					<Card key={item.id} className="bg-card border-border hover:border-border/80 transition-all rounded-xl shadow-sm">
						<CardContent className="p-3 flex items-center justify-between gap-2">
							{/* Left: Key Icon, Key Name & Monospace Preview */}
							<div className="flex items-center gap-2.5 min-w-0 flex-1">
								<Key className="w-3.5 h-3.5 text-primary shrink-0" />
								<div className="flex flex-col min-w-0 flex-1">
									<h3 className="text-xs font-bold text-foreground truncate">{item.name}</h3>
									<span className="text-[11px] font-mono text-muted-foreground truncate mt-0.5">
										{item.public_key ? item.public_key.slice(0, 24) + '...' : 'No public key'}
									</span>
								</div>
							</div>

							{/* Right: Quick Copy Button & 3-Dots Dropdown Menu */}
							<div className="flex items-center gap-1 shrink-0">
								<Button
									variant="ghost"
									size="icon"
									onClick={() => handleCopyPublicKey(item.id, item.public_key, item.name)}
									title="Copy Public Key"
									className="h-7 w-7 text-muted-foreground hover:text-foreground shrink-0"
								>
									{isCopied ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
								</Button>

								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<Button variant="ghost" size="icon" className="h-7 w-7 text-muted-foreground hover:text-foreground shrink-0" />
										}
									>
										<MoreVertical className="w-4 h-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-40 bg-card border-border shadow-xl rounded-xl p-1 text-xs z-50">
										<DropdownMenuItem
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => onViewKey(item)}
										>
											View Details
										</DropdownMenuItem>

										<DropdownMenuItem
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => handleCopyPublicKey(item.id, item.public_key, item.name)}
										>
											Copy Public Key
										</DropdownMenuItem>

										<DropdownMenuSeparator className="my-1 border-border/50" />

										<DropdownMenuItem
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted/80 text-rose-500 text-xs font-medium"
											onClick={() => onDeleteKey(item)}
										>
											Delete Key
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</div>
						</CardContent>
					</Card>
				);
			})}
		</div>
	);
}
