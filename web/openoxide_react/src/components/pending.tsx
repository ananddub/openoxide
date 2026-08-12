import {Logo} from '#/components/shared/logo';

export const RootPending = () => (
	<div className="relative flex min-h-screen w-screen flex-col items-center justify-center overflow-hidden bg-background">
		{/* Soft ambient glowing background blobs */}
		<div className="pointer-events-none absolute -top-40 -left-40 h-100 w-100 rounded-full bg-primary/5 blur-[120px]" />
		<div className="pointer-events-none absolute -right-40 -bottom-40 h-100 w-100 rounded-full bg-primary/5 blur-[120px]" />
		{/* Subtle grid overlay */}
		<div className="pointer-events-none absolute inset-0 bg-[linear-gradient(to_right,#80808003_1px,transparent_1px),linear-gradient(to_bottom,#80808003_1px,transparent_1px)] bg-size-[24px_24px]" />
		{/* Central Loader Section */}
		<div className="relative z-10 flex flex-col items-center gap-6">
			{/* Pulsing Logo inside a glowing border */}
			<div className="relative flex items-center justify-center">
				{/* Glowing outer ring */}
				<div
					className="absolute size-20 animate-ping rounded-2xl border border-primary/20 bg-primary/5 opacity-40"
					style={{animationDuration: '2s'}}
				/>
				{/* Logo Box */}
				<div
					className="relative flex size-16 animate-pulse items-center justify-center rounded-2xl border border-border/60 bg-card p-3 shadow-xl"
					style={{animationDuration: '3s'}}>
					<Logo className="size-10 rounded-lg" />
				</div>
			</div>
			{/* Loading Text & Description */}
			<div className="flex flex-col items-center gap-1.5 text-center select-none">
				<h2 className="text-sm font-semibold tracking-wide text-foreground">
					GoPloy
				</h2>
				<p
					className="animate-pulse font-mono text-[11px] font-medium text-muted-foreground/80"
					style={{animationDuration: '2.5s'}}>
					Connecting to node cluster...
				</p>
			</div>
		</div>
	</div>
);
