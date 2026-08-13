import {useEffect, useMemo, useState} from 'react';
import {subscribeLive, type LiveEndpoint} from './runtime';

export type LiveResult<T> = {
	data: T | undefined;
	loading: boolean;
	error: Error | undefined;
};

export function useLive<TArgs extends readonly unknown[], TData>(
	createEndpoint: (...args: TArgs) => LiveEndpoint<TArgs, TData>,
	...args: TArgs
): LiveResult<TData> {
	const argsKey = JSON.stringify(args);
	const endpoint = useMemo(() => createEndpoint(...args), [createEndpoint, argsKey]);
	const [result, setResult] = useState<LiveResult<TData>>({data: undefined, loading: true, error: undefined});

	useEffect(() => {
		setResult({data: undefined, loading: true, error: undefined});
		try {
			return subscribeLive(
				endpoint,
				(data) => setResult({data, loading: false, error: undefined}),
				(error) => setResult((current) => ({...current, loading: false, error})),
			);
		} catch (error) {
			setResult({data: undefined, loading: false, error: error instanceof Error ? error : new Error(String(error))});
		}
	}, [endpoint]);

	return result;
}

export function createLiveHook<TArgs extends readonly unknown[], TData>(
	endpoint: Omit<LiveEndpoint<TArgs, TData>, 'args'>,
) {
	const createEndpoint = (...args: TArgs): LiveEndpoint<TArgs, TData> => ({...endpoint, args});
	return (...args: TArgs) => useLive(createEndpoint, ...args);
}
