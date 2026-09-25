import {X} from 'lucide-react';
import {cn} from '#/api/utils';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

/** Quiet terminal chrome: title + context on the left, compact controls on
 *  the right — no decorative icons, everything fits one slim row. */
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
	onClose?: () => void;
}) {
	return (
		<div className="flex h-12 items-center gap-3 border-b border-border/50 px-4">
			<div className="flex min-w-0 flex-1 items-baseline gap-2">
				<h2 className="shrink-0 text-[13px] font-semibold tracking-tight text-foreground">
					Terminal
				</h2>
				<span
					className="truncate font-mono text-xs text-muted-foreground"
					title={subtitle}>
					{displayName}
				</span>
			</div>
			<div className="flex shrink-0 items-center gap-1.5">
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
							'inline-flex h-7 shrink-0 cursor-pointer items-center rounded-lg px-2.5 text-[11px] font-medium transition-colors',
							hostMode
								? 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400'
								: 'text-muted-foreground hover:bg-muted hover:text-foreground',
						)}>
						Host shell
					</button>
				)}
				{!isRemoteServer &&
					!hostMode &&
					isCompose &&
					servicesList.length > 1 && (
						<Select
							value={targetContainer}
							onValueChange={val => onSelectService(val ?? '')}>
							<SelectTrigger className="h-7 min-h-7 w-[130px] rounded-lg px-2 text-[11px] font-medium shadow-none">
								<SelectValue placeholder="Service" />
							</SelectTrigger>
							<SelectContent>
								{servicesList.map(svc => (
									<SelectItem
										key={svc}
										value={svc}
										className="font-mono text-xs">
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
					<SelectTrigger className="h-7 min-h-7 w-[76px] rounded-lg px-2 font-mono text-[11px] shadow-none">
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
				{onClose && (
					<button
						type="button"
						onClick={onClose}
						aria-label="Close terminal"
						title="Close"
						className="inline-flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-muted hover:text-foreground">
						<X className="size-3.5" />
					</button>
				)}
			</div>
		</div>
	);
}
