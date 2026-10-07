import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/src/gen/openoxide/api/v1/auth.pb.dart' show SetupStatus;
import 'package:mobile/src/grpc/app_error.dart';
import 'package:mobile/src/state/auth_controller.dart';
import 'package:mobile/src/ui/boot_screen.dart';
import 'package:mobile/src/ui/sign_in_screen.dart';
import 'package:mobile/src/ui/tokens.dart';

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
    await scope.controller.boot();
    expect(scope.controller.stage, AppStage.unreachable);

    await pumpAtSizes(
      tester,
      () => appScope(
        child: BootScreen(controller: scope.controller),
        auth: scope.controller,
      ),
      finds: [find.text('Cannot reach the panel')],
    );
  });

  testWidgets('a transport failure is shown under a sentence, not as one', (
    tester,
  ) async {
    final scope = authScope()..api.setupError = PanelUnreachable(_handshakeNoise);
    await scope.controller.boot();
    await tester.pumpWidget(
      appScope(
        child: BootScreen(controller: scope.controller),
        auth: scope.controller,
      ),
    );
    await pumpEntrance(tester);

    expect(
      find.textContaining('Nothing answered at the configured address'),
      findsOneWidget,
    );
    final detail = tester.widget<Text>(find.textContaining('HandshakeException'));
    expect(detail.maxLines, 3);
    expect(detail.overflow, TextOverflow.ellipsis);
  });

  testWidgets('the empty panel sends the reader to the web dashboard', (
    tester,
  ) async {
    final scope = authScope()..api.setupStatus = SetupStatus(ownerPresent: false);
    await scope.controller.boot();
    expect(scope.controller.stage, AppStage.needsSetup);

    await pumpAtSizes(
      tester,
      () => appScope(
        child: BootScreen(controller: scope.controller),
        auth: scope.controller,
      ),
      finds: [find.text('This panel has no owner yet')],
    );
  });

  testWidgets('trying again reaches the panel once it answers', (tester) async {
    final scope = authScope()..api.setupError = PanelUnreachable(_handshakeNoise);
    await scope.controller.boot();
    await tester.pumpWidget(
      appScope(
        child: BootScreen(controller: scope.controller),
        auth: scope.controller,
      ),
    );
    await pumpEntrance(tester);

    scope.api.setupError = null;
    await tester.tap(find.text('Try again'));
    await pumpEntrance(tester);

    expect(scope.controller.stage, AppStage.signedOut);
    expect(find.byType(SignInScreen), findsOneWidget);
  });
}
