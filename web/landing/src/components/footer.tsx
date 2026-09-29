import {Logo} from './logo';

const columns = [
	{
		title: 'Panel',
		links: [
			{label: 'Deployments', href: '#deployments'},
			{label: 'Architecture', href: '#architecture'},
			{label: 'Networking', href: '#networking'},
			{label: 'Catalogue', href: '#catalogue'},
		],
	},
	{
		title: 'Operations',
		links: [
			{label: 'Fleet', href: '#fleet'},
			{label: 'Capabilities', href: '#capabilities'},
			{label: 'Install', href: '#install'},
			{label: 'Command palette', href: '#keyboard'},
		],
	},
];

export function Footer() {
	return (
		<footer className="border-t border-border">
			<div className="mx-auto grid max-w-[1200px] gap-10 px-5 py-14 sm:px-8 lg:grid-cols-[minmax(0,1fr)_auto_auto] lg:gap-24">
				<div>
					<Logo />
					<p className="mt-4 max-w-[38ch] text-[13.5px] leading-relaxed text-muted-foreground">
						A self-hosted control panel for applications, Compose stacks,
						databases, domains and certificates.
					</p>
				</div>
				{columns.map(col => (
					<nav key={col.title} aria-label={col.title}>
						<h2 className="font-mono text-[11px] tracking-[0.12em] text-muted-foreground uppercase">
							{col.title}
						</h2>
						<ul className="mt-4 space-y-2.5">
							{col.links.map(l => (
								<li key={l.href}>
									<a
										href={l.href}
										className="text-[13.5px] text-muted-foreground transition-colors hover:text-foreground">
										{l.label}
									</a>
								</li>
							))}
						</ul>
					</nav>
				))}
			</div>
			<div className="mx-auto flex max-w-[1200px] flex-wrap items-center justify-between gap-3 border-t border-border px-5 py-6 sm:px-8">
				<p className="font-mono text-[11.5px] text-muted-foreground">
					dubeyanand/openoxide · dubeyanand/openoxide-agent
				</p>
				<p className="font-mono text-[11.5px] text-muted-foreground">
					Rust · Axum · React 19 · one SQLite file
				</p>
			</div>
		</footer>
	);
}
