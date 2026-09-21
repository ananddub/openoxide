import {
	AlertCircle,
	AlertTriangle,
	CheckCircle2,
	Info,
} from 'lucide-react';
import {cn} from '#/api/utils';

interface Props extends React.ComponentPropsWithoutRef<'div'> {
	icon?: React.ReactNode;
	type?: 'info' | 'success' | 'warning' | 'error';
}

const iconMap = {
	info: {
		className: 'bg-info-bg text-info',
		icon: Info,
	},
	success: {
		className: 'bg-success-bg text-success',
		icon: CheckCircle2,
	},
	warning: {
		className: 'bg-warning-bg text-warning',
		icon: AlertCircle,
	},
	error: {
		className: 'bg-danger-bg text-danger',
		icon: AlertTriangle,
	},
};

export function AlertBlock({
	type = 'info',
	icon,
	children,
	className,
	...props
}: Props) {
	const {className: iconClassName, icon: Icon} = iconMap[type];
	return (
		<div
			{...props}
			className={cn(
				'flex flex-row items-start gap-4 rounded-lg p-2',
				iconClassName,
				className,
			)}>
			<div className="mt-0.5 shrink-0">
				{icon || <Icon className="text-current" />}
			</div>
			<div className="min-w-0 flex-1">
				<span className="overflow-wrap-anywhere text-sm wrap-break-word whitespace-pre-wrap text-current">
					{children}
				</span>
			</div>
		</div>
	);
}
