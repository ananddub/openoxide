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
	/** Omit on pages that can't create environments (e.g. service detail)
	 *  — hides the "Create Environment" option. */
	onCreateNew?: () => void;
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
						onCreateNew?.();
					} else if (val !== 'none') {
						onSelect(Number(val));
					}
				}}>
				<SelectTrigger className="h-9 w-auto cursor-pointer gap-1.5 rounded-lg bg-card px-3 text-sm transition-colors duration-150 hover:border-foreground/25 focus:ring-0 focus-visible:ring-0">
					<Layers className="size-3.5 shrink-0 text-muted-foreground" />
					<span className="text-foreground">{label}</span>
				</SelectTrigger>
				<SelectContent className="bg-card">
					<div className="mb-1 border-b px-3 py-1.5 text-[10px] font-semibold tracking-wider text-muted-foreground uppercase">
						Environments
					</div>
					{envs.map(env => (
						<SelectItem key={env.id} value={String(env.id)}>
							{env.is_default ? `${env.name} (Default)` : env.name}
						</SelectItem>
					))}
					{onCreateNew && (
						<SelectItem
							value="create"
							className="font-medium text-primary focus:text-primary">
							<div className="flex items-center gap-1.5">
								<Plus className="size-3.5" />
								New Environment
							</div>
						</SelectItem>
					)}
				</SelectContent>
			</Select>
		</div>
	);
}
