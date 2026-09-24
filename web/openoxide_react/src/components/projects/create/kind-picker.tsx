import {useState} from 'react';
import {Search} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {OptionRow} from '#/components/shared/sheet-wizard';
import {OPTIONS, type ServiceKind} from './constants';

/** Service-kind chooser as a searchable list — replaces the old card grid. Keeps the same `chosen` / `onPick` contract so the create page is unaware of the internal search state. */
export function KindPicker({
	chosen,
	onPick,
}: {
	chosen: ServiceKind;
	onPick: (kind: ServiceKind) => void;
}) {
	const [query, setQuery] = useState('');
	const q = query.trim().toLowerCase();
	const list = OPTIONS.filter(
		o =>
			!q ||
			o.title.toLowerCase().includes(q) ||
			o.description.toLowerCase().includes(q),
	);

	return (
		<div className="flex flex-col gap-3">
			<div className="relative">
				<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
				<Input
					autoFocus
					value={query}
					onChange={e => setQuery(e.target.value)}
					placeholder="Search services…"
					className="bg-background pl-9"
				/>
			</div>

			<div className="flex max-h-[min(55vh,480px)] flex-col gap-2 overflow-y-auto pr-0.5">
				{list.map(o => (
					<OptionRow
						key={o.id}
						selected={chosen === o.id}
						onClick={() => onPick(o.id)}
						icon={<o.icon className="size-5" />}
						title={o.title}
						subtitle={o.description}
					/>
				))}
				{list.length === 0 && (
					<p className="py-6 text-center text-[13px] text-muted-foreground">
						No services match “{query.trim()}”.
					</p>
				)}
			</div>
		</div>
	);
}
