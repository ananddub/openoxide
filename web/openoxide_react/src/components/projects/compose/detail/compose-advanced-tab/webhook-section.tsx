import {useState} from 'react';
import {Copy, KeyRound, RefreshCw} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {SheetGroup, SheetNote} from '#/components/shared/field';
import {formatApiError} from '#/api/utils';
import {useComposeRotateWebhook} from '#/api/live';

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
		<SettingsSheet
			title="Deploy Webhook"
			description="Trigger auto-deploys from your git provider"
			icon={<KeyRound className="size-4" />}
			onOpenChange={next => {
				if (!next) setToken('');
			}}>
			<SheetGroup
				title="Webhook token"
				description="Git providers call this token to trigger auto-deploys.">
				{token ? (
					<div className="flex items-center gap-2">
						<code className="min-w-0 flex-1 truncate rounded-lg bg-muted/40 px-3 py-2 font-mono text-xs text-foreground">
							{token}
						</code>
						<Button
							variant="ghost"
							size="icon"
							title="Copy token"
							onClick={copy}
							className="size-9 shrink-0">
							<Copy className="size-4" />
						</Button>
						<Button
							variant="ghost"
							size="icon"
							title="Rotate again"
							disabled={busy}
							onClick={rotateToken}
							className="size-9 shrink-0">
							<RefreshCw className="size-4" />
						</Button>
					</div>
				) : (
					<>
						<SheetNote tone="warn">
							Rotating invalidates the previous secret. The token is
							shown once — copy it before closing.
						</SheetNote>
						<Button
							variant="outline"
							disabled={busy}
							onClick={rotateToken}
							className="h-9 w-fit gap-1.5 px-4 text-sm">
							<RefreshCw className="size-4" />
							{busy ? 'Rotating...' : 'Generate / rotate token'}
						</Button>
					</>
				)}
			</SheetGroup>
		</SettingsSheet>
	);
}
