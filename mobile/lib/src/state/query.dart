import 'dart:async';

import 'package:flutter/foundation.dart';

import '../gen/openoxide/api/v1/realtime.pb.dart';
import '../grpc/app_error.dart';
import 'realtime_watch.dart';

/// One screen's fetch.
///
/// The panel gives no per-resource freshness signal, so a screen holds its own
/// last result and refetches when a [RealtimeWatch] table it declared moves.
/// That is the whole caching model, deliberately: no cache keys, no invalidation
/// graph, nothing to keep in step by hand.
class Query<T> extends ChangeNotifier {
  Query({
    required this.load,
    RealtimeWatch? realtime,
    Iterable<WatchedTable> listensOn = const [],
    this.autoRefresh = true,
  }) : _realtime = realtime,
       _listensOn = List<WatchedTable>.unmodifiable(listensOn) {
    _seen = _snapshot();
    if (realtime != null && _listensOn.isNotEmpty && autoRefresh) {
      _listener = () => _onTick(realtime);
      realtime.addListener(_listener!);
    }
  }

  static const Duration _tickCooldown = Duration(milliseconds: 400);

  final Future<T> Function() load;
  final RealtimeWatch? _realtime;
  final List<WatchedTable> _listensOn;
  final bool autoRefresh;

  VoidCallback? _listener;
  Timer? _debounce;
  Map<WatchedTable, int> _seen = {};
  bool _missedTick = false;

  T? _data;
  AppError? _error;
  bool _loading = false;
  bool _disposed = false;

  T? get data => _data;
  AppError? get error => _error;
  bool get loading => _loading;
  bool get hasData => _data != null;

  Map<WatchedTable, int> _snapshot() {
    final realtime = _realtime;
    if (realtime == null) return {};
    return {for (final table in _listensOn) table: realtime.versionOf(table)};
  }

  /// First load; renders a spinner rather than an empty screen.
  Future<void> start() async {
    if (_disposed || _loading) return;
    _loading = true;
    _notify();
    await _run();
  }

  /// Refetch, keeping the previous data on screen while it is in flight.
  Future<void> refresh() async {
    if (_disposed || _loading) return;
    _seen = _snapshot();
    await _run();
  }

  Future<void> _run() async {
    try {
      final result = await load();
      if (_disposed) return;
      _data = result;
      _error = null;
    } on Object catch (cause) {
      if (_disposed) return;
      _error = toAppError(cause);
    } finally {
      _loading = false;
      _notify();
      // A tick that arrived during this fetch is still a tick worth acting on.
      if (_missedTick) {
        _missedTick = false;
        _schedule();
      }
    }
  }

  void _onTick(RealtimeWatch realtime) {
    if (_disposed) return;
    if (_loading) {
      _missedTick = true;
      return;
    }
    _schedule();
  }

  void _schedule() {
    final realtime = _realtime;
    if (_disposed || realtime == null) return;
    var changed = false;
    for (final table in _listensOn) {
      final version = realtime.versionOf(table);
      if (version != _seen[table]) {
        _seen[table] = version;
        changed = true;
      }
    }
    if (!changed) return;
    // Table ticks arrive in bursts around a deploy; one fetch per burst is enough.
    _debounce?.cancel();
    _debounce = Timer(_tickCooldown, refresh);
  }

  void _notify() {
    if (!_disposed) notifyListeners();
  }

  @override
  void dispose() {
    _disposed = true;
    _debounce?.cancel();
    final listener = _listener;
    if (listener != null) _realtime?.removeListener(listener);
    super.dispose();
  }
}
