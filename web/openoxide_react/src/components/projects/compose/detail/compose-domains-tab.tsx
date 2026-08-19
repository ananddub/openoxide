import {useState, useMemo, useEffect} from 'react';
import {Globe, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {ComposeDomainsTable} from './domains/compose-domains-table';
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
	const createMutation = $api.useMutation('post', '/domains');
	const updateMutation = $api.useMutation('put', '/domains/{id}');
	const deleteMutation = $api.useMutation('delete', '/domains/{id}');

	const domains = Array.isArray(passedDomains) ? passedDomains : [];
	const isLoading = passedIsLoading ?? false;

	const handleSave = async (formData: any) => {
		try {
			if (editingDomain?.id) {
				await updateMutation.mutateAsync({
					params: {path: {id: editingDomain.id}},
					body: {
						...formData,
						compose_id: composeId,
					} as any,
				});
				toast.success('Compose domain route updated');
			} else {
				await createMutation.mutateAsync({
					body: {
						...formData,
						compose_id: composeId,
					} as any,
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
			await deleteMutation.mutateAsync({params: {path: {id}}});
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
		<div className="flex w-full animate-in flex-col gap-6 duration-200 fade-in">
			{/* Top Header Toolbar */}
			<div className="flex flex-wrap items-center justify-between gap-4 border-b border-border/40 pb-4">
				<div className="flex flex-col gap-1">
					<h3 className="flex items-center gap-2 text-lg font-bold tracking-tight text-foreground">
						<Globe className="size-4 text-primary" /> Compose Domains
					</h3>
					<p className="text-xs text-muted-foreground">
						Configure domain routes and SSL ingress rules for compose stack
						services
					</p>
				</div>
				<div className="flex items-center gap-3">
					<Badge variant="outline" className="px-3 py-1 font-mono text-xs">
						Active Domains: {domains.length}
					</Badge>
					<Button
						onClick={handleOpenCreate}
						size="sm"
						className="flex h-8 items-center gap-1.5 text-xs font-bold">
						<Plus className="size-3.5" /> Add Compose Domain
					</Button>
				</div>
			</div>

			{/* Domains Table Component */}
			<ComposeDomainsTable
				domains={domains}
				isLoading={isLoading}
				onEdit={handleOpenEdit}
				onDelete={handleDelete}
			/>

			{/* Modal Component */}
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
