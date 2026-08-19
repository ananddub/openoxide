import {useState, useMemo, useRef, useEffect} from 'react';
import type {LogLevel, StreamType, ParsedLogEntry} from './types';
import {parseLogEntry} from './log-parser';

interface UseLogViewerOptions {
	logs?: string[] | null;
	isLive?: boolean;
	showFilter?: boolean;
	isDeployment?: boolean;
}

const STAGE_MARKERS = [
	'QUEUED',
	'PREPARING',
	'SOURCE_READY',
	'BUILDING',
	'DEPLOYING',
	'ROUTING',
	'HEALTH_CHECK',
	'DEPLOYED',
];

export function useLogViewer({
	logs,
	isLive = true,
	showFilter = true,
	isDeployment = false,
}: UseLogViewerOptions) {
	const [searchQuery, setSearchQuery] = useState('');
	const [selectedLevel, setSelectedLevel] = useState<LogLevel>('ALL');
	const [selectedStream, setSelectedStream] = useState<StreamType>('ALL');
	const [isAutoScroll, setIsAutoScroll] = useState(isLive);
	const [isWrapLines, setIsWrapLines] = useState(true);
	const [isCompact, setIsCompact] = useState(false);
	const [selectedDetailEntry, setSelectedDetailEntry] =
		useState<ParsedLogEntry | null>(null);

	const scrollRef = useRef<HTMLDivElement>(null);
	const safeLogs = useMemo(
		() => (Array.isArray(logs) ? logs : []),
		[logs],
	);

	// Parse raw logs into rich entries
	const parsedEntries = useMemo(() => {
		let currentStage: string | undefined = undefined;

		return safeLogs.map((raw, idx) => {
			if (isDeployment) {
				const upper = (raw || '').toUpperCase();
				for (const marker of STAGE_MARKERS) {
					if (upper.includes(`[${marker}]`) || upper.trim() === marker) {
						currentStage = marker;
						break;
					}
				}
			}
			return parseLogEntry(raw, idx, currentStage);
		});
	}, [safeLogs, isDeployment]);

	// Filter entries based on search query, level, and stream
	const filteredEntries = useMemo(() => {
		return parsedEntries.filter(entry => {
			if (showFilter && searchQuery.trim()) {
				const query = searchQuery.toLowerCase();
				const matchesText =
					entry.clean.toLowerCase().includes(query) ||
					entry.raw.toLowerCase().includes(query) ||
					(entry.stage && entry.stage.toLowerCase().includes(query));
				if (!matchesText) return false;
			}

			if (selectedLevel !== 'ALL') {
				if (entry.level !== selectedLevel) return false;
			}

			if (selectedStream !== 'ALL') {
				if (entry.stream !== selectedStream) return false;
			}

			return true;
		});
	}, [
		parsedEntries,
		searchQuery,
		selectedLevel,
		selectedStream,
		showFilter,
	]);

	// Auto-scroll effect
	useEffect(() => {
		if (isAutoScroll && scrollRef.current) {
			scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
		}
	}, [filteredEntries.length, isAutoScroll]);

	return {
		scrollRef,
		parsedEntries,
		filteredEntries,
		searchQuery,
		setSearchQuery,
		selectedLevel,
		setSelectedLevel,
		selectedStream,
		setSelectedStream,
		isAutoScroll,
		setIsAutoScroll,
		isWrapLines,
		setIsWrapLines,
		isCompact,
		setIsCompact,
		selectedDetailEntry,
		setSelectedDetailEntry,
	};
}
