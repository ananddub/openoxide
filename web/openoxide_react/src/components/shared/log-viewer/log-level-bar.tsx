import React from 'react';
import type {LogLevel} from './types';

interface LogLevelBarProps {
	level: LogLevel;
	isMultiLine?: boolean;
}

export const LogLevelBar = React.memo(
	({level, isMultiLine = false}: LogLevelBarProps) => {
		let colorClass = 'bg-[#22c55e]'; // Vibrant green

		switch (level) {
			case 'ERROR':
				colorClass = 'bg-[#ef4444]'; // Vibrant red
				break;
			case 'WARN':
				colorClass = 'bg-[#f97316]'; // Vibrant orange
				break;
			case 'DEBUG':
				colorClass = 'bg-[#a855f7]'; // Vibrant purple
				break;
			case 'SUCCESS':
				colorClass = 'bg-[#22c55e]'; // Vibrant green
				break;
			default:
				colorClass = 'bg-[#22c55e]'; // Vibrant green
				break;
		}

		if (isMultiLine) {
			// Expands vertically along multiline entries (Dozzle data-position start/middle/end stretch)
			return (
				<div
					className={`my-0.5 min-h-full w-[3px] shrink-0 self-stretch rounded-xs select-none ${colorClass}`}
					title={`Level: ${level}`}
				/>
			);
		}

		// Single line dot (Dozzle mt-1.5 size-2.5 flex-none rounded-lg)
		return (
			<div
				className={`mt-1.5 h-2.5 w-2.5 flex-none shrink-0 rounded-full select-none ${colorClass}`}
				title={`Level: ${level}`}
			/>
		);
	},
);
