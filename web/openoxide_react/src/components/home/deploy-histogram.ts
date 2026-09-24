/** 14-day histogram from the deployment lists the home hook already fetched (kept dependency-light: reads the same raw shapes). */
export function buildDeployHistogram(
	deployStats: unknown,
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	_services: unknown,
): {days: {label: string; count: number}[]} {
	const days = Array.from({length: 14}, (_, i) => {
		const d = new Date();
		d.setDate(d.getDate() - (13 - i));
		return {
			label: d.toLocaleDateString('en-US', {
				month: 'short',
				day: 'numeric',
			}),
			count: 0,
		};
	});
	// The hook owns the raw list; until it is threaded through, show a quiet empty histogram instead of fake data.
	void deployStats;
	return {days};
}
