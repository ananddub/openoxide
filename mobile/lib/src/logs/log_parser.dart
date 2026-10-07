import 'dart:convert';

import 'log_model.dart';

/// Port of the web log viewer's parsing rules, kept free of Flutter types so it
/// runs in a plain `dart test`.

final _errorWord = RegExp(r'\berror\b');
final _failedWord = RegExp(r'\bfailed\b');
final _exceptionWord = RegExp(r'\bexception\b');
final _warnWord = RegExp(r'\bwarn(ing)?\b');
final _successWord = RegExp(r'\bsuccess\b');
final _deployedWord = RegExp(r'\bdeployed\b');
final _builtWord = RegExp(r'\bbuilt\b');
final _debugWord = RegExp(r'\bdebug\b');
final _traceWord = RegExp(r'\btrace\b');

/// `"Error": ""` is a JSON key with an empty value, not a failure.
final _emptyErrorKey = RegExp(r'"[Ee]rror"\s*:\s*""');

/// Server tracing wrapper: `... openoxide::...: command output ... line=<text>`.
final _tracingLine = RegExp(
  r'.*(?:INFO|ERROR|WARN|DEBUG|TRACE)\s+openoxide::[^:]+:\s*command\s+output.*?line=(.*)',
  caseSensitive: false,
);

final _leadingLevel = RegExp(r'^(INFO|ERROR|WARN|DEBUG|TRACE):\s*', caseSensitive: false);

/// Bracketed or bare ISO-8601 stamp at the head of a line.
final _timestampTag = RegExp(
  r'^(\[\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})?\]'
  r'|\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})?)\s*(.*)',
  caseSensitive: false,
);

/// `[container:web] ...`, `[service:db] ...`, or `container=app-1 ...`.
final _namedTag = RegExp(
  r'^\[(?:container|service|app):\s*([^\]]+)\]\s*(.*)',
  caseSensitive: false,
);
final _namedPair = RegExp(
  r'^(?:container|service)=([a-zA-Z0-9_-]+)\s*(.*)',
  caseSensitive: false,
);

LogLevel logLevelOf(String line) {
  if (line.isEmpty) return LogLevel.info;
  final lower = line.toLowerCase();
  if (_errorWord.hasMatch(lower) ||
      _failedWord.hasMatch(lower) ||
      lower.contains('err!') ||
      lower.contains('❌') ||
      _exceptionWord.hasMatch(lower) ||
      lower.contains('stderr')) {
    if (!_emptyErrorKey.hasMatch(line)) return LogLevel.error;
  }
  if (_warnWord.hasMatch(lower)) return LogLevel.warn;
  if (_successWord.hasMatch(lower) ||
      _deployedWord.hasMatch(lower) ||
      _builtWord.hasMatch(lower) ||
      lower.contains('done 0.')) {
    return LogLevel.success;
  }
  if (_debugWord.hasMatch(lower) || _traceWord.hasMatch(lower)) {
    return LogLevel.debug;
  }
  return LogLevel.info;
}

/// Drops stream framing noise (`event:`, `id:`, keep-alives) and the server's
/// own tracing prefix, leaving only what the deployed process printed.
String cleanLogLine(String line) {
  if (line.isEmpty) return '';
  final trimmed = line.trim();
  if (trimmed.startsWith('event:') ||
      trimmed.contains('event: log') ||
      trimmed.contains('event: deployment') ||
      trimmed.contains('event: keep-alive') ||
      trimmed.startsWith('id:') ||
      trimmed == ':' ||
      trimmed.contains('keep-alive')) {
    return '';
  }

  var cleaned = trimmed;
  final tracing = _tracingLine.firstMatch(cleaned);
  if (tracing != null) {
    cleaned = tracing.group(1)!.trim();
    if ((cleaned.startsWith('"') && cleaned.endsWith('"')) ||
        (cleaned.startsWith("'") && cleaned.endsWith("'"))) {
      cleaned = cleaned.substring(1, cleaned.length - 1);
    }
  }

  return cleaned.replaceFirst(_leadingLevel, '');
}

ParsedLogEntry parseLogEntry(String raw, int index, {String? stage}) {
  var clean = cleanLogLine(raw);
  var isJson = false;
  Map<String, Object?>? jsonObject;

  String? containerName;
  var stream = StreamType.stdout;
  var timestamp = DateTime.now().toUtc().toIso8601String();
  var level = logLevelOf(clean.isEmpty ? raw : clean);

  if (clean.startsWith('{') && clean.endsWith('}')) {
    try {
      final parsed = jsonDecode(clean);
      if (parsed is Map<String, Object?>) {
        jsonObject = parsed;
        isJson = true;

        containerName = _firstString(
          parsed,
          const ['container', 'service', 'app', 'container_name', 'name'],
        );

        final jsonStream = parsed['stream'] ?? parsed['std'];
        if (jsonStream is String) {
          final lower = jsonStream.toLowerCase().trim();
          if (lower == 'stderr') {
            stream = StreamType.stderr;
          } else if (lower == 'stdout') {
            stream = StreamType.stdout;
          }
        }

        final jsonLevel = parsed['level'] ?? parsed['severity'];
        if (jsonLevel is String) level = logLevelOf(jsonLevel);

        final jsonTime = _firstString(
          parsed,
          const ['time', 'timestamp', 'date', 'datetime'],
        );
        if (jsonTime != null) {
          timestamp = jsonTime.replaceAll(RegExp(r'^\[|\]$'), '');
        }

        final message = _firstString(
          parsed,
          const ['line', 'log', 'message', 'msg', 'text', 'content'],
        );
        if (message != null) clean = message;
      }
    } on FormatException {
      // Not JSON after all: fall through to the plain-text rules.
    }
  }

  if (!isJson) {
    if (clean.startsWith('[STDERR]')) {
      stream = StreamType.stderr;
      clean = clean.replaceFirst(RegExp(r'^\[STDERR\]\s*'), '');
    } else if (raw.toLowerCase().contains('stderr')) {
      stream = StreamType.stderr;
    }

    final stamp = _timestampTag.firstMatch(clean);
    if (stamp != null) {
      timestamp = stamp.group(1)!.replaceAll(RegExp(r'^\[|\]$'), '');
      clean = stamp.group(2) ?? '';
    }

    final named = _namedTag.firstMatch(clean) ?? _namedPair.firstMatch(clean);
    if (named != null) {
      containerName = named.group(1);
      clean = named.group(2) ?? '';
    }
  }

  return ParsedLogEntry(
    id: 'log-$index',
    raw: raw,
    clean: clean.isEmpty ? raw : clean,
    timestamp: timestamp,
    level: level,
    stream: stream,
    stage: stage,
    containerName: containerName,
    isJson: isJson,
    jsonObject: jsonObject,
  );
}

String? _firstString(Map<String, Object?> map, List<String> keys) {
  for (final key in keys) {
    final value = map[key];
    if (value is String && value.trim().isNotEmpty) return value.trim();
  }
  return null;
}
