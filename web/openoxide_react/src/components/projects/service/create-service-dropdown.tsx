import {Plus, Box, Layers2, Database} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
} from '#/components/ui/dropdown';

interface CreateServiceDropdownProps {
	onSelect: (type: 'application' | 'compose' | 'database') => void;
	disabled?: boolean;
}

export function CreateServiceDropdown({onSelect, disabled}: CreateServiceDropdownProps) {
	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<Button
						disabled={disabled}
						className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold h-8 px-3.5 rounded-lg flex items-center gap-1.5 shadow-lg shadow-primary/10 text-xs">
						<Plus className="size-3.5" />
						Create Service
					</Button>
				}
			/>
			<DropdownMenuContent align="end" className="w-48 border border-border bg-popover p-1.5">
				<DropdownMenuItem
					onClick={() => onSelect('application')}
					className="flex items-center gap-2.5 cursor-pointer text-xs p-2 rounded-md">
					<Box className="size-4 text-primary" />
					<div className="flex flex-col">
						<span className="font-semibold text-foreground">Application</span>
						<span className="text-[9px] text-muted-foreground">Deploy Git repo / Docker image</span>
					</div>
				</DropdownMenuItem>
				<DropdownMenuItem
					onClick={() => onSelect('compose')}
					className="flex items-center gap-2.5 cursor-pointer text-xs p-2 rounded-md">
					<Layers2 className="size-4 text-secondary-foreground" />
					<div className="flex flex-col">
						<span className="font-semibold text-foreground">Compose Stack</span>
						<span className="text-[9px] text-muted-foreground">Deploy multi-container app</span>
					</div>
				</DropdownMenuItem>
				<DropdownMenuItem
					onClick={() => onSelect('database')}
					className="flex items-center gap-2.5 cursor-pointer text-xs p-2 rounded-md">
					<Database className="size-4 text-foreground" />
					<div className="flex flex-col">
						<span className="font-semibold text-foreground">Database</span>
						<span className="text-[9px] text-muted-foreground">Deploy PostgreSQL, Redis, etc.</span>
					</div>
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}
