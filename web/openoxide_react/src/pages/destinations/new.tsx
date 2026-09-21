import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {HardDrive} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Titlebar} from '#/components/shared/titlebar';
import {DestinationFormFields} from '#/components/destinations/destination-form-fields';
import {DestinationTestButton} from '#/components/destinations/destination-test-button';
import {
	PROVIDERS,
	useDestinationForm,
} from '#/components/destinations/use-destination-form';

export const Route = createFileRoute('/_app/destinations/new')({
	component: DestinationCreatePage,
});

function DestinationCreatePage() {
	const navigate = useNavigate();
	// Reuse the destination sheet's form hook: create-only, close is a no-op,
	// success navigates back to the destinations list.
	const f = useDestinationForm({
		isOpen: true,
		onClose: () => {},
		onSuccess: () => void navigate({to: '/destinations'}),
		editingDestination: null,
	});

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* ── Vercel-style titlebar ── */}
			<Titlebar
				breadcrumbs={[
					{
						label: 'S3 Destinations',
						to: '/destinations',
						icon: HardDrive,
					},
					{label: 'Create Destination'},
				]}
				icon={<HardDrive className="text-foreground/70" />}
				title="Create Destination"
				description="S3 storage bucket credentials for backups and volume snapshots."
			/>

			{/* ── Form card ── */}
			<div className="mx-auto w-full max-w-2xl min-w-0 rounded-xl bg-card p-6 ring-1 ring-border/50">
				<form onSubmit={f.handleSubmit} className="flex flex-col gap-4">
					<DestinationFormFields
						name={f.name}
						setName={f.setName}
						provider={f.provider}
						onProviderChange={f.handleProviderChange}
						bucket={f.bucket}
						setBucket={f.setBucket}
						region={f.region}
						setRegion={f.setRegion}
						endpoint={f.endpoint}
						setEndpoint={f.setEndpoint}
						accessKey={f.accessKey}
						setAccessKey={f.setAccessKey}
						secretKey={f.secretKey}
						setSecretKey={f.setSecretKey}
						providers={PROVIDERS}
					/>

					<div className="mt-1 flex items-center justify-between gap-3 border-t border-border/50 pt-4">
						<DestinationTestButton
							status={f.testStatus}
							onClick={f.handleTestConnection}
						/>
						<Button
							type="submit"
							disabled={f.submitting}
							className="h-9 px-5 text-xs font-semibold">
							{f.submitting ? 'Saving...' : 'Create'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);
}
