import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

function RailpackSelect({
	version,
	versions,
	onVersionChange,
	onManualChange,
}: {
	version: string;
	versions: readonly string[];
	onVersionChange: (v: string) => void;
	onManualChange: (manual: boolean) => void;
}) {
	return (
		<Select
			value={version}
			onValueChange={val => {
				if (val === 'manual') {
					onManualChange(true);
					onVersionChange('');
				} else {
					onVersionChange(val ?? '');
				}
			}}>
			<SelectTrigger className="border- h-9 w-full bg-card text-xs">
				<SelectValue placeholder="Select Railpack version" />
			</SelectTrigger>
			<SelectContent className="bg-popover">
				<SelectItem
					value="manual"
					className="cursor-pointer text-xs font-bold text-primary hover:bg-muted">
					✏️ Custom Version (Manual)
				</SelectItem>
				{versions.map(v => (
					<SelectItem
						key={v}
						value={v}
						className="cursor-pointer font-mono text-xs hover:bg-muted">
						v{v}
					</SelectItem>
				))}
			</SelectContent>
		</Select>
	);
}

/** Railpack version picker with a manual-entry fallback. */
export function RailpackVersionField({
	version,
	isManual,
	versions,
	onVersionChange,
	onManualChange,
}: {
	version: string;
	isManual: boolean;
	versions: readonly string[];
	onVersionChange: (v: string) => void;
	onManualChange: (manual: boolean) => void;
}) {
	return (
		<div className="flex flex-col gap-3">
			<span className="text-xs font-bold text-muted-foreground">
				Railpack Version
			</span>
			{isManual ? (
				<div className="flex flex-col gap-2">
					<Input
						placeholder="Enter custom version (e.g. 0.15.4)"
						value={version}
						onChange={e => onVersionChange(e.target.value)}
						className="border- bg-card font-mono text-xs"
					/>
					<Button
						type="button"
						variant="outline"
						size="sm"
						onClick={() => {
							onManualChange(false);
							onVersionChange('0.15.4');
						}}
						className="h-7 w-fit  text-[10px] font-semibold">
						Use predefined versions
					</Button>
				</div>
			) : (
				<RailpackSelect
					version={version}
					versions={versions}
					onVersionChange={onVersionChange}
					onManualChange={onManualChange}
				/>
			)}
		</div>
	);
}
