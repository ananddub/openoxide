import {
	DomainDialog,
	type DomainFormData,
} from '#/components/projects/common/domain-dialog';

interface ComposeDomainModalProps {
	isOpen: boolean;
	onClose: () => void;
	editingDomain: any | null;
	servicesList: string[];
	onSave: (data: DomainFormData) => Promise<void>;
}

export function ComposeDomainModal(props: ComposeDomainModalProps) {
	return (
		<DomainDialog
			isOpen={props.isOpen}
			onClose={props.onClose}
			editingDomain={props.editingDomain}
			services={props.servicesList}
			onSave={props.onSave}
		/>
	);
}
