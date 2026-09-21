import {Rocket, RefreshCw, Hammer} from 'lucide-react';
import {Button} from '#/components/ui/button';
import type {ActionType} from './actions';

function ActionButton({
	action,
	label,
	loadingLabel,
	activeLoading,
	variant,
	icon,
	disabled,
	onExecute,
}: {
	action: ActionType;
	label: string;
	loadingLabel: string;
	activeLoading: string | null;
	variant?: 'outline';
	icon: React.ReactNode;
	disabled: boolean;
	onExecute: (action: ActionType) => void;
}) {
	const busy = activeLoading === action;
	return (
		<Button
			onClick={() => onExecute(action)}
			disabled={disabled}
			variant={variant}
			size="sm"
			className={`flex h-9 cursor-pointer items-center gap-1.5 rounded-lg font-semibold disabled:cursor-not-allowed disabled:opacity-50 ${
				variant === 'outline'
					? 'text-foreground hover:bg-muted'
					: 'bg-primary text-primary-foreground hover:bg-primary/90'
			}`}>
			{busy ? <RefreshCw className="h-4 w-4 animate-spin" /> : icon}
			{busy ? loadingLabel : label}
		</Button>
	);
}

/** Deploy / Reload / Rebuild button row. */
export function PrimaryActionButtons({
	isBuilding,
	isProcessing,
	activeLoading,
	onExecute,
}: {
	isBuilding: boolean;
	isProcessing: boolean;
	activeLoading: string | null;
	onExecute: (action: ActionType) => void;
}) {
	const disabled = isProcessing || isBuilding;
	return (
		<>
			<ActionButton
				action="deploy"
				label="Deploy"
				loadingLabel="Deploying..."
				activeLoading={activeLoading}
				icon={<Rocket className="h-4 w-4" />}
				disabled={disabled}
				onExecute={onExecute}
			/>
			<ActionButton
				action="reload"
				label="Reload"
				loadingLabel="Reloading..."
				activeLoading={activeLoading}
				variant="outline"
				icon={<RefreshCw className="h-4 w-4" />}
				disabled={disabled}
				onExecute={onExecute}
			/>
			<ActionButton
				action="rebuild"
				label="Rebuild"
				loadingLabel="Rebuilding..."
				activeLoading={activeLoading}
				variant="outline"
				icon={<Hammer className="h-4 w-4" />}
				disabled={disabled}
				onExecute={onExecute}
			/>
		</>
	);
}
