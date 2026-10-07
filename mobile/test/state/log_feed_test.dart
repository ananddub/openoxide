import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart';
import 'package:mobile/src/grpc/app_error.dart';
import 'package:mobile/src/state/log_feed.dart';

DeploymentEvent _event(String payload, {String type = ''}) =>
    DeploymentEvent(type: type, payload: payload);

/// Two lines of output followed by the connection dying, which is what a panel
/// restart mid-build looks like from the phone.
Stream<DeploymentEvent> _linesThenDrop() async* {
  yield _event('one\n');
  yield _event('two\n');
  throw GrpcError.unavailable('agent went away');
}

void main() {
  group('LogFeed', () {
    test('collects whole lines and labels them with the pipeline stage', () async {
      final source = StreamController<DeploymentEvent>();
      final feed = LogFeed(open: () => source.stream);
      final repaints = <int>[];
      feed.addListener(() => repaints.add(feed.buffer.entries.length));

      await feed.start();
      source.add(_event('building started\nstep two\n', type: 'BUILDING'));
      source.add(_event('pushed image\n', type: 'DEPLOYING'));
      await Future<void>.delayed(const Duration(milliseconds: 200));

      expect(
        feed.buffer.entries.map((entry) => entry.clean),
        ['building started', 'step two', 'pushed image'],
      );
      expect(
        feed.buffer.entries.map((entry) => entry.stage),
        ['BUILDING', 'BUILDING', 'DEPLOYING'],
      );
      expect(feed.isRunning, isTrue);
      expect(repaints, isNotEmpty);

      await feed.stop();
      feed.dispose();
      await source.close();
    });

    test('repaints a burst of lines once, not once per line', () async {
      final lines = List<String>.generate(60, (i) => 'line $i');
      final feed = LogFeed(
        open: () => Stream<DeploymentEvent>.fromIterable(
          lines.map((line) => _event('$line\n')),
        ),
      );
      var repaints = 0;
      feed.addListener(() => repaints++);

      await feed.start();
      await Future<void>.delayed(const Duration(milliseconds: 300));

      expect(feed.buffer.entries.length, 60);
      expect(repaints, lessThan(10));

      feed.dispose();
    });

    test('a line without its newline lands when the panel closes the channel',
        () async {
      var ended = 0;
      final feed = LogFeed(
        open: () => Stream<DeploymentEvent>.fromIterable([
          _event('build output\nlast word without a newline'),
        ]),
        onEnded: () => ended++,
      );

      await feed.start();
      await Future<void>.delayed(Duration.zero);

      expect(feed.isDone, isTrue);
      expect(feed.isRunning, isFalse);
      expect(
        feed.buffer.entries.map((entry) => entry.clean),
        ['build output', 'last word without a newline'],
      );
      expect(ended, 1);

      feed.dispose();
    });

    test('an error payload becomes a line and keeps the raw text', () async {
      final feed = LogFeed(
        open: () => Stream<DeploymentEvent>.value(
          DeploymentEvent(type: 'BUILDING', error: 'ERROR: build failed'),
        ),
      );

      await feed.start();
      await Future<void>.delayed(Duration.zero);

      expect(feed.buffer.entries.single.clean, 'build failed');
      expect(feed.buffer.entries.single.raw, contains('ERROR'));
      expect(feed.buffer.entries.single.stage, 'BUILDING');

      feed.dispose();
    });

    test('a dropped connection leaves the lines and reports an outage', () async {
      final feed = LogFeed(open: _linesThenDrop);

      await feed.start();
      await Future<void>.delayed(const Duration(milliseconds: 50));

      expect(feed.error, isA<PanelUnreachable>());
      expect(feed.isRunning, isFalse);
      expect(feed.isDone, isFalse);
      expect(feed.buffer.entries.map((entry) => entry.clean), ['one', 'two']);

      feed.dispose();
    });

    test('stop is idempotent and silences a stream that ends later', () async {
      final source = StreamController<DeploymentEvent>();
      var ended = 0;
      final feed = LogFeed(open: () => source.stream, onEnded: () => ended++);

      await feed.start();
      source.add(_event('mid-build\n'));
      await Future<void>.delayed(const Duration(milliseconds: 20));
      await feed.stop();
      await source.close();

      expect(feed.buffer.entries.map((entry) => entry.clean), ['mid-build']);
      expect(ended, 0);
      await feed.stop();
      expect(ended, 0);

      feed.dispose();
    });

    test('restart after the channel closed follows the stream again', () async {
      var opens = 0;
      final feed = LogFeed(
        open: () {
          opens++;
          return Stream<DeploymentEvent>.value(_event('run $opens\n'));
        },
      );

      await feed.start();
      await Future<void>.delayed(const Duration(milliseconds: 20));
      expect(feed.isDone, isTrue);
      expect(opens, 1);

      await feed.stop();
      await feed.start();
      await Future<void>.delayed(const Duration(milliseconds: 20));

      expect(opens, 2);
      expect(feed.isDone, isTrue);
      expect(feed.buffer.entries.map((entry) => entry.clean), ['run 1', 'run 2']);

      feed.dispose();
    });
  });
}
