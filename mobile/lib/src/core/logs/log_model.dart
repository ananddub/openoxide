/// The log-line model shared by every log surface in the app — the same shape
/// the web log viewer parses into.
enum LogLevel { all, error, warn, success, debug, info }

enum StreamType { all, stdout, stderr }

class ParsedLogEntry {
  const ParsedLogEntry({
    required this.id,
    required this.raw,
    required this.clean,
    required this.timestamp,
    required this.level,
    required this.stream,
    this.stage,
    this.containerName,
    this.isJson = false,
    this.jsonObject,
  });

  final String id;
  final String raw;
  final String clean;
  final String timestamp;
  final LogLevel level;
  final StreamType stream;
  final String? stage;
  final String? containerName;
  final bool isJson;
  final Map<String, Object?>? jsonObject;

  bool get isError => level == LogLevel.error || stream == StreamType.stderr;
}
