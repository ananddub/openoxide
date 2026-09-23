import {Link} from '@tanstack/react-router';
import {
	ArrowUpRight,
	BookOpen,
	KeyRound,
	Rocket,
	Server,
} from 'lucide-react';
import type {LucideIcon} from 'lucide-react';

const SHADOW =
	'shadow-[0_1px_1px_rgba(0,0,0,0.02),0_2px_2px_rgba(0,0,0,0.04)]';

const LINKS: {
	label: string;
	sub: string;
	icon: LucideIcon;
	to: string;
	external?: boolean;
}[] = [
	{
		label: 'Remote Servers',
		sub: 'Connect hosts over SSH',
		icon: Server,
		to: '/remote-servers',
	},
	{
		label: 'SSH Keys',
		sub: 'Keys for server & git access',
		icon: KeyRound,
		to: '/ssh-keys',
	},
	{
		label: 'Deployments',
		sub: 'Build & deploy history',
		icon: Rocket,
		to: '/Deployments',
	},
	{
		label: 'Docs',
		sub: 'Guides & API reference',
		icon: BookOpen,
		to: 'https://github.com/vajra-labs/goploy',
		external: true,
	},
];

/**
 * Shortcut grid under the projects list — the surfaces the rail carries
 * (Remote Servers, SSH Keys) surfaced inline, Vercel onboarding-tile style.
 */
export function QuickLinksCard() {
	return (
		<div className={`rounded-lg border bg-card p-4 ${SHADOW}`}>
			<div className="grid grid-cols-2 gap-2 sm:grid-cols-4">
				{LINKS.map(({label, sub, icon: Icon, to, external}) => {
					const className =
						'group flex flex-col gap-3 rounded-md p-3 transition-colors hover:bg-muted/40';
					const inner = (
						<>
							{/* Icon row: glyph left, hover arrow right. */}
							<div className="flex items-start justify-between">
								<div className="flex size-8 items-center justify-center rounded-md border bg-muted/50 transition-transform group-hover:scale-105">
									<Icon
										className="size-4 text-muted-foreground"
										strokeWidth={1.7}
									/>
								</div>
								<ArrowUpRight
									className="size-4 shrink-0 text-muted-foreground/40 transition-all group-hover:translate-x-0.5 group-hover:-translate-y-0.5 group-hover:text-foreground"
									strokeWidth={1.7}
								/>
							</div>
							<span>
								<span className="block text-[13px] font-medium text-foreground">
									{label}
								</span>
								<span className="mt-0.5 block text-xs text-muted-foreground">
									{sub}
								</span>
							</span>
						</>
					);
					return external ? (
						<a
							key={label}
							href={to}
							target="_blank"
							rel="noopener noreferrer"
							className={className}>
							{inner}
						</a>
					) : (
						<Link key={label} to={to} className={className}>
							{inner}
						</Link>
					);
				})}
			</div>
		</div>
	);
}
