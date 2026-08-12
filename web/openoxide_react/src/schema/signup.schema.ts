import * as v from 'valibot';

export const signUpSchema = v.pipe(
	v.object({
		firstName: v.pipe(v.string(), v.nonEmpty('First name is required')),
		lastName: v.pipe(v.string(), v.nonEmpty('Last name is required')),
		email: v.pipe(
			v.string(),
			v.nonEmpty('Email is required'),
			v.email('Email must be a valid email'),
		),
		password: v.pipe(
			v.string(),
			v.nonEmpty('Password is required'),
			v.minLength(8, 'Password must be at least 8 characters'),
		),
		confirmPassword: v.pipe(
			v.string(),
			v.nonEmpty('Confirm password is required'),
			v.minLength(8, 'Password must be at least 8 characters'),
		),
	}),
	v.forward(
		v.check(
			input => input.password === input.confirmPassword,
			'Passwords do not match',
		),
		['confirmPassword'],
	),
);

export type SignUpSchema = v.InferInput<typeof signUpSchema>;
