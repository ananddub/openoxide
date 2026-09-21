import {RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';

/** One Setup & Audit row: icon, title, hint and a busy-aware action button. */
export function SetupActionRow({
	icon: Icon,
	title,
	description,
	busy,
	busyLabel,
	idleLabel,
	idleIcon,
	accent = false,
	onClick,
}: {
	icon: typeof RefreshCw;
	title: string;
	description: string;
	busy: boolean;
	busyLabel: string;
	idleLabel: string;
	idleIcon: typeof RefreshCw;
	/** Highlighted (primary-tinted) card variant. */
	accent?: boolean;
	onClick: () => void;
}) {
	const Idle = idleIcon;
	return (
		<div
			className={`flex items-center justify-between rounded-xl p-3.5 ${
				accent ? 'bg-primary/10' : 'bg-muted/20'
			}`}>
			<div className="flex items-center gap-2.5">
				<Icon className="h-5 w-5 text-primary" />
				<div>
					<h4 className="text-xs font-bold text-foreground">{title}</h4>
					<p className="text-[11px] text-muted-foreground">
						{description}
					</p>
				</div>
			</div>
			<Button
				variant={accent ? 'default' : 'outline'}
				size="sm"
				onClick={onClick}
				disabled={busy}
				className="h-8 gap-1.5 text-xs font-semibold">
				{busy ? (
					<RefreshCw className="h-3.5 w-3.5 animate-spin" />
				) : (
					<Idle className="h-3.5 w-3.5" />
				)}
				{busy ? busyLabel : idleLabel}
			</Button>
		</div>
	);
}
