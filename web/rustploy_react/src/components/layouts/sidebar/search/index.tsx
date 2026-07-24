import {Command, Search} from 'lucide-react';

// Triggers the search dialog by dispatching a synthetic Cmd+K keydown event.
export const SearchButton = () => (
	<button
		onClick={() => {
			const event = new KeyboardEvent('keydown', {
				key: 'k',
				metaKey: true,
				bubbles: true,
			});
			window.dispatchEvent(event);
		}}
		className="mx-3 mt-1 flex items-center gap-2 rounded-md border border-border/50 bg-background/50 px-3 py-2 text-[12px] text-muted-foreground transition-colors select-none group-data-[collapsible=icon]:hidden hover:cursor-pointer hover:bg-muted/50">
		<Search className="size-4 shrink-0" />
		<span className="flex-1 text-left">Search...</span>
		<kbd className="flex items-center gap-1 rounded border border-border/60 bg-muted px-1 py-0.5 font-mono text-[10px]">
			<Command className="size-3" /> K
		</kbd>
	</button>
);
