import {toast} from 'sonner';
import {
	useNotificationCreate,
	useNotificationUpdate,
	type NotificationPayload,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import type {ProviderMeta} from './notification-providers';
import type {DraftState} from './notification-triggers';

/** Validate required provider fields and coerce numbers/address lists. */
function buildConfig(draft: DraftState, provider: ProviderMeta | null) {
	for (const f of provider?.fields ?? []) {
		if (!f.optional && !(draft.config[f.key] ?? '').trim()) {
			toast.error(`${f.label} is required`);
			return null;
		}
	}
	const config: Record<string, unknown> = {...draft.config};
	for (const f of provider?.fields ?? []) {
		if (f.type === 'number' && config[f.key] !== undefined) {
			config[f.key] = Number(config[f.key]);
		}
		if (f.key === 'to_addresses' && typeof config[f.key] === 'string') {
			config[f.key] = (config[f.key] as string)
				.split(',')
				.map(s => s.trim())
				.filter(Boolean);
		}
	}
	return config;
}

/** Create/update flow for the notification channel dialog. */
export function useNotificationSave({
	draft,
	editing,
	provider,
	orgId,
	onSaved,
}: {
	draft: DraftState | null;
	editing: NotificationPayload | null;
	provider: ProviderMeta | null;
	orgId: bigint;
	onSaved: () => void;
}) {
	const createMutation = useNotificationCreate();
	const updateMutation = useNotificationUpdate();

	return async () => {
		if (!draft) return;
		if (!draft.name.trim()) {
			toast.error('Name is required');
			return;
		}
		try {
			if (editing) {
				// Config edits need the provider fields re-supplied; the patch
				// covers name + triggers, which is what the edit dialog exposes.
				await updateMutation.mutateAsync({
					id: editing.id,
					name: draft.name.trim(),
					...draft.triggers,
				});
				toast.success('Channel updated');
			} else {
				const config = buildConfig(draft, provider);
				if (!config) return;
				await createMutation.mutateAsync({
					settings: {
						name: draft.name.trim(),
						organization_id: Number(orgId),
						...draft.triggers,
					},
					provider: {
						notification_type: draft.provider,
						config,
					},
				});
				toast.success('Channel created');
			}
			onSaved();
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};
}
