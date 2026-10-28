import {useState} from 'react';
import {Check, Copy, Gamepad2} from 'lucide-react';
import {gsap, once, useMotion} from '../lib/motion';

const cygSteps = [
	{
		comment: 'latest release binary — Linux, macOS aur Windows sab',
		command:
			'curl -fsSL https://raw.githubusercontent.com/ananddub/cyg/main/scripts/install.sh | sh',
	},
	{
		comment: 'project scaffold — schema, room, website sab ready',
		command: 'cyg-gaming init my-game && cd my-game',
	},
	{
		comment: 'server live — console, WebRTC + QUIC, 128 Hz',
		command: 'cyg-gaming run',
	},
];

function CygStep({index, step}: {index: number; step: (typeof cygSteps)[0]}) {
	const [copied, setCopied] = useState(false);
	const copy = async () => {
		try {
			await navigator.clipboard.writeText(step.command);
			setCopied(true);
			setTimeout(() => setCopied(false), 1600);
		} catch {
			/* clipboard blocked — text is selectable anyway */
		}
	};
	return (
		<li
			data-cyg-step
			className="relative border-b border-border pb-5 last:border-b-0">
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
				data-cyg-cmd
				className="mt-2 block pr-9 font-mono text-[13px] leading-relaxed break-all select-all sm:text-[14px]">
				{step.command}
			</code>
			<p className="mt-2 text-[13px] text-muted-foreground">{step.comment}</p>
		</li>
	);
}

export function CygInstall() {
	const ref = useMotion<HTMLElement>(scope => {
		gsap.from('[data-cyg-head]', {
			y: 20,
			autoAlpha: 0,
			duration: 0.75,
			ease: 'power3.out',
			scrollTrigger: once(scope.querySelector('[data-cyg-head]')!),
		});
		gsap.from('[data-cyg-step]', {
			y: 26,
			autoAlpha: 0,
			duration: 0.7,
			stagger: 0.16,
			ease: 'power3.out',
			scrollTrigger: once(scope.querySelector('[data-cyg-steps]')!),
		});
	});

	return (
		<section
			ref={ref}
			id="cyg"
			className="mx-auto max-w-[1200px] px-5 sm:px-8">
			<div className="border-t border-border py-20 md:py-24">
				<div data-cyg-head className="max-w-[54ch]">
					<p className="flex items-center gap-2 font-mono text-[12px] tracking-[0.12em] text-muted-foreground uppercase">
						<Gamepad2 size={14} aria-hidden /> from the same team
					</p>
					<h2 className="mt-3 text-[30px] leading-[1.1] font-semibold tracking-[-0.025em] text-balance md:text-[38px]">
						CYG — a multiplayer game server in 2 minutes
					</h2>
					<p className="mt-4 text-[16px] leading-relaxed text-muted-foreground">
						Authoritative WASM rooms, WebRTC + UDP + QUIC transports, physics,
						accounts and a web console — one self-hosted Rust binary. Open
						source, Apache-2.0.
					</p>
				</div>
				<ol data-cyg-steps className="mt-10 max-w-[720px] space-y-5">
					{cygSteps.map((s, i) => (
						<CygStep key={s.command} index={i} step={s} />
					))}
				</ol>
			</div>
		</section>
	);
}
