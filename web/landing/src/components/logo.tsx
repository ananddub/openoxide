import {Power} from 'lucide-react';

export function Logo({className = ''}: {className?: string}) {
	return (
		<span className={`flex items-center gap-2 ${className}`}>
			<span className="grid size-7 place-items-center rounded-md border border-border bg-card shadow-[inset_0_1px_0_rgba(255,255,255,.06)]">
				<Power
					size={14}
					strokeWidth={2.4}
					className="text-foreground"
					aria-hidden
				/>
			</span>
			<span className="text-[15px] font-medium tracking-tight">
				OpenOxide
			</span>
		</span>
	);
}
