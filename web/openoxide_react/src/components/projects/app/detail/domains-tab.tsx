import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {RefreshCw} from 'lucide-react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	DomainDialog,
	type DomainFormData,
} from '#/components/projects/common/domain-dialog';
import {Button} from '#/components/ui/button';
import {DomainsSection} from '#/components/projects/detail/domains/domains-section';
import {
	useDomainCreate,
	useDomainDelete,
	useDomainUpdate,
} from '#/api/live';

interface DomainsTabProps {
	app: any;
	domains?: any[];
	onRefresh?: () => void;
}

export function DomainsTab({
	app,
	domains: passedDomains,
	onRefresh,
}: DomainsTabProps) {
	const [dialogOpen, setDialogOpen] = useState(false);
	const [editingDomain, setEditingDomain] = useState<any | null>(null);
	const queryClient = useQueryClient();
	const domains = Array.isArray(passedDomains) ? passedDomains : [];
	const createDomain = useDomainCreate();
	const updateDomain = useDomainUpdate();
	const deleteDomain = useDomainDelete();

	const openCreate = () => {
		setEditingDomain(null);
		setDialogOpen(true);
	};
	const saveDomain = async (data: DomainFormData) => {
		try {
			if (editingDomain?.id)
				await updateDomain.mutateAsync({
					id: editingDomain.id,
					...data,
				} as any);
			else
				await createDomain.mutateAsync({
					...data,
					application_id: app.id,
				} as any);
			toast.success(
				editingDomain
					? 'Domain updated successfully'
					: 'Domain added successfully',
			);
			await queryClient.invalidateQueries();
			onRefresh?.();
		} catch (error) {
			throw new Error(formatApiError(error));
		}
	};

	const removeDomain = async (id: number) => {
		try {
			await deleteDomain.mutateAsync(id);
			toast.success('Domain deleted successfully');
			await queryClient.invalidateQueries();
			onRefresh?.();
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	return (
		<div className="flex w-full flex-col gap-4">
			<DomainsSection
				domains={domains}
				onCreate={openCreate}
				onEdit={domain => {
					setEditingDomain(domain);
					setDialogOpen(true);
				}}
				onDelete={removeDomain}
				description="Routes, paths and TLS certificates for this application."
				headerAction={
					<Button
						variant="ghost"
						size="icon"
						onClick={onRefresh}
						aria-label="Refresh domains"
						title="Refresh"
						className="size-9 text-muted-foreground hover:text-foreground">
						<RefreshCw className="size-4" />
					</Button>
				}
				previewTitle={app?.name ? `${app.name} preview` : 'Site preview'}
			/>

			<DomainDialog
				isOpen={dialogOpen}
				onClose={() => {
					setDialogOpen(false);
					setEditingDomain(null);
				}}
				editingDomain={editingDomain}
				onSave={saveDomain}
			/>
		</div>
	);
}
