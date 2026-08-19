if (typeof BigInt !== 'undefined' && !(BigInt.prototype as any).toJSON) {
	(BigInt.prototype as any).toJSON = function () {
		return typeof this === 'bigint'
			? this <= BigInt(Number.MAX_SAFE_INTEGER) &&
				this >= BigInt(Number.MIN_SAFE_INTEGER)
				? Number(this)
				: this.toString()
			: this;
	};
}

import {StrictMode} from 'react';
import ReactDOM from 'react-dom/client';
import {routeTree} from './routeTree.gen';
import {NotFound} from './components/not-found';
import {createRouter, RouterProvider} from '@tanstack/react-router';

const router = createRouter({
	routeTree,
	defaultPreload: 'intent',
	scrollRestoration: true,
	defaultNotFoundComponent: NotFound,
});

declare module '@tanstack/react-router' {
	interface Register {
		router: typeof router;
	}
}

const rootElement = document.getElementById('app');

if (rootElement && !rootElement.innerHTML) {
	const root = ReactDOM.createRoot(rootElement);
	root.render(
		<StrictMode>
			<RouterProvider router={router} />
		</StrictMode>,
	);
}
