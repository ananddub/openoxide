import React from 'react';

export const LogDate = React.memo(({ timestamp }: { timestamp: string }) => {
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
		<div className="bg-muted/80 border border-border/80 text-[11px] font-mono rounded-xs px-2 py-[0.12rem] shrink-0 select-none inline-flex items-baseline gap-0">
			<span className="text-sky-400 font-medium tracking-tight">{timePart}</span>
			{msPart && (
				<span className="text-sky-600/60 text-[9px]">.{msPart}</span>
			)}
		</div>
	);
});
