import {useState} from 'react';
import {Copy, KeyRound, RefreshCw} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {formatApiError} from '#/api/utils';
import {useComposeRotateWebhook} from '#/api/live';
import {AdvancedSection} from '#/components/projects/detail/advanced/shared';

export function WebhookSection({compose}: {compose: any}) {
	const rotate = useComposeRotateWebhook();
	const [token, setToken] = useState<string>('');
	const [busy, setBusy] = useState(false);

	const rotateToken = async () => {
		setBusy(true);
		try {
			const next = await rotate.mutateAsync(compose.id);
			setToken(next);
			toast.success('Webhook token rotated');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setBusy(false);
		}
	};

	const copy = async () => {
		if (!token) return;
		await navigator.clipboard.writeText(token);
		toast.success('Token copied');
	};

	return (
		<AdvancedSection
			icon={<KeyRound className="size-4 text-primary" />}
			title="Deploy Webhook"
			description="Git providers call this token to trigger auto-deploys. Rotating it invalidates the previous secret.">
			{token ? (
				<div className="flex items-center gap-2">
					<code className="min-w-0 flex-1 truncate rounded-lg bg-muted/40 px-3 py-2 font-mono text-xs text-foreground">
						{token}
					</code>
					<Button
						variant="outline"
						size="icon"
						title="Copy token"
						onClick={copy}
						className="size-8 rounded-lg">
						<Copy className="size-3.5" />
					</Button>
					<Button
						variant="outline"
						size="icon"
						title="Rotate again"
						disabled={busy}
						onClick={rotateToken}
						className="size-8 rounded-lg">
						<RefreshCw className="size-3.5" />
					</Button>
				</div>
			) : (
				<Button
					variant="outline"
					disabled={busy}
					onClick={rotateToken}
					className="flex h-9 w-fit items-center gap-1.5 rounded-lg text-xs font-semibold">
					<RefreshCw className="size-3.5" />
					{busy ? 'Rotating…' : 'Generate / Rotate Token'}
				</Button>
			)}
		</AdvancedSection>
	);
}

// ── Danger zone ────────────────────────────────────────────────────────
