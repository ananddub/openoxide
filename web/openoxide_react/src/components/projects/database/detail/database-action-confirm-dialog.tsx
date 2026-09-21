import {Button} from '#/components/ui/button';
import {X} from 'lucide-react';
import type {DatabaseActionType} from './database-deploy-settings-card';

const TITLES: Record<DatabaseActionType, string> = {
	deploy: 'Deploy Database',
	reload: 'Reload Database',
	start: 'Start Database',
	stop: 'Stop Database',
	cancel: 'Stop Database',
};

const DESCRIPTIONS: Record<DatabaseActionType, string> = {
	deploy:
		'Are you sure you want to deploy this database? This will provision/update the stack.',
	reload: 'Are you sure you want to reload this database service?',
	start: 'Are you sure you want to start this database container?',
	stop: 'Are you sure you want to stop this database container?',
	cancel: 'Are you sure you want to stop this database container?',
};

export function DatabaseActionConfirmDialog({
	action,
	onClose,
	onConfirm,
}: {
	action: DatabaseActionType;
	onClose: () => void;
	onConfirm: () => void;
}) {
	return (
		<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm">
			<div className="flex w-full max-w-sm animate-in flex-col gap-4 rounded-xl bg-card p-5 shadow-lg duration-150 zoom-in-95 fade-in">
				<div className="flex items-center justify-between border-b  pb-3">
					<h3 className="text-sm font-bold text-foreground">
						{TITLES[action]}
					</h3>
					<Button
						variant="ghost"
						size="sm"
						onClick={onClose}
						className="h-7 w-7 p-0 text-muted-foreground">
						<X className="h-4 w-4" />
					</Button>
				</div>
				<p className="text-xs leading-relaxed text-muted-foreground">
					{DESCRIPTIONS[action]}
				</p>
				<div className="flex justify-end gap-2 border-t  pt-3">
					<Button
						variant="outline"
						size="sm"
						onClick={onClose}
						className="h-8 text-xs font-semibold">
						Cancel
					</Button>
					<Button
						size="sm"
						variant={action === 'stop' ? 'destructive' : 'default'}
						onClick={onConfirm}
						className="h-8 text-xs font-semibold">
						Confirm
					</Button>
				</div>
			</div>
		</div>
	);
}
