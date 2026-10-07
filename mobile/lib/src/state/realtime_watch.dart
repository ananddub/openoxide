import 'dart:async';

import 'package:flutter/foundation.dart';

import '../gen/openoxide/api/v1/realtime.pb.dart';

/// The invalidation bus.
///
/// `RealtimeService.WatchTables` says only *which* tables changed, never what
/// changed in them, so a tick is a version counter: a screen records the counter
/// it rendered at and refetches when that counter moves. The first event after a
/// connect is a full resync, and bursts are coalesced server-side.
class RealtimeWatch extends ChangeNotifier {
  RealtimeWatch({required this.open});

  /// Reopens the watch stream. The panel closes it when the connection drops,
  /// so reconnecting is the normal path, not an error.
  final Stream<WatchTablesEvent> Function() open;

  static const Duration reconnectDelay = Duration(seconds: 3);

  final Map<WatchedTable, int> _versions = {};
  StreamSubscription<WatchTablesEvent>? _subscription;
  Timer? _reconnect;
  bool _wantsStream = false;

  int versionOf(WatchedTable table) => _versions[table] ?? 0;

  bool get isConnected => _subscription != null;

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
    notifyListeners();
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
    for (final table in event.tables) {
      _versions[table] = (_versions[table] ?? 0) + 1;
    }
    notifyListeners();
  }

  void _onClosed() {
    _subscription?.cancel();
    _subscription = null;
    notifyListeners();
    if (!_wantsStream) return;
    _reconnect?.cancel();
    _reconnect = Timer(reconnectDelay, _connect);
  }

  @override
  void dispose() {
    stop();
    super.dispose();
  }
}
