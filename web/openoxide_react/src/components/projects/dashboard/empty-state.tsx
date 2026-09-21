import * as React from 'react';
import {FolderClosed, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';

type EmptyStateProps = {
	onCreateClick: () => void;
	disabled?: boolean;
};

export const EmptyState: React.FC<EmptyStateProps> = ({
	onCreateClick,
	disabled,
}) => {
	return (
		<div className="flex animate-in flex-col items-center justify-center rounded-xl border bg-card p-16 text-center shadow-[0_1px_1px_rgba(0,0,0,0.02),0_2px_2px_rgba(0,0,0,0.04)] duration-200 fade-in">
			<div className="mb-4 rounded-lg bg-foreground/[0.05] p-4 text-muted-foreground">
				<FolderClosed className="size-8" />
			</div>
			<h3 className="text-xl font-semibold tracking-[-0.01em] text-foreground">
				No Projects Yet
			</h3>
			<p className="mt-1.5 max-w-sm text-sm leading-relaxed text-muted-foreground">
				Projects group your environments, applications and services. Create
				your first project to get started.
			</p>
			<Button
				onClick={onCreateClick}
				disabled={disabled}
				className="mt-6 flex items-center gap-2">
				<Plus className="size-4" />
				Create Project
			</Button>
		</div>
	);
};
