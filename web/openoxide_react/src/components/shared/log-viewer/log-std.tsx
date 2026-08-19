import React from 'react';
import type {StreamType} from './types';

export const LogStd = React.memo(({std}: {std: StreamType}) => {
	const isStderr = std === 'STDERR';

	return (
		<div className="inline-flex shrink-0 items-center justify-center rounded-xs border border-border/80 bg-muted/80 px-2 py-[0.12rem] font-mono text-[11px] font-bold select-none">
			<span className={isStderr ? 'text-rose-400' : 'text-sky-400'}>
				{isStderr ? 'stderr' : 'stdout'}
			</span>
		</div>
	);
});
