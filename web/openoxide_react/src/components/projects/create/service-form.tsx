import {Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {cn} from '#/api/utils';
import {OPTIONS} from './constants';
import {Field} from './field';
import {EngineField, DbDetailFields} from './database-fields';
import type {useCreateService} from './use-create-service';

type Form = ReturnType<typeof useCreateService>;

/** The create-service form: fields adapt to the chosen service kind. `sheet` mode renders plain Geist fields (no card chrome) with submit placed outside via `formId`; page mode keeps the card + inline footer. */
export function ServiceForm({
	form,
	sheet,
	formId,
	hideServer,
}: {
	form: Form;
	sheet?: boolean;
	formId?: string;
	hideServer?: boolean;
}) {
	const {
		chosen,
		submitting,
		name,
		setName,
		description,
		setDescription,
		serverId,
		setServerId,
		composeType,
		setComposeType,
		servers,
		selectedEnv,
		selectedEnvId,
		handleSubmit,
	} = form;

	const option = OPTIONS.find(o => o.id === chosen);
	const isDatabase = chosen === 'database';
	const namePlaceholder =
		chosen === 'database'
			? 'my-database'
			: chosen === 'compose'
				? 'my-stack'
				: 'my-application';

	const fields = (
		<>
			{isDatabase && !sheet && <EngineField form={form} />}

			<div
				className={cn(
					'grid gap-4',
					sheet || hideServer ? 'grid-cols-1' : 'sm:grid-cols-2',
				)}>
				<Field label="Name">
					<Input
						autoFocus
						placeholder={namePlaceholder}
						value={name}
						onChange={e => setName(e.target.value)}
						className="bg-background"
					/>
				</Field>
				{!hideServer && (
					<Field label="Server">
						<Select
							value={serverId}
							onValueChange={v => v && setServerId(v)}>
							<SelectTrigger className="w-full bg-background">
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="local" className="text-sm">
									Local (this server)
								</SelectItem>
								{servers.map((s: any) => (
									<SelectItem
										key={String(s.id)}
										value={String(s.id)}
										className="text-sm">
										{s.name} · {s.ip_address}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</Field>
				)}
			</div>

			<Field label="Description (optional)">
				<Textarea
					rows={2}
					placeholder="What is this service for?"
					value={description}
					onChange={e => setDescription(e.target.value)}
					className="bg-background"
				/>
			</Field>

			{/* In the sheet wizard the orchestrator is asked on its own
			    up-front step, so the dropdown only lives on the page form. */}
			{chosen === 'compose' && !sheet && (
				<Field label="Compose Type">
					<Select
						value={composeType}
						onValueChange={v => v && setComposeType(v)}>
						<SelectTrigger className="w-full max-w-xs bg-background">
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value="DOCKER-COMPOSE" className="text-sm">
								docker-compose
							</SelectItem>
							<SelectItem value="STACK" className="text-sm">
								docker stack (swarm)
							</SelectItem>
						</SelectContent>
					</Select>
				</Field>
			)}

			{isDatabase && <DbDetailFields form={form} sheet={sheet} />}
		</>
	);

	if (sheet) {
		return (
			<form
				id={formId}
				onSubmit={handleSubmit}
				className="flex flex-col gap-4">
				{fields}
			</form>
		);
	}

	return (
		<form
			onSubmit={handleSubmit}
			className="space-y-5 rounded-xl border border-border/50 bg-card p-6">
			{fields}
			<div className="flex items-center justify-between border-t border-border/40 pt-4">
				<p className="text-xs text-muted-foreground">
					Deploying into{' '}
					<span className="font-semibold text-foreground">
						{selectedEnv?.name || 'default'}
					</span>
				</p>
				<Button
					type="submit"
					disabled={submitting || !name.trim() || !selectedEnvId}
					className={cn(
						'flex h-9 items-center gap-1.5 text-xs font-semibold',
					)}>
					<Save className="size-3.5" />
					{submitting
						? 'Creating…'
						: `Create ${option?.title.toLowerCase() || 'service'}`}
				</Button>
			</div>
		</form>
	);
}
