import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Copy, Check, Eye, EyeOff} from 'lucide-react';
import {toast} from 'sonner';

/** Private key reveal / copy / masked display block. */
export function ViewKeyPrivateBlock({privateKey}: {privateKey?: string}) {
	const [showPrivate, setShowPrivate] = useState(false);
	const [copiedPrivate, setCopiedPrivate] = useState(false);

	const handleCopyPriv = () => {
		if (!privateKey) return;
		navigator.clipboard.writeText(privateKey);
		setCopiedPrivate(true);
		toast.success('Private SSH Key copied to clipboard');
		setTimeout(() => setCopiedPrivate(false), 2000);
	};

	return (
		<div className="flex flex-col gap-1.5">
			<div className="flex items-center justify-between">
				<label className="text-xs font-semibold text-foreground">
					Private Key
				</label>
				<div className="flex items-center gap-1.5">
					<Button
						variant="ghost"
						size="sm"
						onClick={() => setShowPrivate(!showPrivate)}
						className="h-7 gap-1.5 px-2 text-xs">
						{showPrivate ? (
							<EyeOff className="h-3.5 w-3.5" />
						) : (
							<Eye className="h-3.5 w-3.5" />
						)}
						{showPrivate ? 'Hide' : 'Reveal'}
					</Button>
					{showPrivate && (
						<Button
							variant="ghost"
							size="sm"
							onClick={handleCopyPriv}
							className="h-7 gap-1.5 px-2 text-xs">
							{copiedPrivate ? (
								<Check className="h-3.5 w-3.5 text-emerald-500" />
							) : (
								<Copy className="h-3.5 w-3.5" />
							)}
							{copiedPrivate ? 'Copied' : 'Copy'}
						</Button>
					)}
				</div>
			</div>

			{showPrivate ? (
				<div className="max-h-36 overflow-y-auto rounded-md bg-zinc-950/90 p-3 font-mono text-xs leading-relaxed break-all whitespace-pre-wrap text-emerald-400">
					{privateKey || 'No private key stored for this SSH key.'}
				</div>
			) : (
				<div className="flex h-10 items-center rounded-md bg-zinc-950/60 px-3 font-mono text-xs text-zinc-500 italic">
					•••••••••••••••••••••••• (Hidden for security)
				</div>
			)}
		</div>
	);
}
