import type {Feature} from '../data';
import {features} from '../data';
import {Shot} from './shot';
import {gsap, useMotion} from '../lib/motion';

function Check() {
	return (
		<svg
			viewBox="0 0 24 24"
			aria-hidden
			fill="none"
			stroke="currentColor"
			strokeWidth="2.6"
			className="mt-[3px] size-3.5 shrink-0 text-muted-foreground">
			<path d="m4 13 5 5L20 6" />
		</svg>
	);
}

function Row({
	feature,
	flip,
	index,
}: {
	feature: Feature;
	flip: boolean;
	index: number;
}) {
	const ref = useMotion<HTMLElement>(scope => {
		gsap
			.timeline({
				defaults: {ease: 'power3.out'},
				scrollTrigger: {trigger: scope, start: 'top 76%', once: true},
			})
			.from('[data-feature-copy] > *', {
				y: 16,
				autoAlpha: 0,
				stagger: 0.09,
				duration: 0.7,
			})
			.from(
				'[data-feature-bullet]',
				{x: -12, autoAlpha: 0, stagger: 0.07, duration: 0.5},
				'-=0.35',
			)
			.from(
				'[data-shot-primary]',
				{y: 54, autoAlpha: 0, scale: 0.98, duration: 1},
				'-=0.55',
			)
			.from(
				'[data-shot-secondary]',
				{y: 38, autoAlpha: 0, duration: 0.85},
				'-=0.6',
			);

		gsap.to('[data-feature-media]', {
			yPercent: -4,
			ease: 'none',
			scrollTrigger: {
				trigger: scope,
				start: 'top bottom',
				end: 'bottom top',
				scrub: 0.6,
			},
		});
	});

	return (
		<article
			ref={ref}
			id={feature.id}
			className="grid items-center gap-10 border-t border-border py-16 md:py-20 lg:grid-cols-2 lg:gap-16">
			<div data-feature-copy className={flip ? 'lg:order-2' : undefined}>
				<p className="font-mono text-[11px] tracking-[0.14em] text-muted-foreground uppercase">
					{String(index + 1).padStart(2, '0')} — {feature.eyebrow}
				</p>
				<h3 className="mt-3.5 text-[26px] leading-[1.15] font-semibold tracking-[-0.02em] text-balance md:text-[32px]">
					{feature.title}
				</h3>
				<p className="mt-4 max-w-[52ch] text-[15.5px] leading-relaxed text-muted-foreground">
					{feature.body}
				</p>
				<ul className="mt-7 space-y-2.5">
					{feature.bullets.map(b => (
						<li
							data-feature-bullet
							key={b}
							className="flex max-w-[46ch] items-start gap-2.5 text-[14px] leading-snug">
							<Check />
							{b}
						</li>
					))}
				</ul>
			</div>

			<div
				data-feature-media
				className={flip ? 'lg:order-1' : 'lg:order-2'}>
				<div data-shot-primary>
					<Shot {...feature.shots[0]} />
				</div>
				{feature.shots[1] && (
					<div
						data-shot-secondary
						className="-mt-12 ml-auto w-[68%] sm:w-[64%]">
						<Shot {...feature.shots[1]} />
					</div>
				)}
			</div>
		</article>
	);
}

export function Features() {
	const ref = useMotion<HTMLElement>(() => {
		gsap.from('[data-features-intro]', {
			y: 20,
			autoAlpha: 0,
			duration: 0.8,
			ease: 'power3.out',
			scrollTrigger: {
				trigger: '[data-features-intro]',
				start: 'top 85%',
				once: true,
			},
		});
	});

	return (
		<section
			ref={ref}
			id="features"
			className="mx-auto max-w-[1200px] px-5 sm:px-8">
			<div data-features-intro className="max-w-[62ch] pt-20 md:pt-24">
				<h2 className="text-[30px] leading-[1.1] font-semibold tracking-[-0.025em] text-balance md:text-[38px]">
					Six screens you end up living in
				</h2>
				<p className="mt-4 text-[16px] leading-relaxed text-muted-foreground">
					Every image on this page came out of a running panel. No redraws,
					no mockups — if a screen looks like this, it behaves like this.
				</p>
			</div>
			{features.map((f, i) => (
				<Row key={f.id} feature={f} flip={i % 2 === 1} index={i} />
			))}
		</section>
	);
}
