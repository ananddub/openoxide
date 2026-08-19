import * as React from 'react';
import {
	CircleCheckIcon,
	InfoIcon,
	Loader2Icon,
	OctagonXIcon,
	TriangleAlertIcon,
} from 'lucide-react';
import {Toaster as Sonner, toast, type ToasterProps} from 'sonner';

import {useTheme} from '#/hooks/use-theme';

export function showToastOnReload(
	message: string,
	type: 'success' | 'error' | 'info' = 'success',
) {
	try {
		sessionStorage.setItem(
			'pending_toast',
			JSON.stringify({message, type}),
		);
	} catch (e) {}
}

const Toaster = ({...props}: ToasterProps) => {
	const {theme} = useTheme();

	// Check pending toast on page refresh / load
	React.useEffect(() => {
		try {
			const pending = sessionStorage.getItem('pending_toast');
			if (pending) {
				sessionStorage.removeItem('pending_toast');
				const {message, type} = JSON.parse(pending);
				if (message) {
					setTimeout(() => {
						if (type === 'error') toast.error(message);
						else if (type === 'info') toast.info(message);
						else toast.success(message);
					}, 150);
				}
			}
		} catch (e) {}
	}, []);

	// Right-click listener to immediately dismiss toast on right click
	React.useEffect(() => {
		const handleContextMenu = (e: MouseEvent) => {
			const toastEl = (e.target as HTMLElement)?.closest(
				'[data-sonner-toast]',
			);
			if (toastEl) {
				e.preventDefault();
				const toastId = toastEl.getAttribute('data-id');
				if (toastId) {
					toast.dismiss(toastId);
				} else {
					toast.dismiss();
				}
			}
		};
		window.addEventListener('contextmenu', handleContextMenu, true);
		return () =>
			window.removeEventListener('contextmenu', handleContextMenu, true);
	}, []);

	return (
		<Sonner
			theme={theme}
			className="toaster group"
			richColors
			icons={{
				success: (
					<CircleCheckIcon className="size-4 shrink-0 text-emerald-400" />
				),
				info: <InfoIcon className="size-4 shrink-0 text-sky-400" />,
				warning: (
					<TriangleAlertIcon className="size-4 shrink-0 text-amber-400" />
				),
				error: <OctagonXIcon className="size-4 shrink-0 text-rose-400" />,
				loading: (
					<Loader2Icon className="size-4 shrink-0 animate-spin text-primary" />
				),
			}}
			toastOptions={{
				style: {
					borderRadius: '10px',
					fontSize: '12px',
					fontWeight: '600',
					padding: '12px 14px',
				},
				classNames: {
					toast:
						'group toast group-[.toaster]:bg-background group-[.toaster]:text-foreground group-[.toaster]:border-border group-[.toaster]:shadow-lg cursor-pointer select-none',
					success:
						'!bg-[#06291a] !text-[#ecfdf5] !border-[#10b981]/60 !shadow-[0_4px_20px_rgba(16,185,129,0.25)]',
					error:
						'!bg-[#2b090e] !text-[#fff1f2] !border-[#ef4444]/60 !shadow-[0_4px_20px_rgba(239,68,68,0.25)]',
					warning:
						'!bg-[#261a07] !text-[#fffbeb] !border-[#f59e0b]/60 !shadow-[0_4px_20px_rgba(245,158,11,0.25)]',
					info: '!bg-[#081a2e] !text-[#f0f9ff] !border-[#0ea5e9]/60 !shadow-[0_4px_20px_rgba(14,165,233,0.25)]',
					description: 'group-[.toast]:text-muted-foreground',
					actionButton:
						'group-[.toast]:bg-primary group-[.toast]:text-primary-foreground',
					cancelButton:
						'group-[.toast]:bg-muted group-[.toast]:text-muted-foreground',
				},
			}}
			{...props}
		/>
	);
};

export {Toaster};
