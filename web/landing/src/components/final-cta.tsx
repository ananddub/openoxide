import {ArrowRight} from 'lucide-react';
import {btnGhost, btnPrimary} from './buttons';
import {gsap, once, SplitText, useMotion} from '../lib/motion';

const principles = [
	[
		'State you can copy',
		'Projects, records, certificates and history sit in one SQLite file. Backing the panel up is a file copy, not a migration.',
	],
	[
		'No accounts, no phone home',
		'Nothing registers with a vendor. Outbound calls go where you pointed them: your Git host, your DNS provider, your ACME server.',
	],
	[
		'Small on purpose',
		'Rust on Axum and Tokio, React 19 on the front. The stock compose file caps the panel at 512 MB and it stays inside it.',
	],
] as const;

export function FinalCta() {
	const ref = useMotion<HTMLElement>(scope => {
		gsap.from('[data-principle]', {
			y: 26,
			autoAlpha: 0,
			duration: 0.75,
			stagger: 0.12,
			ease: 'power3.out',
			scrollTrigger: once(scope.querySelector('[data-principles]')!),
		});

		const title = scope.querySelector('[data-cta-title]');
		const split = title
			? new SplitText(title, {type: 'lines', mask: 'lines'})
			: null;

		gsap
			.timeline({
				defaults: {ease: 'power3.out'},
				scrollTrigger: once(
					scope.querySelector('[data-cta-card]')!,
					'top 84%',
				),
			})
			.from('[data-cta-card]', {y: 46, autoAlpha: 0, duration: 0.9})
			.from(split ? split.lines : '[data-cta-title]', {
				yPercent: 115,
				autoAlpha: 0,
				duration: 0.85,
				stagger: 0.08,
			})
			.from(
				'[data-cta-lede]',
				{y: 14, autoAlpha: 0, duration: 0.65},
				'-=0.5',
			)
			.from(
				'[data-cta-actions] > a',
				{y: 12, autoAlpha: 0, duration: 0.5, stagger: 0.09},
				'-=0.45',
			);
	});

	return (
		<section ref={ref} aria-labelledby="principles-label">
			<h2
				id="principles-label"
				className="mx-auto max-w-[1200px] px-5 font-mono text-[12px] tracking-[0.12em] text-muted-foreground uppercase sm:px-8">
				Why it is built this way
			</h2>
			<div
				data-principles
				className="mx-auto mt-6 grid max-w-[1200px] gap-x-14 border-t border-border px-5 sm:px-8 lg:grid-cols-3">
				{principles.map(([title, body]) => (
					<div
						data-principle
						key={title}
						className="border-b border-border py-8 lg:border-r lg:pr-8 lg:last:border-r-0 lg:last:pr-0">
						<h3 className="text-[15px] font-medium">{title}</h3>
						<p className="mt-2 text-[14px] leading-relaxed text-muted-foreground">
							{body}
						</p>
					</div>
				))}
			</div>

			<div className="mx-auto max-w-[1200px] px-5 py-20 sm:px-8 md:py-28">
				<div
					data-cta-card
					className="overflow-hidden rounded-2xl border border-border bg-card shadow-[var(--shadow-card)]">
					<div className="grid gap-8 p-8 sm:p-12 lg:grid-cols-[minmax(0,1fr)_auto] lg:items-end lg:gap-16">
						<div>
							<h2
								data-cta-title
								className="max-w-[24ch] text-[30px] leading-[1.08] font-semibold tracking-[-0.03em] text-balance md:text-[40px]">
								Run your own version of this
							</h2>
							<p
								data-cta-lede
								className="mt-4 max-w-[52ch] text-[15.5px] leading-relaxed text-muted-foreground">
								Two images on Docker Hub, two compose files in the
								repository, and a panel listening on 4000 before the coffee
								is done.
							</p>
						</div>
						<div
							data-cta-actions
							className="flex flex-wrap items-center gap-3">
							<a href="#install" className={btnPrimary}>
								Install OpenOxide
							</a>
							<a href="/singin" className={btnGhost}>
								Open the panel
								<ArrowRight size={15} strokeWidth={2} aria-hidden />
							</a>
						</div>
					</div>
				</div>
			</div>
		</section>
	);
}
