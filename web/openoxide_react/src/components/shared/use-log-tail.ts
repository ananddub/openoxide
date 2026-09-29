import {useEffect, useRef, type RefObject} from 'react';

/** Slack in px that still counts as "reading the newest line". */
const FOLLOW_ZONE_PX = 40;

/**
 * Opens a log surface at the newest line and keeps it pinned there while the
 * stream runs, so a deploy you are watching shows live output instead of the
 * first build step. Scrolling up releases the pin; a line appended while pinned
 * re-jumps, and an emptying corpus (a reload or a different deployment streamed
 * into the same surface) makes the next fill jump again.
 *
 * "Pinned" is sampled at append time by comparing the reader's position against
 * the height recorded before the append — no scroll listener, so it also works
 * when the surface mounts lazily, as collapsible panels do.
 */
export function useLogTail(
	scrollRef: RefObject<HTMLElement | null>,
	lineCount: number,
	isLive: boolean,
) {
	const filledRef = useRef(false);
	const prevHeightRef = useRef(0);

	useEffect(() => {
		const el = scrollRef.current;
		if (!el) return;

		// Empty corpus = a reload in flight; the next fill should jump again.
		if (lineCount === 0) {
			filledRef.current = false;
			prevHeightRef.current = 0;
			return;
		}

		const firstFill = !filledRef.current;
		filledRef.current = true;
		const atTail =
			firstFill ||
			el.scrollTop + el.clientHeight >=
				prevHeightRef.current - FOLLOW_ZONE_PX;
		if (atTail && (firstFill || isLive)) el.scrollTop = el.scrollHeight;
		prevHeightRef.current = el.scrollHeight;
	}, [scrollRef, lineCount, isLive]);
}
