import {useState, useMemo, useEffect} from 'react';
import {DomainsSection} from '#/components/projects/detail/domains/domains-section';
import {toast} from 'sonner';
import {
	useDomainCreate,
	useDomainDelete,
	useDomainUpdate,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {ComposeDomainModal} from './domains/compose-domain-modal';
import {
	buildRawGitUrl,
	getComposeServiceNames,
} from '#/utils/compose-services';

interface ComposeDomainsTabProps {
	composeId: number;
	compose?: any;
	domains?: any[];
	isLoading?: boolean;
}

export function ComposeDomainsTab({
	composeId,
	compose,
	domains: passedDomains,
	isLoading: passedIsLoading,
}: ComposeDomainsTabProps) {
	const [isModalOpen, setIsModalOpen] = useState(false);
	const [editingDomain, setEditingDomain] = useState<any | null>(null);
	const [fetchedYaml, setFetchedYaml] = useState<string>('');

	useEffect(() => {
		if (compose?.compose_file && compose.compose_file.trim()) {
			setFetchedYaml(compose.compose_file);
			return;
		}
		const rawUrl = buildRawGitUrl(compose);
		if (rawUrl) {
			let isMounted = true;
			fetch(rawUrl)
				.then(res => (res.ok ? res.text() : ''))
				.then(text => {
					if (isMounted && text && text.trim()) {
						setFetchedYaml(text);
					}
				})
				.catch(() => {});
			return () => {
				isMounted = false;
			};
		}
	}, [compose]);

	// Extract list of compose service names
	const servicesList = useMemo(() => {
		return getComposeServiceNames(compose, fetchedYaml);
	}, [compose, fetchedYaml]);

	// Mutations
	const createMutation = useDomainCreate();
	const updateMutation = useDomainUpdate();
	const deleteMutation = useDomainDelete();

	const domains = Array.isArray(passedDomains) ? passedDomains : [];
	const isLoading = passedIsLoading ?? false;

	const handleSave = async (formData: any) => {
		try {
			if (editingDomain?.id) {
				await updateMutation.mutateAsync({
					id: editingDomain.id,
					...formData,
				});
				toast.success('Compose domain route updated');
			} else {
				await createMutation.mutateAsync({
					...formData,
					compose_id: composeId,
				});
				toast.success('Compose domain route created');
			}
			setIsModalOpen(false);
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleDelete = async (id: number) => {
		try {
			await deleteMutation.mutateAsync(id);
			toast.success('Compose domain route deleted');
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleOpenCreate = () => {
		setEditingDomain(null);
		setIsModalOpen(true);
	};

	const handleOpenEdit = (domain: any) => {
		setEditingDomain(domain);
		setIsModalOpen(true);
	};

	return (
		<div className="flex w-full animate-in flex-col gap-4 duration-200 fade-in">
			{/* Shared domains surface (header + search + TLS filter + grid/list) */}
			<DomainsSection
				domains={domains}
				isLoading={isLoading}
				onCreate={handleOpenCreate}
				onEdit={handleOpenEdit}
				onDelete={handleDelete}
				description="Domain routes and TLS ingress rules for this compose stack."
				previewTitle="Compose domain preview"
			/>

			{/* Domain route form — right sheet */}
			<ComposeDomainModal
				isOpen={isModalOpen}
				onClose={() => setIsModalOpen(false)}
				editingDomain={editingDomain}
				servicesList={servicesList}
				onSave={handleSave}
			/>
		</div>
	);
}
