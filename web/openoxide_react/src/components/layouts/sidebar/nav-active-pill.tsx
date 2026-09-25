import {
	useLayoutEffect,
	useState,
	type RefObject,
} from 'react';
import {cn} from '#/api/utils';
import {SIDEBAR_COLLAPSE} from '#/components/ui/sidebar';

type Box = {x: number; y: number; w: number; h: number; animate: boolean};

/** One pill that slides to the selected rail row instead of every row repainting its own fill — the Vercel rail behaviour, so a click reads as motion. Rows stay transparent and mark themselves with `data-nav-active`. */
export function NavActivePill({navRef}: {navRef: RefObject<HTMLElement | null>}) {
	const [box, setBox] = useState<Box | null>(null);

	useLayoutEffect(() => {
		const nav = navRef.current;
		if (!nav) return;
		let frame = 0;
		let lastWidth = -1;
		const measure = () => {
			const row = nav.querySelector<HTMLElement>('[data-nav-active]');
			if (!row) {
				setBox(null);
				return;
			}
			const navBox = nav.getBoundingClientRect();
			const rowBox = row.getBoundingClientRect();
			// While the rail resizes the pill tracks frame by frame; easing would lag it.
			const animate = navBox.width === lastWidth;
			lastWidth = navBox.width;
			setBox({
				x: rowBox.left - navBox.left,
				// The row lives in the scrolled content, so add scrollTop back.
				y: rowBox.top - navBox.top + nav.scrollTop,
				w: rowBox.width,
				h: rowBox.height,
				animate,
			});
		};
		const schedule = () => {
			cancelAnimationFrame(frame);
			frame = requestAnimationFrame(measure);
		};
		schedule();
		// Any rail change — new selection, collapse, context swap — re-measures.
		const mutations = new MutationObserver(schedule);
		mutations.observe(nav, {
			subtree: true,
			childList: true,
			attributes: true,
			attributeFilter: ['data-nav-active', 'class'],
		});
		const resize = new ResizeObserver(schedule);
		resize.observe(nav);
		window.addEventListener('resize', schedule);
		return () => {
			cancelAnimationFrame(frame);
			mutations.disconnect();
			resize.disconnect();
			window.removeEventListener('resize', schedule);
		};
	}, [navRef]);

	return (
		<div
			aria-hidden
			style={
				box
				? {
						transform: `translate(${box.x}px, ${box.y}px)`,
						width: box.w,
						height: box.h,
					}
				: {opacity: 0}
			}
			className={cn(
				'pointer-events-none absolute top-0 left-0 rounded-md bg-sidebar-active transition-[transform,width,height,opacity]',
				box?.animate ? SIDEBAR_COLLAPSE : 'duration-0',
			)}
		/>
	);
}
