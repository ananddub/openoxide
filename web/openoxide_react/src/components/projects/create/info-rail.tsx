import {Rocket, Check, ShieldCheck} from 'lucide-react';
import {OPTIONS, TYPE_INFO} from './constants';
import type {useCreateService} from './use-create-service';

type Form = ReturnType<typeof useCreateService>;

/** Sticky right rail: kind guidance bullets + deployment destination summary. */
export function InfoRail({form}: {form: Form}) {
	const {chosen, project, selectedEnv} = form;
	const option = OPTIONS.find(o => o.id === chosen);
	const info = TYPE_INFO[chosen];

	return (
		<div className="hidden space-y-4 lg:sticky lg:top-6 lg:block lg:self-start">
			<div className="rounded-xl border border-border/50 bg-card p-5">
				<div className="mb-3 flex items-center gap-2">
					<Rocket className="size-4 text-primary" />
					<h3 className="text-sm font-semibold text-foreground">
						{info.heading}
					</h3>
				</div>
				<ul className="space-y-2.5">
					{info.steps.map(step => (
						<li
							key={step}
							className="flex items-start gap-2 text-xs leading-relaxed text-muted-foreground">
							<Check className="mt-0.5 size-3.5 shrink-0 text-emerald-500" />
							{step}
						</li>
					))}
				</ul>
			</div>

			<div className="rounded-xl border border-border/50 bg-card p-5">
				<div className="mb-3 flex items-center gap-2">
					<ShieldCheck className="size-4 text-muted-foreground" />
					<h3 className="text-sm font-semibold text-foreground">
						Where it lands
					</h3>
				</div>
				<div className="space-y-2 text-xs text-muted-foreground">
					<p className="flex items-center justify-between">
						<span>Project</span>
						<span className="font-medium text-foreground">
							{project?.name || '—'}
						</span>
					</p>
					<p className="flex items-center justify-between">
						<span>Environment</span>
						<span className="font-medium text-foreground">
							{selectedEnv?.name || 'default'}
						</span>
					</p>
					<p className="flex items-center justify-between">
						<span>Service</span>
						<span className="font-medium text-foreground">
							{option?.title}
						</span>
					</p>
				</div>
			</div>
		</div>
	);
}
