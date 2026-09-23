import {useEffect, useRef, useState} from 'react';
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

/** Live site preview: desktop frame fitted into a house dialog. */
export function DomainPreviewModal({
	open,
	onClose,
	url,
	title,
}: DomainPreviewModalProps) {
	const [loading, setLoading] = useState(true);
	const [scale, setScale] = useState(0.8);
	const boxRef = useRef<HTMLDivElement>(null);

	useEffect(() => {
		if (open) setLoading(true);
	}, [open, url]);

	// Keep the frame inside the dialog width and ~72% of the viewport height.
	useEffect(() => {
		const measure = () => {
			const width = boxRef.current?.clientWidth ?? FRAME_W;
			const height = window.innerHeight * 0.72;
			setScale(Math.min(width / FRAME_W, height / FRAME_H));
		};
		measure();
		const observer = new ResizeObserver(measure);
		if (boxRef.current) observer.observe(boxRef.current);
		window.addEventListener('resize', measure);
		return () => {
			observer.disconnect();
			window.removeEventListener('resize', measure);
		};
	}, []);

	return (
		<Dialog open={open} onOpenChange={next => !next && onClose()}>
			<DialogContent className="max-w-[min(1100px,92vw)] gap-0 overflow-hidden p-0">
				<DialogTitle className="sr-only">
					{title || 'Site preview'}
				</DialogTitle>
				<DialogDescription className="sr-only">
					Live preview of {url} in an embedded frame.
				</DialogDescription>

				{/* Address bar — carries the Close X on the right. */}
				<div className="flex items-center gap-3 border-b py-3 pr-14 pl-4">
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
					className="flex w-full justify-center bg-muted/40 py-4">
					<div
						className="relative overflow-hidden rounded-md border bg-white shadow-sm"
						style={{
							width: FRAME_W * scale,
							height: FRAME_H * scale,
						}}>
						{loading && (
							<div className="absolute inset-0 z-10 flex items-center justify-center bg-white">
								<Loader2 className="size-5 animate-spin text-zinc-400" />
							</div>
						)}
						{open && (
							<iframe
								src={url}
								title={title || url}
								className="absolute top-0 left-0 border-0"
								style={{
									width: FRAME_W,
									height: FRAME_H,
									colorScheme: 'light',
									transform: `scale(${scale})`,
									transformOrigin: 'top left',
								}}
								sandbox="allow-scripts allow-same-origin allow-forms allow-popups"
								onLoad={() => setLoading(false)}
							/>
						)}
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
