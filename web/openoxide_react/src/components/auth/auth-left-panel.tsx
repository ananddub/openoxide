import {Logo} from '#/components/shared/logo';

export const AuthLeftPanel = () => (
	<div className="relative hidden flex-col justify-between overflow-hidden border-r border-border bg-muted p-12 select-none lg:flex lg:w-1/2">
		{/* Subtle grid pattern overlay */}
		<div className="pointer-events-none absolute inset-0 bg-[linear-gradient(to_right,#80808006_1px,transparent_1px),linear-gradient(to_bottom,#80808006_1px,transparent_1px)] bg-size-[24px_24px]" />
		{/* 1. LOGO */}
		<div className="relative z-10 flex items-center gap-2">
			<Logo className="size-8" />
			<span className="text-lg font-bold tracking-tight text-foreground">
				OpenOxide
			</span>
		</div>
		{/* Center Content Section */}
		<div className="relative z-10 my-auto flex max-w-lg flex-col gap-6">
			{/* 2. Headline: Deploy anything. Own everything. */}
			<div className="flex flex-col gap-1">
				<h1 className="text-4xl leading-tight font-extrabold tracking-tight text-foreground">
					Deploy anything.
				</h1>
				<h1 className="text-4xl leading-tight font-extrabold tracking-tight text-primary">
					Own everything.
				</h1>
			</div>
			{/* 3. Small description */}
			<p className="text-sm leading-relaxed text-muted-foreground">
				OpenOxide is the self-hosted developer platform that puts you in full
				control. Build, deploy, and manage your applications and Docker
				containers on your own servers.
			</p>
			<p className="text-sm text-muted-foreground/80 italic">
				"The Open Source alternative to Netlify, Vercel, Heroku."
			</p>
			{/* 5. Features Badges: Fast, Docker, Self Hosted */}
			<div className="mt-2 flex flex-wrap items-center gap-3 text-xs font-semibold text-muted-foreground">
				<span className="flex items-center gap-1.5 rounded-full border border-border bg-card px-3 py-1.5 shadow-sm">
					<span>🚀</span> Fast
				</span>
				<span className="flex items-center gap-1.5 rounded-full border border-border bg-card px-3 py-1.5 shadow-sm">
					<span>🐳</span> Docker
				</span>
				<span className="flex items-center gap-1.5 rounded-full border border-border bg-card px-3 py-1.5 shadow-sm">
					<span>🔒</span> Self Hosted
				</span>
			</div>
		</div>
		{/* 6. Footer Social Links */}
		<div className="relative z-10 flex gap-6">
			<a
				href="https://github.com/vajra-labs/goploy"
				target="_blank"
				rel="noopener noreferrer"
				aria-label="GitHub"
				className="text-muted-foreground/60 transition-colors hover:text-foreground">
				<div
					className="size-5 bg-current"
					style={{
						maskImage: 'url(/icons/github.svg)',
						WebkitMaskImage: 'url(/icons/github.svg)',
						maskSize: 'contain',
						WebkitMaskSize: 'contain',
						maskRepeat: 'no-repeat',
						WebkitMaskRepeat: 'no-repeat',
					}}
				/>
			</a>
		</div>
	</div>
);
