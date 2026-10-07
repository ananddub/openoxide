import 'dart:async';

import 'package:flutter/foundation.dart';

import '../logs/log_buffer.dart';
import '../grpc/app_error.dart';
import '../gen/openoxide/api/v1/deployment.pb.dart';

/// Owns one live log stream and the lines it has produced.
///
/// The panel has no end-of-log sentinel: it closes the channel once a deployment
/// reaches a terminal state. So `done` is the normal ending, not an error, and a
/// screen that wants the final status has to ask for it separately.
class LogFeed extends ChangeNotifier {
  LogFeed({
    required this.open,
    this.capacity = 2000,
    this.onEnded,
  }) : buffer = LogBuffer(capacity: capacity);

  /// Reopens the stream; called again after a refresh of the same target.
  final Stream<DeploymentEvent> Function() open;

  final int capacity;

  /// Fired when the panel closes the channel, so the owner can refetch status.
  final void Function()? onEnded;

  final LogBuffer buffer;
  StreamSubscription<DeploymentEvent>? _subscription;
  Timer? _batch;
  AppError? _error;
  bool _done = false;
  bool _running = false;
  int _linesSinceNotify = 0;

  static const Duration _batchDelay = Duration(milliseconds: 120);

  bool get isRunning => _running;
  bool get isDone => _done;
  AppError? get error => _error;
  bool get isEmpty => buffer.isEmpty;

  /// Start following. Returns once the subscription is up, not when data arrives.
  Future<void> start() async {
    if (_running) return;
    _running = true;
    _done = false;
    _error = null;
    _subscription = open().listen(
      _onEvent,
      onDone: _onDone,
      onError: (Object cause) => _fail(cause),
      cancelOnError: false,
    );
    notifyListeners();
  }

  void _onEvent(DeploymentEvent event) {
    final stage = event.type.isEmpty ? null : event.type;
    var appended = 0;
    if (event.payload.isNotEmpty) {
      appended += buffer.append(event.payload, stage: stage).length;
    }
    if (event.error.isNotEmpty) {
      appended += buffer.appendError(event.error, stage: stage).length;
    }
    if (appended == 0) return;
    _linesSinceNotify += appended;
    // A build can emit hundreds of lines a second, so repaint in batches — but a
    // timer, not a count alone, or a container that logs one line a minute would
    // never repaint at all.
    if (_linesSinceNotify >= 32) {
      _batch?.cancel();
      _batch = null;
      _flushNotify();
    } else {
      _batch ??= Timer(_batchDelay, _flushNotify);
    }
  }

  void _flushNotify() {
    _batch?.cancel();
    _batch = null;
    _linesSinceNotify = 0;
    if (!_done) notifyListeners();
  }

  void _onDone() {
    if (!_running) return;
    _running = false;
    _done = true;
    _batch?.cancel();
    _batch = null;
    buffer.finish();
    _subscription = null;
    notifyListeners();
    onEnded?.call();
  }

  void _fail(Object cause) {
    if (!_running) return;
    _running = false;
    _error = toAppError(cause);
    notifyListeners();
  }

  /// Stop listening. Safe to call twice, and after the stream already ended.
  Future<void> stop() async {
    _running = false;
    _batch?.cancel();
    _batch = null;
    final subscription = _subscription;
    _subscription = null;
    await subscription?.cancel();
  }

  @override
  void dispose() {
    unawaited(stop());
    super.dispose();
  }
}
