import * as v from 'valibot';

export const signInSchema = v.object({
	email: v.pipe(
		v.string(),
		v.nonEmpty('Email is required'),
		v.email('Invalid email address.'),
	),
	password: v.pipe(v.string(), v.nonEmpty('Password is required')),
});

export type SignInSchema = v.InferInput<typeof signInSchema>;
