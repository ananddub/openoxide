import 'dart:async';

import '../session/session.dart';

/// New tokens handed back by the panel for a valid refresh token.
class RefreshedTokens {
  const RefreshedTokens({
    required this.accessToken,
    required this.refreshToken,
  });

  final String accessToken;
  final String refreshToken;
}

/// Outcome of a refresh attempt, mirroring the web client's three states:
/// only an explicit server rejection ends the session.
sealed class RefreshResult {
  const RefreshResult();
}

final class RefreshSucceeded extends RefreshResult {
  const RefreshSucceeded(this.accessToken);

  final String accessToken;
}

/// The backend rejected the refresh token — the session is genuinely dead.
final class RefreshExpired extends RefreshResult {
  const RefreshExpired();
}

/// Transport/server failure. The stored refresh token is untouched, so the
/// session must survive and a later attempt may succeed.
final class RefreshTransient extends RefreshResult {
  const RefreshTransient();
}

typedef RefreshCaller = Future<RefreshedTokens> Function(String refreshToken);

typedef RefreshRejection = bool Function(Object error);

/// Coordinates token refresh the way `client.ts` does: one attempt in flight at
/// a time, a cooldown so a broken backend is not hammered on every call, and a
/// session drop only when the server itself rejects the refresh token.
class AuthCoordinator {
  AuthCoordinator({
    required this.session,
    required this._caller,
    required this._isRejected,
    DateTime Function()? now,
    this._cooldown = const Duration(seconds: 15),
  }) : _now = now ?? DateTime.now;

  final Session session;
  final RefreshCaller _caller;
  final RefreshRejection _isRejected;
  final DateTime Function() _now;
  final Duration _cooldown;

  Completer<RefreshResult>? _inFlight;
  String _retryToken = '';
  DateTime? _retryAfter;

  Future<RefreshResult> refresh() {
    final existing = _inFlight;
    if (existing != null) return existing.future;

    final completer = Completer<RefreshResult>();
    _inFlight = completer;
    unawaited(_attempt(completer));
    return completer.future;
  }

  Future<void> _attempt(Completer<RefreshResult> completer) async {
    final refreshToken = session.refreshToken;
    try {
      if (refreshToken.isEmpty) {
        await session.signOut();
        completer.complete(const RefreshExpired());
        return;
      }

      final retryAfter = _retryAfter;
      if (_retryToken == refreshToken &&
          retryAfter != null &&
          _now().isBefore(retryAfter)) {
        completer.complete(const RefreshTransient());
        return;
      }

      final tokens = await _caller(refreshToken);
      await session.update(
        (current) => current.copyWith(
          accessToken: tokens.accessToken,
          refreshToken: tokens.refreshToken.isEmpty
              ? current.refreshToken
              : tokens.refreshToken,
        ),
      );
      _retryToken = '';
      _retryAfter = null;
      completer.complete(RefreshSucceeded(tokens.accessToken));
    } catch (error) {
      if (_isRejected(error)) {
        _retryToken = '';
        _retryAfter = null;
        await session.signOut();
        completer.complete(const RefreshExpired());
        return;
      }
      // Keep the session: the next attempt can succeed once the backend does.
      _retryToken = refreshToken;
      _retryAfter = _now().add(_cooldown);
      completer.complete(const RefreshTransient());
    } finally {
      _inFlight = null;
    }
  }
}
