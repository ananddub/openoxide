import {Header} from './components/header';
import {Hero} from './components/hero';
import {BrandWall} from './components/brand-wall';
import {Stats} from './components/stats';
import {Features} from './components/features';
import {Capabilities} from './components/capabilities';
import {Install} from './components/install';
import {CygInstall} from './components/cyg-install';
import {FinalCta} from './components/final-cta';
import {Footer} from './components/footer';
import {useScrollRefresh} from './lib/motion';

export function App() {
	// ScrollTrigger positions are measured before the webp shots decode.
	useScrollRefresh();

	return (
		<div className="min-h-dvh bg-background">
			<Header />
			<main>
				<Hero />
				<BrandWall />
				<Stats />
				<Features />
				<Capabilities />
				<Install />
				<CygInstall />
				<FinalCta />
			</main>
			<Footer />
		</div>
	);
}
