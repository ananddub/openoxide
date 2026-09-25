import {useCallback, useEffect, useState} from 'react';
import {ExternalLink, Globe, Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogTitle,
} from '#/components/ui/dialog';

interface DomainPreviewModalProps {
	open: boolean;
	onClose: () => void;
	url: string;
	title?: string;
}

/** The site is always rendered at a desktop viewport and scaled to fit. */
const FRAME_W = 1280;
const FRAME_H = 800;

/** Live site preview — the dialog owns a definite height and the frame is scaled into whatever space is left, so it can never overflow its own shell. */
export function DomainPreviewModal({
	open,
	onClose,
	url,
	title,
}: DomainPreviewModalProps) {
	const [loading, setLoading] = useState(true);
	const [scale, setScale] = useState(0);

	useEffect(() => {
		if (open) setLoading(true);
	}, [open, url]);

	// The frame box only exists once the dialog mounts, so measurement rides a ref callback with its own cleanup. A mount-time effect saw a zero-size box, fell back to the unscaled 1280px frame, and the dialog clipped it in half.
	const boxRef = useCallback((box: HTMLDivElement | null) => {
		if (!box) {
			setScale(0);
			return;
		}
		const measure = () => {
			const {clientWidth: width, clientHeight: height} = box;
			if (width === 0 || height === 0) return;
			setScale(Math.min(width / FRAME_W, height / FRAME_H));
		};
		measure();
		const observer = new ResizeObserver(measure);
		observer.observe(box);
		return () => observer.disconnect();
	}, []);

	const ready = scale > 0;

	return (
		<Dialog open={open} onOpenChange={next => !next && onClose()}>
			<DialogContent className="flex h-[min(90dvh,880px)] w-[min(1200px,94vw)] max-w-none flex-col gap-0 overflow-hidden p-0 sm:max-w-none">
				<DialogTitle className="sr-only">
					{title || 'Site preview'}
				</DialogTitle>
				<DialogDescription className="sr-only">
					Live preview of {url} in an embedded frame.
				</DialogDescription>

				{/* Address bar — the dialog's own close button sits over the right padding. */}
				<div className="flex shrink-0 items-center gap-3 border-b border-border/70 bg-muted/30 py-2.5 pr-14 pl-4">
					<Globe
						className="size-4 shrink-0 text-muted-foreground"
						strokeWidth={1.7}
					/>
					<span className="min-w-0 flex-1 truncate font-mono text-[13px] text-foreground">
						{url}
					</span>
					<Button
						variant="outline"
						size="sm"
						onClick={() => window.open(url, '_blank', 'noopener')}
						className="shrink-0 cursor-pointer gap-1.5 text-xs">
						<ExternalLink className="size-3.5" />
						Open
					</Button>
				</div>

				<div
					ref={boxRef}
					className="grid min-h-0 flex-1 place-items-center bg-muted/40 p-4">
					<div
						className="relative overflow-hidden rounded-lg border border-border bg-white shadow-sm"
						style={{
							width: FRAME_W * scale,
							height: FRAME_H * scale,
						}}>
						{!ready || loading ? (
							<div className="absolute inset-0 z-10 grid place-items-center bg-white">
								<Loader2 className="size-5 animate-spin text-zinc-400" />
							</div>
						) : null}
						{open && ready ? (
							<iframe
								src={url}
								title={title || url}
								onLoad={() => setLoading(false)}
								className="absolute top-0 left-0 border-0"
								style={{
									width: FRAME_W,
									height: FRAME_H,
									colorScheme: 'light',
									transform: `scale(${scale})`,
									transformOrigin: 'top left',
								}}
								sandbox="allow-scripts allow-same-origin allow-forms allow-popups"
							/>
						) : null}
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
