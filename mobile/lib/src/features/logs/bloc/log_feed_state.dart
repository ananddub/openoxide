import 'package:equatable/equatable.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/logs/log_model.dart';

/// One live log stream as the pane renders it.
///
/// `isDone` means the panel closed the channel — there is no end-of-log
/// sentinel on the wire, so the owner refetches the deployment's status when
/// this turns true rather than trusting the stream alone.
class LogFeedState extends Equatable {
  const LogFeedState({
    this.entries = const [],
    this.droppedLines = 0,
    this.isRunning = false,
    this.isDone = false,
    this.error,
  });

  final List<ParsedLogEntry> entries;
  final int droppedLines;
  final bool isRunning;
  final bool isDone;
  final AppError? error;

  bool get isEmpty => entries.isEmpty;

  LogFeedState copyWith({
    List<ParsedLogEntry>? entries,
    int? droppedLines,
    bool? isRunning,
    bool? isDone,
    AppError? error,
  }) => LogFeedState(
    entries: entries ?? this.entries,
    droppedLines: droppedLines ?? this.droppedLines,
    isRunning: isRunning ?? this.isRunning,
    isDone: isDone ?? this.isDone,
    error: error ?? this.error,
  );

  @override
  List<Object?> get props => [entries, droppedLines, isRunning, isDone, error];
}
