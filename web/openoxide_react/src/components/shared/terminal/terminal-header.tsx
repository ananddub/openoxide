import {
	Box,
	Maximize2,
	Minimize2,
	Server as ServerIcon,
	Terminal as TerminalIcon,
	X,
} from 'lucide-react';
import {cn} from '#/api/utils';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

/** Terminal card header: icon tile, title/subtitle, host + service + shell
 * pickers, expand and close controls. */
export function TerminalHeader({
	displayName,
	subtitle,
	isRemoteServer,
	hostMode,
	onToggleHost,
	isCompose,
	servicesList,
	targetContainer,
	onSelectService,
	shell,
	onShellChange,
	expanded,
	onToggleExpanded,
	onClose,
}: {
	displayName: string;
	subtitle: string;
	isRemoteServer: boolean;
	hostMode: boolean;
	onToggleHost: () => void;
	isCompose: boolean;
	servicesList: string[];
	targetContainer: string;
	onSelectService: (svc: string) => void;
	shell: string;
	onShellChange: (shell: 'sh' | 'bash') => void;
	expanded: boolean;
	onToggleExpanded: () => void;
	onClose?: () => void;
}) {
	return (
		<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
			<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted ring-1 ring-border/50">
				<TerminalIcon className="size-[18px] text-muted-foreground" />
			</div>
			<div className="min-w-0 flex-1">
				<h2 className="truncate text-[15px] font-semibold text-foreground">
					Terminal{' '}
					<span className="text-muted-foreground">· {displayName}</span>
				</h2>
				<p className="truncate text-xs text-muted-foreground">
					{subtitle}
				</p>
			</div>
			<div className="flex shrink-0 items-center gap-2">
				{!isRemoteServer && (
					<button
						type="button"
						onClick={onToggleHost}
						title={
							hostMode
								? 'Switch back to container shell'
								: 'Open a root shell on the host (via Agent)'
						}
						className={cn(
							'inline-flex h-8 shrink-0 cursor-pointer items-center gap-1.5 rounded-xl px-2.5 text-[12px] font-medium ring-1 transition-colors',
							hostMode
								? 'bg-emerald-500/15 text-emerald-500 ring-emerald-500/40'
								: 'text-muted-foreground ring-border/50 hover:bg-muted hover:text-foreground',
						)}>
						<ServerIcon className="size-3.5" />
						Host
					</button>
				)}
				{!isRemoteServer &&
					!hostMode &&
					isCompose &&
					servicesList.length > 1 && (
						<Select
							value={targetContainer}
							onValueChange={val => onSelectService(val ?? '')}>
							<SelectTrigger className="h-8 min-h-8 w-[130px] gap-1 rounded-xl px-2.5 text-[12px] shadow-none">
								<Box className="size-3.5 shrink-0 text-muted-foreground" />
								<SelectValue placeholder="Service" />
							</SelectTrigger>
							<SelectContent>
								{servicesList.map(svc => (
									<SelectItem key={svc} value={svc} className="text-xs">
										{svc}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					)}
				<Select
					value={shell}
					onValueChange={val =>
						val && onShellChange(val as 'sh' | 'bash')
					}>
					<SelectTrigger className="h-8 min-h-8 w-[84px] rounded-xl px-2.5 font-mono text-[12px] shadow-none">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="bash" className="font-mono text-xs">
							bash
						</SelectItem>
						<SelectItem value="sh" className="font-mono text-xs">
							sh
						</SelectItem>
					</SelectContent>
				</Select>
				<button
					type="button"
					onClick={onToggleExpanded}
					aria-label={expanded ? 'Collapse terminal' : 'Expand terminal'}
					title={expanded ? 'Collapse (Esc)' : 'Expand'}
					className="inline-flex size-9 shrink-0 cursor-pointer items-center justify-center rounded-xl text-muted-foreground ring-1 ring-border/50 transition-colors hover:bg-muted hover:text-foreground">
					{expanded ? (
						<Minimize2 className="size-[18px]" />
					) : (
						<Maximize2 className="size-[18px]" />
					)}
				</button>
				{onClose && (
					<button
						type="button"
						onClick={onClose}
						aria-label="Close terminal"
						title="Close"
						className="inline-flex size-9 shrink-0 cursor-pointer items-center justify-center rounded-xl text-muted-foreground ring-1 ring-border/50 transition-colors hover:bg-muted hover:text-foreground">
						<X className="size-[18px]" />
					</button>
				)}
			</div>
		</div>
	);
}
