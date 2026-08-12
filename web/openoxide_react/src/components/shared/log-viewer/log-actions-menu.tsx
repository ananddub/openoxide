import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {MoreVertical, Copy, Link, Code} from 'lucide-react';
import {useState} from 'react';
import type {ParsedLogEntry} from './types';

interface LogActionsMenuProps {
	entry: ParsedLogEntry;
	onShowDetails: (entry: ParsedLogEntry) => void;
}

export function LogActionsMenu({entry, onShowDetails}: LogActionsMenuProps) {
	const [copied, setCopied] = useState(false);

	const handleCopy = () => {
		navigator.clipboard.writeText(entry.clean || entry.raw);
		setCopied(true);
		setTimeout(() => setCopied(false), 1500);
	};

	const handleCopyPermalink = () => {
		const currentUrl = new URL(window.location.href);
		currentUrl.searchParams.set('logId', entry.id);
		navigator.clipboard.writeText(currentUrl.toString());
		setCopied(true);
		setTimeout(() => setCopied(false), 1500);
	};

	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				title="Log Line Actions"
				className="opacity-0 group-hover/entry:opacity-100 transition-opacity p-0.5 rounded hover:bg-zinc-800 text-zinc-400 hover:text-zinc-100 shrink-0 inline-flex items-center cursor-pointer"
			>
				<MoreVertical className="w-3.5 h-3.5" />
			</DropdownMenuTrigger>
			<DropdownMenuContent align="start" className="w-44 bg-card border-border font-mono text-xs">
				<DropdownMenuItem onClick={handleCopy} className="cursor-pointer gap-2">
					<Copy className="w-3.5 h-3.5 text-muted-foreground" />
					<span>{copied ? 'Copied!' : 'Copy log message'}</span>
				</DropdownMenuItem>

				<DropdownMenuItem onClick={handleCopyPermalink} className="cursor-pointer gap-2">
					<Link className="w-3.5 h-3.5 text-muted-foreground" />
					<span>Copy permalink</span>
				</DropdownMenuItem>

				{(entry.isJson || entry.clean.length > 80) && (
					<DropdownMenuItem onClick={() => onShowDetails(entry)} className="cursor-pointer gap-2">
						<Code className="w-3.5 h-3.5 text-muted-foreground" />
						<span>Show details</span>
					</DropdownMenuItem>
				)}
			</DropdownMenuContent>
		</DropdownMenu>
	);
}
