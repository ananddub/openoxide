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

export function EnvDropdown({envs, selectedId, onSelect, onCreateNew}: EnvDropdownProps) {
	const selectedEnv = envs.find(e => e.id === selectedId);
	const label = selectedEnv
		? (selectedEnv.is_default ? `${selectedEnv.name} (Default)` : selectedEnv.name)
		: 'Select Environment';

	const hasSelectedEnv = envs.some(e => e.id === selectedId);
	const selectValue = hasSelectedEnv && selectedId ? String(selectedId) : 'none';

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
				<SelectTrigger className="bg-muted/30 border border-border/50 hover:bg-muted/60 hover:border-border/80 px-2.5 py-1 rounded-lg text-xs font-semibold text-foreground flex items-center gap-1.5 transition-all shadow-sm cursor-pointer h-7 focus-visible:ring-0 focus:ring-0 dark:bg-muted/10 dark:border-border/40 dark:hover:bg-muted/20">
					<Layers className="size-3 text-muted-foreground shrink-0" />
					<span className="text-foreground font-semibold hover:text-primary transition-colors">
						{label}
					</span>
				</SelectTrigger>
				<SelectContent className="bg-card border-border">
					<div className="px-3 py-1.5 text-[9px] font-bold uppercase tracking-wider text-muted-foreground border-b border-border/40 mb-1">
						Environments
					</div>
					{envs.map(env => (
						<SelectItem key={env.id} value={String(env.id)} className="text-xs">
							{env.is_default ? `${env.name} (Default)` : env.name}
						</SelectItem>
					))}
					<SelectItem value="create" className="text-xs text-primary font-medium focus:text-primary">
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
