/** Hashtags in a project description (`#prod`, `#api`) double as its tag list — stripped of the leading `#` and lowercased. */
export const getTagsFromDescription = (description?: string): string[] => {
	if (!description) return [];
	const matches = description.match(/#[\w-]+/g);
	return matches
		? matches.map(m => m.replace(/^#/, '').toLowerCase())
		: [];
};
