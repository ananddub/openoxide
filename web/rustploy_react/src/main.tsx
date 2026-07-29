import { StrictMode } from 'react';
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
		</StrictMode>
	);
}
