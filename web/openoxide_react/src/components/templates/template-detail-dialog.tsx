import {useState} from 'react';
import {ExternalLink, Rocket} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
} from '#/components/ui/tabs';
import {TemplateLogo} from './template-logo';
import {TemplateFilePreview} from './template-file-preview';
import {TemplateDeployDialog} from './template-deploy-dialog';
import {useTemplateFile, type TemplateMeta} from '#/api/live';

/** Template preview as a right side panel: metadata + links, the compose manifest and template config, and the Deploy entry point. */
export function TemplateDetailDialog({
	template,
	onOpenChange,
}: {
	template: TemplateMeta | null;
	onOpenChange: (open: boolean) => void;
}) {
	const [deployOpen, setDeployOpen] = useState(false);
	const compose = useTemplateFile(
		template?.id ?? '',
		'docker-compose.yml',
	);
	const toml = useTemplateFile(template?.id ?? '', 'template.toml');
	const links = template ? Object.entries(template.links ?? {}) : [];

	return (
		<>
			<Sheet open={template !== null} onOpenChange={onOpenChange}>
				<SheetContent>
					{template && (
						<>
							<SheetHeader>
								<SheetTitle>{template.name}</SheetTitle>
								<SheetDescription className="text-xs leading-relaxed">
									{template.description}
								</SheetDescription>
							</SheetHeader>

							<div className="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-6 py-6">
								<div className="flex items-center gap-3">
									<TemplateLogo
										template={template}
										className="size-10 shrink-0"
									/>
									<Badge
										variant="outline"
										className="shrink-0 font-mono text-[10px] font-normal">
										{template.version}
									</Badge>
								</div>

								{template.tags?.length > 0 && (
									<div className="flex flex-wrap gap-1">
										{template.tags.map(tag => (
											<Badge
												key={tag}
												variant="secondary"
												className="font-mono text-[10px] font-normal">
												{tag}
											</Badge>
										))}
									</div>
								)}

								{links.length > 0 && (
									<div className="flex flex-wrap gap-4">
										{links.map(([label, href]) => (
											<a
												key={label}
												href={href}
												target="_blank"
												rel="noreferrer noopener"
												className="inline-flex items-center gap-1 text-xs font-medium text-primary hover:underline">
												<ExternalLink className="size-3" />
												{label}
											</a>
										))}
									</div>
								)}

								<Tabs defaultValue="compose">
									<TabsList>
										<TabsTrigger value="compose">
											docker-compose.yml
										</TabsTrigger>
										<TabsTrigger value="toml">
											template.toml
										</TabsTrigger>
									</TabsList>
									<TabsContent value="compose">
										<TemplateFilePreview
											filename="docker-compose.yml"
											language="yaml"
											data={compose.data}
											isLoading={compose.isLoading}
										/>
									</TabsContent>
									<TabsContent value="toml">
										<TemplateFilePreview
											filename="template.toml"
											language="ini"
											data={toml.data}
											isLoading={toml.isLoading}
										/>
									</TabsContent>
								</Tabs>
							</div>

							<SheetFooter>
								<Button
									onClick={() => setDeployOpen(true)}
									className="h-9 gap-1.5">
									<Rocket className="size-3.5" />
									Deploy
								</Button>
							</SheetFooter>
						</>
					)}
				</SheetContent>
			</Sheet>

			<TemplateDeployDialog
				template={template}
				open={deployOpen}
				onOpenChange={setDeployOpen}
			/>
		</>
	);
}
