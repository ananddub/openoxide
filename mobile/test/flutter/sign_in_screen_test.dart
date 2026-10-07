import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart' show GrpcError;
import 'package:mobile/src/session/session.dart';
import 'package:mobile/src/state/auth_controller.dart';
import 'package:mobile/src/ui/sign_in_screen.dart';
import 'package:mobile/src/ui/tokens.dart';

import '../fakes.dart';
import 'harness.dart';

final _email = find.byKey(const ValueKey('email'));
final _password = find.byKey(const ValueKey('password'));
final _code = find.byKey(const ValueKey('code'));
final _submit = find.byKey(const ValueKey('submit'));
final _brand = find.byKey(const ValueKey('brand'));

void main() {
  // The stagger rhythm is a global static, so a test must not inherit whatever
  // the previous one left behind.
  setUp(Motion.install);

  testWidgets('lays out at every supported size', (tester) async {
    final scope = authScope();
    await pumpAtSizes(
      tester,
      () => appScope(child: const SignInScreen(), auth: scope.controller),
      finds: [_brand, _email, _password, _submit],
      textScalers: const [1.0, 1.3],
    );
  });

  testWidgets('refuses an empty submit and names both fields', (tester) async {
    final scope = authScope();
    await tester.pumpWidget(
      appScope(child: const SignInScreen(), auth: scope.controller),
    );
    await pumpEntrance(tester);

    await tester.tap(_submit);
    await tester.pump();

    expect(scope.api.attempts, isEmpty);
    expect(find.text('Enter an email'), findsOneWidget);
    expect(find.text('Enter your password'), findsOneWidget);
  });

  testWidgets('shows what the panel said when the credentials are wrong',
    (tester) async {
      final scope = authScope();
      scope.api.onSignIn =
          (_) async =>
              throw GrpcError.unauthenticated('invalid credentials');
      await tester.pumpWidget(
        appScope(child: const SignInScreen(), auth: scope.controller),
      );
      await pumpEntrance(tester);

      await tester.enterText(_email, 'test@example.com');
      await tester.enterText(_password, 'wrong');
      await tester.tap(_submit);
      await tester.pump();
      await pumpEntrance(tester);

      expect(find.text('invalid credentials'), findsOneWidget);
      expect(_code, findsNothing);
    },
  );

  testWidgets('asks for the second factor only after the panel demands it',
    (tester) async {
      final scope = authScope();
      scope.api.onSignIn = (attempt) async {
        if (attempt.twoFactorCode == null) {
          throw GrpcError.unauthenticated(
            'two-factor authentication code is required',
          );
        }
        return const StoredSession(
          accessToken: 'access',
          refreshToken: 'refresh',
          userId: 7,
        );
      };
      await tester.pumpWidget(
        appScope(child: const SignInScreen(), auth: scope.controller),
      );
      await pumpEntrance(tester);

      expect(_code, findsNothing);
      await tester.enterText(_email, 'test@example.com');
      await tester.enterText(_password, 'a-very-specific-password');
      await tester.tap(_submit);
      await tester.pump();
      // The field grows in, so it takes an animated-size frame before it is
      // laid out and the scroll-to-bottom callback has run.
      await pumpEntrance(tester);

      expect(_code, findsOneWidget);
      expect(scope.api.attempts.single.twoFactorCode, isNull);

      await tester.enterText(_code, '123456');
      await tester.tap(_submit);
      await tester.pump();
      await pumpEntrance(tester);

      expect(scope.api.attempts, hasLength(2));
      expect(scope.api.attempts.last.twoFactorCode, '123456');
      // Growing the code row must not rebuild the form: a remount would silently
      // drop what the user already typed.
      expect(
        tester.widget<TextFormField>(_password).controller!.text,
        'a-very-specific-password',
      );
      expect(scope.controller.stage, AppStage.signedIn);
    },
  );

  testWidgets('disables the button while the panel is still answering',
    (tester) async {
      final scope = authScope();
      final gate = Completer<StoredSession>();
      scope.api.onSignIn = (_) => gate.future;
      await tester.pumpWidget(
        appScope(child: const SignInScreen(), auth: scope.controller),
      );
      await pumpEntrance(tester);

      await tester.enterText(_email, 'test@example.com');
      await tester.enterText(_password, 'secret');
      await tester.tap(_submit);
      await tester.pump();

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      expect(scope.controller.busy, isTrue);

      // A second tap while in flight must not send a second login.
      await tester.tap(_submit);
      await tester.pump();
      expect(scope.api.attempts, hasLength(1));

      gate.complete(
        const StoredSession(
          accessToken: 'access',
          refreshToken: 'refresh',
          userId: 7,
        ),
      );
      await tester.pump();
      await pumpEntrance(tester);

      expect(find.byType(CircularProgressIndicator), findsNothing);
      expect(scope.controller.busy, isFalse);
      expect(scope.controller.stage, AppStage.signedIn);
    },
  );
}
