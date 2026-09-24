import {useEffect, useState} from 'react';
import {isOverrideStale} from '../shared/resource-status';

/** Transient optimistic status override for the compose detail page. Cleared once the fetched status overtakes it. */
export function useComposeStatusOverride(
	composeStatus: string | undefined,
) {
	const [localStatusOverride, setLocalStatusOverride] = useState<
		string | null
	>(null);

	useEffect(() => {
		if (isOverrideStale(localStatusOverride, composeStatus)) {
			setLocalStatusOverride(null);
		}
	}, [composeStatus, localStatusOverride]);

	return {localStatusOverride, setLocalStatusOverride};
}
