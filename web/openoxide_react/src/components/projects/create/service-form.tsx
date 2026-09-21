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
import {OPTIONS} from './constants';
import {Field} from './field';
import {EngineField, DbDetailFields} from './database-fields';
import type {useCreateService} from './use-create-service';

type Form = ReturnType<typeof useCreateService>;

/** The create-service form card: fields adapt to the chosen service kind. */
export function ServiceForm({form}: {form: Form}) {
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

	return (
		<form
			onSubmit={handleSubmit}
			className="space-y-5 rounded-xl border border-border/50 bg-card p-6">
			{isDatabase && <EngineField form={form} />}

			<div className="grid grid-cols-1 gap-5 sm:grid-cols-2">
				<Field label="Name">
					<Input
						autoFocus
						placeholder={namePlaceholder}
						value={name}
						onChange={e => setName(e.target.value)}
						className="h-10 rounded-xl border-border/50 bg-background text-sm"
					/>
				</Field>
				<Field label="Server">
					<Select
						value={serverId}
						onValueChange={v => v && setServerId(v)}>
						<SelectTrigger className="!h-10 w-full rounded-xl border-border/50 bg-background text-sm">
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
			</div>

			<Field label="Description (optional)">
				<Textarea
					rows={2}
					placeholder="What is this service for?"
					value={description}
					onChange={e => setDescription(e.target.value)}
					className="rounded-xl border-border/50 bg-background text-sm"
				/>
			</Field>

			{chosen === 'compose' && (
				<Field label="Compose Type">
					<Select
						value={composeType}
						onValueChange={v => v && setComposeType(v)}>
						<SelectTrigger className="!h-10 w-full max-w-xs rounded-xl border-border/50 bg-background text-sm">
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

			{isDatabase && <DbDetailFields form={form} />}

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
					className="flex h-9 items-center gap-1.5 rounded-xl text-xs font-semibold">
					<Save className="size-3.5" />
					{submitting
						? 'Creating…'
						: `Create ${option?.title.toLowerCase() || 'service'}`}
				</Button>
			</div>
		</form>
	);
}
