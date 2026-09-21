import {Button} from '#/components/ui/button';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {DestinationFormFields} from '#/components/destinations/destination-form-fields';
import {DestinationTestButton} from '#/components/destinations/destination-test-button';
import {
	PROVIDERS,
	useDestinationForm,
} from '#/components/destinations/use-destination-form';
import type {DestinationResponse} from '#/types/api-helpers';

interface CreateDestinationModalProps {
	isOpen: boolean;
	onClose: () => void;
	onSuccess: () => void;
	editingDestination?: DestinationResponse | null;
}

export function CreateDestinationModal({
	isOpen,
	onClose,
	onSuccess,
	editingDestination,
}: CreateDestinationModalProps) {
	const f = useDestinationForm({
		isOpen,
		onClose,
		onSuccess,
		editingDestination,
	});

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>
						{editingDestination
							? 'Edit Destination'
							: 'Create Destination'}
					</SheetTitle>
					<SheetDescription>
						Configure your S3 storage bucket credentials for backups and
						volume snapshots
					</SheetDescription>
				</SheetHeader>

				<form
					onSubmit={f.handleSubmit}
					className="mt-2 flex flex-col gap-4">
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

					<div className="mt-1 flex items-center justify-between gap-3 border-t  pt-4">
						<DestinationTestButton
							status={f.testStatus}
							onClick={f.handleTestConnection}
						/>
						<Button
							type="submit"
							disabled={f.submitting}
							className="h-9 px-5 text-xs font-semibold">
							{f.submitting
								? 'Saving...'
								: editingDestination
									? 'Save'
									: 'Create'}
						</Button>
					</div>
				</form>
			</SheetContent>
		</Sheet>
	);
}
