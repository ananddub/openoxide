import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/logs/log_model.dart';
import 'package:mobile/src/core/logs/log_parser.dart';

void main() {
  group('cleanLogLine', () {
    test('drops stream framing and keep-alives', () {
      expect(cleanLogLine('event: log'), '');
      expect(cleanLogLine('id: 42'), '');
      expect(cleanLogLine(': keep-alive'), '');
      expect(cleanLogLine(''), '');
    });

    test('unwraps the server tracing wrapper and its quotes', () {
      const raw =
          '2026-10-06T03:11:00.000Z  INFO openoxide::deployment: command output line="npm run build"';
      expect(cleanLogLine(raw), 'npm run build');
    });

    test('strips a leading level tag', () {
      expect(cleanLogLine('WARN: disk almost full'), 'disk almost full');
    });
  });

  group('logLevelOf', () {
    test('classifies by keyword', () {
      expect(logLevelOf('error during build'), LogLevel.error);
      expect(logLevelOf('Command failed with exit code 1'), LogLevel.error);
      expect(logLevelOf('warning: deprecated flag'), LogLevel.warn);
      expect(logLevelOf('image built in 12s'), LogLevel.success);
      expect(logLevelOf('debug resolver attached'), LogLevel.debug);
      expect(logLevelOf('listening on 3000'), LogLevel.info);
    });

    test('an empty JSON error field is not a failure', () {
      expect(logLevelOf('{"Error":"","level":"info"}'), LogLevel.info);
    });
  });

  group('parseLogEntry', () {
    test('reads a json container log line', () {
      const raw =
          '{"container":"web-1","stream":"stderr","level":"error",'
          '"time":"2026-10-06T03:11:00Z","message":"boom"}';
      final entry = parseLogEntry(raw, 7, stage: 'DEPLOYING');
      expect(entry.isJson, isTrue);
      expect(entry.id, 'log-7');
      expect(entry.clean, 'boom');
      expect(entry.containerName, 'web-1');
      expect(entry.stream, StreamType.stderr);
      expect(entry.level, LogLevel.error);
      expect(entry.timestamp, '2026-10-06T03:11:00Z');
      expect(entry.stage, 'DEPLOYING');
    });

    test('reads a plain line with a timestamp and a container tag', () {
      final entry = parseLogEntry(
        '[2026-10-06 03:11:00] [container:api] GET /healthz 200',
        0,
      );
      expect(entry.timestamp, '2026-10-06 03:11:00');
      expect(entry.containerName, 'api');
      expect(entry.clean, 'GET /healthz 200');
      expect(entry.isJson, isFalse);
    });

    test('marks a bare stderr line as an error', () {
      final entry = parseLogEntry('[STDERR] permission denied', 1);
      expect(entry.stream, StreamType.stderr);
      expect(entry.clean, 'permission denied');
      expect(entry.isError, isTrue);
    });

    test('keeps a line that only looks like json', () {
      final entry = parseLogEntry('{not json}', 2);
      expect(entry.isJson, isFalse);
      expect(entry.clean, '{not json}');
    });

    test('falls back to the raw line when cleaning empties it', () {
      final entry = parseLogEntry('plain text', 3);
      expect(entry.clean, 'plain text');
    });
  });
}
