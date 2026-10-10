import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/features/logs/bloc/log_feed_bloc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart';

DeploymentEvent _event(String payload, {String type = ''}) =>
    DeploymentEvent(type: type, payload: payload);

/// Two lines of output followed by the connection dying, which is what a panel
/// restart mid-build looks like from the phone.
Stream<DeploymentEvent> _linesThenDrop() async* {
  yield _event('one\n');
  yield _event('two\n');
  throw GrpcError.unavailable('agent went away');
}

/// Lets a bloc run its event queue out, including the batch timers a stream
/// burst schedules.
Future<void> _settled(LogFeedBloc bloc) async {
  for (var i = 0; i < 10; i++) {
    await Future<void>.delayed(const Duration(milliseconds: 20));
  }
}

void main() {
  group('LogFeedBloc', () {
    test(
      'collects whole lines and labels them with the pipeline stage',
      () async {
        final source = StreamController<DeploymentEvent>();
        final bloc = LogFeedBloc(open: () => source.stream);
        final lengths = <int>[];
        final subscription = bloc.stream.listen(
          (state) => lengths.add(state.entries.length),
        );

        bloc.start();
        await _settled(bloc);
        source.add(_event('building started\nstep two\n', type: 'BUILDING'));
        source.add(_event('pushed image\n', type: 'DEPLOYING'));
        await _settled(bloc);

        expect(bloc.buffer.entries.map((entry) => entry.clean), [
          'building started',
          'step two',
          'pushed image',
        ]);
        expect(bloc.buffer.entries.map((entry) => entry.stage), [
          'BUILDING',
          'BUILDING',
          'DEPLOYING',
        ]);
        expect(bloc.state.isRunning, isTrue);
        expect(bloc.state.entries, hasLength(3));
        expect(lengths, isNotEmpty);

        await subscription.cancel();
        await bloc.close();
        await source.close();
      },
    );

    test('repaints a burst of lines once, not once per line', () async {
      final lines = List<String>.generate(60, (i) => 'line $i');
      final bloc = LogFeedBloc(
        open: () => Stream<DeploymentEvent>.fromIterable(
          lines.map((line) => _event('$line\n')),
        ),
      );
      var repaints = 0;
      final subscription = bloc.stream.listen((_) => repaints++);

      bloc.start();
      await _settled(bloc);

      expect(bloc.buffer.entries.length, 60);
      expect(repaints, lessThan(10));

      await subscription.cancel();
      await bloc.close();
    });

    test(
      'a line without its newline lands when the panel closes the channel',
      () async {
        final bloc = LogFeedBloc(
          open: () => Stream<DeploymentEvent>.fromIterable([
            _event('build output\nlast word without a newline'),
          ]),
        );

        bloc.start();
        await _settled(bloc);

        expect(bloc.state.isDone, isTrue);
        expect(bloc.state.isRunning, isFalse);
        expect(bloc.buffer.entries.map((entry) => entry.clean), [
          'build output',
          'last word without a newline',
        ]);

        await bloc.close();
      },
    );

    test('an error payload becomes a line and keeps the raw text', () async {
      final bloc = LogFeedBloc(
        open: () => Stream<DeploymentEvent>.value(
          DeploymentEvent(type: 'BUILDING', error: 'ERROR: build failed'),
        ),
      );

      bloc.start();
      await _settled(bloc);

      expect(bloc.buffer.entries.single.clean, 'build failed');
      expect(bloc.buffer.entries.single.raw, contains('ERROR'));
      expect(bloc.buffer.entries.single.stage, 'BUILDING');

      await bloc.close();
    });

    test(
      'a dropped connection leaves the lines and reports an outage',
      () async {
        final bloc = LogFeedBloc(open: _linesThenDrop);

        bloc.start();
        await _settled(bloc);

        expect(bloc.state.error, isA<PanelUnreachable>());
        expect(bloc.state.isRunning, isFalse);
        expect(bloc.state.isDone, isFalse);
        expect(bloc.buffer.entries.map((entry) => entry.clean), ['one', 'two']);

        await bloc.close();
      },
    );

    test('stop is idempotent and silences a stream that ends later', () async {
      final source = StreamController<DeploymentEvent>();
      final bloc = LogFeedBloc(open: () => source.stream);

      bloc.start();
      await _settled(bloc);
      source.add(_event('mid-build\n'));
      await _settled(bloc);
      await bloc.close();
      await source.close();

      expect(bloc.buffer.entries.map((entry) => entry.clean), ['mid-build']);
      // Closed blocs ignore further events; the lines stay as they were.
      expect(bloc.state.entries.map((entry) => entry.clean), ['mid-build']);
    });

    test('restart after the channel closed clears and follows again', () async {
      var opens = 0;
      final bloc = LogFeedBloc(
        open: () {
          opens++;
          return Stream<DeploymentEvent>.value(_event('run $opens\n'));
        },
      );

      bloc.start();
      await _settled(bloc);
      expect(bloc.state.isDone, isTrue);
      expect(opens, 1);

      bloc.restart();
      await _settled(bloc);

      expect(opens, 2);
      expect(bloc.state.isDone, isTrue);
      expect(bloc.buffer.entries.map((entry) => entry.clean), ['run 2']);

      await bloc.close();
    });
  });
}
