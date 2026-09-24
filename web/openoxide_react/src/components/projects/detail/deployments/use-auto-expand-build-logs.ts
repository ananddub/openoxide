import {
	useEffect,
	useRef,
	type Dispatch,
	type SetStateAction,
} from 'react';
import {isBuildActive, isBuildSucceeded} from './build-status';

/** Keeps the inline build-log row of the active deployment open while it builds, closes it automatically when the build succeeds, and leaves it open when the build fails so the error is visible without a click. */
export function useAutoExpandBuildLogs(
	deployments: any[],
	setExpandedId: Dispatch<SetStateAction<number | null>>,
) {
	const watched = useRef<number | null>(null);
	const activeId = Number(deployments.find(isBuildActive)?.id) || null;

	useEffect(() => {
		if (activeId !== null) {
			if (watched.current !== activeId) {
				watched.current = activeId;
				setExpandedId(activeId);
			}
			return;
		}
		if (watched.current === null) return;
		const finishedId = watched.current;
		watched.current = null;
		const finished = deployments.find(d => Number(d.id) === finishedId);
		if (finished && isBuildSucceeded(finished)) {
			setExpandedId(prev => (prev === finishedId ? null : prev));
		}
	}, [activeId, deployments, setExpandedId]);
}
