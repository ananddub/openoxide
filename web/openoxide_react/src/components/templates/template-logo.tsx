import {useState} from 'react';
import {templateFileUrl, type TemplateMeta} from '#/api/live';
import {cn} from '#/api/utils';

/** Registry logo with an initials fallback when the remote image is missing or fails to load. */
export function TemplateLogo({
	template,
	className,
}: {
	template: TemplateMeta;
	className?: string;
}) {
	const [failed, setFailed] = useState(false);
	if (failed || !template.logo) {
		return (
			<div
				className={cn(
					'flex items-center justify-center rounded-lg bg-muted font-semibold text-muted-foreground',
					className,
				)}>
				{template.name.charAt(0).toUpperCase()}
			</div>
		);
	}
	return (
		<img
			src={templateFileUrl(template.id, template.logo)}
			alt={`${template.name} logo`}
			loading="lazy"
			onError={() => setFailed(true)}
			className={cn('object-contain', className)}
		/>
	);
}
