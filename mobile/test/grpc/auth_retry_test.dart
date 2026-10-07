import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart';
import 'package:mobile/src/grpc/app_error.dart';
import 'package:mobile/src/grpc/auth_coordinator.dart';
import 'package:mobile/src/grpc/auth_retry.dart';
import 'package:mobile/src/session/session.dart';

/// A coordinator whose refresh endpoint and clock are both under the test's
/// control, so token expiry can be exercised even though the dev panel runs with
/// time checks switched off.
class _Harness {
  _Harness({this.refreshOutcome})
    : session = Session(
        vault: InMemorySessionVault(),
        initial: const StoredSession(
          accessToken: 'old',
          refreshToken: 'refresh-1',
          organizationId: 3,
        ),
      ),
      _now = DateTime.utc(2026, 10, 6);

  final Session session;
  Future<RefreshedTokens> Function(String refreshToken)? refreshOutcome;
  int refreshCalls = 0;

  DateTime _now;

  AuthCoordinator coordinator() {
    return AuthCoordinator(
      session: session,
      isRejected: isUnauthenticated,
      now: () => _now,
      caller: (refreshToken) async {
        refreshCalls++;
        final outcome = refreshOutcome;
        if (outcome == null) {
          return const RefreshedTokens(
            accessToken: 'new',
            refreshToken: 'refresh-2',
          );
        }
        return outcome(refreshToken);
      },
    );
  }

  GuardedCalls calls() => GuardedCalls(coordinator: coordinator());
}

/// Delivers two events and then rejects, the way a log stream breaks when the
/// panel drops the connection mid-flight.
Stream<int> _twoThenReject() async* {
  yield 1;
  yield 2;
  throw GrpcError.unauthenticated('mid-stream');
}

