import {Bot, Loader2, Pencil, Plus, Trash2} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {Card} from '#/components/ui/card';
import type {AiSetting} from './ai-types';

type Props = {
	items: AiSetting[];
	loading: boolean;
	onCreate: () => void;
	onEdit: (item: AiSetting) => void;
	onDelete: (id: number) => void;
};

export function AiSettingsList({
	items,
	loading,
	onCreate,
	onEdit,
	onDelete,
}: Props) {
	if (loading)
		return (
			<div className="flex justify-center py-12">
				<Loader2 className="size-5 animate-spin" />
			</div>
		);
	if (items.length === 0)
		return (
			<div className="flex flex-col items-center gap-3 py-12 text-center text-sm text-muted-foreground">
				<Bot className="size-8" /> No AI configurations yet.
				<Button variant="outline" onClick={onCreate}>
					<Plus className="mr-2 size-4" /> Configure AI
				</Button>
			</div>
		);
	return (
		<div className="space-y-3">
			{items.map(item => (
				<Card
					key={item.id}
					className="flex items-center justify-between p-4">
					<div>
						<div className="flex items-center gap-2">
							<span className="font-semibold">{item.name}</span>
							<Badge variant={item.is_enabled ? 'secondary' : 'outline'}>
								{item.is_enabled ? 'Enabled' : 'Disabled'}
							</Badge>
							<Badge variant="outline">{item.provider}</Badge>
						</div>
						<p className="mt-1 text-xs text-muted-foreground">
							{item.model} · {item.api_url}
						</p>
					</div>
					<div className="flex gap-1">
						<Button
							variant="ghost"
							size="icon"
							title="Edit"
							onClick={() => onEdit(item)}>
							<Pencil className="size-4" />
						</Button>
						<Button
							variant="ghost"
							size="icon"
							title="Delete"
							onClick={() => onDelete(item.id)}>
							<Trash2 className="size-4 text-destructive" />
						</Button>
					</div>
				</Card>
			))}
		</div>
	);
}
