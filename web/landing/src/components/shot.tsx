type ShotProps = {
	url: string;
	light: string;
	dark: string;
	alt: string;
	priority?: boolean;
	loading?: 'lazy' | 'eager';
};

/**
 * Chrome around a real capture. The panel is 16:10 at 2x, so the frame keeps
 * that box and only draws the bar — nothing here invents interface.
 */
export function Shot({
	url,
	light,
	dark,
	alt,
	priority,
	loading = 'lazy',
}: ShotProps) {
	const host = url.replace(/^https?:\/\//, '');

	return (
		<figure className="overflow-hidden rounded-xl border border-border bg-card shadow-[var(--shadow-shot)]">
			<div className="flex h-9 items-center gap-2 border-b border-border bg-[#E5E5E5] px-3 dark:bg-[#2b2b2b]">
				<span className="flex gap-1.5" aria-hidden>
					<span className="size-2 rounded-full bg-black/15 dark:bg-white/20" />
					<span className="size-2 rounded-full bg-black/15 dark:bg-white/20" />
					<span className="size-2 rounded-full bg-black/15 dark:bg-white/20" />
				</span>
				<span className="mx-auto flex max-w-full items-center gap-1.5 truncate rounded-md bg-white px-2.5 py-1 font-mono text-[10px] text-[#737373] dark:bg-[#1b1b1b] dark:text-[#a3a3a3]">
					<svg
						viewBox="0 0 24 24"
						className="size-2.5 shrink-0"
						fill="none"
						stroke="currentColor"
						strokeWidth="2.4"
						aria-hidden>
						<path d="M7 11V8a5 5 0 0 1 10 0v3" />
						<rect x="4.5" y="11" width="15" height="10" rx="2.5" />
					</svg>
					<span className="truncate">{host}</span>
				</span>
			</div>
			<div className="aspect-[16/10] w-full">
				<img
					src={light}
					alt={alt}
					width={2400}
					height={1500}
					loading={loading}
					decoding={priority ? 'sync' : 'async'}
					fetchPriority={priority ? 'high' : 'auto'}
					className="block size-full object-cover object-top dark:hidden"
				/>
				<img
					src={dark}
					alt=""
					aria-hidden
					width={2400}
					height={1500}
					loading={loading}
					decoding="async"
					className="hidden size-full object-cover object-top dark:block"
				/>
			</div>
		</figure>
	);
}
