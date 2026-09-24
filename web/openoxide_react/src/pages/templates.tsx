import {createFileRoute} from '@tanstack/react-router';
import {TemplatesPage} from '#/components/templates/templates-page';

export const Route = createFileRoute('/_app/templates')({
	component: TemplatesPage,
});
