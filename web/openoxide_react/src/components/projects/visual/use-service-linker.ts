import {
	useCallback,
	useState,
	type Dispatch,
	type SetStateAction,
} from 'react';
import {addEdge, type Connection, type Edge} from '@xyflow/react';

export type PendingConn = {
	sourceId: number;
	targetId: number;
	sourceName: string;
	targetName: string;
	targetDbType?: string;
};

/** App→DB connect detection: opens the linker modal when a wire lands on a DB. */
export function useServiceLinker(
	applications: any[],
	databases: any[],
	setEdges: Dispatch<SetStateAction<Edge[]>>,
) {
	const [pendingConn, setPendingConn] = useState<PendingConn | null>(null);

	const onConnect = useCallback(
		(conn: Connection) => {
			const srcId = parseInt(conn.source.replace(/^[a-z]+-/, ''));
			const tgtId = parseInt(conn.target.replace(/^[a-z]+-/, ''));
			const srcApp = applications.find((a: any) => a.id === srcId);
			const tgtDb = databases.find((d: any) => d.id === tgtId);
			if (srcApp && tgtDb)
				setPendingConn({
					sourceId: srcId,
					targetId: tgtId,
					sourceName: srcApp.name ?? srcApp.app_name,
					targetName: tgtDb.name ?? tgtDb.database_name,
					targetDbType: tgtDb.db_type,
				});
			setEdges(eds =>
				addEdge({...conn, type: 'customEdge', data: {}}, eds),
			);
		},
		[applications, databases, setEdges],
	);

	const clearPending = useCallback(() => setPendingConn(null), []);

	return {pendingConn, onConnect, clearPending};
}
