import 'dart:async';

export 'package:mobile/src/features/logs/bloc/log_feed_event.dart';
export 'package:mobile/src/features/logs/bloc/log_feed_state.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/logs/log_buffer.dart';
import 'package:mobile/src/features/logs/bloc/log_feed_event.dart';
import 'package:mobile/src/features/logs/bloc/log_feed_state.dart';
import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart';

/// Owns one live log stream and the lines it has produced, as a bloc.
///
/// A build can emit hundreds of lines a second, so the wire is repainted into
/// state in batches — but a timer, not a count alone, or a container that logs
/// one line a minute would never repaint at all.
class LogFeedBloc extends Bloc<LogFeedEvent, LogFeedState> {
  LogFeedBloc({required this._open})
    : buffer = LogBuffer(capacity: 2000),
      super(const LogFeedState()) {
    on<LogFeedStarted>(_onStarted);
    on<LogFeedStopped>(_onStopped);
    on<LogFeedRestarted>(_onRestarted);
    on<LogFeedAppended>(_onAppended);
    on<LogFeedEnded>(_onEnded);
    on<LogFeedFailed>(_onFailed);
    on<LogFeedFlush>((event, emit) => _flush(emit));
  }

  static const Duration _batchDelay = Duration(milliseconds: 120);

  /// Reopens the stream; called again after a refresh of the same target.
  final Stream<DeploymentEvent> Function() _open;

  /// The rendered log, newest kept when the cap is hit. The pane reads the
  /// copies [LogFeedState] carries; this remains the bloc's working set.
  final LogBuffer buffer;

  StreamSubscription<DeploymentEvent>? _subscription;
  Timer? _batch;
  int _linesSinceNotify = 0;

  /// Set the moment the stream reports an error, so the done event that
  /// follows it on the same subscription cannot be mistaken for the panel's
  /// normal end-of-stream signal.
  bool _failed = false;

  void start() => add(const LogFeedStarted());

  void stop() => add(const LogFeedStopped());

  /// Stop, drop the lines, and start again — the retry path for the same
  /// target.
  void restart() => add(const LogFeedRestarted());

  Future<void> _onStarted(
    LogFeedStarted event,
    Emitter<LogFeedState> emit,
  ) async {
    if (state.isRunning) return;
    _failed = false;
    _subscription = _open().listen(
      (event) => add(LogFeedAppended(event)),
      onDone: () {
        if (_failed) return;
        add(const LogFeedEnded());
      },
      onError: (Object cause) {
        _failed = true;
        add(LogFeedFailed(cause));
      },
      cancelOnError: false,
    );
    emit(const LogFeedState(isRunning: true));
  }

  Future<void> _onStopped(
    LogFeedStopped event,
    Emitter<LogFeedState> emit,
  ) async {
    _teardown();
    emit(state.copyWith(isRunning: false));
  }

  Future<void> _onRestarted(
    LogFeedRestarted event,
    Emitter<LogFeedState> emit,
  ) async {
    _teardown();
    buffer.clear();
    _failed = false;
    _subscription = _open().listen(
      (event) => add(LogFeedAppended(event)),
      onDone: () {
        if (_failed) return;
        add(const LogFeedEnded());
      },
      onError: (Object cause) {
        _failed = true;
        add(LogFeedFailed(cause));
      },
      cancelOnError: false,
    );
    emit(const LogFeedState(isRunning: true));
  }

  Future<void> _onAppended(
    LogFeedAppended event,
    Emitter<LogFeedState> emit,
  ) async {
    final stage = event.event.type.isEmpty ? null : event.event.type;
    var appended = 0;
    if (event.event.payload.isNotEmpty) {
      appended += buffer.append(event.event.payload, stage: stage).length;
    }
    if (event.event.error.isNotEmpty) {
      appended += buffer.appendError(event.event.error, stage: stage).length;
    }
    if (appended == 0) return;
    _linesSinceNotify += appended;
    if (_linesSinceNotify >= 32) {
      _batch?.cancel();
      _batch = null;
      _flush(emit);
    } else {
      _batch ??= Timer(_batchDelay, () => add(const LogFeedFlush()));
    }
  }

  void _flush(Emitter<LogFeedState> emit) {
    _batch?.cancel();
    _batch = null;
    _linesSinceNotify = 0;
    if (state.isDone) return;
    emit(
      LogFeedState(
        entries: List.unmodifiable(buffer.entries),
        droppedLines: buffer.droppedLines,
        isRunning: state.isRunning,
        isDone: state.isDone,
        error: state.error,
      ),
    );
  }

  Future<void> _onEnded(LogFeedEnded event, Emitter<LogFeedState> emit) async {
    // The done event that trails a stream error is not the panel closing a
    // settled deployment; the failure already said what happened.
    if (!state.isRunning || _failed) return;
    _batch?.cancel();
    _batch = null;
    _linesSinceNotify = 0;
    buffer.finish();
    _subscription = null;
    emit(
      LogFeedState(
        entries: List.unmodifiable(buffer.entries),
        droppedLines: buffer.droppedLines,
        isRunning: false,
        isDone: true,
      ),
    );
  }

  Future<void> _onFailed(
    LogFeedFailed event,
    Emitter<LogFeedState> emit,
  ) async {
    if (state.error != null || state.isDone) return;
    _teardown();
    emit(state.copyWith(isRunning: false, error: toAppError(event.cause)));
  }

  /// Stops listening. The cancel is fire-and-forget on purpose: awaiting it
  /// here would suspend the handler on a future that can outlive the frame the
  /// state was meant to change in, and the subscription is going away either
  /// way.
  void _teardown() {
    _batch?.cancel();
    _batch = null;
    _linesSinceNotify = 0;
    _subscription?.cancel();
    _subscription = null;
  }

  @override
  Future<void> close() async {
    _teardown();
    return super.close();
  }
}

/// Internal: the batch timer decided enough lines are waiting.
class LogFeedFlush extends LogFeedEvent {
  const LogFeedFlush();
}
