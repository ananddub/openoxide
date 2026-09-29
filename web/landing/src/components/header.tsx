import {useEffect, useState} from 'react';
import {Logo} from './logo';
import {ThemeToggle} from './theme-toggle';
import {btnSmall} from './buttons';
import {gsap, useMotion} from '../lib/motion';

const links = [
	{href: '#deployments', label: 'Deployments'},
	{href: '#networking', label: 'Networking'},
	{href: '#capabilities', label: 'Capabilities'},
	{href: '#install', label: 'Install'},
];

export function Header() {
	const [pinned, setPinned] = useState(false);
	const ref = useMotion<HTMLElement>(() => {
		gsap.from('[data-header-item]', {
			y: -14,
			autoAlpha: 0,
			duration: 0.6,
			stagger: 0.07,
			delay: 0.05,
			ease: 'power3.out',
		});
	});

	useEffect(() => {
		const onScroll = () => setPinned(window.scrollY > 8);
		onScroll();
		window.addEventListener('scroll', onScroll, {passive: true});
		return () => window.removeEventListener('scroll', onScroll);
	}, []);

	return (
		<header
			ref={ref}
			className={`sticky top-0 z-50 border-b transition-[background-color,border-color,box-shadow] duration-300 ${
				pinned
					? 'border-border bg-background/85 backdrop-blur-xl'
					: 'border-transparent bg-transparent'
			}`}>
			<div className="mx-auto flex h-14 max-w-[1200px] items-center gap-6 px-5 sm:px-8">
				<a data-header-item href="#top" className="shrink-0">
					<Logo />
				</a>
				<nav
					data-header-item
					aria-label="Sections"
					className="hidden flex-1 items-center gap-1 md:flex">
					{links.map(l => (
						<a
							key={l.href}
							href={l.href}
							className="rounded-md px-2.5 py-1.5 text-[13px] text-muted-foreground transition-colors hover:bg-muted hover:text-foreground">
							{l.label}
						</a>
					))}
				</nav>
				<div
					data-header-item
					className="ml-auto flex items-center gap-2 md:ml-0">
					<ThemeToggle />
					<a href="/singin" className={btnSmall}>
						Open panel
					</a>
				</div>
			</div>
		</header>
	);
}
