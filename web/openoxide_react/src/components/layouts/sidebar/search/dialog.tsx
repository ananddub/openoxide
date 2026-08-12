import * as React from 'react';
import {cn} from '#/api/utils';
import {ROUTES, type RouteItem} from '../enum';
import {useNavigate} from '@tanstack/react-router';
import {Dialog, DialogContent} from '#/components/ui/dialog';
import {ArrowDown, ArrowUp, CornerDownLeft, Search} from 'lucide-react';

// Keyboard-driven command palette. Opens on Cmd/Ctrl+K.
// Routes are grouped dynamically from ROUTES — no hardcoded group names.
export function SearchDialog() {
	const [open, setOpen] = React.useState(false);
	const [query, setQuery] = React.useState('');
	const [selectedIndex, setSelectedIndex] = React.useState(0);
	const navigate = useNavigate();
	const inputRef = React.useRef<HTMLInputElement>(null);

	React.useEffect(() => {
		const handleKeydown = (e: KeyboardEvent) => {
			if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
				e.preventDefault();
				setOpen(prev => !prev);
			}
		};
		window.addEventListener('keydown', handleKeydown);
		return () => window.removeEventListener('keydown', handleKeydown);
	}, []);

	// Reset state and focus input when dialog opens
	React.useEffect(() => {
		if (open) {
			setQuery('');
			setSelectedIndex(0);
			setTimeout(() => inputRef.current?.focus(), 50);
		}
	}, [open]);

	const filteredRoutes = ROUTES.filter(
		route =>
			route.label.toLowerCase().includes(query.toLowerCase()) ||
			route.group.toLowerCase().includes(query.toLowerCase()),
	);

	// Reset selection index when query changes
	React.useEffect(() => {
		setSelectedIndex(0);
	}, [query]);

	// Close dialog first, then navigate to avoid dialog blocking route transition.
	const handleNavigate = (path: string) => {
		setOpen(false);
		navigate({to: path as any});
	};

	const handleKeyDown = (e: React.KeyboardEvent) => {
		if (filteredRoutes.length === 0) return;
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			setSelectedIndex(prev => (prev + 1) % filteredRoutes.length);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			setSelectedIndex(
				prev => (prev - 1 + filteredRoutes.length) % filteredRoutes.length,
			);
		} else if (e.key === 'Enter') {
			e.preventDefault();
			handleNavigate(filteredRoutes[selectedIndex].path);
		}
	};

	// Dynamically group routes by their group key — no hardcoding
	const groupedRoutes = filteredRoutes.reduce<Record<string, RouteItem[]>>(
		(acc, route) => {
			(acc[route.group] ??= []).push(route);
			return acc;
		},
		{},
	);
	const groups = Object.entries(groupedRoutes);

	return (
		<Dialog open={open} onOpenChange={setOpen}>
			<DialogContent className="overflow-hidden rounded-xl border border-border/60 bg-popover p-0 shadow-2xl sm:max-w-md">
				<div className="flex items-center border-b border-border/50 px-3 py-2">
					<Search className="mr-2 size-4 shrink-0 text-muted-foreground" />
					<input
						ref={inputRef}
						value={query}
						onChange={e => setQuery(e.target.value)}
						onKeyDown={handleKeyDown}
						placeholder="Type a command or search..."
						className="w-full border-0 bg-transparent py-1.5 text-sm text-foreground placeholder:text-muted-foreground/60 focus:ring-0 focus:outline-hidden"
					/>
				</div>
				<div className="flex h-72 flex-col gap-2 overflow-y-auto p-2">
					{groups.length > 0 ? (
						groups.map(([groupName, routes], groupIdx) => (
							<React.Fragment key={groupName}>
								{/* Separator between groups */}
								{groupIdx > 0 && (
									<div className="mx-2 my-1 h-px bg-border/40" />
								)}
								<div className="flex flex-col gap-0.5">
									<div className="px-3 py-1.5 text-[10px] font-semibold tracking-wider text-muted-foreground/60 uppercase select-none">
										{groupName}
									</div>
									{routes.map(route => {
										const absoluteIndex = filteredRoutes.indexOf(route);
										const isSelected = absoluteIndex === selectedIndex;
										return (
											<button
												key={route.path}
												onClick={() => handleNavigate(route.path)}
												className={cn(
													'flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-3 py-2 text-left text-sm outline-hidden transition-all duration-100',
													isSelected
														? 'bg-primary/10 font-medium text-primary'
														: 'text-foreground hover:bg-muted/50 hover:text-foreground',
												)}>
												<route.icon
													className={cn(
														'size-4 shrink-0',
														isSelected
															? 'text-primary'
															: 'text-muted-foreground',
													)}
												/>
												<span className="truncate">{route.label}</span>
											</button>
										);
									})}
								</div>
							</React.Fragment>
						))
					) : (
						<div className="flex flex-1 items-center justify-center py-8 text-center text-sm text-muted-foreground select-none">
							No results found.
						</div>
					)}
				</div>
				<div className="flex shrink-0 items-center justify-end gap-4 border-t border-border/40 bg-muted/40 px-4 py-2.5 text-[10px] text-muted-foreground/75 select-none">
					<div className="flex items-center gap-1.5">
						<kbd className="flex items-center gap-0.5 rounded border border-border/70 bg-muted px-1.5 py-0.5 shadow-xs">
							<ArrowUp className="size-2.5" />
							<ArrowDown className="size-2.5" />
						</kbd>
						to navigate
					</div>
					<div className="flex items-center gap-1.5">
						<kbd className="flex items-center rounded border border-border/70 bg-muted px-1.5 py-0.5 shadow-xs">
							<CornerDownLeft className="size-2.5" />
						</kbd>
						to select
					</div>
					<div className="flex items-center gap-1.5">
						<kbd className="rounded border border-border/70 bg-muted px-1.5 py-0.5 font-mono text-[9px] shadow-xs">
							esc
						</kbd>
						to close
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
