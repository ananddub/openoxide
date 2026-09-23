import {Container, Hammer} from 'lucide-react';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {cn} from '#/api/utils';

interface ComposeLogsHeaderProps {
	logMode: 'container' | 'build';
	setLogMode: (mode: 'container' | 'build') => void;
	activeService: string;
	setSelectedContainer: (v: string) => void;
	availableServices: string[];
}

/** Slim compose logs rail: mode toggle + service picker (container mode only). */
export function ComposeLogsHeader({
	logMode,
	setLogMode,
	activeService,
	setSelectedContainer,
	availableServices,
}: ComposeLogsHeaderProps) {
	return (
		<div className="flex flex-wrap items-center gap-2">
			<div className="flex items-center gap-1">
				{(
					[
						{key: 'container', label: 'Container', Icon: Container},
						{key: 'build', label: 'Build', Icon: Hammer},
					] as const
				).map(({key, label, Icon}) => (
					<button
						key={key}
						type="button"
						onClick={() => setLogMode(key)}
						className={cn(
							'flex cursor-pointer items-center gap-1.5 rounded-md px-2 py-1 text-xs font-medium transition-colors',
							logMode === key
								? 'bg-foreground/[0.06] text-foreground'
								: 'text-muted-foreground hover:bg-foreground/[0.04] hover:text-foreground',
						)}>
						<Icon className="size-3.5" />
						{label}
					</button>
				))}
			</div>

			{logMode === 'container' && availableServices.length > 1 && (
				<Select
					value={activeService}
					onValueChange={val => val && setSelectedContainer(val)}>
					<SelectTrigger className="h-8 w-auto min-w-[180px] bg-card font-mono text-xs">
						<SelectValue placeholder="Select service" />
					</SelectTrigger>
					<SelectContent className="bg-card">
						{availableServices.map(srv => (
							<SelectItem
								key={srv}
								value={srv}
								className="font-mono text-xs">
								{srv}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			)}
		</div>
	);
}
