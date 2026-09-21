import {type ComponentType} from 'react';
import {type LucideProps} from 'lucide-react';

export interface ServiceCardProps {
	projectId: number;
	type: 'APP' | 'COMPOSE' | 'DATABASE';
	id: number;
	name: string;
	subtitle: string;
	status: string;
	createdAt: number;
	dbKind?: string;
	serverName?: string;
	onDeleted?: () => void;
}

export interface ServiceAction {
	id: string;
	label: string;
	Icon: ComponentType<LucideProps>;
	onClick: (e?: React.MouseEvent) => void;
	disabled?: boolean;
	destructive?: boolean;
}
