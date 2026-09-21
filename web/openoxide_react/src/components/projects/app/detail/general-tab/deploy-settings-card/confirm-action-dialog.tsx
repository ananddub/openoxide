import {X} from 'lucide-react';
import {Button} from '#/components/ui/button';

export function ConfirmActionDialog({
	title,
	description,
	destructive,
	onCancel,
	onConfirm,
}: {
	title: string;
	description: string;
	destructive: boolean;
	onCancel: () => void;
	onConfirm: () => void;
}) {
	return (
		<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm">
			<div className="flex w-full max-w-sm flex-col gap-4 rounded-xl bg-card p-5 shadow-lg">
				<div className="flex items-center justify-between border-b  pb-3">
					<h3 className="text-sm font-bold text-foreground">{title}</h3>
					<Button
						variant="ghost"
						size="sm"
						onClick={onCancel}
						className="h-7 w-7 p-0 text-muted-foreground">
						<X className="h-4 w-4" />
					</Button>
				</div>
				<p className="text-xs leading-relaxed text-muted-foreground">
					{description}
				</p>
				<div className="flex justify-end gap-2 border-t  pt-3">
					<Button
						variant="outline"
						size="sm"
						onClick={onCancel}
						className="h-8 text-xs font-semibold">
						Cancel
					</Button>
					<Button
						size="sm"
						variant={destructive ? 'destructive' : 'default'}
						onClick={onConfirm}
						className="h-8 text-xs font-semibold">
						Confirm
					</Button>
				</div>
			</div>
		</div>
	);
}
