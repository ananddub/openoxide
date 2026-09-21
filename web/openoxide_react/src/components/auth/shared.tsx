import type React from 'react';

type Props = {
	title: string;
	subtitle: string;
};

export const Header: React.FC<Props> = ({title, subtitle}) => (
	<div className="mb-8 flex flex-col items-center gap-2">
		<h1 className="text-2xl font-semibold tracking-[-0.02em] text-foreground">
			{title}
		</h1>
		<p className="text-center text-sm text-muted-foreground">{subtitle}</p>
	</div>
);

export const Wrapper: React.FC<React.PropsWithChildren> = ({children}) => {
	return (
		<div className="relative z-10 w-full max-w-md rounded-xl border bg-card p-8 shadow-[0_1px_1px_rgba(0,0,0,0.02),0_8px_16px_-4px_rgba(0,0,0,0.06),0_24px_32px_-8px_rgba(0,0,0,0.08)]">
			{children}
		</div>
	);
};

export const AuthError: React.FC<{message: string | null}> = ({
	message,
}) => {
	if (!message) return null;
	return (
		<div className="animate-in rounded-md border border-danger-border bg-danger-bg p-3 text-sm text-danger duration-200 fade-in slide-in-from-top-1">
			{message}
		</div>
	);
};
