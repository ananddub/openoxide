import {load as yamlLoad} from 'js-yaml';

/** Compose service names from a compose file (used by callers too). */
export const extractServicesFromYaml = (yamlStr?: string): string[] => {
	if (!yamlStr || !yamlStr.trim()) return [];
	try {
		const doc = yamlLoad(yamlStr) as {services?: Record<string, unknown>};
		return Object.keys(doc?.services ?? {});
	} catch {}
	return [];
};

export type ShellStatus =
	| 'connecting'
	| 'connected'
	| 'error'
	| 'disconnected';

export interface ShellEntry {
	id: string;
	label: string;
	status: ShellStatus;
}

export interface TerminalApp {
	app_name?: string;
	appName?: string;
	name?: string;
	compose_file?: string;
	compose_status?: unknown;
	compose_type?: unknown;
	kind?: string;
	database_kind?: string;
	database_name?: string;
	database_password?: string;
	server_id?: number;
	serverId?: number;
	isRemoteServer?: boolean;
	id?: number;
}

export interface TerminalCardProps {
	/** Resource descriptor: container/app/compose payload, or a remote
	 *  server row with `{isRemoteServer: true, id}`. */
	app: TerminalApp;
	/** Only mount the PTY while true (keeps sessions from idling). */
	enabled: boolean;
	onClose?: () => void;
	className?: string;
	/** Open straight into the host root shell (server detail terminal). */
	hostModeDefault?: boolean;
}
