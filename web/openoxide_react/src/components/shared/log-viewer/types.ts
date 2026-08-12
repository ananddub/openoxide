export type LogLevel = 'ALL' | 'ERROR' | 'WARN' | 'SUCCESS' | 'DEBUG' | 'INFO';
export type StreamType = 'ALL' | 'STDOUT' | 'STDERR';

export interface ParsedLogEntry {
	id: string;
	raw: string;
	clean: string;
	timestamp?: string;
	level: LogLevel;
	stream: StreamType;
	stage?: string;
	containerName?: string;
	isJson: boolean;
	jsonObject?: Record<string, unknown>;
}

export interface LogViewerOptions {
	searchQuery: string;
	setSearchQuery: (query: string) => void;
	selectedLevel: LogLevel;
	setSelectedLevel: (level: LogLevel) => void;
	selectedStream: StreamType;
	setSelectedStream: (stream: StreamType) => void;
	isAutoScroll: boolean;
	setIsAutoScroll: (auto: boolean) => void;
	isWrapLines: boolean;
	setIsWrapLines: (wrap: boolean) => void;
	isCompact: boolean;
	setIsCompact: (compact: boolean) => void;
}
