import {stats} from '../data';
import {gsap, useMotion} from '../lib/motion';

export function Stats() {
	const ref = useMotion<HTMLElement>(scope => {
		gsap.utils.toArray<HTMLElement>('[data-count]').forEach(el => {
			/* The target lives in the attribute, never scraped back out of the
			 * text — so a second mount starts from zero instead of from the
			 * zero the first mount left behind. */
			gsap.fromTo(
				el,
				{textContent: 0},
				{
					textContent: Number(el.dataset.count),
					duration: 1.5,
					ease: 'power2.out',
					snap: {textContent: 1},
					scrollTrigger: {trigger: el, start: 'top 92%', once: true},
				},
			);
		});

		gsap.from('[data-stat-label]', {
			y: 10,
			autoAlpha: 0,
			duration: 0.6,
			stagger: 0.06,
			scrollTrigger: {trigger: scope, start: 'top 88%', once: true},
		});
	});

	return (
		<section
			ref={ref}
			aria-label="What ships in the box"
			className="mx-auto max-w-[1200px] px-5 sm:px-8">
			<dl className="grid grid-cols-2 divide-border border-y border-border md:grid-cols-4 md:divide-x">
				{stats.map(s => (
					<div
						key={s.label}
						data-stat-label
						className="border-b border-border px-1 py-8 last:border-r-0 md:border-b-0 md:py-10 md:pl-6 md:last:pr-6">
						<dt className="sr-only">{s.label}</dt>
						<dd>
							<span
								data-count={s.value}
								className="block font-mono text-[32px] leading-none font-medium tracking-tight md:text-[38px]">
								{s.value}
							</span>
							<span className="mt-2.5 block text-[14px] font-medium">
								{s.label}
							</span>
							<span className="mt-1 block text-[12.5px] text-muted-foreground">
								{s.note}
							</span>
						</dd>
					</div>
				))}
			</dl>
		</section>
	);
}
