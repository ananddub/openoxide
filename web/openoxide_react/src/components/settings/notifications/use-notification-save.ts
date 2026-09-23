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
	const config: Record<string, unknown> = {};
	for (const f of provider?.fields ?? []) {
		const raw = (draft.config[f.key] ?? '').trim();
		if (!raw) {
			if (!f.optional) {
				toast.error(`${f.label} is required`);
				return null;
			}
			// A blank optional field is left out entirely — sending '' (or the
			// Number('') === 0 it coerces to) would overwrite a real default.
			continue;
		}
		if (f.type === 'number') config[f.key] = Number(raw);
		else if (f.key === 'to_addresses')
			config[f.key] = raw
				.split(',')
				.map(s => s.trim())
				.filter(Boolean);
		else config[f.key] = raw;
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
