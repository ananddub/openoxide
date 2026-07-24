import * as v from 'valibot';
import * as React from 'react';
import {useForm} from 'react-hook-form';
import {Button} from '#/components/ui/button';
import {valibotResolver} from '@hookform/resolvers/valibot';
import {Loader2} from 'lucide-react';
import {
	Dialog,
	DialogTitle,
	DialogFooter,
	DialogHeader,
	DialogContent,
	DialogDescription,
} from '#/components/ui/dialog';
import {toast} from 'sonner';
import {Input} from '#/components/ui/input';
import {organizations} from '#/consts/data';
import {Field, FieldLabel, FieldError} from '#/components/ui/field';

const organizationSchema = v.object({
	name: v.pipe(
		v.string(),
		v.minLength(1, 'Organization name is required'),
	),
	logo: v.optional(v.string()),
});

type OrganizationFormValues = v.InferOutput<typeof organizationSchema>;
type Props = {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	organizationId?: string;
};

// Doubles as add and edit dialog depending on whether organizationId is provided.
export function AddOrganization({
	open,
	onOpenChange,
	organizationId,
}: Props) {
	const isEdit = !!organizationId;
	const [isPending, setIsPending] = React.useState(false);

	const {
		register,
		handleSubmit,
		formState: {errors},
		reset,
	} = useForm<OrganizationFormValues>({
		resolver: valibotResolver(organizationSchema),
		defaultValues: {name: '', logo: ''},
	});

	// Reset form when dialog opens.
	React.useEffect(() => {
		if (open) {
			if (isEdit) {
				const org = organizations.find(
					o => String(o.id) === organizationId,
				);
				reset({
					name: org ? org.name : 'Goploy Self-Hosted',
					logo: org ? org.logo : 'https://example.com/logo.png',
				});
			} else {
				reset({name: '', logo: ''});
			}
		}
	}, [organizationId, reset, open, isEdit]);

	const onSubmit = async (_: OrganizationFormValues) => {
		setIsPending(true);
		try {
			// Simulate api delay
			await new Promise(resolve => setTimeout(resolve, 800));
			toast.success(
				`Organization ${isEdit ? 'updated' : 'created'} successfully (Mock)`,
			);
			// Close dialog upon successful save
			onOpenChange(false);
		} catch (error) {
			console.error(error);
			toast.error('Failed to save organization');
		} finally {
			setIsPending(false);
		}
	};

	return (
		<Dialog open={open} onOpenChange={onOpenChange}>
			<DialogContent className="sm:max-w-106.25">
				<DialogHeader>
					<DialogTitle>
						{isEdit ? 'Update organization' : 'Add organization'}
					</DialogTitle>
					<DialogDescription>
						{isEdit
							? 'Update the organization name and logo'
							: 'Create a new organization to manage your projects.'}
					</DialogDescription>
				</DialogHeader>
				<form
					onSubmit={handleSubmit(onSubmit)}
					className="flex flex-col gap-4">
					<Field>
						<FieldLabel htmlFor="name">Name</FieldLabel>
						<Input
							id="name"
							placeholder="Organization name"
							disabled={isPending}
							{...register('name')}
						/>
						<FieldError>{errors.name?.message}</FieldError>
					</Field>
					<Field>
						<FieldLabel htmlFor="logo">Logo URL</FieldLabel>
						<Input
							id="logo"
							placeholder="https://example.com/logo.png"
							disabled={isPending}
							{...register('logo')}
						/>
						<FieldError>{errors.logo?.message}</FieldError>
					</Field>
					<DialogFooter className="mt-2">
						<Button
							type="submit"
							disabled={isPending}
							className="flex gap-2">
							{isPending && <Loader2 className="size-4 animate-spin" />}
							{isEdit ? 'Update organization' : 'Create organization'}
						</Button>
					</DialogFooter>
				</form>
			</DialogContent>
		</Dialog>
	);
}
