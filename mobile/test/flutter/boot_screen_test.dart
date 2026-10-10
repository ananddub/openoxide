import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/core/grpc/app_error.dart';
import 'package:mobile/src/core/tokens/motion.dart';
import 'package:mobile/src/features/auth/screens/boot_gate.dart';
import 'package:mobile/src/features/auth/screens/sign_in_screen.dart';
import 'package:mobile/src/gen/openoxide/api/v1/auth.pb.dart' show SetupStatus;

import '../fakes.dart';
import 'harness.dart';

/// The shape a real BoringSSL rejection arrives in: one long line of C++ that
/// explains nothing to a reader. The gate keeps it underneath an actionable
/// sentence and cuts it off, which is also what makes it a layout risk at
/// 320 logical pixels wide.
const _handshakeNoise =
    'HandshakeException: Handshake error in client '
    '(ERROR_VERIFY_SIGNATURE: digital signature routines) '
    'external/boringssl/src/ssl/handshake.cc:393';

void main() {
  setUp(Motion.install);

  testWidgets('the unreachable gate lays out at every size', (tester) async {
    final scope = authScope()
      ..api.setupError = PanelUnreachable(_handshakeNoise);
    scope.bloc.add(const AuthBootRequested());
    await pumpAtSizes(
      tester,
      () => appScope(child: const BootGate(), auth: scope.bloc),
      finds: [find.text('Cannot reach the panel')],
    );
    expect(scope.bloc.state.stage, AppStage.unreachable);
  });

  testWidgets('a transport failure is shown under a sentence, not as one', (
    tester,
  ) async {
    final scope = authScope()
      ..api.setupError = PanelUnreachable(_handshakeNoise);
    scope.bloc.add(const AuthBootRequested());
    await tester.pumpWidget(
      appScope(child: const BootGate(), auth: scope.bloc),
    );
    await pumpEntrance(tester);

    expect(scope.bloc.state.stage, AppStage.unreachable);
    expect(
      find.textContaining('Nothing answered at the configured address'),
      findsOneWidget,
    );
    final detail = tester.widget<Text>(
      find.textContaining('HandshakeException'),
    );
    expect(detail.maxLines, 3);
    expect(detail.overflow, TextOverflow.ellipsis);
    // A handshake failure names a fix: the run flag that trusts the dev
    // certificate, since the dev stack's certificate is issued for localhost
    // and the emulator dials 10.0.2.2.
    expect(find.textContaining('DEV_ALLOW_ANY_CERT'), findsOneWidget);
  });

  testWidgets('the empty panel sends the reader to the web dashboard', (
    tester,
  ) async {
    final scope = authScope()
      ..api.setupStatus = SetupStatus(ownerPresent: false);
    scope.bloc.add(const AuthBootRequested());
    await pumpAtSizes(
      tester,
      () => appScope(child: const BootGate(), auth: scope.bloc),
      finds: [find.text('This panel has no owner yet')],
    );
    expect(scope.bloc.state.stage, AppStage.needsSetup);
  });

  testWidgets('trying again reaches the panel once it answers', (tester) async {
    final scope = authScope()
      ..api.setupError = PanelUnreachable(_handshakeNoise);
    scope.bloc.add(const AuthBootRequested());
    await tester.pumpWidget(
      appScope(child: const BootGate(), auth: scope.bloc),
    );
    await pumpEntrance(tester);

    scope.api.setupError = null;
    await tester.tap(find.text('Try again'));
    await pumpEntrance(tester);

    expect(scope.bloc.state.stage, AppStage.signedOut);
    expect(find.byType(SignInScreen), findsOneWidget);
  });
}
