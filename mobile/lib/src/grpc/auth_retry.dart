import 'dart:async';

import 'app_error.dart';
import 'auth_coordinator.dart';

/// Wraps every panel call with the same recovery the web transport implements:
/// an `unauthenticated` rejection triggers exactly one refresh and one replay,
/// a refresh that fails on the network keeps the session, and a refresh the
/// server rejects ends it. Every failure leaves as a mapped [AppError].
///
/// Replaying is enough on its own because the auth metadata provider runs per
/// RPC: by the time the retry goes out, the coordinator has already written the
/// new access token into the session.
class GuardedCalls {
  GuardedCalls({required this.coordinator});

  final AuthCoordinator coordinator;

  Future<T> unary<T>(
    Future<T> Function() invoke, {
    bool allowRetry = true,
  }) async {
    try {
      return await invoke();
    } catch (error) {
      if (!allowRetry || !isUnauthenticated(error)) throw toAppError(error);

      final result = await coordinator.refresh();
      if (result is RefreshSucceeded) return await invoke();
      // The server itself rejected the refresh token: the session is gone, so
      // the original rejection is the accurate error to surface.
      if (result is RefreshExpired) throw toAppError(error);
      throw const PanelUnreachable(
        'unable to reach the panel to refresh the session',
      );
    }
  }

  /// Server-streaming call. A stream rejected before it delivered anything is
  /// replayed once after a refresh; a stream that already produced events is
  /// not replayed, because the consumer would see duplicates.
  ///
  /// The panel ends a rejected stream with an error *and* a done event, so the
  /// done handler must not close out from under a refresh that is still
  /// deciding whether to replay, and a replayed subscription must not inherit
  /// the tail of the stream it replaced.
  Stream<T> serverStream<T>(Stream<T> Function() open) {
    late final StreamController<T> controller;
    StreamSubscription<T>? active;
    var generation = 0;
    var delivered = false;
    var replayed = false;
    var awaitingRefresh = false;

    void subscribe() {
      final attempt = ++generation;
      active = open().listen(
        (event) {
          if (attempt != generation) return;
          delivered = true;
          if (!controller.isClosed) controller.add(event);
        },
        onDone: () {
          if (attempt != generation || awaitingRefresh) return;
          if (!controller.isClosed) controller.close();
        },
        onError: (Object error, StackTrace stackTrace) async {
          if (attempt != generation) return;
          if (!delivered && !replayed && isUnauthenticated(error)) {
            awaitingRefresh = true;
            final result = await coordinator.refresh();
            awaitingRefresh = false;
            if (result is RefreshSucceeded && !controller.isClosed) {
              replayed = true;
              subscribe();
              return;
            }
          }
          if (controller.isClosed) return;
          controller.addError(toAppError(error), stackTrace);
          await controller.close();
        },
        cancelOnError: false,
      );
    }

    controller = StreamController<T>(
      onListen: subscribe,
      onCancel: () async => active?.cancel(),
      sync: true,
    );
    return controller.stream;
  }
}
