import {CheckCircle2, Plug, RefreshCw, XCircle} from 'lucide-react';
import {Button} from '#/components/ui/button';

export type TestResult = 'idle' | 'success' | 'failed';

const STATES: Record<
	TestResult | 'testing',
	{title: string; cls: string; icon: typeof Plug; iconCls: string}
> = {
	testing: {
		title: 'Testing SSH Connection...',
		cls: ' hover:bg-muted/80',
		icon: RefreshCw,
		iconCls: 'animate-spin text-primary',
	},
	success: {
		title: 'SSH Connection Verified',
		cls: 'border-emerald-500/50 bg-emerald-500/10 text-emerald-500 hover:bg-emerald-500/20',
		icon: CheckCircle2,
		iconCls: 'text-emerald-500',
	},
	failed: {
		title: 'SSH Connection Failed',
		cls: 'border-rose-500/50 bg-rose-500/10 text-rose-500 hover:bg-rose-500/20',
		icon: XCircle,
		iconCls: 'text-rose-500',
	},
	idle: {
		title: 'Test SSH Connection',
		cls: ' hover:bg-muted/80',
		icon: Plug,
		iconCls: 'text-muted-foreground hover:text-foreground',
	},
};

/** Icon-only "Test SSH Connection" button with per-result styling. */
export function TestConnectionButton({
	testing,
	result,
	disabled,
	onClick,
}: {
	testing: boolean;
	result: TestResult;
	disabled: boolean;
	onClick: () => void;
}) {
	const state = STATES[testing ? 'testing' : result];
	const Icon = state.icon;
	return (
		<Button
			type="button"
			variant="outline"
			size="icon"
			onClick={onClick}
			disabled={disabled}
			title={state.title}
			className={`h-9 w-9 rounded-lg transition-all ${state.cls}`}>
			<Icon className={`h-4 w-4 ${state.iconCls}`} />
		</Button>
	);
}
