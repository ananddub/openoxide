import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/logs/log_tone.dart';

void main() {
  group('lineTone', () {
    test('gives each pipeline stage its own tone', () {
      expect(lineTone('[QUEUED] waiting for a runner'), LogTone.queued);
      expect(lineTone('[PREPARING] clone'), LogTone.preparing);
      expect(lineTone('[SOURCE_READY] commit abc123'), LogTone.sourceReady);
      expect(lineTone('[BUILDING] docker build'), LogTone.building);
      expect(lineTone('[IMAGE_READY] sha256:1234'), LogTone.imageReady);
      expect(lineTone('[DEPLOYING] service web'), LogTone.deploying);
      expect(lineTone('[ROUTING] traefik'), LogTone.routing);
      expect(lineTone('[HEALTH_CHECK] waiting'), LogTone.healthCheck);
      expect(lineTone('[DEPLOYED] 2 replicas'), LogTone.deployed);
    });

    test('a failure outranks the stage it happened in', () {
      expect(lineTone('[BUILDING] error: cannot resolve base image'),
          LogTone.error);
      expect(lineTone('warning: legacy field'), LogTone.warn);
    });

    test('recognises completion, commands and asset lines', () {
      expect(lineTone('✔ deployed in 12s'), LogTone.ok);
      expect(lineTone('> app@1.0.0 build'), LogTone.command);
      expect(lineTone('dist/index-Ba1x.js  12.4 kB'), LogTone.asset);
      expect(lineTone('just some output'), LogTone.plain);
    });
  });

  group('splitTimestamp', () {
    test('separates a bare clock stamp', () {
      final (:timestamp, :rest) = splitTimestamp('03:11:00 pulling image');
      expect(timestamp.trim(), '03:11:00');
      expect(rest, 'pulling image');
    });

    test('separates a bracketed date-time stamp', () {
      final (:timestamp, :rest) = splitTimestamp(
        '[2026-10-06T03:11:00.123Z] build started',
      );
      expect(timestamp.trim(), '[2026-10-06T03:11:00.123Z]');
      expect(rest, 'build started');
    });

    test('leaves a line without a stamp alone', () {
      final (:timestamp, :rest) = splitTimestamp('no stamp here');
      expect(timestamp, isEmpty);
      expect(rest, 'no stamp here');
    });
  });

  test('every tone has a distinct dark colour from its light one', () {
    for (final tone in LogTone.values) {
      final colors = toneColors(tone);
      expect(colors.dark, isNonZero);
      expect(colors.light, isNonZero);
    }
    expect(toneColors(LogTone.error).light, isNot(toneColors(LogTone.ok).light));
  });
}
