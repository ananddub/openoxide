import {useState} from 'react';
import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {Database, Package, Plug, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Titlebar} from '#/components/shared/titlebar';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useRegistryCreate, useRegistryTest} from '#/api/live';
import {RegistryFormFields} from '#/components/registry/registry-form-fields';

export const Route = createFileRoute('/_app/registry/new')({
	component: RegistryCreatePage,
});

function RegistryCreatePage() {
	const navigate = useNavigate();
	const [name, setName] = useState('');
	const [registryUrl, setRegistryUrl] = useState('');
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');
	const [serverId, setServerId] = useState<string>('local');
	const [isTesting, setIsTesting] = useState(false);
	const [submitting, setSubmitting] = useState(false);

	const createMutation = useRegistryCreate();
	const testRawMutation = useRegistryTest();

	const handleTestRaw = async () => {
		if (!registryUrl || !username) {
			toast.error('Image Prefix (URL) and Username are required to test');
			return;
		}
		if (!password) {
			toast.error('Password is required to test connection');
			return;
		}
		setIsTesting(true);
		try {
			await testRawMutation.mutateAsync({
				registry_url: registryUrl,
				username,
				password,
			});
			toast.success('Registry authentication test passed!');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsTesting(false);
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name || !registryUrl || !username || !password) {
			toast.error('Please fill in all required fields');
			return;
		}
		setSubmitting(true);
		try {
			await createMutation.mutateAsync({
				name,
				registry_url: registryUrl,
				username,
				password,
				server_id: serverId === 'local' ? undefined : parseInt(serverId),
			});
			toast.success('Registry added successfully');
			void navigate({to: '/registry'});
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			<Titlebar
				breadcrumbs={[
					{label: 'Registries', to: '/registry', icon: Package},
					{label: 'Add Registry'},
				]}
				icon={<Database className="text-foreground/70" />}
				title="Add an external registry"
				description="Fill in the following fields to add an external registry."
			/>

			<div className="mx-auto w-full max-w-2xl min-w-0 rounded-xl bg-card p-6 ring-1 ring-border/50">
				<form
					onSubmit={handleSubmit}
					className="flex flex-col gap-4 text-xs">
					<RegistryFormFields
						name={name}
						setName={setName}
						registryUrl={registryUrl}
						setRegistryUrl={setRegistryUrl}
						username={username}
						setUsername={setUsername}
						password={password}
						setPassword={setPassword}
						serverId={serverId}
						setServerId={setServerId}
					/>

					<div className="mt-1 flex items-center justify-between border-t border-border/50 pt-4">
						<Button
							type="button"
							variant="outline"
							size="icon"
							onClick={handleTestRaw}
							disabled={isTesting}
							title="Test Registry Connection"
							className="h-9 w-9 shrink-0">
							{isTesting ? (
								<RefreshCw className="h-4 w-4 animate-spin text-primary" />
							) : (
								<Plug className="h-4 w-4 text-muted-foreground hover:text-foreground" />
							)}
						</Button>

						<Button
							type="submit"
							disabled={submitting}
							className="h-9 px-5 text-xs font-semibold">
							{submitting ? 'Saving...' : 'Save Registry'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);
}
