import {Input} from '#/components/ui/input';
import {Button} from '#/components/ui/button';
import {Plus} from 'lucide-react';

/** Editable health-check test command list (CMD-SHELL + args). */
export function HealthCommands({
	value,
	set,
}: {
	value: any;
	set: (value: string[]) => void;
}) {
	const commands =
		Array.isArray(value) && value.length ? value : ['CMD-SHELL', ''];
	return (
		<div className="space-y-2">
			<div>
				<h4 className="text-sm font-medium">Test Commands</h4>
				<p className="text-xs text-muted-foreground">
					Command to run for the health check.
				</p>
			</div>
			{commands.map((command, index) => (
				<div key={index} className="flex gap-2">
					<Input
						value={command}
						onChange={e =>
							set(
								commands.map((item, i) =>
									i === index ? e.target.value : item,
								),
							)
						}
						placeholder={
							index === 0 ? 'CMD-SHELL' : 'curl -f http://localhost/health'
						}
					/>
					{commands.length > 1 && (
						<Button
							type="button"
							variant="destructive"
							size="sm"
							onClick={() => set(commands.filter((_, i) => i !== index))}>
							Remove
						</Button>
					)}
				</div>
			))}
			<Button
				type="button"
				variant="outline"
				size="sm"
				className="w-fit"
				onClick={() => set([...commands, ''])}>
				<Plus className="mr-1 size-3" />
				Add command
			</Button>
		</div>
	);
}
