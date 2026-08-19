import React from 'react';

export const LogDate = React.memo(({timestamp}: {timestamp: string}) => {
	if (!timestamp) return null;

	let timePart = '';
	let msPart = '';

	const isoMatch = timestamp.match(
		/(\d{4}-\d{2}-\d{2})[T ](\d{2}:\d{2}:\d{2})(?:\.(\d+))?/i,
	);

	if (isoMatch) {
		timePart = isoMatch[2];
		msPart = isoMatch[3] ? isoMatch[3].slice(0, 3) : '';
	} else {
		const timeOnly = timestamp.match(/^(\d{2}:\d{2}:\d{2})(?:\.(\d+))?/);
		if (timeOnly) {
			timePart = timeOnly[1];
			msPart = timeOnly[2] ? timeOnly[2].slice(0, 3) : '';
		} else {
			timePart = timestamp;
		}
	}

	return (
		<div className="inline-flex shrink-0 items-baseline gap-0 rounded-xs border border-border/80 bg-muted/80 px-2 py-[0.12rem] font-mono text-[11px] select-none">
			<span className="font-medium tracking-tight text-sky-400">
				{timePart}
			</span>
			{msPart && (
				<span className="text-[9px] text-sky-600/60">.{msPart}</span>
			)}
		</div>
	);
});
