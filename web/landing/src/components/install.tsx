import {useState} from 'react';
import {Check, Copy} from 'lucide-react';
import {installSteps, type InstallStep} from '../data';
import {gsap, once, useMotion} from '../lib/motion';

function Step({index, step}: {index: number; step: InstallStep}) {
	const [copied, setCopied] = useState(false);

	const copy = async () => {
		try {
			await navigator.clipboard.writeText(step.command);
			setCopied(true);
			setTimeout(() => setCopied(false), 1600);
		} catch {
			/* Clipboard is blocked outside a secure context; the text is
			 * selectable anyway, so a failure here needs no theatre. */
		}
	};

	return (
		<li
			data-install-step
			className="relative border-b border-border pb-6 last:border-b-0">
			<span className="font-mono text-[11px] text-muted-foreground">
				{String(index + 1).padStart(2, '0')}
			</span>
			<button
				type="button"
				onClick={copy}
				aria-label={copied ? 'Copied' : `Copy: ${step.command}`}
				className="absolute top-0 right-0 grid size-7 place-items-center rounded-md border border-border text-muted-foreground transition-colors hover:bg-muted hover:text-foreground active:scale-[0.96]">
				{copied ? (
					<Check size={13} strokeWidth={2.4} aria-hidden />
				) : (
					<Copy size={13} strokeWidth={1.9} aria-hidden />
				)}
			</button>
			<code
				data-install-cmd
				className="mt-2 block pr-9 font-mono text-[13px] leading-relaxed break-all select-all sm:text-[14px]">
				{step.command}
			</code>
			<p className="mt-2 text-[13px] text-muted-foreground">
				{step.comment}
			</p>
		</li>
	);
}

const flags = [
	['SECRET_KEY', 'signs anything the panel hands out'],
	['JWT_ACCESS_SECRET', 'short-lived session tokens'],
	['JWT_REFRESH_SECRET', 'long-lived session tokens'],
	['METRICS_TOKEN', 'shared with every agent that reports in'],
];

export function Install() {
	const ref = useMotion<HTMLElement>(scope => {
		gsap.from('[data-install-head]', {
			y: 20,
			autoAlpha: 0,
			duration: 0.75,
			ease: 'power3.out',
			scrollTrigger: once(scope.querySelector('[data-install-head]')!),
		});

		gsap.from('[data-install-step]', {
			y: 26,
			autoAlpha: 0,
			duration: 0.7,
			stagger: 0.16,
			ease: 'power3.out',
			scrollTrigger: once(scope.querySelector('[data-install-steps]')!),
		});

		gsap.from('[data-install-cmd]', {
			x: -14,
			autoAlpha: 0,
			duration: 0.6,
			stagger: 0.16,
			ease: 'power2.out',
			scrollTrigger: once(
				scope.querySelector('[data-install-steps]')!,
				'top 78%',
			),
		});

		gsap.from('[data-install-flags]', {
			y: 34,
			autoAlpha: 0,
			duration: 0.85,
			ease: 'power3.out',
			scrollTrigger: once(scope.querySelector('[data-install-flags]')!),
		});

		gsap.from('[data-install-flag]', {
			x: 14,
			autoAlpha: 0,
			duration: 0.55,
			stagger: 0.09,
			ease: 'power2.out',
			scrollTrigger: once(
				scope.querySelector('[data-install-flags]')!,
				'top 82%',
			),
		});

		gsap.from('[data-install-network]', {
			y: 12,
			autoAlpha: 0,
			duration: 0.6,
			ease: 'power2.out',
			scrollTrigger: once(
				scope.querySelector('[data-install-network]')!,
				'top 94%',
			),
		});
	});

	return (
		<section
			ref={ref}
			id="install"
			className="mx-auto max-w-[1200px] px-5 sm:px-8">
			<div className="grid gap-12 border-t border-border py-20 md:py-24 lg:grid-cols-[minmax(0,1fr)_minmax(0,420px)] lg:gap-20">
				<div>
					<div data-install-head>
						<h2 className="text-[30px] leading-[1.1] font-semibold tracking-[-0.025em] text-balance md:text-[38px]">
							Three commands, then a browser
						</h2>
						<p className="mt-4 max-w-[54ch] text-[16px] leading-relaxed text-muted-foreground">
							The panel and the agent are separate images. The panel holds
							the build pipeline and the UI; the agent is what you put on
							every other host.
						</p>
					</div>
					<ol data-install-steps className="mt-10 space-y-6">
						{installSteps.map((s, i) => (
							<Step key={s.command} index={i} step={s} />
						))}
					</ol>
				</div>

				<aside className="lg:pt-[86px]">
					<div
						data-install-flags
						className="rounded-xl border border-border bg-card p-6 shadow-[var(--shadow-card)]">
						<h3 className="font-mono text-[12px] tracking-[0.12em] text-muted-foreground uppercase">
							Set these first
						</h3>
						<p className="mt-3 text-[13.5px] leading-relaxed text-muted-foreground">
							They live in the compose file. The defaults there are
							placeholders, so change them before the port is reachable.
						</p>
						<dl className="mt-6 space-y-4">
							{flags.map(([k, v]) => (
								<div data-install-flag key={k}>
									<dt className="font-mono text-[12px]">{k}</dt>
									<dd className="mt-1 text-[13px] leading-snug text-muted-foreground">
										{v}
									</dd>
								</div>
							))}
						</dl>
					</div>
					<p
						data-install-network
						className="mt-5 font-mono text-[12px] leading-relaxed text-muted-foreground">
						docker network create openoxide-network
						<br />
						<span className="text-foreground/70">
							→ both compose files expect it to exist
						</span>
					</p>
				</aside>
			</div>
		</section>
	);
}
