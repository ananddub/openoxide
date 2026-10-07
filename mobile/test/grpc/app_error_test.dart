import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart';
import 'package:mobile/src/grpc/app_error.dart';

void main() {
  group('toAppError', () {
    test('maps the codes the panel actually sends', () {
      expect(toAppError(GrpcError.unauthenticated('nope')), isA<SessionRejected>());
      expect(toAppError(GrpcError.permissionDenied('nope')), isA<AccessDenied>());
      expect(toAppError(GrpcError.invalidArgument('nope')), isA<InvalidRequest>());
      expect(toAppError(GrpcError.notFound('nope')), isA<InvalidRequest>());
      expect(
          toAppError(GrpcError.failedPrecondition('nope')), isA<InvalidRequest>());
      expect(toAppError(GrpcError.unavailable('nope')), isA<PanelUnreachable>());
      expect(
          toAppError(GrpcError.deadlineExceeded('nope')), isA<PanelUnreachable>());
      expect(toAppError(GrpcError.internal('nope')), isA<UnexpectedError>());
    });

    test('keeps the server message for the reader', () {
      final error = toAppError(GrpcError.invalidArgument('name is required'));
      expect(error.userMessage, 'name is required');
    });

    test('maps a non-grpc failure without losing it', () {
      final cause = StateError('boom');
      final error = toAppError(cause);
      expect(error, isA<UnexpectedError>());
      expect((error as UnexpectedError).cause, same(cause));
    });

    test('a cancellation is not an outage', () {
      expect(toAppError(GrpcError.cancelled()), isA<UnexpectedError>());
    });

    test('is idempotent, so a second mapping cannot bury the real kind', () {
      const first = AccessDenied('this organization is not yours');
      final second = toAppError(first);
      expect(second, same(first));
      expect(second.userMessage, 'this organization is not yours');
    });
  });

  group('isUnauthenticated', () {
    test('is the only signal worth refreshing for', () {
      expect(isUnauthenticated(GrpcError.unauthenticated()), isTrue);
      expect(isUnauthenticated(GrpcError.permissionDenied()), isFalse);
      expect(isUnauthenticated(GrpcError.unavailable()), isFalse);
      expect(isUnauthenticated(StateError('boom')), isFalse);
    });
  });

  group('AppErrorDisplay', () {
    test('spots the panel request for a second factor', () {
      const error = SessionRejected('two-factor authentication code is required');
      expect(error.demandsSecondFactor, isTrue);
      expect(const SessionRejected('invalid email or password').demandsSecondFactor,
          isFalse);
    });

    test('says something useful when the server said nothing', () {
      expect(const PanelUnreachable().userMessage, isNotEmpty);
      expect(const SessionRejected().userMessage, contains('Sign in'));
    });
  });
}
