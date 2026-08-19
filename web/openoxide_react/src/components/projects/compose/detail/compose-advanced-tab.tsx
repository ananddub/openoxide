import {Settings2} from 'lucide-react';

interface ComposeAdvancedTabProps {
	compose: any;
}

export function ComposeAdvancedTab({compose}: ComposeAdvancedTabProps) {
	return (
		<div className="flex min-h-[300px] flex-col items-center justify-center gap-3 rounded-xl border border-border bg-card p-8 text-center shadow-sm">
			<div className="flex h-12 w-12 items-center justify-center rounded-xl border border-border/50 bg-secondary text-muted-foreground">
				<Settings2 className="h-6 w-6 opacity-60" />
			</div>
			<div>
				<h3 className="text-sm font-bold text-foreground">
					Advanced Settings
				</h3>
				<p className="mt-1 max-w-sm text-xs text-muted-foreground">
					No advanced configurations currently set for compose stack '
					{compose?.app_name || compose?.name || 'project'}'.
				</p>
			</div>
		</div>
	);
}
