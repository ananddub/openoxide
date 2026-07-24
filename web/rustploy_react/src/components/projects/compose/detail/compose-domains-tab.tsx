import {useState, useMemo} from 'react';
import {Globe, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {ComposeDomainModal} from './domains/compose-domain-modal';
import {ComposeDomainsTable} from './domains/compose-domains-table';

interface ComposeDomainsTabProps {
	composeId: number;
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

export function ComposeDomainsTab({composeId}: ComposeDomainsTabProps) {
	const [isModalOpen, setIsModalOpen] = useState(false);
	const [editingDomain, setEditingDomain] = useState<any | null>(null);

	// Fetch compose details query
	const {data: compose} = $api.useQuery('get', '/compose/{id}', {
		params: {path: {id: composeId}},
	});

	const availableServices = useMemo(() => {
		return extractServicesFromYaml(compose?.compose_file);
	}, [compose?.compose_file]);

	const servicesList = availableServices.length > 0 ? availableServices : ['app'];

	// Real-time domains query with safe array fallback
	const {data: rawDomains = [], isLoading, refetch} = $api.useQuery('get', '/domains', {
		params: {query: {compose_id: composeId}},
	});

	const domains = useMemo(() => (Array.isArray(rawDomains) ? rawDomains : []), [rawDomains]);

	// Mutations
	const createMutation = $api.useMutation('post', '/domains');
	const patchMutation = $api.useMutation('patch', '/domains/{id}');
	const deleteMutation = $api.useMutation('delete', '/domains/{id}');

	const handleSave = async (data: {
		domain: string;
		serviceName: string;
		containerPort: number;
		https: boolean;
		path: string;
	}) => {
		try {
			if (editingDomain) {
				await patchMutation.mutateAsync({
					params: {path: {id: editingDomain.id}},
					body: {
						domain: data.domain,
						service_name: data.serviceName,
						container_port: data.containerPort,
						https: data.https,
						path: data.path,
					} as any,
				});
				toast.success('Compose domain route updated');
			} else {
				await createMutation.mutateAsync({
					body: {
						domain: data.domain,
						compose_id: composeId,
						service_name: data.serviceName,
						container_port: data.containerPort,
						https: data.https,
						path: data.path,
					} as any,
				});
				toast.success('Compose domain route added');
			}
			refetch();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleDelete = async (id: number) => {
		try {
			await deleteMutation.mutateAsync({params: {path: {id}}});
			toast.success('Compose domain route deleted');
			refetch();
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
		<div className="flex flex-col gap-6">
			{/* Top Header Card */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between flex-wrap gap-4 shadow-sm">
				<div>
					<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
						<Globe className="w-4 h-4 text-primary" /> Compose Domains
					</h3>
					<p className="text-xs text-muted-foreground mt-1">Configure domain routes and SSL ingress rules for compose stack services</p>
				</div>
				<div className="flex items-center gap-3">
					<Badge variant="outline" className="text-xs font-mono px-3 py-1">
						Active Domains: {domains.length}
					</Badge>
					<Button onClick={handleOpenCreate} size="sm" className="h-8 text-xs font-semibold flex items-center gap-1.5">
						<Plus className="w-4 h-4" /> Add Compose Domain
					</Button>
				</div>
			</section>

			{/* Domains Table Component (< 200 lines) */}
			<ComposeDomainsTable
				domains={domains}
				isLoading={isLoading}
				onEdit={handleOpenEdit}
				onDelete={handleDelete}
			/>

			{/* Modal Component (< 200 lines) */}
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
