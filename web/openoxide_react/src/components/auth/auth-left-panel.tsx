import {Logo} from '#/components/shared/logo';

/*
 * Auth showcase band: the polarity-flipped ink surface from the reference.
 * fixed #171717 regardless of theme, the brand mesh gradient as the ONLY
 * decoration (hero scale, never miniaturised), sentence-case display type at
 * weight 600 with negative tracking, mono caption labels. No emoji, no
 * gradient text, no decoration grid lines.
 */
export const AuthLeftPanel = () => (
	<div className="relative hidden flex-col justify-between overflow-hidden bg-[#171717] p-12 text-[#fafafa] select-none lg:flex lg:w-1/2">
		{/* Brand mesh gradient (develop / preview / ship stops), hero scale. */}
		<div
			aria-hidden
			className="pointer-events-none absolute inset-0 opacity-70"
			style={{
				background: [
					'radial-gradient(55% 60% at 18% 8%, rgba(0,124,240,0.38), transparent 70%)',
					'radial-gradient(45% 50% at 82% 0%, rgba(0,223,216,0.30), transparent 70%)',
					'radial-gradient(50% 55% at 88% 72%, rgba(255,0,128,0.26), transparent 70%)',
					'radial-gradient(42% 45% at 12% 92%, rgba(249,203,40,0.18), transparent 70%)',
				].join(','),
			}}
		/>

		{/* 1. LOGO */}
		<div className="relative z-10 flex items-center gap-2">
			<Logo className="size-8 text-white" />
			<span className="text-lg font-semibold tracking-tight text-white">
				OpenOxide
			</span>
		</div>

		{/* 2. Center content */}
		<div className="relative z-10 my-auto flex max-w-lg flex-col gap-6">
			<h1 className="text-4xl leading-[1.15] font-semibold tracking-[-0.03em] text-white">
				Deploy anything. Own everything.
			</h1>
			<p className="max-w-[55ch] text-sm leading-relaxed text-white/60">
				OpenOxide is the self-hosted developer platform that puts you in
				full control. Build, deploy, and manage your applications and
				Docker containers on your own servers.
			</p>
			<p className="text-sm leading-relaxed text-white/80">
				The open source alternative to Netlify, Vercel, and Heroku.
			</p>
			{/* 3. Feature captions: mono label voice, hairline pills. */}
			<div className="mt-2 flex flex-wrap items-center gap-2">
				{['Fast', 'Docker', 'Self hosted'].map(label => (
					<span
						key={label}
						className="rounded-full border border-white/15 bg-white/5 px-2.5 py-1 font-mono text-[11px] tracking-[0.08em] text-white/70 uppercase">
						{label}
					</span>
				))}
			</div>
		</div>

		{/* 4. Footer social links */}
		<div className="relative z-10 flex gap-6">
			<a
				href="https://github.com/vajra-labs/goploy"
				target="_blank"
				rel="noopener noreferrer"
				aria-label="GitHub"
				className="text-white/50 transition-colors hover:text-white">
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
