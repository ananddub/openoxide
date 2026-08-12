import {Settings2} from 'lucide-react';

interface ComposeAdvancedTabProps {
	compose: any;
}

export function ComposeAdvancedTab({compose}: ComposeAdvancedTabProps) {
	return (
		<div className="bg-card border border-border rounded-xl p-8 flex flex-col items-center justify-center text-center gap-3 min-h-[300px] shadow-sm">
			<div className="w-12 h-12 rounded-xl bg-secondary flex items-center justify-center text-muted-foreground border border-border/50">
				<Settings2 className="w-6 h-6 opacity-60" />
			</div>
			<div>
				<h3 className="text-sm font-bold text-foreground">Advanced Settings</h3>
				<p className="text-xs text-muted-foreground mt-1 max-w-sm">
					No advanced configurations currently set for compose stack '{compose?.app_name || compose?.name || 'project'}'.
				</p>
			</div>
		</div>
	);
}
