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
		<div className="flex flex-col items-center justify-center border border-dashed border-border/80 rounded-2xl p-16 text-center bg-card/10 backdrop-blur-[2px] animate-in fade-in duration-200">
			<div className="p-4 rounded-full bg-muted/40 border border-border/30 text-muted-foreground mb-4">
				<FolderClosed className="size-8" />
			</div>
			<h3 className="text-xl font-bold text-foreground">No Projects Yet</h3>
			<p className="text-muted-foreground max-w-sm mt-1.5 text-sm leading-relaxed">
				Projects group your environments, applications and services. Create your
				first project to get started.
			</p>
			<Button
				onClick={onCreateClick}
				disabled={disabled}
				className="mt-6 bg-primary hover:bg-primary/95 text-primary-foreground font-semibold px-4 rounded-lg flex items-center gap-2 shadow-lg shadow-primary/5">
				<Plus className="size-4" />
				Create Project
			</Button>
		</div>
	);
};
