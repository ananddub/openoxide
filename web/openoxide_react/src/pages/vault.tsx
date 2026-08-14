import { createFileRoute } from '@tanstack/react-router';
import { VaultProvidersPage } from '#/components/vault/vault-providers-page';

export const Route = createFileRoute('/_app/vault')({
	component: VaultProvidersPage,
});
