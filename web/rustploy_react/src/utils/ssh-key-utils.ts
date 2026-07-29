import {toast} from 'sonner';

export const downloadKeyFile = (filename: string, content: string) => {
	if (!content) {
		toast.error('Key content is empty');
		return;
	}
	const blob = new Blob([content], {type: 'text/plain;charset=utf-8'});
	const url = URL.createObjectURL(blob);
	const link = document.createElement('a');
	link.href = url;
	link.download = filename;
	document.body.appendChild(link);
	link.click();
	document.body.removeChild(link);
	URL.revokeObjectURL(url);
	toast.success(`Downloaded ${filename}`);
};
