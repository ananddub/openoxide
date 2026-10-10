import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart';
import 'package:mobile/src/core/data/query_bloc.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/realtime/realtime_watch.dart';
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pb.dart';

const _projects = WatchedTable.WATCHED_TABLE_PROJECTS;
const _domains = WatchedTable.WATCHED_TABLE_DOMAINS;

/// A watch whose events the test pushes by hand, so a tick is a statement about
/// the world rather than a race against the panel.
class _WatchHarness {
  _WatchHarness() : _source = StreamController<WatchTablesEvent>();

  final StreamController<WatchTablesEvent> _source;
  late final RealtimeWatch watch = RealtimeWatch(open: () => _source.stream);

  void tick(WatchedTable table) =>
      _source.add(WatchTablesEvent(tables: [table]));

  /// Longer than `QueryBloc`'s tick cooldown, so a scheduled refetch has run.
  static Future<void> settle() =>
      Future<void>.delayed(const Duration(milliseconds: 700));

  Future<void> dispose() async {
    await watch.dispose();
    // The watch already cancelled its subscription, so the controller's done
    // future has no listener to complete against — fire it, never await it.
    unawaited(_source.close());
  }
}

/// Lets a bloc run its event queue out. The loads are immediate futures, so a
/// few event-loop turns are always enough.
Future<void> _settled(QueryBloc<dynamic> bloc) async {
  for (var i = 0; i < 10; i++) {
    await Future<void>.delayed(Duration.zero);
  }
}

void main() {
  group('QueryBloc', () {
    test('start loads once and keeps the result', () async {
      final harness = _WatchHarness();
      var loads = 0;
      final bloc = QueryBloc<int>(
        load: () async => ++loads,
        realtime: harness.watch,
        listensOn: const [_projects],
      )..start();
      await _settled(bloc);

      expect(loads, 1);
      expect(bloc.state.data, 1);
      expect(bloc.state.error, isNull);
      expect(bloc.state.loading, isFalse);

      await bloc.close();
      await harness.dispose();
    });

    test('a burst of ticks for a watched table is one refetch', () async {
      final harness = _WatchHarness()..watch.start();
      var loads = 0;
      final bloc = QueryBloc<int>(
        load: () async => ++loads,
        realtime: harness.watch,
        listensOn: const [_projects],
      )..start();
      await _settled(bloc);
      expect(loads, 1);

      harness.tick(_projects);
      harness.tick(_projects);
      harness.tick(_projects);
      await _WatchHarness.settle();
      await _settled(bloc);

      expect(loads, 2);
      expect(bloc.state.data, 2);

      await bloc.close();
      await harness.dispose();
    });

    test('a tick for a table the screen ignores is not a refetch', () async {
      final harness = _WatchHarness()..watch.start();
      var loads = 0;
      final bloc = QueryBloc<int>(
        load: () async => ++loads,
        realtime: harness.watch,
        listensOn: const [_projects],
      )..start();
      await _settled(bloc);

      harness.tick(_domains);
      await _WatchHarness.settle();
      await _settled(bloc);

      expect(loads, 1);

      await bloc.close();
      await harness.dispose();
    });

    test(
      'a failed refetch keeps the stale rows and reports the error',
      () async {
        final harness = _WatchHarness()..watch.start();
        var loads = 0;
        final bloc = QueryBloc<List<String>>(
          load: () {
            loads++;
            if (loads == 1) return Future.value(['row']);
            return Future<List<String>>.error(GrpcError.unavailable('down'));
          },
          realtime: harness.watch,
          listensOn: const [_projects],
        )..start();
        await _settled(bloc);

        harness.tick(_projects);
        await _WatchHarness.settle();
        await _settled(bloc);

        expect(loads, 2);
        expect(bloc.state.data, ['row']);
        expect(bloc.state.error, isA<PanelUnreachable>());

        await bloc.close();
        await harness.dispose();
      },
    );

    test(
      'a tick that lands during the first fetch still earns a refetch',
      () async {
        final harness = _WatchHarness()..watch.start();
        final gate = Completer<List<String>>();
        var loads = 0;
        final bloc = QueryBloc<List<String>>(
          load: () {
            loads++;
            return loads == 1 ? gate.future : Future.value(['fresh']);
          },
          realtime: harness.watch,
          listensOn: const [_projects],
        )..start();
        await _settled(bloc);

        harness.tick(_projects);
        gate.complete(['first']);
        await _WatchHarness.settle();
        await _settled(bloc);

        expect(loads, 2);
        expect(bloc.state.data, ['fresh']);

        await bloc.close();
        await harness.dispose();
      },
    );

    test('close stops the stream of refetches', () async {
      final harness = _WatchHarness()..watch.start();
      var loads = 0;
      final bloc = QueryBloc<int>(
        load: () async => ++loads,
        realtime: harness.watch,
        listensOn: const [_projects],
      )..start();
      await _settled(bloc);

      await bloc.close();
      harness.tick(_projects);
      await _WatchHarness.settle();

      expect(loads, 1);
      await harness.dispose();
    });

    test('refresh refetches without waiting for a tick', () async {
      final harness = _WatchHarness()..watch.start();
      var loads = 0;
      final bloc = QueryBloc<int>(
        load: () async => ++loads,
        realtime: harness.watch,
        listensOn: const [_projects],
      )..start();
      await _settled(bloc);

      bloc.refresh();
      await _settled(bloc);
      expect(loads, 2);

      await bloc.close();
      await harness.dispose();
    });
  });

  group('RealtimeWatch', () {
    test('broadcasts the moved tables per tick', () async {
      final source = StreamController<WatchTablesEvent>();
      final watch = RealtimeWatch(open: () => source.stream)..start();

      final seen = <Set<WatchedTable>>[];
      final subscription = watch.ticks.listen(seen.add);

      source.add(WatchTablesEvent(tables: [_projects, _domains]));
      source.add(WatchTablesEvent(tables: [_projects]));
      // A tick naming no table is a resync marker, not a change.
      source.add(WatchTablesEvent());
      await Future<void>.delayed(Duration.zero);

      expect(seen, [
        {_projects, _domains},
        {_projects},
      ]);

      await subscription.cancel();
      await watch.dispose();
      unawaited(source.close());
    });

    test('a closed channel reconnects while the app wants the stream', () async {
      var opens = 0;
      final watch = RealtimeWatch(
        open: () {
          opens++;
          return opens == 1
              ? Stream<WatchTablesEvent>.error(GrpcError.unavailable('dropped'))
              : const Stream<WatchTablesEvent>.empty();
        },
      )..start();
      await Future<void>.delayed(Duration.zero);
      expect(opens, 1);

      // `reconnectDelay` is three seconds; waiting it out keeps the test honest
      // about the real constant rather than mocking the clock away.
      await Future<void>.delayed(
        RealtimeWatch.reconnectDelay + const Duration(milliseconds: 200),
      );
      expect(opens, 2);

      watch.stop();
      await watch.dispose();
    });
  });
}
