import React from 'react';
import {renderRichLogText} from './log-highlighter';

// ANSI escape code parser to React nodes with URL & syntax highlighting
export function renderAnsiText(text: string, searchQuery?: string): React.ReactNode {
	if (!text) return null;

	const ansiRegex = /\x1B\[([0-9;]*)m/g;

	if (!ansiRegex.test(text)) {
		// Plain text rendering with URL & syntax highlight
		return renderRichLogText(text, searchQuery);
	}

	// Reset regex state
	ansiRegex.lastIndex = 0;

	const elements: React.ReactNode[] = [];
	let lastIndex = 0;
	let currentClasses: string[] = [];
	let match: RegExpExecArray | null;

	while ((match = ansiRegex.exec(text)) !== null) {
		const textChunk = text.slice(lastIndex, match.index);
		if (textChunk) {
			elements.push(
				<span key={lastIndex} className={currentClasses.join(' ')}>
					{renderRichLogText(textChunk, searchQuery)}
				</span>
			);
		}

		const codes = match[1].split(';').map(Number);
		for (const code of codes) {
			if (code === 0 || isNaN(code)) {
				currentClasses = [];
			} else if (code === 1) {
				currentClasses.push('font-bold');
			} else if (code === 2) {
				currentClasses.push('opacity-70');
			} else if (code === 4) {
				currentClasses.push('underline');
			} else if (code >= 30 && code <= 37) {
				const colors = [
					'text-zinc-400', // 30: black
					'text-rose-400', // 31: red
					'text-emerald-400', // 32: green
					'text-amber-300', // 33: yellow
					'text-sky-400', // 34: blue
					'text-purple-400', // 35: magenta
					'text-cyan-400', // 36: cyan
					'text-zinc-100', // 37: white
				];
				currentClasses = currentClasses.filter((c) => !c.startsWith('text-'));
				currentClasses.push(colors[code - 30] || 'text-zinc-200');
			} else if (code >= 90 && code <= 97) {
				const brightColors = [
					'text-zinc-500', // 90: bright black
					'text-rose-300 font-semibold', // 91: bright red
					'text-emerald-300 font-medium', // 92: bright green
					'text-amber-200 font-medium', // 93: bright yellow
					'text-sky-300', // 94: bright blue
					'text-purple-300', // 95: bright magenta
					'text-cyan-300', // 96: bright cyan
					'text-white', // 97: bright white
				];
				currentClasses = currentClasses.filter((c) => !c.startsWith('text-'));
				currentClasses.push(brightColors[code - 90] || 'text-zinc-100');
			}
		}

		lastIndex = ansiRegex.lastIndex;
	}

	const remaining = text.slice(lastIndex);
	if (remaining) {
		elements.push(
			<span key={lastIndex} className={currentClasses.join(' ')}>
				{renderRichLogText(remaining, searchQuery)}
			</span>
		);
	}

	return elements;
}
