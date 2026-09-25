import {useMemo, useState} from 'react';
import {Plus} from 'lucide-react';
import {cn} from '#/api/utils';
import {Dialog, DialogContent} from '#/components/ui/dialog';
import {TerminalHeader} from './terminal/terminal-header';
import {ShellTabStrip} from './terminal/shell-tab-strip';
import {ShellSession} from './terminal/shell-session';
import {useShellTabs} from './terminal/use-shell-tabs';
import {
	type TerminalCardProps,
	extractServicesFromYaml,
} from './terminal/types';

export {extractServicesFromYaml} from './terminal/types';

/**
 * OpenShip's terminal surface (terminal-tab + ServerTerminalTabs pattern):
 * an icon-tile header, a shell tab strip, and xterm bodies. All tabs stay
 * mounted — hidden ones keep their stream alive so scrollback keeps flowing
 * (VSCode "tabs run in background"). Render pieces live in ./terminal.
 *
 * By default it presents as a centered overlay modal (`asModal`). Callers
 * with a persistent in-page tab pass `asModal={false}` to keep the inline
 * card. The "expand" control maximizes within whichever surface hosts it.
 */
export function TerminalCard({
	app,
	enabled,
	onClose,
	className,
	hostModeDefault = false,
	asModal = true,
}: TerminalCardProps) {
	const [shell, setShell] = useState<'sh' | 'bash'>('bash');
	const [selectedService, setSelectedService] = useState('');
	const [hostMode, setHostMode] = useState(hostModeDefault);

	const {shells, activeId, setActiveId, setStatus, addShell, closeShell} =
		useShellTabs();

	const availableServices = useMemo(
		() => extractServicesFromYaml(app?.compose_file),
		[app?.compose_file],
	);
	const isCompose =
		app?.compose_status !== undefined ||
		app?.compose_type !== undefined ||
		app?.compose_file !== undefined;

	const defaultContainer = useMemo(() => {
		if (availableServices.length > 0) return availableServices[0];
		if (isCompose) return 'app';
		const name = app?.app_name || app?.appName || app?.name || 'app';
		const isDatabase = Boolean(
			app?.kind ||
			app?.database_kind ||
			app?.database_name ||
			app?.database_password,
		);
		return isDatabase && !name.endsWith('_db') ? `${name}_db` : name;
	}, [availableServices, isCompose, app]);

	const targetContainer = selectedService || defaultContainer;
	const servicesList =
		availableServices.length > 0 ? availableServices : ['app'];
	const isRemoteServer = Boolean(app?.isRemoteServer);
	const serverId =
		app?.server_id ||
		app?.serverId ||
		(isRemoteServer ? app?.id : undefined);

	const displayName =
		app?.name || app?.app_name || app?.appName || 'shell';
	const subtitle = isRemoteServer
		? `Interactive SSH shell on ${targetContainer}.`
		: hostMode
			? `Host root shell via the Agent on ${targetContainer}.`
			: `Container shell for ${targetContainer}.`;

	const body = (
		<>
			<TerminalHeader
				displayName={displayName}
				subtitle={subtitle}
				isRemoteServer={isRemoteServer}
				hostMode={hostMode}
				onToggleHost={() => setHostMode(v => !v)}
				isCompose={isCompose}
				servicesList={servicesList}
				targetContainer={targetContainer}
				onSelectService={setSelectedService}
				shell={shell}
				onShellChange={setShell}
				onClose={onClose}
			/>

			<ShellTabStrip
				shells={shells}
				activeId={activeId}
				onSelect={setActiveId}
				onClose={closeShell}
				onAdd={addShell}
			/>

			<div
				className={cn(
					'relative min-h-0 flex-1',
					!asModal && 'h-[60vh] min-h-[480px]',
				)}>
				{shells.length === 0 && (
					<div className="absolute inset-0 flex flex-col items-center justify-center gap-3 text-center">
						<p className="text-sm text-muted-foreground/70">
							No terminals
						</p>
						<button
							type="button"
							onClick={addShell}
							className="inline-flex cursor-pointer items-center gap-1.5 rounded-xl border border-border/50 bg-muted/30 px-4 py-2 text-[13px] font-medium text-foreground transition-colors hover:bg-muted/60">
							<Plus className="size-3.5" />
							New Shell
						</button>
					</div>
				)}
				{shells.map(entry => (
					<div
						key={entry.id}
						className={
							entry.id === activeId ? 'absolute inset-0' : 'hidden'
						}>
						<ShellSession
							visible={entry.id === activeId}
							enabled={enabled}
							targetContainer={targetContainer}
							shell={shell}
							isRemoteServer={isRemoteServer}
							hostMode={hostMode}
							serverId={serverId}
							onStatus={status => setStatus(entry.id, status)}
						/>
					</div>
				))}
			</div>
		</>
	);

	if (asModal) {
		return (
			<Dialog open onOpenChange={open => !open && onClose?.()}>
				<DialogContent
					showCloseButton={false}
					className="w-[92vw] max-w-5xl gap-0 overflow-hidden rounded-xl border-border/60 bg-card p-0! ring-1 ring-black/[0.04] sm:max-w-5xl dark:ring-white/[0.06]">
					<div className="flex h-[70vh] max-h-[calc(100dvh-4rem)] w-full flex-col overflow-hidden bg-card">
						{body}
					</div>
				</DialogContent>
			</Dialog>
		);
	}

	return (
		<div className={className}>
			<div className="flex w-full flex-col overflow-hidden rounded-xl bg-card ring-1 ring-border/50">
				{body}
			</div>
		</div>
	);
}
