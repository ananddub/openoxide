import React from 'react';
import type {StreamType} from './types';

export const LogStd = React.memo(({std}: {std: StreamType}) => {
	const isStderr = std === 'STDERR';

	return (
		<div className="bg-muted/80 border border-border/80 text-[11px] font-mono font-bold rounded-xs px-2 py-[0.12rem] shrink-0 select-none inline-flex items-center justify-center">
			<span className={isStderr ? 'text-rose-400' : 'text-sky-400'}>
				{isStderr ? 'stderr' : 'stdout'}
			</span>
		</div>
	);
});
