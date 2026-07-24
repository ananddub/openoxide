import * as React from 'react';
import type {
	UseFormRegisterReturn,
	FieldError as HookFieldError,
} from 'react-hook-form';
import {Input} from '#/components/ui/input';
import {Lock, Eye, EyeOff} from 'lucide-react';
import {Field, FieldLabel, FieldError} from '#/components/ui/field';

type AuthInputProps = React.ComponentPropsWithoutRef<typeof Input> & {
	label: string;
	icon: React.ComponentType<{className?: string}>;
	error?: HookFieldError;
	registration: UseFormRegisterReturn;
};

export const AuthInput: React.FC<AuthInputProps> = ({
	label,
	id,
	icon: Icon,
	error,
	registration,
	disabled,
	...props
}) => {
	return (
		<Field data-invalid={!!error}>
			<FieldLabel
				htmlFor={id}
				className="text-xs font-semibold tracking-wider text-muted-foreground uppercase">
				{label}
			</FieldLabel>
			<div className="relative">
				<span className="absolute inset-y-0 left-0 flex items-center pl-3.5 text-muted-foreground">
					<Icon className="size-4" />
				</span>
				<Input
					id={id}
					className="h-10 pl-10"
					disabled={disabled}
					aria-invalid={!!error}
					{...registration}
					{...props}
				/>
			</div>
			<FieldError errors={[error]} />
		</Field>
	);
};

type AuthPasswordInputProps = React.ComponentPropsWithoutRef<
	typeof Input
> & {
	label: string;
	error?: HookFieldError;
	registration: UseFormRegisterReturn;
	forgotPasswordLink?: React.ReactNode;
};

export const AuthPasswordInput: React.FC<AuthPasswordInputProps> = ({
	label,
	id,
	error,
	registration,
	disabled,
	forgotPasswordLink,
	...props
}) => {
	const [showPassword, setShowPassword] = React.useState(false);

	return (
		<Field data-invalid={!!error}>
			<div className="flex items-center justify-between">
				<FieldLabel
					htmlFor={id}
					className="text-xs font-semibold tracking-wider text-muted-foreground uppercase">
					{label}
				</FieldLabel>
				{forgotPasswordLink}
			</div>
			<div className="relative">
				<span className="absolute inset-y-0 left-0 flex items-center pl-3.5 text-muted-foreground">
					<Lock className="size-4" />
				</span>
				<Input
					id={id}
					type={showPassword ? 'text' : 'password'}
					className="h-10 pr-10 pl-10"
					disabled={disabled}
					aria-invalid={!!error}
					{...registration}
					{...props}
				/>
				<button
					type="button"
					onClick={() => setShowPassword(!showPassword)}
					className="absolute inset-y-0 right-0 flex items-center pr-3 text-muted-foreground transition-colors hover:text-foreground"
					tabIndex={-1}>
					{showPassword ? (
						<EyeOff className="size-4" />
					) : (
						<Eye className="size-4" />
					)}
				</button>
			</div>
			<FieldError errors={[error]} />
		</Field>
	);
};
