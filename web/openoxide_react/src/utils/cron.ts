const DAY_NAMES = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

const pad = (value: string) => value.padStart(2, '0');
const every = (field: string) => field === '*';
const numeric = (field: string) => /^\d{1,2}$/.test(field);
const step = (field: string) => /^\*\/\d+$/.test(field);
const stepOf = (field: string) => Number(field.split('/')[1]);

/** `1-5`, `0,6` or a list of day numbers → the words a person would say. */
function dayWords(field: string): string | null {
	if (field === '1-5') return 'Weekdays';
	if (field === '0,6' || field === '6,0') return 'Weekends';
	if (!/^[\d,-]+$/.test(field)) return null;

	const days: number[] = [];
	for (const part of field.split(',')) {
		const [from, to] = part.split('-');
		if (!numeric(from) || Number(from) > 7) return null;
		if (to === undefined) {
			days.push(Number(from) % 7);
			continue;
		}
		if (!numeric(to) || Number(to) > 7) return null;
		for (let i = Number(from); i <= Number(to); i++) days.push(i % 7);
	}
	if (days.length === 0) return null;
	return days.map(d => DAY_NAMES[d]).join(', ');
}

/** Plain-English cadence for a 5-field cron expression — "Every day at 02:00" instead of `0 2 * * *`. Returns null for shapes too unusual to read out, so the caller can fall back to the expression itself. */
export function describeCron(expression?: string): string | null {
	const fields = (expression ?? '').trim().split(/\s+/);
	if (fields.length !== 5) return null;
	const [minute, hour, dayOfMonth, month, dayOfWeek] = fields;
	if (month !== '*') return null;

	if (
		every(minute) &&
		every(hour) &&
		every(dayOfMonth) &&
		every(dayOfWeek)
	)
		return 'Every minute';

	if (step(minute) && every(hour) && every(dayOfMonth) && every(dayOfWeek))
		return `Every ${stepOf(minute)} minutes`;

	const at = numeric(minute) ? `:${pad(minute)}` : null;

	if (at && step(hour) && every(dayOfMonth) && every(dayOfWeek))
		return `Every ${stepOf(hour)} hours${at === ':00' ? '' : ` at ${at}`}`;

	if (at && every(hour) && every(dayOfMonth) && every(dayOfWeek))
		return at === ':00' ? 'Every hour' : `Every hour at ${at}`;

	if (!at || !numeric(hour)) return null;
	const time = `${pad(hour)}:${pad(minute)}`;

	if (every(dayOfMonth) && every(dayOfWeek)) return `Every day at ${time}`;
	if (step(dayOfMonth) && every(dayOfWeek))
		return `Every ${stepOf(dayOfMonth)} days at ${time}`;
	if (numeric(dayOfMonth) && every(dayOfWeek))
		return `On day ${dayOfMonth} of the month at ${time}`;
	if (every(dayOfMonth)) {
		const days = dayWords(dayOfWeek);
		if (!days) return null;
		return days === 'Weekdays' || days === 'Weekends'
			? `${days} at ${time}`
			: `Every ${days} at ${time}`;
	}
	return null;
}
