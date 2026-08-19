import {Plus, Layers} from 'lucide-react';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
} from '#/components/ui/select';

interface EnvDropdownProps {
	envs: any[];
	selectedId: number | null;
	onSelect: (id: number) => void;
	onCreateNew: () => void;
}

export function EnvDropdown({
	envs,
	selectedId,
	onSelect,
	onCreateNew,
}: EnvDropdownProps) {
	const selectedEnv = envs.find(e => Number(e.id) === Number(selectedId));
	const label = selectedEnv
		? selectedEnv.is_default
			? `${selectedEnv.name} (Default)`
			: selectedEnv.name
		: 'Select Environment';

	const hasSelectedEnv = envs.some(
		e => Number(e.id) === Number(selectedId),
	);
	const selectValue =
		hasSelectedEnv && selectedId !== null ? String(selectedId) : 'none';

	return (
		<div className="flex items-center gap-1">
			<Select
				value={selectValue}
				onValueChange={val => {
					if (val === 'create') {
						onCreateNew();
					} else if (val !== 'none') {
						onSelect(Number(val));
					}
				}}>
				<SelectTrigger className="flex h-7 cursor-pointer items-center gap-1.5 rounded-lg border border-border/50 bg-muted/30 px-2.5 py-1 text-xs font-semibold text-foreground shadow-sm transition-all hover:border-border/80 hover:bg-muted/60 focus:ring-0 focus-visible:ring-0 dark:border-border/40 dark:bg-muted/10 dark:hover:bg-muted/20">
					<Layers className="size-3 shrink-0 text-muted-foreground" />
					<span className="font-semibold text-foreground transition-colors hover:text-primary">
						{label}
					</span>
				</SelectTrigger>
				<SelectContent className="border-border bg-card">
					<div className="mb-1 border-b border-border/40 px-3 py-1.5 text-[9px] font-bold tracking-wider text-muted-foreground uppercase">
						Environments
					</div>
					{envs.map(env => (
						<SelectItem
							key={env.id}
							value={String(env.id)}
							className="text-xs">
							{env.is_default ? `${env.name} (Default)` : env.name}
						</SelectItem>
					))}
					<SelectItem
						value="create"
						className="text-xs font-medium text-primary focus:text-primary">
						<div className="flex items-center gap-1.5">
							<Plus className="size-3.5" />
							Create Environment
						</div>
					</SelectItem>
				</SelectContent>
			</Select>
		</div>
	);
}
