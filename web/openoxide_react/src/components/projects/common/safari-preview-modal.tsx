import {useState} from 'react';
import {ExternalLink, Loader2} from 'lucide-react';
import {Safari} from '#/components/ui/safari';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogTitle,
} from '#/components/ui/dialog';

interface SafariPreviewModalProps {
	open: boolean;
	onClose: () => void;
	url: string;
	title?: string;
}

/** Device-mock preview: renders a live URL inside the Aceternity Safari frame. */
export function SafariPreviewModal({
	open,
	onClose,
	url,
	title,
}: SafariPreviewModalProps) {
	const [loading, setLoading] = useState(true);

	return (
		<Dialog open={open} onOpenChange={next => !next && onClose()}>
			<DialogContent className="max-w-6xl border-none !bg-transparent bg-transparent p-0 shadow-none sm:max-w-[92vw] [&>button]:text-white">
				<DialogTitle className="sr-only">
					{title || 'Site preview'}
				</DialogTitle>
				<DialogDescription className="sr-only">
					Live preview of {url} inside a browser device mock.
				</DialogDescription>
				<div className="relative">
					<Safari url={url.replace(/^https?:\/\//, '')}>
						<div className="relative size-full overflow-hidden rounded-b-[11px] bg-white">
							{loading && (
								<div className="absolute inset-0 z-10 flex items-center justify-center bg-zinc-100">
									<Loader2 className="size-6 animate-spin text-zinc-400" />
								</div>
							)}
							{open && (
								<iframe
									src={url}
									title={title || url}
									className="size-full border-0"
									sandbox="allow-scripts allow-same-origin allow-forms allow-popups"
									onLoad={() => setLoading(false)}
								/>
							)}
						</div>
					</Safari>
					<div className="mt-3 flex justify-center gap-2">
						<Button
							variant="outline"
							size="sm"
							onClick={() => window.open(url, '_blank', 'noopener')}
							className="gap-1.5 text-xs">
							<ExternalLink className="size-3.5" /> Open in new tab
						</Button>
						<Button
							variant="outline"
							size="sm"
							onClick={onClose}
							className="text-xs">
							Close
						</Button>
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
