import type React from 'react';
import {Logo} from '#/components/shared/logo';

type Props = {
	title: string;
	subtitle: string;
};

export const Header: React.FC<Props> = ({title, subtitle}) => (
	<div className="mb-8 flex flex-col items-center gap-2">
		<Logo className="size-16" />
		<h1 className="text-2xl font-bold tracking-tight text-foreground">
			{title}
		</h1>
		<p className="text-center text-sm text-muted-foreground">{subtitle}</p>
	</div>
);

export const Wrapper: React.FC<React.PropsWithChildren> = ({children}) => {
	return (
		<div className="relative z-10 w-full max-w-md rounded-2xl border border-border bg-card/70 p-8 shadow-2xl backdrop-blur-md transition-all hover:border-border">
			{children}
		</div>
	);
};

export const AuthError: React.FC<{message: string | null}> = ({
	message,
}) => {
	if (!message) return null;
	return (
		<div className="animate-in rounded-lg border border-destructive/20 bg-destructive/10 p-3 text-sm text-destructive duration-200 fade-in slide-in-from-top-1">
			{message}
		</div>
	);
};
