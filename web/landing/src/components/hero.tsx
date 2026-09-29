import {ArrowRight, Terminal} from 'lucide-react';
import {panelSpec} from '../data';
import {Shot} from './shot';
import {btnGhost, btnPrimary} from './buttons';
import {gsap, SplitText, useMotion} from '../lib/motion';

export function Hero() {
	const ref = useMotion<HTMLElement>(scope => {
		const title = scope.querySelector('[data-hero-title]');
		const split = title
			? new SplitText(title, {type: 'lines', mask: 'lines'})
			: null;

		gsap.set('[data-hero-shot]', {yPercent: 4});

		const tl = gsap.timeline({
			defaults: {ease: 'power3.out', duration: 0.7},
			delay: 0.1,
		});

		tl.from('[data-hero-badge]', {y: 10, autoAlpha: 0, duration: 0.5})
			.from(
				split ? split.lines : '[data-hero-title]',
				{yPercent: 115, autoAlpha: 0, duration: 0.95, stagger: 0.1},
				'-=0.15',
			)
			.from('[data-hero-lede]', {y: 14, autoAlpha: 0}, '-=0.6')
			.from(
				'[data-hero-cta]',
				{y: 14, autoAlpha: 0, stagger: 0.08},
				'-=0.5',
			)
			.from('[data-hero-spec]', {y: 22, autoAlpha: 0}, '-=0.9')
			.from(
				'[data-hero-spec-row]',
				{x: 12, autoAlpha: 0, stagger: 0.05, duration: 0.5},
				'-=0.45',
			)
			.from(
				'[data-hero-shot]',
				{y: 56, autoAlpha: 0, scale: 0.985, duration: 1.1},
				'-=0.7',
			);

		/* The capture drifts slower than the page, so the frame reads as depth
		 * rather than a sticker sliding past. */
		gsap.to('[data-hero-shot]', {
			yPercent: -5,
			ease: 'none',
			scrollTrigger: {
				trigger: scope,
				start: 'top top',
				end: 'bottom top',
				scrub: 0.5,
			},
		});
	});

	return (
		<section ref={ref} id="top" className="relative overflow-hidden">
			<div
				className="landing-grid pointer-events-none absolute inset-x-0 top-0 h-[560px]"
				aria-hidden
			/>
			<div className="relative mx-auto max-w-[1200px] px-5 pt-16 pb-0 sm:px-8 sm:pt-24 lg:pt-28">
				<div className="grid items-start gap-12 lg:grid-cols-12 lg:gap-10">
					<div className="lg:col-span-7">
						<span
							data-hero-badge
							className="inline-flex items-center gap-2 rounded-full border border-border bg-card px-3 py-1 text-[12px] text-muted-foreground">
							<span className="size-1.5 rounded-full bg-[var(--success)]" />
							Fully self-hosted. One container per role.
						</span>
						<h1
							data-hero-title
							className="mt-6 text-[40px] leading-[1.05] font-semibold tracking-[-0.03em] text-balance sm:text-[56px] lg:text-[64px]">
							The control panel for the{' '}
							<span className="text-muted-foreground">
								servers you already rent
							</span>
						</h1>
						<p
							data-hero-lede
							className="mt-6 max-w-[56ch] text-[16px] leading-relaxed text-muted-foreground">
							OpenOxide builds from Git, runs the containers, issues the
							wildcard certificates and keeps the records in one SQLite
							file. One panel, one small agent per host, and every byte on
							metal you control.
						</p>
						<div className="mt-8 flex flex-wrap items-center gap-3">
							<a data-hero-cta href="#install" className={btnPrimary}>
								<Terminal size={15} strokeWidth={2} aria-hidden />
								Install in three commands
							</a>
							<a data-hero-cta href="#deployments" className={btnGhost}>
								See what it does
								<ArrowRight size={15} strokeWidth={2} aria-hidden />
							</a>
						</div>
					</div>

					<aside data-hero-spec className="lg:col-span-5 lg:pt-2">
						<dl className="rounded-xl border border-border bg-card shadow-[var(--shadow-card)]">
							<div
								data-hero-spec-row
								className="flex items-center justify-between border-b border-border px-4 py-2.5">
								<span className="font-mono text-[11px] tracking-wide text-muted-foreground uppercase">
									Panel runtime
								</span>
								<span className="font-mono text-[11px] text-muted-foreground">
									x86_64 · musl
								</span>
							</div>
							{panelSpec.map(row => (
								<div
									data-hero-spec-row
									key={row.key}
									className="flex items-baseline justify-between gap-4 border-b border-border px-4 py-2.5 last:border-b-0">
									<dt className="font-mono text-[12px] text-muted-foreground">
										{row.key}
									</dt>
									<dd className="text-right font-mono text-[12px] text-foreground">
										{row.value}
									</dd>
								</div>
							))}
						</dl>
						<p className="mt-3 px-1 text-[12px] text-muted-foreground">
							Everything above is what the container ships with. There is
							no second service to provision.
						</p>
					</aside>
				</div>

				<div data-hero-shot className="mt-16 sm:mt-20">
					<Shot
						priority
						loading="eager"
						url="panel.example.com/projects/1"
						light="/screenshots/app-light.webp"
						dark="/screenshots/app-dark.webp"
						alt="OpenOxide project overview: services, domains, health and resource gauges"
					/>
				</div>
			</div>
		</section>
	);
}
