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
		<div className="flex animate-in flex-col items-center justify-center rounded-2xl border border-dashed border-border/80 bg-card/10 p-16 text-center backdrop-blur-[2px] duration-200 fade-in">
			<div className="mb-4 rounded-full border border-border/30 bg-muted/40 p-4 text-muted-foreground">
				<FolderClosed className="size-8" />
			</div>
			<h3 className="text-xl font-bold text-foreground">
				No Projects Yet
			</h3>
			<p className="mt-1.5 max-w-sm text-sm leading-relaxed text-muted-foreground">
				Projects group your environments, applications and services. Create
				your first project to get started.
			</p>
			<Button
				onClick={onCreateClick}
				disabled={disabled}
				className="mt-6 flex items-center gap-2 rounded-lg bg-primary px-4 font-semibold text-primary-foreground shadow-lg shadow-primary/5 hover:bg-primary/95">
				<Plus className="size-4" />
				Create Project
			</Button>
		</div>
	);
};
