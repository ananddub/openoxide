export interface ComposeArchitectureTabProps {
	compose: any;
	domains?: any[];
	schedules?: any[];
	backups?: any[];
	onNavigateTab?: (tab: string) => void;
	onRefresh?: () => void;
}
