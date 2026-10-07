/** Lightweight markdown renderer for AI assistant replies. Handles bold,
 *  inline code, code blocks, lists, and links — enough for the assistant's
 *  output without pulling in a full markdown library. */
export function MiniMarkdown({text}: {text: string}) {
	const blocks = parseBlocks(text);
	return (
		<div className="flex flex-col gap-1.5 text-[13px] leading-relaxed">
			{blocks.map((block, i) => (
				<Block key={i} block={block} />
			))}
		</div>
	);
}

type MdBlock =
	| {type: 'paragraph'; content: string}
	| {type: 'code'; lang: string; content: string}
	| {type: 'list'; items: string[]};

function parseBlocks(text: string): MdBlock[] {
	const blocks: MdBlock[] = [];
	const lines = text.split('\n');
	let i = 0;
	while (i < lines.length) {
		const line = lines[i];
		// Code block
		if (line.startsWith('```')) {
			const lang = line.slice(3).trim();
			const codeLines: string[] = [];
			i++;
			while (i < lines.length && !lines[i].startsWith('```')) {
				codeLines.push(lines[i]);
				i++;
			}
			blocks.push({type: 'code', lang, content: codeLines.join('\n')});
			i++; // skip closing ```
			continue;
		}
		// List items (- or *)
		if (/^[-*] /.test(line)) {
			const items: string[] = [];
			while (i < lines.length && /^[-*] /.test(lines[i])) {
				items.push(lines[i].replace(/^[-*] /, ''));
				i++;
			}
			blocks.push({type: 'list', items});
			continue;
		}
		// Empty line → skip
		if (line.trim() === '') {
			i++;
			continue;
		}
		// Paragraph (collect consecutive non-empty, non-special lines)
		const paraLines: string[] = [];
		while (
			i < lines.length &&
			lines[i].trim() !== '' &&
			!lines[i].startsWith('```') &&
			!/^[-*] /.test(lines[i])
		) {
			paraLines.push(lines[i]);
			i++;
		}
		blocks.push({type: 'paragraph', content: paraLines.join(' ')});
	}
	return blocks;
}

function Block({block}: {block: MdBlock}) {
	switch (block.type) {
		case 'code':
			return (
				<pre className="overflow-x-auto rounded-md bg-muted/60 px-3 py-2 font-mono text-[12px] text-foreground">
					<code>{block.content}</code>
				</pre>
			);
		case 'list':
			return (
				<ul className="list-inside list-disc space-y-0.5 pl-1">
					{block.items.map((item, i) => (
						<li key={i}>
							<InlineText text={item} />
						</li>
					))}
				</ul>
			);
		case 'paragraph':
			return (
				<p>
					<InlineText text={block.content} />
				</p>
			);
	}
}

/** Renders bold, inline code, and links within a text span. */
function InlineText({text}: {text: string}) {
	// Split on bold (**...**), inline code (`...`), and links ([text](url))
	const parts = text.split(/(\*\*[^*]+\*\*|`[^`]+`|\[[^\]]+\]\([^)]+\))/);
	return (
		<>
			{parts.map((part, i) => {
				if (part.startsWith('**') && part.endsWith('**'))
					return (
						<strong key={i} className="font-semibold">
							{part.slice(2, -2)}
						</strong>
					);
				if (part.startsWith('`') && part.endsWith('`'))
					return (
						<code
							key={i}
							className="rounded bg-muted/60 px-1 py-0.5 font-mono text-[12px]">
							{part.slice(1, -1)}
						</code>
					);
				const linkMatch = part.match(/^\[([^\]]+)\]\(([^)]+)\)$/);
				if (linkMatch)
					return (
						<a
							key={i}
							href={linkMatch[2]}
							target="_blank"
							rel="noopener noreferrer"
							className="text-primary underline underline-offset-2">
							{linkMatch[1]}
						</a>
					);
				return <span key={i}>{part}</span>;
			})}
		</>
	);
}
