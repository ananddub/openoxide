import 'package:equatable/equatable.dart';
import 'package:mobile/src/core/grpc/app_error.dart';

/// Events of [QueryBloc]. Public ones are what a screen sends; the table-tick
/// event arrives from the realtime subscription inside the bloc.
sealed class QueryEvent {
  const QueryEvent();
}

/// First load; renders a spinner rather than an empty screen.
class QueryStarted extends QueryEvent {
  const QueryStarted();
}

/// Refetch, keeping the previous data on screen while it is in flight.
class QueryRefreshRequested extends QueryEvent {
  const QueryRefreshRequested();
}

/// One realtime tick: the named tables moved on the panel.
class QueryTablesChanged extends QueryEvent {
  const QueryTablesChanged();
}

/// One fetch's outcome, shaped for the three ways a screen renders it.
///
/// The panel gives no per-resource freshness signal, so a query holds its own
/// last result and refetches when a realtime table it declared moves. A failed
/// refresh over data that already arrived keeps the data; the screen only
/// surfaces [error] where there is nothing to show instead.
class QueryState<T> extends Equatable {
  const QueryState({this.data, this.error, this.loading = false});

  final T? data;
  final AppError? error;
  final bool loading;

  bool get hasData => data != null;

  @override
  List<Object?> get props => [data, error, loading];
}
