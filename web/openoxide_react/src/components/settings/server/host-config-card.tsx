import {BadgeCheck, Save, Server, Shield, Wrench} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Field, SectionLabel, ToggleRow} from './settings-parts';
import {
	useHostSettingsForm,
	type CertType,
} from './use-host-settings-form';

/** "Host Configuration" card: address, TLS and maintenance settings. */
export function HostConfigCard() {
	const {canManage, form, set, handleSave, isSaving, isLoading} =
		useHostSettingsForm();

	if (!canManage) {
		return (
			<p className="rounded-xl bg-card px-4 py-3 text-xs text-muted-foreground ring-1 ring-border/50">
				Host configuration is managed by the organization owner (requires
				org:write).
			</p>
		);
	}

	return (
		<div className="overflow-hidden rounded-xl bg-card ring-1 ring-border/50">
			<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
				<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted">
					<Shield className="size-[18px] text-muted-foreground" />
				</div>
				<div className="min-w-0 flex-1">
					<h3 className="text-[15px] font-semibold text-foreground">
						Host Configuration
					</h3>
					<p className="text-xs text-muted-foreground">
						How this panel is addressed, secured, and kept tidy.
					</p>
				</div>
				<Button
					size="sm"
					onClick={handleSave}
					disabled={isSaving || isLoading}
					className="h-9 shrink-0 cursor-pointer gap-2 px-4 text-xs font-semibold">
					<Save className="size-3.5" />
					{isSaving ? 'Saving…' : 'Save Settings'}
				</Button>
			</div>
			<div className="space-y-6 p-5">
				{/* Panel address */}
				<section className="flex flex-col gap-3">
					<SectionLabel icon={Server}>Panel Address</SectionLabel>
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
				</section>

				{/* TLS */}
				<section className="flex flex-col gap-3">
					<SectionLabel icon={BadgeCheck}>
						TLS &amp; Certificates
					</SectionLabel>
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
									onChange={e =>
										set('custom_cert_resolver', e.target.value)
									}
									placeholder="my-resolver"
									className="h-9 font-mono text-xs"
								/>
							</Field>
						)}
					</div>
				</section>

				{/* Maintenance */}
				<section className="flex flex-col gap-3">
					<SectionLabel icon={Wrench}>Maintenance</SectionLabel>
					<div className="grid gap-4 sm:grid-cols-2">
						<Field
							label="Log Cleanup Cron"
							hint="Blank disables the schedule">
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
					</div>
				</section>
			</div>
		</div>
	);
}
