import {Globe, Server, User} from 'lucide-react';
import {ConnectionRow} from './server-detail-parts';

/** Persistent right sidebar: connection summary shown on every tab. */
export function ServerConnectionCard({
	host,
	port,
	username,
	isHealthy,
}: {
	host: string;
	port: number | string;
	username: string;
	isHealthy: boolean;
}) {
	return (
		<div className="lg:sticky lg:top-6 lg:self-start">
			<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
				<div className="mb-4 flex items-center gap-2">
					<Server
						className="size-4 text-muted-foreground"
						strokeWidth={2}
					/>
					<h3 className="text-sm font-semibold text-foreground">
						Connection
					</h3>
				</div>
				<ConnectionRow
					icon={Globe}
					label="Host"
					value={
						<span className="flex items-center gap-2">
							<span
								className={`size-2 shrink-0 rounded-full ${isHealthy ? 'bg-emerald-500' : 'bg-rose-500'}`}
							/>
							<span className="font-mono text-[13px]">{host}</span>
						</span>
					}
				/>
				<ConnectionRow icon={Server} label="Port" value={port} mono />
				<ConnectionRow icon={User} label="User" value={username} mono />
			</div>
		</div>
	);
}
