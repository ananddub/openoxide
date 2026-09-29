import type {LucideIcon} from 'lucide-react';
import {Activity, Boxes, Database, Globe} from 'lucide-react';
import {capabilityGroups, type CapabilityGroup} from '../data';
import {gsap, once, useMotion} from '../lib/motion';

const glyphs: Record<CapabilityGroup['glyph'], LucideIcon> = {
	Boxes,
	Globe,
	Database,
	Activity,
};

export function Capabilities() {
	const ref = useMotion<HTMLElement>(scope => {
		gsap.from('[data-cap-lede]', {
			y: 18,
			autoAlpha: 0,
			duration: 0.7,
			ease: 'power3.out',
			scrollTrigger: once(scope.querySelector('[data-cap-lede]')!),
		});

		gsap.from('[data-cap-group]', {
			y: 26,
			autoAlpha: 0,
			duration: 0.8,
			stagger: 0.12,
			ease: 'power3.out',
			scrollTrigger: once(
				scope.querySelector('[data-cap-groups]')!,
				'top 80%',
			),
		});

		gsap.from('[data-cap-item]', {
			x: -10,
			autoAlpha: 0,
			duration: 0.5,
			stagger: 0.02,
			ease: 'power2.out',
			scrollTrigger: once(
				scope.querySelector('[data-cap-groups]')!,
				'top 74%',
			),
		});
	});

	return (
		<section
			ref={ref}
			id="capabilities"
			className="border-t border-border bg-card/40">
			<div className="mx-auto grid max-w-[1200px] gap-12 px-5 py-20 sm:px-8 md:py-24 lg:grid-cols-[minmax(0,260px)_minmax(0,1fr)] lg:gap-20">
				<div className="lg:sticky lg:top-24 lg:self-start">
					<h2
						data-cap-eyebrow
						className="text-[28px] leading-[1.12] font-semibold tracking-[-0.025em] text-balance md:text-[34px]">
						The whole surface area
					</h2>
					<p className="mt-4 text-[15px] leading-relaxed text-muted-foreground">
						Grouped the way the panel is organised. Nothing here is a
						roadmap slide — it is what the binary does today.
					</p>
				</div>

				<div
					data-cap-groups
					className="grid gap-x-14 gap-y-12 sm:grid-cols-2">
					{capabilityGroups.map(g => {
						const Glyph = glyphs[g.glyph];
						return (
							<div data-cap-group key={g.title}>
								<div className="flex items-center gap-2.5 border-b border-border pb-3">
									<Glyph
										size={15}
										strokeWidth={1.9}
										aria-hidden
										className="text-muted-foreground"
									/>
									<h3 className="font-mono text-[12px] tracking-[0.12em] uppercase">
										{g.title}
									</h3>
								</div>
								<ul className="mt-4 space-y-2.5">
									{g.items.map(i => (
										<li
											data-cap-item
											key={i}
											className="flex gap-2.5 text-[14px] leading-snug text-muted-foreground">
											<span
												aria-hidden
												className="mt-[7px] size-1 shrink-0 rounded-full bg-[var(--ga-500)]"
											/>
											<span className="text-foreground/90">{i}</span>
										</li>
									))}
								</ul>
							</div>
						);
					})}
				</div>
			</div>
		</section>
	);
}
