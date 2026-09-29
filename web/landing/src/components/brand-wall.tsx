import {brands} from '../data';
import {gsap, useMotion} from '../lib/motion';

/** A doubled track so the marquee loops without a visible seam. */
function Row({ariaHidden}: {ariaHidden?: boolean}) {
	return (
		<ul
			aria-hidden={ariaHidden}
			className="flex shrink-0 items-center gap-x-10 pr-10">
			{brands.map(b => (
				<li
					key={b.name}
					className="flex items-center gap-2 whitespace-nowrap">
					<svg
						viewBox="0 0 24 24"
						aria-hidden
						className="size-[15px] shrink-0 fill-current text-muted-foreground">
						<path d={b.path} />
					</svg>
					<span className="text-[13px] text-muted-foreground">
						{b.name}
					</span>
				</li>
			))}
		</ul>
	);
}

export function BrandWall() {
	const ref = useMotion<HTMLElement>(scope => {
		const loop = gsap.to('[data-marquee]', {
			xPercent: -50,
			duration: 46,
			ease: 'none',
			repeat: -1,
		});

		/* Slowing rather than stopping keeps the row from feeling dead while
		 * it is being read. */
		scope.addEventListener('pointerenter', () =>
			gsap.to(loop, {timeScale: 0.15, duration: 0.6, ease: 'power2.out'}),
		);
		scope.addEventListener('pointerleave', () =>
			gsap.to(loop, {timeScale: 1, duration: 0.9, ease: 'power2.inOut'}),
		);

		gsap.from(scope.children, {
			autoAlpha: 0,
			duration: 0.7,
			scrollTrigger: {trigger: scope, start: 'top 92%', once: true},
		});
	});

	return (
		<section
			ref={ref}
			aria-labelledby="brand-wall-label"
			className="border-y border-border bg-card/40 py-10">
			<p
				id="brand-wall-label"
				className="mx-auto mb-7 max-w-[1200px] px-5 text-[12px] tracking-wide text-muted-foreground uppercase sm:px-8">
				Talks to
			</p>
			<div className="relative flex overflow-hidden [mask-image:linear-gradient(to_right,transparent,black_9%,black_91%,transparent)]">
				<div data-marquee className="flex w-max">
					<Row />
					<Row ariaHidden />
				</div>
			</div>
		</section>
	);
}
