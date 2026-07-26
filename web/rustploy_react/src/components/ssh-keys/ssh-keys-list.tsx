import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Card, CardContent} from '#/components/ui/card';
import {Key, Copy, Check, Eye, Trash2, FileKey2} from 'lucide-react';
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

	const handleCopyPublicKey = (id: number, pubKey: string) => {
		if (!pubKey) {
			toast.error('No public key available to copy');
			return;
		}
		navigator.clipboard.writeText(pubKey);
		setCopiedId(id);
		toast.success('Public SSH Key copied to clipboard');
		setTimeout(() => setCopiedId(null), 2000);
	};

	if (isLoading) {
		return (
			<div className="grid grid-cols-1 md:grid-cols-2 gap-4 py-4">
				{[1, 2].map(i => (
					<div key={i} className="h-36 rounded-xl bg-muted/40 animate-pulse border border-border/60" />
				))}
			</div>
		);
	}

	if (!keys || keys.length === 0) {
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
		<div className="grid grid-cols-1 md:grid-cols-2 gap-4 py-4 w-full">
			{keys.map((item: any) => {
				const isCopied = copiedId === item.id;
				const createdDate = item.created_at ? new Date(item.created_at * 1000).toLocaleDateString() : 'N/A';

				return (
					<Card key={item.id} className="bg-card border-border hover:border-border/80 transition-all rounded-xl shadow-sm">
						<CardContent className="p-4 flex flex-col gap-3">
							{/* Header: Key Icon, Name, Description & Delete */}
							<div className="flex items-start justify-between gap-3 min-w-0">
								<div className="flex items-center gap-2.5 min-w-0 flex-1">
									<Key className="w-4 h-4 text-primary shrink-0" />
									<div className="min-w-0 flex-1">
										<h3 className="text-sm font-bold text-foreground truncate">{item.name}</h3>
										{item.description && (
											<p className="text-xs text-muted-foreground truncate mt-0.5 font-normal">
												{item.description}
											</p>
										)}
									</div>
								</div>

								<Button
									variant="ghost"
									size="icon"
									onClick={() => onDeleteKey(item)}
									title="Delete key"
									className="h-7 w-7 text-muted-foreground hover:text-destructive shrink-0"
								>
									<Trash2 className="w-3.5 h-3.5" />
								</Button>
							</div>

							{/* Public Key Single-Line Preview */}
							<div className="bg-muted/40 px-3 py-2 rounded-lg border border-border/50 text-xs font-mono text-muted-foreground truncate">
								{item.public_key || 'No public key preview'}
							</div>

							{/* Bottom Action Bar */}
							<div className="flex items-center justify-between pt-2 border-t border-border/50 text-xs gap-2">
								<span className="text-[11px] text-muted-foreground truncate">
									Added: {createdDate}
								</span>

								<div className="flex items-center gap-1.5 shrink-0">
									<Button
										variant="outline"
										size="sm"
										onClick={() => handleCopyPublicKey(item.id, item.public_key)}
										className="h-8 text-xs font-medium gap-1.5 px-3"
									>
										{isCopied ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
										{isCopied ? 'Copied' : 'Copy Key'}
									</Button>
									<Button
										variant="secondary"
										size="sm"
										onClick={() => onViewKey(item)}
										className="h-8 text-xs font-medium gap-1.5 px-3"
									>
										<Eye className="w-3 h-3" />
										View Details
									</Button>
								</div>
							</div>
						</CardContent>
					</Card>
				);
			})}
		</div>
	);
}
