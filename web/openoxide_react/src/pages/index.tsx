import {createFileRoute, Navigate} from '@tanstack/react-router';

export const Route = createFileRoute('/_app/')({
	// `/` used to render a home dashboard that duplicated the Projects list — the workspace now lands on Projects directly.
	component: () => <Navigate to="/projects" replace />,
});
