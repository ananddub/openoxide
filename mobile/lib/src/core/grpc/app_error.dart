import 'package:grpc/grpc.dart';

/// Sealed failure the whole app speaks in: screens branch on the subtype, never
/// on a gRPC status number or a message string.
sealed class AppError implements Exception {
  const AppError([this.message]);

  final String? message;

  @override
  String toString() => '$runtimeType: ${message ?? ''}';
}

/// No session, or the server rejected the access token and refreshing failed.
final class SessionRejected extends AppError {
  const SessionRejected([super.message]);
}

/// The session is fine; this organization/resource is not reachable by it.
final class AccessDenied extends AppError {
  const AccessDenied([super.message]);
}

/// A request the server refused to validate.
final class InvalidRequest extends AppError {
  const InvalidRequest([super.message]);
}

/// The panel could not be reached, or went away mid-call. The session is kept.
final class PanelUnreachable extends AppError {
  const PanelUnreachable([super.message]);
}

/// Anything else, including a caller-initiated cancellation.
final class UnexpectedError extends AppError {
  const UnexpectedError([super.message, this.cause]);

  final Object? cause;
}

/// What a reader sees for a failure: the server's own words when they explain
/// the problem, a fixed sentence when the status code is the whole story.
extension AppErrorDisplay on AppError {
  bool get demandsSecondFactor =>
      this is SessionRejected &&
      (message ?? '').toLowerCase().contains('two-factor');

  String get userMessage => switch (this) {
    SessionRejected(:final message) =>
      message == null || message.isEmpty
      ? 'Your session is no longer valid. Sign in again.'
      : message,
    AccessDenied(:final message) =>
      message == null || message.isEmpty
      ? 'This account cannot see that.'
      : message,
    InvalidRequest(:final message) =>
      message == null || message.isEmpty ? 'The panel rejected that request.' : message,
    PanelUnreachable(:final message) =>
      message == null || message.isEmpty
      ? 'Cannot reach the panel.'
      : message,
    UnexpectedError(:final message) => message ?? 'Something went wrong.',
  };
}

/// Maps a thrown gRPC status onto [AppError]. Codes follow
/// `StatusCode.*`: 16 unauthenticated, 7 permission-denied, 3 invalid-argument,
/// 5 not-found, 2/4/14 unavailable-equivalent.
///
/// An [AppError] passes through unchanged: a guarded call has already mapped its
/// gRPC status, and re-mapping it would bury the real kind under
/// `UnexpectedError` and show the reader a class name.
AppError toAppError(Object error) => switch (error) {
  AppError() => error,
  GrpcError(:final code, :final message) => switch (code) {
    StatusCode.unauthenticated => SessionRejected(message),
    StatusCode.permissionDenied => AccessDenied(message),
    StatusCode.invalidArgument => InvalidRequest(message),
    StatusCode.failedPrecondition => InvalidRequest(message),
    StatusCode.notFound => InvalidRequest(message),
    StatusCode.unavailable => PanelUnreachable(message),
    StatusCode.deadlineExceeded => PanelUnreachable(message),
    StatusCode.resourceExhausted => PanelUnreachable(message),
    StatusCode.cancelled => UnexpectedError('cancelled', error),
    _ => UnexpectedError(message ?? 'grpc error $code', error),
  },
  _ => UnexpectedError(error.toString(), error),
};

/// True only for a rejection the server marked as `unauthenticated`, which is
/// the sole signal that the stored token is stale and worth refreshing. A
/// transport failure is never that signal — the web client treats the same
/// distinction as the difference between logging out and retrying later.
bool isUnauthenticated(Object error) =>
    error is GrpcError && error.code == StatusCode.unauthenticated;
