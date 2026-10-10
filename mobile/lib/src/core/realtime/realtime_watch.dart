import 'dart:async';

import 'package:mobile/src/gen/openoxide/api/v1/realtime.pb.dart';

/// The invalidation bus.
///
/// `RealtimeService.WatchTables` says only *which* tables changed, never what
/// changed in them, so a tick is just the set of moved tables: a query records
/// that it fetched and refetches when one of its tables appears again. The
/// first event after a connect is a full resync, and bursts are coalesced
/// server-side.
///
/// Listeners get a broadcast stream, so any number of query blocs can follow
/// the same connection.
class RealtimeWatch {
  RealtimeWatch({required this.open});

  /// Reopens the watch stream. The panel closes it when the connection drops,
  /// so reconnecting is the normal path, not an error.
  final Stream<WatchTablesEvent> Function() open;

  static const Duration reconnectDelay = Duration(seconds: 3);

  final _ticks = StreamController<Set<WatchedTable>>.broadcast();
  StreamSubscription<WatchTablesEvent>? _subscription;
  Timer? _reconnect;
  bool _wantsStream = false;

  /// The tables that moved, per event. Emits only while started and connected.
  Stream<Set<WatchedTable>> get ticks => _ticks.stream;

  void start() {
    if (_wantsStream) return;
    _wantsStream = true;
    _connect();
  }

  void stop() {
    _wantsStream = false;
    _reconnect?.cancel();
    _reconnect = null;
    final subscription = _subscription;
    _subscription = null;
    unawaited(subscription?.cancel());
  }

  void _connect() {
    if (!_wantsStream || _subscription != null) return;
    _subscription = open().listen(
      _onEvent,
      onDone: _onClosed,
      onError: (Object _) => _onClosed(),
    );
  }

  void _onEvent(WatchTablesEvent event) {
    if (event.tables.isEmpty) return;
    _ticks.add(event.tables.toSet());
  }

  void _onClosed() {
    _subscription?.cancel();
    _subscription = null;
    if (!_wantsStream) return;
    _reconnect?.cancel();
    _reconnect = Timer(reconnectDelay, _connect);
  }

  Future<void> dispose() async {
    stop();
    await _ticks.close();
  }
}
