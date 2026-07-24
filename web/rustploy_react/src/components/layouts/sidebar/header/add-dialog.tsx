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
import {Field, FieldLabel, FieldError} from '#/components/ui/field';
import {$api} from '#/api/query';
import {useQueryClient} from '@tanstack/react-query';
import {useOrganizationStore} from '#/stores/organization-store';
import {formatApiError} from '#/api/utils';

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
	const queryClient = useQueryClient();
	const orgList = useOrganizationStore(state => state.organizations);

	const createMutation = $api.useMutation('post', '/organizations');
	const patchMutation = $api.useMutation('patch', '/organizations/{id}');

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
				const org = orgList.find(
					o => String(o.id) === organizationId,
				);
				reset({
					name: org ? org.name : '',
					logo: org ? org.logo || '' : '',
				});
			} else {
				reset({name: '', logo: ''});
			}
		}
	}, [organizationId, reset, open, isEdit, orgList]);

	const onSubmit = async (data: OrganizationFormValues) => {
		setIsPending(true);
		try {
			if (isEdit && organizationId) {
				await patchMutation.mutateAsync({
					params: {
						path: {
							id: Number(organizationId),
						},
					},
					body: {
						name: data.name,
						logo: data.logo || undefined,
					},
				});

				toast.success('Organization updated successfully');
				queryClient.invalidateQueries({queryKey: ['get', '/organizations']});
				onOpenChange(false);
			} else {
				await createMutation.mutateAsync({
					body: {
						name: data.name,
						logo: data.logo || undefined,
					},
				});

				toast.success('Organization created successfully');
				queryClient.invalidateQueries({queryKey: ['get', '/organizations']});
				onOpenChange(false);
			}
		} catch (err: any) {
			toast.error(formatApiError(err));
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
