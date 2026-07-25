import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Card, CardContent} from '#/components/ui/card';
import {Key, Copy, Check, Eye, Trash2, ShieldCheck, Clock, FileKey2} from 'lucide-react';
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
		navigator.clipboard.writeText(pubKey);
		setCopiedId(id);
		toast.success('Public SSH Key copied to clipboard');
		setTimeout(() => setCopiedId(null), 2000);
	};

	if (isLoading) {
		return (
			<div className="grid grid-cols-1 md:grid-cols-2 gap-4 py-6">
				{[1, 2].map(i => (
					<div key={i} className="h-36 rounded-xl bg-card/60 animate-pulse border border-border/40" />
				))}
			</div>
		);
	}

	if (!keys || keys.length === 0) {
		return (
			<div className="flex flex-col items-center justify-center p-12 border border-dashed border-border/60 rounded-2xl bg-card/40 my-6 text-center">
				<div className="p-4 rounded-full bg-primary/10 mb-4 text-primary">
					<FileKey2 className="w-8 h-8" />
				</div>
				<h3 className="text-base font-bold text-foreground">No SSH Keys Found</h3>
				<p className="text-xs text-muted-foreground max-w-sm mt-1 mb-4">
					You have not added any SSH key pairs yet. Add an existing key or generate a new ED25519/RSA key pair to get started.
				</p>
			</div>
		);
	}

	return (
		<div className="grid grid-cols-1 md:grid-cols-2 gap-4 py-4">
			{keys.map((item: any) => {
				const isCopied = copiedId === item.id;
				const createdDate = item.created_at ? new Date(item.created_at * 1000).toLocaleDateString() : 'N/A';
				const lastUsed = item.last_used_at ? new Date(item.last_used_at * 1000).toLocaleDateString() : 'Never';

				return (
					<Card key={item.id} className="bg-card border-border/70 hover:border-primary/40 transition-colors shadow-sm">
						<CardContent className="p-5 flex flex-col justify-between gap-4 h-full">
							<div className="flex items-start justify-between gap-3">
								<div className="flex items-start gap-3">
									<div className="p-2.5 rounded-xl bg-muted/40 border border-border/50 text-primary mt-0.5">
										<Key className="w-5 h-5" />
									</div>
									<div className="flex flex-col">
										<div className="flex items-center gap-2">
											<span className="text-sm font-bold text-foreground">{item.name}</span>
											{item.has_private_key && (
												<Badge variant="outline" className="text-[10px] bg-primary/5 text-primary border-primary/20 px-1.5 py-0">
													<ShieldCheck className="w-3 h-3 mr-1" /> Key Pair
												</Badge>
											)}
										</div>
										<p className="text-xs text-muted-foreground line-clamp-1 mt-0.5">
											{item.description || 'No description provided'}
										</p>
									</div>
								</div>

								<Button
									variant="ghost"
									size="icon"
									onClick={() => onDeleteKey(item)}
									className="h-8 w-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10"
								>
									<Trash2 className="w-4 h-4" />
								</Button>
							</div>

							<div className="bg-muted/30 p-2.5 rounded-lg border border-border/40 text-[11px] font-mono text-muted-foreground truncate overflow-hidden max-w-full">
								{item.public_key || 'No public key attached'}
							</div>

							<div className="flex items-center justify-between pt-2 border-t border-border/40 text-xs">
								<div className="flex items-center gap-3 text-[11px] text-muted-foreground">
									<span className="flex items-center gap-1">
										<Clock className="w-3 h-3" /> Used: {lastUsed}
									</span>
									<span>•</span>
									<span>Added: {createdDate}</span>
								</div>

								<div className="flex items-center gap-2">
									<Button
										variant="outline"
										size="sm"
										onClick={() => handleCopyPublicKey(item.id, item.public_key)}
										className="h-8 text-xs font-medium gap-1 px-2.5"
									>
										{isCopied ? <Check className="w-3.5 h-3.5 text-primary" /> : <Copy className="w-3.5 h-3.5" />}
										{isCopied ? 'Copied' : 'Copy Public'}
									</Button>
									<Button
										variant="secondary"
										size="sm"
										onClick={() => onViewKey(item)}
										className="h-8 text-xs font-semibold gap-1 px-2.5"
									>
										<Eye className="w-3.5 h-3.5" />
										View Key
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
