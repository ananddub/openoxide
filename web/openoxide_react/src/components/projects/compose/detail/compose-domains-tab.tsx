import { useState, useMemo } from 'react';
import { Globe, Plus } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Badge } from '#/components/ui/badge';
import { toast } from 'sonner';
import { $api } from '#/api/query';
import { formatApiError } from '#/api/utils';
import { useBackupListVolumeBackups } from 'virtual:openoxide-live';
import { ComposeDomainsTable } from './domains/compose-domains-table';
import { ComposeDomainModal } from './domains/compose-domain-modal';

interface ComposeDomainsTabProps {
	composeId: number;
	compose?: any;
	domains?: any[];
	isLoading?: boolean;
}

// Extract service names defined under 'services:' in docker-compose.yml content
const extractServicesFromYaml = (yamlStr?: string): string[] => {
	if (!yamlStr) return [];
	const lines = yamlStr.split('\n');
	const services: string[] = [];
	let inServicesBlock = false;
	let servicesIndent = 0;

	for (const line of lines) {
		const trimmed = line.trimEnd();
		if (!trimmed || trimmed.trimStart().startsWith('#')) continue;

		const indent = line.search(/\S/);
		const text = trimmed.trim();

		if (text === 'services:' || text.startsWith('services:')) {
			inServicesBlock = true;
			servicesIndent = indent;
			continue;
		}

		if (inServicesBlock) {
			if (indent <= servicesIndent && text.endsWith(':') && !text.startsWith('-')) {
				inServicesBlock = false;
			} else if (indent > servicesIndent && text.endsWith(':') && !text.includes(' ') && !text.includes('.')) {
				const serviceName = text.slice(0, -1).trim();
				if (serviceName && !services.includes(serviceName)) {
					services.push(serviceName);
				}
			}
		}
	}
	return services;
};

export function ComposeDomainsTab({
	composeId,
	compose,
	domains: passedDomains,
	isLoading: passedIsLoading,
}: ComposeDomainsTabProps) {
	const [isModalOpen, setIsModalOpen] = useState(false);
	const [editingDomain, setEditingDomain] = useState<any | null>(null);

	// Extract list of compose service names
	const availableServices = useMemo(() => {
		return extractServicesFromYaml(compose?.compose_file);
	}, [compose?.compose_file]);

	const servicesList = availableServices.length > 0 ? availableServices : ['app'];

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
					params: { path: { id: editingDomain.id } },
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
			await deleteMutation.mutateAsync({ params: { path: { id } } });
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
		<div className="flex flex-col gap-6 w-full animate-in fade-in duration-200">
			{/* Top Header Toolbar */}
			<div className="flex items-center justify-between flex-wrap gap-4 border-b border-border/40 pb-4">
				<div className="flex flex-col gap-1">
					<h3 className="text-lg font-bold text-foreground tracking-tight flex items-center gap-2">
						<Globe className="size-4 text-primary" /> Compose Domains
					</h3>
					<p className="text-xs text-muted-foreground">
						Configure domain routes and SSL ingress rules for compose stack services
					</p>
				</div>
				<div className="flex items-center gap-3">
					<Badge variant="outline" className="text-xs font-mono px-3 py-1">
						Active Domains: {domains.length}
					</Badge>
					<Button onClick={handleOpenCreate} size="sm" className="h-8 text-xs font-bold flex items-center gap-1.5">
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