void main() {
  group('AuthCoordinator', () {
    test('writes both rotated tokens into the session on success', () async {
      final harness = _Harness();
      final result = await harness.coordinator().refresh();
      expect(result, isA<RefreshSucceeded>());
      expect(harness.session.accessToken, 'new');
      expect(harness.session.refreshToken, 'refresh-2');
      expect(harness.session.organizationId, 3);
    });

    test('keeps the stored refresh token when the panel returns an empty one',
        () async {
      final harness = _Harness(
        refreshOutcome: (_) async =>
            const RefreshedTokens(accessToken: 'new', refreshToken: ''),
      );
      await harness.coordinator().refresh();
      expect(harness.session.refreshToken, 'refresh-1');
    });

    test('runs one attempt at a time for concurrent callers', () async {
      final gate = Completer<RefreshedTokens>();
      final harness = _Harness(refreshOutcome: (_) => gate.future);
      final coordinator = harness.coordinator();

      final first = coordinator.refresh();
      final second = coordinator.refresh();
      gate.complete(
        const RefreshedTokens(accessToken: 'new', refreshToken: 'refresh-2'),
      );

      await Future.wait([first, second]);
      expect(harness.refreshCalls, 1);
    });

    test('a server rejection ends the session', () async {
      final harness = _Harness(
        refreshOutcome: (_) async => throw GrpcError.unauthenticated('revoked'),
      );
      final result = await harness.coordinator().refresh();
      expect(result, isA<RefreshExpired>());
      expect(harness.session.isSignedIn, isFalse);
    });

    test('a transport failure keeps the session and cools down', () async {
      final harness = _Harness(
        refreshOutcome: (_) async => throw GrpcError.unavailable('down'),
      );
      final coordinator = harness.coordinator();

      expect(await coordinator.refresh(), isA<RefreshTransient>());
      expect(harness.session.isSignedIn, isTrue);

      // Still inside the cooldown: the second caller is answered from memory.
      expect(await coordinator.refresh(), isA<RefreshTransient>());
      expect(harness.refreshCalls, 1);

      harness._now = harness._now.add(const Duration(seconds: 16));
      expect(await coordinator.refresh(), isA<RefreshTransient>());
      expect(harness.refreshCalls, 2);
    });

    test('with no refresh token there is nothing to try', () async {
      final session = Session(
        vault: InMemorySessionVault(),
        initial: const StoredSession(accessToken: 'a', refreshToken: ''),
      );
      final coordinator = AuthCoordinator(
        session: session,
        isRejected: (_) => false,
        caller: (_) async => throw StateError('must not be called'),
      );
      expect(await coordinator.refresh(), isA<RefreshExpired>());
      expect(session.isSignedIn, isFalse);
    });
  });

  group('GuardedCalls.unary', () {
    test('replays once with the refreshed token', () async {
      final harness = _Harness();
      final calls = harness.calls();
      var attempts = 0;

      final value = await calls.unary(() async {
        attempts++;
        if (attempts == 1) throw GrpcError.unauthenticated('expired');
        return harness.session.accessToken;
      });

      expect(value, 'new');
      expect(attempts, 2);
      expect(harness.refreshCalls, 1);
    });

    test('does not retry a call the caller excluded', () async {
      final harness = _Harness();
      final calls = harness.calls();
      var attempts = 0;

      await expectLater(
        calls.unary(() async {
          attempts++;
          throw GrpcError.unauthenticated('expired');
        }, allowRetry: false),
        throwsA(isA<SessionRejected>()),
      );
      expect(attempts, 1);
      expect(harness.refreshCalls, 0);
    });

    test('surfaces a rejected refresh as the original rejection', () async {
      final harness = _Harness(
        refreshOutcome: (_) async => throw GrpcError.unauthenticated('revoked'),
      );
      await expectLater(
        harness.calls().unary<int>(() async => throw GrpcError.unauthenticated('expired')),
        throwsA(isA<SessionRejected>()),
      );
      expect(harness.session.isSignedIn, isFalse);
    });

    test('an unreachable panel during refresh is not a logout', () async {
      final harness = _Harness(
        refreshOutcome: (_) async => throw GrpcError.unavailable('down'),
      );
      await expectLater(
        harness.calls().unary<int>(() async => throw GrpcError.unauthenticated('expired')),
        throwsA(isA<PanelUnreachable>()),
      );
      expect(harness.session.isSignedIn, isTrue);
    });

    test('maps any other failure without refreshing', () async {
      final harness = _Harness();
      await expectLater(
        harness.calls().unary<int>(
          () async => throw GrpcError.permissionDenied('not a member'),
        ),
        throwsA(isA<AccessDenied>()),
      );
      expect(harness.refreshCalls, 0);
    });
  });

  group('GuardedCalls.serverStream', () {
    test('replays a stream rejected before it delivered anything', () async {
      final harness = _Harness();
      final calls = harness.calls();
      var opens = 0;

      final events = await calls
          .serverStream<int>(() {
            opens++;
            if (opens == 1) {
              return Stream<int>.error(GrpcError.unauthenticated('expired'));
            }
            return Stream<int>.value(7);
          })
          .toList();

      expect(events, [7]);
      expect(opens, 2);
      expect(harness.refreshCalls, 1);
    });

    test('never replays a stream that already delivered events', () async {
      final harness = _Harness();
      final calls = harness.calls();
      var opens = 0;

      final stream = calls.serverStream<int>(() {
        opens++;
        return _twoThenReject();
      });

      final collected = <int>[];
      Object? failure;
      try {
        await for (final event in stream) {
          collected.add(event);
        }
      } catch (error) {
        failure = error;
      }
      expect(collected, [1, 2]);
      expect(failure, isA<SessionRejected>());
      expect(opens, 1);
      expect(harness.refreshCalls, 0);
    });

    test('closes with the mapped error when the replay fails again', () async {
      final harness = _Harness();
      final calls = harness.calls();
      var opens = 0;

      await expectLater(
        calls.serverStream<int>(() {
          opens++;
          return Stream<int>.error(GrpcError.unauthenticated('expired $opens'));
        }).toList(),
        throwsA(isA<SessionRejected>()),
      );
      expect(opens, 2);
    });
  });
}
