import {useState} from 'react';
import {Copy, KeyRound, Loader2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {formatApiError} from '#/api/utils';
import {useCreateAgentToken} from '#/api/live/monitoring';

interface AgentTokenPanelProps {
	canAct: boolean;
	serverId: bigint;
}

/**
 * Agent credentials. The panel keeps only a hash, so the plaintext exists for
 * the instant after a rotation and nowhere else — which is why this card offers
 * no "reveal" button and keeps nothing on screen once dismissed.
 */
export function AgentTokenPanel({canAct, serverId}: AgentTokenPanelProps) {
	const rotate = useCreateAgentToken();
	const [token, setToken] = useState<string | null>(null);

	const run = () =>
		rotate.mutate(serverId, {
			onSuccess: value => {
				setToken(value);
				toast.success('Agent token rotated');
			},
			onError: error => toast.error(formatApiError(error)),
		});

	const copy = () => {
		if (!token) return;
		void navigator.clipboard.writeText(token);
		toast.success('Agent token copied');
	};

	return (
		<div className="flex flex-col gap-3 border-t border-border/50 p-4">
			<div className="flex flex-wrap items-center gap-2">
				<span className="text-xs font-semibold text-foreground">
					Agent token
				</span>
				<Button
					size="sm"
					variant="outline"
					disabled={!canAct || rotate.isPending}
					onClick={run}
					className="h-8 cursor-pointer gap-1.5 text-xs disabled:cursor-default">
					{rotate.isPending ? (
						<Loader2 className="size-3.5 animate-spin" />
					) : (
						<KeyRound className="size-3.5" />
					)}
					{rotate.isPending ? 'Rotating…' : 'Rotate token'}
				</Button>
				{token && (
					<Button
						size="sm"
						variant="ghost"
						onClick={copy}
						className="h-8 cursor-pointer gap-1.5 text-xs">
						<Copy className="size-3.5" />
						Copy
					</Button>
				)}
			</div>
			{token ? (
				<>
					<pre className="scrollbar-thin scrollbar-thumb-border/40 overflow-x-auto rounded-lg bg-muted/40 p-3 font-mono text-[11px] break-all">
						{token}
					</pre>
					<p className="text-[11px] leading-snug text-muted-foreground">
						Starting the agent with this value in its{' '}
						<code>METRICS_TOKEN</code> environment replaces the previous
						credential immediately — the panel authenticates each push with{' '}
						<code>x-metrics-token</code>, so an agent still running the old
						token is rejected and shows offline. This text is not stored
						anywhere and cannot be shown again.
					</p>
				</>
			) : (
				<p className="text-[11px] leading-snug text-muted-foreground">
					The credential this server&apos;s agent authenticates with.
					Rotate it after a host is compromised or its compose file was
					world-readable; then recreate the agent container with the new{' '}
					<code>METRICS_TOKEN</code>.
				</p>
			)}
		</div>
	);
}
