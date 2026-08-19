import {useState, useEffect} from 'react';

export type ConnectionStatus = 'idle' | 'testing' | 'success' | 'failed';

class ServerConnectionStore {
	private listeners: Set<() => void> = new Set();
	private statusMap: Record<number, ConnectionStatus> = {};

	getStatus(serverId: number): ConnectionStatus {
		return this.statusMap[serverId] || 'idle';
	}

	setStatus(serverId: number, status: ConnectionStatus) {
		this.statusMap[serverId] = status;
		this.notify();
	}

	subscribe(listener: () => void) {
		this.listeners.add(listener);
		return () => {
			this.listeners.delete(listener);
		};
	}

	private notify() {
		this.listeners.forEach(listener => {
			try {
				listener();
			} catch (err) {
				console.error('Store listener error:', err);
			}
		});
	}
}

export const globalServerConnStore = new ServerConnectionStore();

export function useServerConnectionStatus(serverId: number) {
	const [status, setStatusState] = useState<ConnectionStatus>(() =>
		globalServerConnStore.getStatus(serverId),
	);

	useEffect(() => {
		setStatusState(globalServerConnStore.getStatus(serverId));
		const unsubscribe = globalServerConnStore.subscribe(() => {
			setStatusState(globalServerConnStore.getStatus(serverId));
		});
		return unsubscribe;
	}, [serverId]);

	const setStatus = (newStatus: ConnectionStatus) => {
		globalServerConnStore.setStatus(serverId, newStatus);
	};

	return {
		status,
		setStatus,
	};
}
