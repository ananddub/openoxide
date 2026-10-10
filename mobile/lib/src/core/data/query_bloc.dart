import 'dart:async';

export 'package:mobile/src/core/data/query_event.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mobile/src/core/data/query_event.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/realtime/realtime_watch.dart';
import 'package:mobile/src/gen/openoxide/api/v1/realtime.pbenum.dart' show WatchedTable;

/// One screen's fetch, as a bloc.
///
/// The panel gives no per-resource freshness signal, so a query holds its own
/// last result and refetches when a [RealtimeWatch] table it declared moves.
/// That is the whole caching model, deliberately: no cache keys, no invalidation
/// graph, nothing to keep in step by hand.
class QueryBloc<T> extends Bloc<QueryEvent, QueryState<T>> {
  QueryBloc({
    required this._load,
    RealtimeWatch? realtime,
    Iterable<WatchedTable> listensOn = const [],
  }) : _listensOn = List<WatchedTable>.unmodifiable(listensOn),
       super(QueryState<T>()) {
    on<QueryStarted>(_onStarted);
    on<QueryRefreshRequested>(_onRefreshRequested);
    on<QueryTablesChanged>(_onTablesChanged);
    if (realtime != null && _listensOn.isNotEmpty) {
      _subscription = realtime.ticks.listen(_onTick, onDone: _onTickDone);
    }
  }

  static const Duration _tickCooldown = Duration(milliseconds: 400);

  final Future<T> Function() _load;
  final List<WatchedTable> _listensOn;
  StreamSubscription<Set<WatchedTable>>? _subscription;
  Timer? _debounce;
  bool _busy = false;
  bool _missedTick = false;
  bool _tickStreamClosed = false;

  /// First load; renders a spinner rather than an empty screen.
  void start() => add(const QueryStarted());

  /// Refetch, keeping the previous data on screen while it is in flight.
  void refresh() => add(const QueryRefreshRequested());

  Future<void> _onStarted(QueryStarted event, Emitter<QueryState<T>> emit) =>
      _run(emit);

  Future<void> _onRefreshRequested(
    QueryRefreshRequested event,
    Emitter<QueryState<T>> emit,
  ) => _run(emit);

  Future<void> _run(Emitter<QueryState<T>> emit) async {
    if (_busy) return;
    _busy = true;
    emit(state.copyWith(loading: true));
    try {
      final data = await _load();
      emit(QueryState<T>(data: data));
    } on Object catch (cause) {
      emit(state.copyWith(loading: false, error: toAppError(cause)));
    } finally {
      _busy = false;
    }
    // A tick that arrived during this fetch is still a tick worth acting on.
    if (_missedTick) {
      _missedTick = false;
      _schedule();
    }
  }

  void _onTick(Set<WatchedTable> tables) {
    if (_listensOn.isEmpty) return;
    final relevant = tables.any(_listensOn.contains);
    if (!relevant) return;
    if (_busy) {
      _missedTick = true;
      return;
    }
    _schedule();
  }

  void _onTickDone() => _tickStreamClosed = true;

  void _schedule() {
    if (_tickStreamClosed) return;
    // Table ticks arrive in bursts around a deploy; one fetch per burst is
    // enough.
    _debounce?.cancel();
    _debounce = Timer(_tickCooldown, () => add(const QueryTablesChanged()));
  }

  Future<void> _onTablesChanged(
    QueryTablesChanged event,
    Emitter<QueryState<T>> emit,
  ) => _run(emit);

  @override
  Future<void> close() async {
    _debounce?.cancel();
    await _subscription?.cancel();
    return super.close();
  }
}

extension _QueryStateCopy<T> on QueryState<T> {
  QueryState<T> copyWith({T? data, AppError? error, bool? loading}) =>
      QueryState<T>(
        data: data ?? this.data,
        error: error ?? this.error,
        loading: loading ?? this.loading,
      );
}
