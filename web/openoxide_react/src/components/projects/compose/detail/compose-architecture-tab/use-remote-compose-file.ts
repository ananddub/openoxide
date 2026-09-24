import {useEffect, useState} from 'react';
import {buildRawGitUrl} from './raw-git-url';

/** Resolve the compose file text, falling back to fetching the raw docker-compose.yml over HTTPS from GitHub/GitLab when the DB value is empty. */
export function useRemoteComposeFile(compose: any): string {
	const [fetched, setFetched] = useState('');

	useEffect(() => {
		if (compose?.compose_file && compose.compose_file.trim()) {
			setFetched(compose.compose_file);
			return;
		}
		const rawUrl = buildRawGitUrl(compose);
		if (!rawUrl) return;
		let isMounted = true;
		fetch(rawUrl)
			.then(res => (res.ok ? res.text() : ''))
			.then(text => {
				if (isMounted && text && text.trim()) setFetched(text);
			})
			.catch(() => {});
		return () => {
			isMounted = false;
		};
	}, [compose]);

	return fetched;
}
