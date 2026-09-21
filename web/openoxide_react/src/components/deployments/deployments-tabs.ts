import {Activity, Layers} from 'lucide-react';

export type DeploymentsTabId = 'history' | 'queue';

export const TABS: {
	id: DeploymentsTabId;
	label: string;
	icon: typeof Layers;
}[] = [
	{id: 'history', label: 'Logs & History', icon: Layers},
	{id: 'queue', label: 'Active Queue', icon: Activity},
];
