import {createFileRoute} from '@tanstack/react-router';
import {Plus, Search, ShieldCheck} from 'lucide-react';
import {useMemo, useState} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {CertificateSheet} from '#/components/settings/certificates/certificate-sheet';
import {CertificateRenewSheet} from '#/components/settings/certificates/certificate-renew-sheet';
import {CertificateList} from '#/components/settings/certificates/certificate-list';
import {
	GridSkeleton,
	ListEmpty,
	ListRegion,
	ListSkeleton,
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
import {ConfirmDeleteDialog} from '#/components/settings/confirm-delete-dialog';
import {deleteCertificate} from '#/components/settings/certificates/certificate-api';
import type {
	Certificate,
	RemoteServer,
} from '#/components/settings/certificates/certificate-types';
import {useCertificateItems} from '#/api/live/certificates';
import {useRemoteServerList} from '#/api/live/servers';
import {useConnectMutation} from '#/api/live/query';

export const Route = createFileRoute('/_app/certificates')({
	component: CertificatesPage,
});

export function CertificatesPage() {
	const itemsQuery = useCertificateItems();
	const serversQuery = useRemoteServerList();
	const items = (itemsQuery.data ?? []) as unknown as Certificate[];
	const servers = (serversQuery.data ?? []) as unknown as RemoteServer[];
	const [dialogOpen, setDialogOpen] = useState(false);
	const [historyOpen, setHistoryOpen] = useState(false);
	const [selected, setSelected] = useState<Certificate | null>(null);
	const [deleting, setDeleting] = useState<Certificate | null>(null);
	const [search, setSearch] = useState('');
	const [view, setView] = useListView('settings-certificates-view');

	const q = search.trim().toLowerCase();
	const filtered = useMemo(
		() =>
			items.filter(item => {
				if (!q) return true;
				const server = servers.find(
					value => String(value.id) === item.server_id,
				);
				return [item.name, item.certificate_path, server?.name]
					.filter(Boolean)
					.join(' ')
					.toLowerCase()
					.includes(q);
			}),
		[items, servers, q],
	);

	// Cache invalidation comes from the table-watch stream (certificates table → refetch while this page is mounted); the mutation only needs error handling here.
	const removeMutation = useConnectMutation(
		async (certificate: Certificate) => {
			await deleteCertificate(Number(certificate.id));
			return certificate;
		},
		{
			onSuccess: certificate => {
				toast.success(`Certificate ${certificate.name} deleted`);
			},
			onError: (error: unknown) => {
				toast.error(
					error instanceof Error
						? error.message
						: 'Certificate could not be deleted',
				);
			},
		},
	);

	const create = () => {
		setSelected(null);
		setDialogOpen(true);
	};

	const edit = (certificate: Certificate) => {
		setSelected(certificate);
		setDialogOpen(true);
	};

	const history = (certificate: Certificate) => {
		setSelected(certificate);
		setHistoryOpen(true);
	};

	return (
		<div className="mx-auto flex h-full w-full max-w-[1920px] animate-in flex-col gap-5 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="Certificates"
				description="Install a PEM chain so Traefik serves HTTPS with your own certificate, on local or remote servers. Certificate and key are validated to match before they are written."
				action={
					<Button onClick={create} className="h-9 gap-1.5">
						<Plus className="size-3.5" />
						New certificate
					</Button>
				}
			/>

			<Toolbar action={<ViewToggle view={view} onChange={setView} />}>
				<ToolbarSearch
					value={search}
					onChange={setSearch}
					placeholder="Search by name, path or server…"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			<ListRegion>
				{itemsQuery.isLoading ? (
					view === 'grid' ? (
						<GridSkeleton />
					) : (
						<ListSkeleton />
					)
				) : items.length === 0 ? (
					<ListEmpty
						icon={ShieldCheck}
						title="No custom certificates"
						description="Add a PEM certificate chain and its private key to serve HTTPS for your own domains through Traefik."
						action={
							<Button
								onClick={create}
								className="h-9 gap-1.5 text-xs font-semibold">
								<Plus className="size-3.5" /> New certificate
							</Button>
						}
					/>
				) : filtered.length === 0 ? (
					<ListEmpty
						icon={Search}
						title="No certificates match your search"
						description="Try a different name, certificate path or target server."
						action={
							q !== '' && (
								<Button
									variant="ghost"
									onClick={() => setSearch('')}
									className="text-xs font-semibold text-primary">
									Clear Search
								</Button>
							)
						}
					/>
				) : (
					<CertificateList
						items={filtered}
						servers={servers}
						view={view}
						onEdit={edit}
						onHistory={history}
						onDelete={setDeleting}
					/>
				)}
			</ListRegion>

			<CertificateSheet
				certificate={selected}
				servers={servers}
				open={dialogOpen}
				onOpenChange={setDialogOpen}
				onSaved={async () => {
					/* table-watch invalidates ['certificates'] → refetch */
				}}
			/>
			<CertificateRenewSheet
				certificate={selected}
				open={historyOpen}
				onOpenChange={setHistoryOpen}
				onRenewed={async () => {
					/* table-watch invalidates ['certificates'] → refetch */
				}}
			/>
			<ConfirmDeleteDialog
				open={deleting !== null}
				onOpenChange={o => !o && setDeleting(null)}
				title="Delete certificate"
				description="This removes the certificate files from Traefik and can break HTTPS for domains that use them."
				confirmLabel="Delete certificate"
				busy={removeMutation.isPending}
				onConfirm={() => {
					if (deleting) void removeMutation.mutateAsync(deleting);
					setDeleting(null);
				}}
			/>
		</div>
	);
}
