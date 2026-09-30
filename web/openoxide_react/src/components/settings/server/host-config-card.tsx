import {BadgeCheck, Save, Server, Wrench} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field, ToggleRow} from './settings-parts';
import {
	useHostSettingsForm,
	type CertType,
} from './use-host-settings-form';

/** Small state chip shown on the right of a settings row. */
function Chip({
	label,
	tone,
}: {
	label: string;
	tone: 'solid' | 'muted' | 'mono';
}) {
	return (
		<span
			className={
				tone === 'mono'
					? 'rounded-full bg-foreground/[0.06] px-2 py-0.5 font-mono text-[10px] text-muted-foreground'
					: tone === 'solid'
						? 'rounded-full bg-primary/10 px-2 py-0.5 text-[10px] font-medium text-primary'
						: 'rounded-full bg-foreground/[0.06] px-2 py-0.5 text-[10px] font-medium text-muted-foreground'
			}>
			{label}
		</span>
	);
}

/**
 * Host configuration as a quiet settings strip — one row per group, the
 * form itself opens in the right-side sheet like every other settings
 * surface. All three sheets commit the same form.
 */
export function HostConfigCard() {
	const {canManage, form, set, handleSave, isSaving, isLoading} =
		useHostSettingsForm();

	if (!canManage) {
		return (
			<p className="rounded-xl border border-border bg-card px-4 py-3 text-xs text-muted-foreground">
				Host configuration is managed by the organization owner (requires
				org:update).
			</p>
		);
	}

	const saveButton = (
		<Button
			size="sm"
			onClick={handleSave}
			disabled={isSaving || isLoading}
			className="h-8 shrink-0 cursor-pointer gap-1.5 px-4 text-xs font-semibold">
			<Save className="size-3.5" />
			{isSaving ? 'Saving…' : 'Save'}
		</Button>
	);

	return (
		<div className="overflow-hidden rounded-xl border bg-card">
			<SettingsSheet
				title="Panel Address"
				description="IP and domain this panel is served on"
				icon={<Server className="size-4" />}
				meta={
					<span className="max-w-[220px] truncate font-mono text-xs text-muted-foreground">
						{form.host || '—'}
					</span>
				}
				footer={saveButton}>
				<div className="grid gap-4 sm:grid-cols-2">
					<Field label="Server IP" hint="IPv4/IPv6 of this host">
						<Input
							value={form.server_ip}
							onChange={e => set('server_ip', e.target.value)}
							placeholder="203.0.113.10"
							className="h-9 font-mono text-xs"
						/>
					</Field>
					<Field label="Panel Host" hint="Domain the panel is served on">
						<Input
							value={form.host}
							onChange={e => set('host', e.target.value)}
							placeholder="panel.example.com"
							className="h-9 font-mono text-xs"
						/>
					</Field>
				</div>
			</SettingsSheet>

			<SettingsSheet
				title="TLS & Certificates"
				description="How traffic to the panel is encrypted"
				icon={<BadgeCheck className="size-4" />}
				meta={
					<span className="flex items-center gap-1.5">
						<Chip
							label={`HTTPS ${form.https ? 'on' : 'off'}`}
							tone={form.https ? 'solid' : 'muted'}
						/>
						<Chip
							label={form.certificate_type}
							tone={form.certificate_type === 'None' ? 'muted' : 'solid'}
						/>
					</span>
				}
				footer={saveButton}>
				<ToggleRow
					title="HTTPS"
					description="Serve the panel over TLS"
					checked={form.https}
					onChecked={v => set('https', v)}
				/>
				<div className="grid gap-4 sm:grid-cols-2">
					<Field label="Certificate Type">
						<Select
							value={form.certificate_type}
							onValueChange={value =>
								value && set('certificate_type', value as CertType)
							}>
							<SelectTrigger className="h-9 text-xs">
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="None">None</SelectItem>
								<SelectItem value="Letsencrypt">Letsencrypt</SelectItem>
								<SelectItem value="Custom">Custom</SelectItem>
							</SelectContent>
						</Select>
					</Field>
					{form.certificate_type === 'Letsencrypt' && (
						<Field
							label="Letsencrypt Email"
							hint="Used for expiry notices">
							<Input
								type="email"
								value={form.lets_encrypt_email}
								onChange={e => set('lets_encrypt_email', e.target.value)}
								placeholder="admin@example.com"
								className="h-9 text-xs"
							/>
						</Field>
					)}
					{form.certificate_type === 'Custom' && (
						<Field
							label="Custom Resolver"
							hint="Traefik cert resolver name">
							<Input
								value={form.custom_cert_resolver}
								onChange={e => set('custom_cert_resolver', e.target.value)}
								placeholder="my-resolver"
								className="h-9 font-mono text-xs"
							/>
						</Field>
					)}
				</div>
			</SettingsSheet>

			<SettingsSheet
				title="Maintenance"
				description="Scheduled cleanup on this host"
				icon={<Wrench className="size-4" />}
				meta={
					<span className="flex items-center gap-1.5">
						<Chip
							label={form.log_cleanup_cron || 'Cleanup off'}
							tone={form.log_cleanup_cron ? 'mono' : 'muted'}
						/>
						<Chip
							label={`Prune ${form.enable_docker_cleanup ? 'on' : 'off'}`}
							tone={form.enable_docker_cleanup ? 'solid' : 'muted'}
						/>
					</span>
				}
				footer={saveButton}>
				<Field label="Log Cleanup Cron" hint="Blank disables the schedule">
					<Input
						value={form.log_cleanup_cron}
						onChange={e => set('log_cleanup_cron', e.target.value)}
						placeholder="0 4 * * *"
						className="h-9 font-mono text-xs"
					/>
				</Field>
				<ToggleRow
					title="Docker Cleanup"
					description="Periodically prune unused images/containers"
					checked={form.enable_docker_cleanup}
					onChecked={v => set('enable_docker_cleanup', v)}
				/>
			</SettingsSheet>
		</div>
	);
}
