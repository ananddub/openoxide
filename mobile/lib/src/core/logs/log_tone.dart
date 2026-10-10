/// One tone scale for every log surface, mirroring the web's `log-tone.ts`. The
/// classification is theme-free so it can be tested without a widget tree; the
/// viewer maps a [LogTone] to a colour for the active brightness.
enum LogTone {
  error,
  warn,
  queued,
  preparing,
  sourceReady,
  building,
  imageReady,
  deploying,
  routing,
  healthCheck,
  deployed,
  ok,
  command,
  asset,
  plain,
}

final _errorRe = RegExp(
  r'\b(error|errors|failed|failure|fatal|panic|exception)\b',
  caseSensitive: false,
);
final _warnRe = RegExp(
  r'\b(warn|warning|deprecated)\b',
  caseSensitive: false,
);
final _okRe = RegExp(
  r'(^|\s)(✔|✓|done|completed|success|is healthy|built in)',
  caseSensitive: false,
);

/// Unique tone per deploy-pipeline stage tag, in lifecycle order.
final _stageTones = <(RegExp, LogTone)>[
  (RegExp(r'\[(?:QUEUED|PENDING|QUEUE)\]'), LogTone.queued),
  (RegExp(r'\[(?:PREPARING|STARTING)\]'), LogTone.preparing),
  (RegExp(r'\[SOURCE_READY\]'), LogTone.sourceReady),
  (RegExp(r'\[BUILDING\]'), LogTone.building),
  (RegExp(r'\[IMAGE_READY\]'), LogTone.imageReady),
  (RegExp(r'\[DEPLOYING\]'), LogTone.deploying),
  (RegExp(r'\[ROUTING\]'), LogTone.routing),
  (RegExp(r'\[HEALTH_CHECK\]'), LogTone.healthCheck),
  (
    RegExp(r'\[(?:DEPLOYED|DONE|SUCCESS|COMPLETED|HEALTHY|RUNNING)\]'),
    LogTone.deployed,
  ),
];

/// npm-style command echo: `> app@1.0.0 build`.
final _commandRe = RegExp(r'^\s*>');

/// Build output asset lines: dist/index.html, foo-Ba1x.js
final _assetRe = RegExp(
  r'\S+\.(?:js|css|html|jpg|jpeg|png|svg|woff2?|ts|map)\b',
);

/// Leading clock timestamp, bare or bracketed.
final _timestampRe = RegExp(
  r'^(\s*\[?(?:\d{4}-\d{2}-\d{2}[T ])?\d{2}:\d{2}:\d{2}(?:[.,]\d+)?Z?\]?\s+)',
);

LogTone lineTone(String line) {
  if (_errorRe.hasMatch(line)) return LogTone.error;
  if (_warnRe.hasMatch(line)) return LogTone.warn;
  for (final (pattern, tone) in _stageTones) {
    if (pattern.hasMatch(line)) return tone;
  }
  if (_okRe.hasMatch(line)) return LogTone.ok;
  if (_commandRe.hasMatch(line)) return LogTone.command;
  if (_assetRe.hasMatch(line)) return LogTone.asset;
  return LogTone.plain;
}

/// Splits a leading timestamp so the viewer can render it dimmer than the text.
({String timestamp, String rest}) splitTimestamp(String line) {
  final match = _timestampRe.firstMatch(line);
  if (match == null) return (timestamp: '', rest: line);
  final stamp = match.group(1)!;
  return (timestamp: stamp, rest: line.substring(stamp.length));
}

/// The viewer's colour ramp: the same palette the web uses, one entry per tone
/// and brightness.
({int light, int dark}) toneColors(LogTone tone) => switch (tone) {
  LogTone.error => (light: 0xFFDC2626, dark: 0xFFF87171),
  LogTone.warn => (light: 0xFFD97706, dark: 0xFFF59F0A),
  LogTone.queued => (light: 0xFF64748B, dark: 0xFF94A3B8),
  LogTone.preparing => (light: 0xFF7C3AED, dark: 0xFFA78BFA),
  LogTone.sourceReady => (light: 0xFF4F46E5, dark: 0xFF818CF8),
  LogTone.building => (light: 0xFFD97706, dark: 0xFFF59F0A),
  LogTone.imageReady => (light: 0xFF0D9488, dark: 0xFF2DD4BF),
  LogTone.deploying => (light: 0xFF0284C7, dark: 0xFF38BDF8),
  LogTone.routing => (light: 0xFFC026D3, dark: 0xFFE879F9),
  LogTone.healthCheck => (light: 0xFFEA580C, dark: 0xFFFB923C),
  LogTone.deployed || LogTone.ok => (light: 0xFF10B981, dark: 0xFF34D399),
  LogTone.command => (light: 0xFF171717, dark: 0xFFFFFFFF),
  LogTone.asset => (light: 0xFF059669, dark: 0xFF34D399),
  LogTone.plain => (light: 0xFF262626, dark: 0xFFE5E5E5),
};
