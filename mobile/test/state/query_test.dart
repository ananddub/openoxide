import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart';
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pb.dart';
import 'package:mobile/src/grpc/app_error.dart';
import 'package:mobile/src/state/query.dart';
import 'package:mobile/src/state/realtime_watch.dart';

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

  /// Longer than `Query._tickCooldown`, so a scheduled refetch has run.
  static Future<void> settle() =>
      Future<void>.delayed(const Duration(milliseconds: 700));

  void dispose() {
    watch.dispose();
    _source.close();
  }
}

void main() {
  group('Query', () {
    test('start loads once and keeps the result', () async {
      final harness = _WatchHarness();
      var loads = 0;
      final query = Query<int>(
        load: () async => ++loads,
        realtime: harness.watch,
        listensOn: const [_projects],
      )..start();
      await Future<void>.delayed(Duration.zero);

      expect(loads, 1);
      expect(query.data, 1);
      expect(query.error, isNull);
      expect(query.loading, isFalse);

      query.dispose();
      harness.dispose();
    });

    test('a burst of ticks for a watched table is one refetch', () async {
      final harness = _WatchHarness()..watch.start();
      var loads = 0;
      final query = Query<int>(
        load: () async => ++loads,
        realtime: harness.watch,
        listensOn: const [_projects],
      )..start();
      await Future<void>.delayed(Duration.zero);
      expect(loads, 1);

      harness.tick(_projects);
      harness.tick(_projects);
      harness.tick(_projects);
      await _WatchHarness.settle();

      expect(loads, 2);
      expect(query.data, 2);

      query.dispose();
      harness.dispose();
    });

    test('a tick for a table the screen ignores is not a refetch', () async {
      final harness = _WatchHarness()..watch.start();
      var loads = 0;
      final query = Query<int>(
        load: () async => ++loads,
        realtime: harness.watch,
        listensOn: const [_projects],
      )..start();
      await Future<void>.delayed(Duration.zero);

      harness.tick(_domains);
      await _WatchHarness.settle();

      expect(loads, 1);
      expect(harness.watch.versionOf(_domains), 1);
      expect(harness.watch.versionOf(_projects), 0);

      query.dispose();
      harness.dispose();
    });

    test('a failed refetch keeps the stale rows and reports the error', () async {
      final harness = _WatchHarness()..watch.start();
      var loads = 0;
      final query = Query<List<String>>(
        load: () {
          loads++;
          if (loads == 1) return Future.value(['row']);
          return Future<List<String>>.error(GrpcError.unavailable('down'));
        },
        realtime: harness.watch,
        listensOn: const [_projects],
      )..start();
      await Future<void>.delayed(Duration.zero);

      harness.tick(_projects);
      await _WatchHarness.settle();

      expect(loads, 2);
      expect(query.data, ['row']);
      expect(query.error, isA<PanelUnreachable>());

      query.dispose();
      harness.dispose();
    });

    test('a tick that lands during the first fetch still earns a refetch',
        () async {
      final harness = _WatchHarness()..watch.start();
      final gate = Completer<List<String>>();
      var loads = 0;
      final query = Query<List<String>>(
        load: () {
          loads++;
          return loads == 1 ? gate.future : Future.value(['fresh']);
        },
        realtime: harness.watch,
        listensOn: const [_projects],
      )..start();
      await Future<void>.delayed(Duration.zero);

      harness.tick(_projects);
      gate.complete(['first']);
      await _WatchHarness.settle();

      expect(loads, 2);
      expect(query.data, ['fresh']);

      query.dispose();
      harness.dispose();
    });

    test('dispose stops the stream of refetches', () async {
      final harness = _WatchHarness()..watch.start();
      var loads = 0;
      final query = Query<int>(
        load: () async => ++loads,
        realtime: harness.watch,
        listensOn: const [_projects],
      )..start();
      await Future<void>.delayed(Duration.zero);

      query.dispose();
      harness.tick(_projects);
      await _WatchHarness.settle();

      expect(loads, 1);
      harness.dispose();
    });

    test('autoRefresh false renders a manual-only query', () async {
      final harness = _WatchHarness()..watch.start();
      var loads = 0;
      final query = Query<int>(
        load: () async => ++loads,
        realtime: harness.watch,
        listensOn: const [_projects],
        autoRefresh: false,
      )..start();
      await Future<void>.delayed(Duration.zero);

      harness.tick(_projects);
      await _WatchHarness.settle();
      expect(loads, 1);

      await query.refresh();
      expect(loads, 2);

      query.dispose();
      harness.dispose();
    });
  });

  group('RealtimeWatch', () {
    test('counts one version per table per tick', () async {
      final source = StreamController<WatchTablesEvent>();
      final watch = RealtimeWatch(open: () => source.stream)..start();

      source.add(
        WatchTablesEvent(tables: [_projects, _domains]),
      );
      source.add(WatchTablesEvent(tables: [_projects]));
      // A tick naming no table is a resync marker, not a change.
      source.add(WatchTablesEvent());
      await Future<void>.delayed(Duration.zero);

      expect(watch.versionOf(_projects), 2);
      expect(watch.versionOf(_domains), 1);
      expect(watch.isConnected, isTrue);

      watch.dispose();
      await source.close();
    });

    test('a closed channel reconnects while the app wants the stream', () async {
      var opens = 0;
      final watch = RealtimeWatch(open: () {
        opens++;
        return opens == 1
            ? Stream<WatchTablesEvent>.error(GrpcError.unavailable('dropped'))
            : const Stream<WatchTablesEvent>.empty();
      })..start();
      await Future<void>.delayed(Duration.zero);
      expect(opens, 1);

      // `reconnectDelay` is three seconds; waiting it out keeps the test honest
      // about the real constant rather than mocking the clock away.
      await Future<void>.delayed(
        RealtimeWatch.reconnectDelay + const Duration(milliseconds: 200),
      );
      expect(opens, 2);

      watch.stop();
      expect(watch.isConnected, isFalse);
      watch.dispose();
    });
  });
}
