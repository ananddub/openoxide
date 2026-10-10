import {Activity, KeyRound, RefreshCw, Wrench} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {privateNetworkApi} from './api';
import type {ConnectionMode} from './types';

type Run = (
	name: string,
	operation: () => Promise<unknown>,
	message: string,
) => void;

/** Tunnel maintenance actions, shown once a network is configured. */
const OPS: Array<{
	key: string;
	label: string;
	icon: typeof Activity;
	managedOnly?: boolean;
	message: string;
	call: (id: number) => Promise<unknown>;
}> = [
	{
		key: 'health',
		label: 'Health',
		icon: Activity,
		message: 'Health check completed',
		call: id => privateNetworkApi.health(id),
	},
	{
		key: 'repair',
		label: 'Repair',
		icon: Wrench,
		message: 'Private network repaired',
		call: id => privateNetworkApi.repair(id),
	},
	{
		key: 're-setup',
		label: 'Re-setup',
		icon: RefreshCw,
		managedOnly: true,
		message: 'WireGuard re-setup completed',
		call: id => privateNetworkApi.reSetup(id),
	},
	{
		key: 'rotate',
		label: 'Rotate keys',
		icon: KeyRound,
		managedOnly: true,
		message: 'WireGuard keys rotated',
		call: id => privateNetworkApi.rotateKeys(id),
	},
];

type ActionProps = {
	serverId: number;
	hasConfig: boolean;
	mode: ConnectionMode;
	busy: boolean;
	onRun: Run;
	onSave: () => void;
};

/** The maintenance + save buttons themselves, chrome-free — the parent's flex
 *  row must exist (Save sits at the right via `ml-auto`). */
export function NetworkActionButtons({
	serverId,
	hasConfig,
	mode,
	busy,
	onRun,
	onSave,
}: ActionProps) {
	const managed = mode === 'MANAGED_WIREGUARD';
	return (
		<>
			{hasConfig &&
				mode !== 'DIRECT_SSH' &&
				OPS.filter(op => !op.managedOnly || managed).map(op => (
					<Button
						key={op.key}
						variant="outline"
						size="sm"
						disabled={busy}
						onClick={() =>
							onRun(op.key, () => op.call(serverId), op.message)
						}
						className="cursor-pointer gap-1.5 text-xs">
						<op.icon className="size-3.5" /> {op.label}
					</Button>
				))}
			<Button
				size="sm"
				onClick={onSave}
				disabled={busy}
				className="ml-auto cursor-pointer gap-1.5 text-xs">
				{busy && <RefreshCw className="size-3.5 animate-spin" />}
				{managed ? 'Save & Setup' : 'Save'}
			</Button>
		</>
	);
}

/** Card footer: maintenance actions + save / save & setup. */
export function PanelActions(props: ActionProps) {
	return (
		<div className="flex flex-wrap items-center gap-2 border-t px-5 py-3.5">
			<NetworkActionButtons {...props} />
		</div>
	);
}
