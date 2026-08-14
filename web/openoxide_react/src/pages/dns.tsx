import { createFileRoute } from '@tanstack/react-router';
import { DnsProvidersPage } from '#/components/dns/dns-providers-page';

export const Route = createFileRoute('/_app/dns')({
	component: DnsProvidersPage,
});
