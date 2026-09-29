import {useLayoutEffect, useRef} from 'react';
import gsap from 'gsap';
import {ScrollTrigger} from 'gsap/ScrollTrigger';
import {SplitText} from 'gsap/SplitText';

gsap.registerPlugin(ScrollTrigger, SplitText);

/** The one gate: no JS motion for people who asked for none. */
export const prefersReducedMotion = () =>
	window.matchMedia('(prefers-reduced-motion: reduce)').matches;

/**
 * Binds a GSAP context to a subtree. Selector text inside `setup` is scoped to
 * the returned ref, and every tween, listener and SplitText split is undone on
 * unmount — which is what makes the StrictMode double-invoke harmless.
 */
export function useMotion<T extends HTMLElement>(
	setup: (scope: T) => void,
) {
	const ref = useRef<T>(null);

	useLayoutEffect(() => {
		const el = ref.current;
		if (!el || prefersReducedMotion()) return;
		const ctx = gsap.context(() => setup(el), el);
		return () => ctx.revert();
	}, []);

	return ref;
}

/** Play-once scroll trigger for a block inside a motion scope. */
export const once = (trigger: Element, start = 'top 82%') => ({
	trigger,
	start,
	once: true,
});

/**
 * Screenshots settle after first paint and webfonts reflow the headlines, both
 * of which drag every ScrollTrigger below them out of position. One refresh
 * once each lands keeps the timings honest without watching layout.
 */
export function useScrollRefresh() {
	useLayoutEffect(() => {
		const refresh = () => ScrollTrigger.refresh();
		window.addEventListener('load', refresh);
		document.fonts.ready.then(refresh);
		return () => window.removeEventListener('load', refresh);
	}, []);
}

export {gsap, ScrollTrigger, SplitText};
